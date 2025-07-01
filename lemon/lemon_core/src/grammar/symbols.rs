use serde::{Serialize, Deserialize};

/// Representa símbolos puntuales (delimitadores y operadores simples)
/// que no forman parte de palabras clave ni expresiones complejas.
///
/// Se usan frecuentemente en la sintaxis del lenguaje para delimitar bloques,
/// listas, argumentos, agrupaciones, etc.
#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
pub enum Symbol {
    /// Símbolo de definición o asignación: `=`
    Define,

    /// Paréntesis izquierdo: `(`
    OpenParen,
    /// Paréntesis derecho: `)`
    CloseParen,

    /// Llave izquierda: `{`
    OpenBrace,
    /// Llave derecha: `}`
    CloseBrace,

    /// Corchete izquierdo: `[`
    OpenBraket,
    /// Corchete derecho: `]`
    CloseBraket,

    /// Punto y coma: `;`
    Semicolon,

    /// Coma: `,`
    Comma,

    /// Punto: `.`
    Dot,

    /// Signo de interrogación: `?`
    Question,

    /// Dos puntos: `:`
    Colon,
}

impl Symbol {
    /// Intenta convertir un carácter (`char`) en un símbolo reconocido por Yuka.
    ///
    /// Esta función se usa típicamente durante el análisis léxico.
    ///
    /// # Ejemplo
    /// ```
    /// assert_eq!(Symbol::from_char('('), Some(Symbol::OpenParen));
    /// assert_eq!(Symbol::from_char('x'), None);
    /// ```
    ///
    /// # Parámetros
    /// - `c`: Carácter que se desea interpretar.
    ///
    /// # Retorna
    /// - `Some(Symbol)` si el carácter corresponde a un símbolo conocido.
    /// - `None` si no coincide con ningún símbolo válido.
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '=' => Some(Self::Define),
            '(' => Some(Self::OpenParen),
            ')' => Some(Self::CloseParen),
            '{' => Some(Self::OpenBrace),
            '}' => Some(Self::CloseBrace),
            '[' => Some(Self::OpenBraket),
            ']' => Some(Self::CloseBraket),
            ';' => Some(Self::Semicolon),
            ',' => Some(Self::Comma),
            '.' => Some(Self::Dot),
            '?' => Some(Self::Question),
            ':' => Some(Self::Colon),
            _ => None, // Carácter no reconocido como símbolo
        }
    }

    /// Convierte el símbolo interno en su representación como carácter (`char`).
    ///
    /// # Ejemplo
    /// ```
    /// assert_eq!(Symbol::OpenBrace.to_char(), '{');
    /// ```
    pub fn to_char(&self) -> char {
        match self {
            Self::Define        => '=',
            Self::OpenParen     => '(',
            Self::CloseParen    => ')',
            Self::OpenBrace     => '{',
            Self::CloseBrace    => '}',
            Self::OpenBraket    => '[',
            Self::CloseBraket   => ']',
            Self::Semicolon     => ';',
            Self::Comma         => ',',
            Self::Dot           => '.',
            Self::Question      => '?',
            Self::Colon         => ':',
        }
    }
}
