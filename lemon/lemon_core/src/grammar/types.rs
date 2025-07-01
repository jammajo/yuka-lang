use serde::{Serialize, Deserialize};

/// Enum que representa los tipos de datos admitidos por el lenguaje Yuka.
///
/// Estos tipos se pueden usar para anotaciones, inferencias, validaciones,
/// conversiones o chequeo de tipos en tiempo de interpretación o compilación.
#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
pub enum Type {
    /// Número entero: `int`
    Int,

    /// Número decimal: `float`
    Float,

    /// Cadena de texto: `string`
    String,

    /// Valor booleano: `bool` (`true` o `false`)
    Bool,

    /// Valor nulo o ausente: `null`
    Null,

    /// Lista (colección ordenada de elementos): `list`
    List,

    /// Matriz bidimensional: `matrix`
    Matrix,

    /// Mapa clave-valor (diccionario): `map`
    Map,
}

impl Type {
    /// Convierte una cadena en un tipo válido del lenguaje Yuka.
    ///
    /// Acepta nombres en minúscula (insensible a mayúsculas).
    ///
    /// # Ejemplo
    /// ```
    /// assert_eq!(Type::from_str("Float"), Some(Type::Float));
    /// assert_eq!(Type::from_str("map"), Some(Type::Map));
    /// assert_eq!(Type::from_str("char"), None); // Tipo no reconocido
    /// ```
    ///
    /// # Parámetros
    /// - `s`: La cadena que representa el nombre del tipo.
    ///
    /// # Retorna
    /// `Some(Type)` si se reconoce como tipo válido, `None` en caso contrario.
    pub fn from_str(s: &str) -> Option<Type> {
        match s.to_lowercase().as_str() {
            "int"    => Some(Type::Int),
            "float"  => Some(Type::Float),
            "string" => Some(Type::String),
            "bool"   => Some(Type::Bool),
            "null"   => Some(Type::Null),
            "list"   => Some(Type::List),
            "matrix" => Some(Type::Matrix),
            "map"    => Some(Type::Map),
            _        => None, // Tipo no reconocido
        }
    }
}
