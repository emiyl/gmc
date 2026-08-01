#[derive(Debug, Clone)]
pub enum Statement {
    Assignment {
        name: String,
        value: Expr,
    },
    If {
        condition: Expr,
        then_branch: Box<Vec<Statement>>,
        else_branch: Option<Box<Vec<Statement>>>,
    },
    While {
        condition: Expr,
        body: Box<Vec<Statement>>,
    },
    Repeat {
        count: Expr,
        body: Box<Vec<Statement>>,
    },
    For {
        init: Option<Box<Statement>>,
        condition: Option<Expr>,
        update: Option<Box<Statement>>,
        body: Box<Vec<Statement>>,
    },
    DoUntil {
        body: Box<Vec<Statement>>,
        condition: Expr,
    },
    Switch {
        value: Expr,
        cases: Vec<(Expr, Vec<Statement>)>,
        default: Option<Vec<Statement>>,
    },
    Break,
    Continue,
    Return(Option<Expr>),
    Expression(Expr),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Integer(i32),
    String(String),
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

    Ternary {
        condition: Box<Expr>,
        then_expr: Box<Expr>,
        else_expr: Box<Expr>,
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

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Neg,
    Not,
}
