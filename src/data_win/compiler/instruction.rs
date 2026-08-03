use super::ast::BinaryOp;
use super::bytecode::Opcode;
use super::resolver::{Function, Variable};
use num_enum::TryFromPrimitive;

#[derive(Debug, Clone, Copy, TryFromPrimitive, PartialEq)]
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

#[derive(Debug, TryFromPrimitive)]
#[repr(u16)]
pub enum CmpType {
    None = 0x000,
    Lt = 0x100,
    Lte = 0x200,
    Eq = 0x300,
    Neq = 0x400,
    Gte = 0x500,
    Gt = 0x600,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BranchType {
    Unconditional,
    True,
    False,
}

#[derive(Debug)]
pub enum Instruction {
    PushI(i32),
    PushI32(i32),
    PushF(f32),
    PushD(f64),
    PushFunc(Function),
    PushE(i16),
    PushBool(bool),
    PushS(String),
    Push(Variable),

    Branch(i32, BranchType),

    Pop {
        variable: Variable,
        dst_type: ValueType,
        src_type: ValueType,
    },

    BinaryOp {
        lhs_type: ValueType,
        binary_op: BinaryOp,
        rhs_type: ValueType,
    },

    UnaryOp {
        operand_type: ValueType,
        opcode: Opcode,
    },

    Conv {
        from: ValueType,
        to: ValueType,
    },

    Call {
        function: Function,
        args_len: usize,
    },

    Break(i16),
    Dup(ValueType),

    Ret(ValueType),
    Exit,
    PopZ,
}
