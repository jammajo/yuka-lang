use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
pub enum Logical {
    And,
    Or,
    Not,
}

impl Logical {
    pub fn from_str(s: &str) -> Option<Logical> {
        match s.to_lowercase().as_str() {
            "and" => Some(Logical::And),
            "&&" => Some(Logical::And),
            "or" => Some(Logical::Or),
            "||" => Some(Logical::Or),
            "not" => Some(Logical::Not),
            "!" => Some(Logical::Not),
            _ => None,
        }
    }
}

impl fmt::Display for Logical {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Logical::And => "&&",
            Logical::Or => "||",
            Logical::Not => "!",
        };
        write!(f, "{}", symbol)
    }
}
