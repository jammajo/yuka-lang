#[derive(Debug, Clone)]
pub enum Expression {
    Number(f64),
    String(String),
    Boolean(bool),
    Variable(String),
    Binary {
        left: Box<Expression>,
        op: BinaryOp,
        right: Box<Expression>,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expression>,
    },
    Call {
        function: Box<Expression>,
        args: Vec<Expression>,
    },
    Grouping(Box<Expression>),
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Neq,
    Gt,
    Lt,
    Gte,
    Lte,
    And,
    Or,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Not,
    Neg,
}
