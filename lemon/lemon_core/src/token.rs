pub use crate::grammar::*;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum TokenType {
    Identifier,
    Number,
    Keyword(Keyword),
    Symbol(Symbol),
    Unknown,
    Operator(Operator),
    Comparator(Comparator),
    Logical(Logical),
    Type(Type),
    Comment(Comment),
    StringLiteral,
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub line: usize,
    pub column: usize,
}
