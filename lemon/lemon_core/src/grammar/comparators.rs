use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
pub enum Comparator {
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
}

impl Comparator {
    pub fn from_str(s: &str) -> Option<Comparator> {
        match s {
            "==" => Some(Comparator::Equal),
            "!=" => Some(Comparator::NotEqual),
            ">" => Some(Comparator::Greater),
            "<" => Some(Comparator::Less),
            ">=" => Some(Comparator::GreaterEqual),
            "<=" => Some(Comparator::LessEqual),
            _ => None,
        }
    }
}
