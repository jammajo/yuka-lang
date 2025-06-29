// token.rs

use crate::grammar::*;
use std::fmt;

use serde::{Serialize, Deserialize}; // <- si vas a usar JSON (opcional)

// Representa los distintos tipos de tokens
#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
pub enum TokenType {
    Identifier,
    Number,
    Keyword(Keyword),
    Symbol(Symbol),
    Unknown,
    Operator(Operator),
    Comparator(Comparator),
    Logical(Logical),
    Type(Type),
    Comment(Comment),
    StringLiteral,
    EOF,
}

// Representa un token individual con posición y valor
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub line: usize,
    pub column: usize,
}

// Display bonito para facilitar depuración en consola
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Linea {:<3} Col {:<3} | Tipo: {:<20} Valor: {:?}",
            self.line,
            self.column,
            format!("{:?}", self.token_type),
            self.value
        )
    }
}

// Función opcional para agrupar tokens por línea
pub fn print_tokens_by_line(tokens: &[Token]) {
    let mut current_line = 0;
    for token in tokens {
        if token.line != current_line {
            current_line = token.line;
            println!("\nLínea {}:", current_line);
        }
        println!("  {}", token); // Usa el Display de arriba
    }
}
