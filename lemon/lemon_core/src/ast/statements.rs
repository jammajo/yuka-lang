use crate::ast::expressions::Expression;

#[derive(Debug, Clone)]
pub enum Statement {
    /// let x = expr;
    Let {
        name: String,
        value: Expression,
    },

    /// fun name(params) { body }
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<Statement>,
    },

    /// if (cond) { then_branch } else { else_branch }
    If {
        condition: Expression,
        then_branch: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
    },

    /// while (cond) { body }
    While {
        condition: Expression,
        body: Vec<Statement>,
    },

    /// do { body } while (cond);
    DoWhile {
        body: Vec<Statement>,
        condition: Expression,
    },

    /// return expr;
    Return(Option<Expression>),

    /// expr;
    Expr(Expression),

    /// { ... }
    Block(Vec<Statement>),
}
