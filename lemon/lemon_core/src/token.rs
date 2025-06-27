#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Identifier,
    Number,
    Keyword,
    Symbol(char),
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}
