use super::expressions::Expression;

#[derive(Debug, Clone)]
pub enum Statement {
    Let(String, Expression),
    Function(String, Vec<String>, Vec<Statement>),
    If {
        condition: Expression,
        then_branch: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    DoWhile {
        body: Vec<Statement>,
        condition: Expression,
    },
    Return(Expression),
    Expr(Expression),
    Block(Vec<Statement>),
}
