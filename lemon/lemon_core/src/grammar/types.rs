#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    String,
    Bool,
    Null,
    List,
    Matrix,
    Map,
}

impl Type {
    pub fn from_str(s: &str) -> Option<Type> {
        match s.to_lowercase().as_str() {
            "int" => Some(Type::Int),
            "float" => Some(Type::Float),
            "string" => Some(Type::String),
            "bool" => Some(Type::Bool),
            "null" => Some(Type::Null),
            "list" => Some(Type::List),
            "matrix" => Some(Type::Matrix),
            "map" => Some(Type::Map),
            _ => None,
        }
    }
}
