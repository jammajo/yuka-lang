use crate::grammar::keywords::Keyword;
use crate::grammar::symbols::Symbol;
use crate::grammar::operators::Operator;
use crate::grammar::comparators::Comparator;
use crate::grammar::logicals::Logical;
use crate::grammar::types::Type;
use crate::grammar::comments::Comment;


#[derive(Debug, Clone, PartialEq)]
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
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}
