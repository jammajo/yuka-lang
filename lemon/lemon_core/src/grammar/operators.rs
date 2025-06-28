#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Power,
}

impl Operator {
    pub fn from_str(s: &str) -> Option<Operator> {
        match s {
            "+" => Some(Operator::Add),
            "-" => Some(Operator::Subtract),
            "*" => Some(Operator::Multiply),
            "/" => Some(Operator::Divide),
            "%" => Some(Operator::Modulus),
            "^" => Some(Operator::Power),
            _ => None,
        }
    }
}
