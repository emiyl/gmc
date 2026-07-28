#[derive(Debug)]
pub enum Expr {
    Integer(i32),
    Variable(String),
    Call {
        name: String,
        args: Vec<Expr>,
    },

    Binary {
        left: Box<Expr>,
        operator: BinaryOp,
        right: Box<Expr>,
    },

    Unary {
        operator: UnaryOp,
        operand: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Mul,
    Div,
    Rem,
    Mod,
    Add,
    Sub,
    And,
    Or,
    Xor,
    Shl,
    Shr,
    Lt,
    Lte,
    Eq,
    Neq,
    Gte,
    Gt,
}

#[derive(Debug)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Debug)]
pub enum Statement {
    Assignment { name: String, value: Expr },
    Expression(Expr),
}
