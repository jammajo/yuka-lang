#[derive(Debug, Clone, PartialEq, Copy)]
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
