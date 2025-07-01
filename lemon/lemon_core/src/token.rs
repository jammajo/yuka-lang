// ============================
// token.rs — Definición de tokens
// ============================

use crate::grammar::*; // Importa enums de palabras clave, operadores, símbolos, etc.
use std::fmt;          // Para implementar fmt::Display
use serde::{Serialize, Deserialize}; // (opcional) Para serializar a JSON o similar

// ============================
// 🔠 Tipos de token reconocidos
// ============================

#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
pub enum TokenType {
    Identifier,                     // Identificadores de variables, funciones, etc.
    Number,                         // Números (int o float, se decide en el parser)
    Keyword(Keyword),               // Palabras clave como let, if, while...
    Symbol(Symbol),                 // Símbolos como (, ), {, }, =, ;, etc.
    Unknown,                        // Token no reconocido (error léxico)
    Operator(Operator),             // Operadores aritméticos: +, -, *, /
    Comparator(Comparator),         // Comparadores: ==, !=, <, >
    Logical(Logical),               // Lógicos: and, or, !
    Type(Type),                     // Tipos de dato: string, bool, number, etc.
    Comment(Comment),               // Comentarios: //, /* */
    StringLiteral,                  // Cadenas de texto: "hola mundo"
    EOF,                            // Fin del archivo fuente
}

// ============================
// 📦 Representación completa de un token
// ============================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Token {
    pub token_type: TokenType,  // El tipo de token (Keyword, Operator, etc.)
    pub value: String,          // Texto exacto del código fuente
    pub line: usize,            // Línea en el código fuente
    pub column: usize,          // Columna (posición horizontal)
}

// ============================
// 🖨️ Visualización amigable en consola
// ============================

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

// ============================
// 🧾 Agrupa tokens por línea (útil para debugging visual)
// ============================

pub fn print_tokens_by_line(tokens: &[Token]) {
    let mut current_line = 0;

    for token in tokens {
        if token.line != current_line {
            current_line = token.line;
            println!("\nLínea {}:", current_line);
        }

        println!("  {}", token); // Usa Display implementado arriba
    }
}
