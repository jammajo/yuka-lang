//let symbol: Symbol;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Symbol{
    Define,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenBraket,
    CloseBraket,
    Semicolon,
    Comma,
    Point,
}

impl Symbol {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '=' => Some(Self::Define),
            '(' => Some(Self::OpenParen),
            ')' => Some(Self::CloseParen),
            '{' => Some(Self::OpenBrace),
            '}' => Some(Self::CloseBrace),
            '[' => Some(Self::OpenBraket),
            ']' => Some(Self::CloseBraket),
            ';' => Some(Self::Semicolon),
            ',' => Some(Self::Comma),
            '.' => Some(Self::Point),
            _ => None,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Self::Define => '=',
            Self::OpenParen => '(',
            Self::CloseParen => ')',
            Self::OpenBrace => '{',
            Self::CloseBrace => '}',
            Self::OpenBraket => '[',
            Self::CloseBraket => ']',
            Self::Semicolon => ';',
            Self::Comma => ',',
            Self::Point => '.',
        }
    }
}

