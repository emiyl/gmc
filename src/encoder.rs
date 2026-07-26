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
                let instr_type1 = ValueType::F as u8;
                let instr_type2 = ValueType::None as u8;

                let value_bytes = (value as u16).to_le_bytes();
                let value_u16 = u16::from_le_bytes(value_bytes);

                let word = Word::new(opcode as u8, instr_type1, instr_type2, value_u16).to_u32();
                output.write_u32(word);
            }

            Instruction::Add { lhs_type, rhs_type } => {
                let opcode = Opcode::Add;
                let instr_type1 = lhs_type as u8;
                let instr_type2 = rhs_type as u8;
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

            Instruction::PushVar(var) => {
                let opcode = Opcode::PushVar as u16;
                let type1 = ValueType::Var as u8;
                let type2 = ValueType::None as u8;
                let vari = vari::encode_variable(&var);
                let word = Word::new(opcode as u8, type1, type2, vari).to_u32();
                output.write_u32(word);
                output.write_u32(var.var_ref);
            }

            _ => {
                println!("Unsupported instruction: {:?}", instr);
                output.write_u32(0);
            }
        }
    }

    output
}
