#[derive(Debug)]
pub enum Expr {
    Integer(i32),
    Variable(String),

    Binary {
        left: Box<Expr>,
        operator: BinaryOp,
        right: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
}

#[derive(Debug)]
pub enum Statement {
    Assignment { name: String, value: Expr },
}
