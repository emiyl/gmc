use super::ast::BinaryOp;
use super::bytecode::*;
use super::instruction::*;
use super::resolver::Variable;

#[derive(Debug)]
pub struct Word {
    pub opcode: u8,
    pub instr_type1: u8,
    pub instr_type2: u8,
    pub instr_instance_type: u16,
}

fn encode_variable(variable: &Variable) -> u16 {
    let name = &variable.name;
    let reference = if name.starts_with("global.") {
        0xFFFB
    } else if name.starts_with("self.") {
        0xFFFF
    } else {
        0xFFFA
    };
    reference
}

impl Word {
    pub fn new(opcode: u8, instr_type1: u8, instr_type2: u8, instr_instance_type: u16) -> Self {
        Self {
            opcode,
            instr_type1,
            instr_type2,
            instr_instance_type,
        }
    }

    pub fn from_u32(value: u32) -> Self {
        let opcode = ((value >> 24) & 0xFF) as u8;
        let instr_type1 = ((value >> 20) & 0xF) as u8;
        let instr_type2 = ((value >> 16) & 0xF) as u8;
        let instr_instance_type = (value & 0xFFFF) as u16;

        Self {
            opcode,
            instr_type1,
            instr_type2,
            instr_instance_type,
        }
    }

    pub fn to_u32(&self) -> u32 {
        let opcode = (self.opcode as u32) << 24;
        let instr_type1 = (self.instr_type1 as u32) << 16;
        let instr_type2 = (self.instr_type2 as u32) << 20;
        let instr_instance_type = self.instr_instance_type as u32;
        opcode | instr_type1 | instr_type2 | instr_instance_type
    }
}

fn instruction_word_len(instruction: &Instruction) -> usize {
    match instruction {
        Instruction::PushI(_) => 1,
        Instruction::PushE(_) => 1,
        Instruction::PushS(_) => 2,
        Instruction::Push(_) => 2,
        Instruction::Branch(_, _) => 1,
        Instruction::Pop { .. } => 2,
        Instruction::BinaryOp { .. } => 1,
        Instruction::UnaryOp { .. } => 1,
        Instruction::Conv { .. } => 1,
        Instruction::Call { .. } => 2,
        Instruction::Break(_) => 1,
        Instruction::Ret(_) => 1,
        Instruction::Exit => 1,
        Instruction::PopZ => 1,
    }
}

fn encode_branch_word(opcode: Opcode, rel_words: i32) -> u32 {
    let min = -(1 << 22);
    let max = (1 << 22) - 1;
    assert!(
        rel_words >= min && rel_words <= max,
        "branch offset out of range: {}",
        rel_words
    );

    let offset_bits = (rel_words as u32) & 0x007F_FFFF;
    ((opcode as u32) << 24) | offset_bits
}

pub fn encode(instructions: Vec<Instruction>) -> Bytecode {
    let mut output = Bytecode::new();

    let mut instruction_word_starts = Vec::with_capacity(instructions.len() + 1);
    let mut cursor = 0usize;
    instruction_word_starts.push(cursor);
    for instruction in &instructions {
        cursor += instruction_word_len(instruction);
        instruction_word_starts.push(cursor);
    }

    for (instruction_index, instr) in instructions.into_iter().enumerate() {
        match instr {
            Instruction::PushI(value) => {
                let opcode = Opcode::PushI;
                let instr_type1 = ValueType::Int16 as u8;
                let instr_type2 = ValueType::Double as u8;

                let value_bytes = (value as u16).to_le_bytes();
                let value_u16 = u16::from_le_bytes(value_bytes);

                let word = Word::new(opcode as u8, instr_type1, instr_type2, value_u16).to_u32();
                output.write_u32(word);
            }

            Instruction::PushE(value) => {
                let opcode = Opcode::Push as u16;
                let instr_type1 = ValueType::Int16 as u8;
                let instr_type2 = ValueType::Double as u8;

                let value_bytes = value.to_le_bytes();
                let value_u16 = u16::from_le_bytes(value_bytes);

                let word = Word::new(opcode as u8, instr_type1, instr_type2, value_u16).to_u32();
                output.write_u32(word);
            }

            Instruction::PushS(value) => {
                let opcode = Opcode::Push as u16;
                let type1 = ValueType::String as u8;
                let type2 = ValueType::Double as u8;
                let word = Word::new(opcode as u8, type1, type2, 0).to_u32();
                output.write_u32(word);

                let patch_pos = output.data.len();
                output.write_u32(0);
                output.string_fixups.push((patch_pos, value));
            }

            Instruction::Push(var) => {
                let opcode = Opcode::Push as u16;
                let type1 = ValueType::Var as u8;
                let type2 = ValueType::Double as u8;
                let vari = encode_variable(&var);
                let word = Word::new(opcode as u8, type1, type2, vari).to_u32();
                output.write_u32(word);
                output.write_u32(var.var_ref);
            }

            Instruction::Branch(offset, branch_type) => {
                let opcode = match branch_type {
                    BranchType::Unconditional => Opcode::Branch,
                    BranchType::True => Opcode::BranchTrue,
                    BranchType::False => Opcode::BranchFalse,
                };

                let target_instruction_index = instruction_index as i32 + offset;
                assert!(
                    target_instruction_index >= 0
                        && target_instruction_index <= instruction_word_starts.len() as i32 - 1,
                    "invalid branch target index: {}",
                    target_instruction_index
                );

                let current_word_index = instruction_word_starts[instruction_index] as i32;
                let target_word_index =
                    instruction_word_starts[target_instruction_index as usize] as i32;
                let rel_words = target_word_index - current_word_index;

                let word = encode_branch_word(opcode, rel_words);
                output.write_u32(word);
            }

            Instruction::BinaryOp {
                lhs_type,
                binary_op,
                rhs_type,
            } => {
                let opcode = match binary_op {
                    BinaryOp::Mul => Opcode::Mul,
                    BinaryOp::Div => Opcode::Div,
                    BinaryOp::Rem => Opcode::Rem,
                    BinaryOp::Mod => Opcode::Mod,
                    BinaryOp::Add => Opcode::Add,
                    BinaryOp::Sub => Opcode::Sub,
                    BinaryOp::And => Opcode::And,
                    BinaryOp::Or => Opcode::Or,
                    BinaryOp::Xor => Opcode::Xor,
                    BinaryOp::Shl => Opcode::Shl,
                    BinaryOp::Shr => Opcode::Shr,
                    BinaryOp::Lt => Opcode::Cmp,
                    BinaryOp::Lte => Opcode::Cmp,
                    BinaryOp::Eq => Opcode::Cmp,
                    BinaryOp::Neq => Opcode::Cmp,
                    BinaryOp::Gte => Opcode::Cmp,
                    BinaryOp::Gt => Opcode::Cmp,
                };
                let instr_type1 = rhs_type as u8;
                let instr_type2 = lhs_type as u8;
                let instr_instance_type = match binary_op {
                    BinaryOp::Lt => CmpType::Lt,
                    BinaryOp::Lte => CmpType::Lte,
                    BinaryOp::Eq => CmpType::Eq,
                    BinaryOp::Neq => CmpType::Neq,
                    BinaryOp::Gte => CmpType::Gte,
                    BinaryOp::Gt => CmpType::Gt,
                    _ => CmpType::None,
                } as u16;
                let word =
                    Word::new(opcode as u8, instr_type1, instr_type2, instr_instance_type).to_u32();
                output.write_u32(word);
            }

            Instruction::UnaryOp {
                operand_type,
                opcode,
            } => {
                let opcode = opcode;
                let instr_type1 = operand_type as u8;
                let instr_type2 = 0;
                let word = Word::new(opcode as u8, instr_type1, instr_type2, 0).to_u32();
                output.write_u32(word);
            }

            Instruction::Pop {
                variable,
                dst_type,
                src_type,
            } => {
                let opcode = Opcode::Pop as u16;
                let type1 = dst_type as u8;
                let type2 = src_type as u8;
                let vari = encode_variable(&variable);
                let word = Word::new(opcode as u8, type1, type2, vari).to_u32();
                output.write_u32(word);
                output.write_u32(variable.var_ref);
            }

            Instruction::Conv { from, to } => {
                let opcode = Opcode::Conv as u16;
                let type1 = from as u8;
                let type2 = to as u8;
                let word = Word::new(opcode as u8, type1, type2, 0).to_u32();
                output.write_u32(word);
            }

            Instruction::Call { function, args_len } => {
                let opcode = Opcode::Call as u16;
                let type1 = ValueType::Int32 as u8;
                let type2 = ValueType::Double as u8;
                let word = Word::new(opcode as u8, type1, type2, args_len as u16).to_u32();
                output.write_u32(word);
                output.write_u32(function.var_ref);
            }

            Instruction::Break(sub_opcode) => {
                let opcode = Opcode::Break as u16;
                let type1 = ValueType::Int16 as u8;
                let type2 = ValueType::Double as u8;
                let sub_opcode_u16 = u16::from_le_bytes(sub_opcode.to_le_bytes());
                let word = Word::new(opcode as u8, type1, type2, sub_opcode_u16).to_u32();
                output.write_u32(word);
            }

            Instruction::Ret(return_type) => {
                let opcode = Opcode::Ret as u16;
                let type1 = return_type as u8;
                let type2 = ValueType::Double as u8;
                let word = Word::new(opcode as u8, type1, type2, 0).to_u32();
                output.write_u32(word);
            }

            Instruction::Exit => {
                let opcode = Opcode::Exit as u16;
                let word = Word::new(opcode as u8, 0, 0, 0).to_u32();
                output.write_u32(word);
            }

            Instruction::PopZ => {
                let opcode = Opcode::PopZ as u16;
                let type1 = ValueType::Var as u8;
                let type2 = ValueType::Double as u8;
                let word = Word::new(opcode as u8, type1, type2, 0).to_u32();
                output.write_u32(word);
            }
        }
    }

    output
}
