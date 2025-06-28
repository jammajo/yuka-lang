#[derive(Debug, Clone, PartialEq)]
pub enum Comparator {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

impl Comparator {
    pub fn from_str(s: &str) -> Option<Comparator> {
        match s {
            "==" => Some(Comparator::Equal),
            "!=" => Some(Comparator::NotEqual),
            ">" => Some(Comparator::GreaterThan),
            "<" => Some(Comparator::LessThan),
            ">=" => Some(Comparator::GreaterThanOrEqual),
            "<=" => Some(Comparator::LessThanOrEqual),
            _ => None,
        }
    }
}