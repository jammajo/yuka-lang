// === Módulos públicos y privados ===

pub mod lexer;      // Analizador léxico (tokenizer)
mod parser;         // Analizador sintáctico
mod ast;            // Árbol de sintaxis abstracta
mod error;          // Tipos de errores personalizados
mod eval;           // Evaluador (en desarrollo)
pub mod token;      // Definiciones de tokens
pub mod grammar;    // Gramática general (keywords, símbolos, tipos, etc.)

// === Imports ===

use error::LemonError;
//use token::Token;
use ast::statements::Statement;
use parser::Parser;

// === Punto de entrada ===

/// Interpreta el código fuente completo.
/// Ejecuta las fases de tokenización, parsing y evaluación.
pub fn interpret(code: &str) -> Result<Vec<Statement>, LemonError> {
    // 1. Tokenización
    let tokens = lexer::tokenize(code)?; // Devuelve Vec<Token> o error léxico

    // 2. Parsing
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().map_err(|msg| LemonError::new(&msg, 0, 0))?;

    // 3. (Opcional) Evaluación
    // let result = eval::evaluate(&ast)?;  // Solo si implementas el evaluador

    Ok(ast) // Devuelve el AST por ahora
}
