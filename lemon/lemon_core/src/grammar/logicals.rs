use std::fmt;
use serde::{Serialize, Deserialize};

/// Enum que representa los operadores lógicos en el lenguaje Yuka.
///
/// Estos operadores se utilizan en expresiones booleanas para combinar o negar condiciones.
/// Se aceptan tanto formas simbólicas (`&&`, `||`, `!`) como verbales (`and`, `or`, `not`).
#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
pub enum Logical {
    /// Conjunción lógica: verdadero solo si ambos operandos son verdaderos.
    And,

    /// Disyunción lógica: verdadero si al menos uno de los operandos es verdadero.
    Or,

    /// Negación lógica: invierte el valor de verdad de una expresión booleana.
    Not,
}

impl Logical {
    /// Intenta convertir una cadena en un operador lógico válido.
    ///
    /// Acepta tanto las formas verbales (`"and"`, `"or"`, `"not"`) como las simbólicas (`"&&"`, `"||"`, `"!"`).
    ///
    /// # Ejemplo
    /// ```
    /// assert_eq!(Logical::from_str("and"), Some(Logical::And));
    /// assert_eq!(Logical::from_str("||"), Some(Logical::Or));
    /// ```
    ///
    /// # Parámetros
    /// - `s`: La cadena a interpretar como operador lógico.
    ///
    /// # Retorna
    /// - `Some(Logical)` si la cadena representa un operador válido.
    /// - `None` si no se reconoce.
    pub fn from_str(s: &str) -> Option<Logical> {
        match s.to_lowercase().as_str() {
            "and" => Some(Logical::And),
            "&&"  => Some(Logical::And),
            "or"  => Some(Logical::Or),
            "||"  => Some(Logical::Or),
            "not" => Some(Logical::Not),
            "!"   => Some(Logical::Not),
            _     => None,
        }
    }
}

impl fmt::Display for Logical {
    /// Devuelve la representación simbólica (`&&`, `||`, `!`) del operador lógico.
    ///
    /// Esto es útil para generar código, mostrar mensajes o para usar en pruebas de impresión.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Logical::And => "&&",
            Logical::Or  => "||",
            Logical::Not => "!",
        };
        write!(f, "{}", symbol)
    }
}
