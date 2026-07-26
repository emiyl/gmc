use crate::bytecode::Opcode;
use crate::resolver::Variable;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum ValueType {
    Double = 0x0,
    Float = 0x1,
    Int32 = 0x2,
    Int64 = 0x3,
    Bool = 0x4,
    Var = 0x5,
    String = 0x6,
    Int16 = 0xF,
}

#[derive(Debug)]
pub enum Instruction {
    PushI(i32),
    PushVar(Variable),

    Pop {
        variable: Variable,
        dst_type: ValueType,
        src_type: ValueType,
    },

    BinaryOp {
        lhs_type: ValueType,
        opcode: Opcode,
        rhs_type: ValueType,
    },

    Call {
        function: Variable,
    },
}
