use crate::grammar::keywords::Keyword;
use crate::grammar::symbols::Symbol;


#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Identifier,
    Number,
    Keyword(Keyword),
    Symbol(Symbol),
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}
