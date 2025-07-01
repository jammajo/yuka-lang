use serde::{Serialize, Deserialize};

/// Representa los comparadores relacionales disponibles en el lenguaje Yuka.
///
/// Estos operadores se utilizan para evaluar relaciones entre valores en expresiones condicionales.
#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
pub enum Comparator {
    /// Igualdad: `==`
    Equal,
    /// Desigualdad: `!=`
    NotEqual,
    /// Mayor que: `>`
    Greater,
    /// Menor que: `<`
    Less,
    /// Mayor o igual que: `>=`
    GreaterEqual,
    /// Menor o igual que: `<=`
    LessEqual,
}

impl Comparator {
    /// Intenta construir un `Comparator` a partir de una cadena.
    ///
    /// Esta función se utiliza generalmente durante el análisis léxico
    /// para traducir el texto fuente (`==`, `!=`, etc.) a su representación interna.
    ///
    /// # Ejemplo
    /// ```
    /// assert_eq!(Comparator::from_str(">="), Some(Comparator::GreaterEqual));
    /// ```
    ///
    /// # Parámetros
    /// - `s`: La cadena que representa un comparador.
    ///
    /// # Retorna
    /// - `Some(Comparator)` si la cadena coincide con un comparador conocido.
    /// - `None` si no coincide con ningún operador válido.
    pub fn from_str(s: &str) -> Option<Comparator> {
        match s {
            "==" => Some(Comparator::Equal),
            "!=" => Some(Comparator::NotEqual),
            ">"  => Some(Comparator::Greater),
            "<"  => Some(Comparator::Less),
            ">=" => Some(Comparator::GreaterEqual),
            "<=" => Some(Comparator::LessEqual),
            _    => None, // No coincide con ningún comparador válido
        }
    }
}
