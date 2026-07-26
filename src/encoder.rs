use crate::bytecode::*;
use crate::instruction::*;
use crate::vari;

struct Word {
    opcode: u8,
    instr_type1: u8,
    instr_type2: u8,
    value: u16,
}

impl Word {
    pub fn new(opcode: u8, instr_type1: u8, instr_type2: u8, value: u16) -> Self {
        Self {
            opcode,
            instr_type1,
            instr_type2,
            value,
        }
    }

    pub fn to_u32(&self) -> u32 {
        let opcode = (self.opcode as u32) << 24;
        let instr_type1 = (self.instr_type1 as u32) << 16;
        let instr_type2 = (self.instr_type2 as u32) << 20;
        let value = self.value as u32;
        opcode | instr_type1 | instr_type2 | value
    }
}

pub fn encode(instructions: Vec<Instruction>) -> Bytecode {
    let mut output = Bytecode::new();

    for instr in instructions {
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

            Instruction::BinaryOp {
                lhs_type,
                opcode,
                rhs_type,
            } => {
                let opcode = opcode;
                let instr_type1 = lhs_type as u8;
                let instr_type2 = rhs_type as u8;
                let word = Word::new(opcode as u8, instr_type1, instr_type2, 0).to_u32();
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
                let vari = vari::encode_variable(&variable);
                let word = Word::new(opcode as u8, type1, type2, vari).to_u32();
                output.write_u32(word);
                output.write_u32(variable.var_ref);
            }

            Instruction::Push(var) => {
                let opcode = Opcode::Push as u16;
                let type1 = ValueType::Var as u8;
                let type2 = ValueType::Double as u8;
                let vari = vari::encode_variable(&var);
                let word = Word::new(opcode as u8, type1, type2, vari).to_u32();
                output.write_u32(word);
                output.write_u32(var.var_ref);
            }

            Instruction::Conv { from, to } => {
                let opcode = Opcode::Conv as u16;
                let type1 = from as u8;
                let type2 = to as u8;
                let word = Word::new(opcode as u8, type1, type2, 0).to_u32();
                output.write_u32(word);
            }

            Instruction::Call { function } => {
                let opcode = Opcode::Call as u16;
                let type1 = ValueType::Var as u8;
                let type2 = ValueType::Double as u8;
                let word = Word::new(opcode as u8, type1, type2, 0).to_u32();
                output.write_u32(word);
                output.write_u32(function.var_ref);

                // PopZ after this
                let popz_opcode = Opcode::PopZ as u16;
                let popz_type1 = ValueType::Var as u8;
                let popz_type2 = ValueType::Double as u8;
                let popz_word = Word::new(popz_opcode as u8, popz_type1, popz_type2, 0).to_u32();
                output.write_u32(popz_word);
            }
        }
    }

    output
}
