use std::fmt;
use serde::{Serialize, Deserialize};

/// Representa los operadores aritméticos básicos del lenguaje Yuka.
///
/// Estos operadores se utilizan para realizar cálculos numéricos en expresiones.
#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
pub enum Operator {
    /// Suma: `+`
    Add,
    /// Resta: `-`
    Subtract,
    /// Multiplicación: `*`
    Multiply,
    /// División: `/`
    Divide,
    /// Módulo o residuo: `%`
    Modulus,
    /// Potenciación: `^`
    Power,
}

impl Operator {
    /// Intenta convertir una cadena en un operador aritmético válido.
    ///
    /// Esta función es útil durante el análisis léxico para traducir símbolos
    /// como `"+"` o `"*"` a sus equivalentes en el enum `Operator`.
    ///
    /// # Ejemplo
    /// ```
    /// assert_eq!(Operator::from_str("*"), Some(Operator::Multiply));
    /// ```
    ///
    /// # Parámetros
    /// - `s`: Una cadena que representa un símbolo aritmético.
    ///
    /// # Retorna
    /// - `Some(Operator)` si la cadena representa un operador reconocido.
    /// - `None` si no coincide con ningún operador válido.
    pub fn from_str(s: &str) -> Option<Operator> {
        match s {
            "+" => Some(Operator::Add),
            "-" => Some(Operator::Subtract),
            "*" => Some(Operator::Multiply),
            "/" => Some(Operator::Divide),
            "%" => Some(Operator::Modulus),
            "^" => Some(Operator::Power),
            _   => None,
        }
    }
}

impl fmt::Display for Operator {
    /// Devuelve la representación textual (símbolo) del operador aritmético.
    ///
    /// Esto permite imprimir directamente un `Operator` con su símbolo correspondiente.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match self {
            Operator::Add      => "+",
            Operator::Subtract => "-",
            Operator::Multiply => "*",
            Operator::Divide   => "/",
            Operator::Modulus  => "%",
            Operator::Power    => "^",
        };
        write!(f, "{}", symbol)
    }
}
