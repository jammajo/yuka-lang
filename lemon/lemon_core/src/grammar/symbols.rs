//let symbol: Symbol;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Symbol{
    Equal,
    Plus,
    Minus,
    Multiply,
    Divide,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Semicolon,
    Comma,
    Point,
}

impl Symbol {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '=' => Some(Self::Equal),
            '+' => Some(Self::Plus),
            '-' => Some(Self::Minus),
            '*' => Some(Self::Multiply),
            '/' => Some(Self::Divide),
            '(' => Some(Self::OpenParen),
            ')' => Some(Self::CloseParen),
            '{' => Some(Self::OpenBrace),
            '}' => Some(Self::CloseBrace),
            ';' => Some(Self::Semicolon),
            ',' => Some(Self::Comma),
            '.' => Some(Self::Point),
            _ => None,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Self::Equal => '=',
            Self::Plus => '+',
            Self::Minus => '-',
            Self::Multiply => '*',
            Self::Divide => '/',
            Self::OpenParen => '(',
            Self::CloseParen => ')',
            Self::OpenBrace => '{',
            Self::CloseBrace => '}',
            Self::Semicolon => ';',
            Self::Comma => ',',
            Self::Point => '.',
        }
    }
}

