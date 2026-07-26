use crate::resolver::Variable;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum ValueType {
    None = 0x0,
    Int = 0x2,
    Var = 0x5,
    F = 0xF,
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

    Add {
        lhs_type: ValueType,
        rhs_type: ValueType,
    },
}
