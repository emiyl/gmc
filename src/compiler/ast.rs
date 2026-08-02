#[derive(Debug, Clone)]
pub struct FunctionParameter {
    pub name: String,
    pub default_value: Option<Expr>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Assignment {
        name: String,
        value: Expr,
    },
    /// `var x`, `var x = 5`, `var x = 1, y = 2`
    VarDeclaration {
        declarations: Vec<(String, Option<Expr>)>,
    },
    /// `globalvar score`
    GlobalVarDeclaration {
        name: String,
    },
    /// `static counter = 0`
    StaticDeclaration {
        name: String,
        value: Option<Expr>,
    },
    FunctionDeclaration {
        name: String,
        params: Vec<FunctionParameter>,
        body: Vec<Statement>,
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
pub enum CallArg {
    Positional(Expr),
    Named { name: String, value: Expr },
}

impl CallArg {
    pub fn expr(&self) -> &Expr {
        match self {
            CallArg::Positional(expr) => expr,
            CallArg::Named { value, .. } => value,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Integer(i32),
    String(String),
    Variable(String),

    StructLiteral(Vec<(String, Expr)>),

    MemberAccess {
        target: Box<Expr>,
        field: String,
    },

    Call {
        name: String,
        args: Vec<CallArg>,
    },

    Function {
        name: Option<String>,
        params: Vec<FunctionParameter>,
        body: Vec<Statement>,
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
