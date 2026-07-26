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
}

#[derive(Debug)]
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
}

#[derive(Debug)]
pub enum Statement {
    Assignment { name: String, value: Expr },
    Expression(Expr),
}
