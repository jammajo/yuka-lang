pub mod lexer; 
mod parser; 
mod ast; 
mod error; 
mod eval; 
pub mod token;
pub mod grammar;

use error::LemonError;
use crate::token::Token;

pub fn interpret(code: &str) -> Result<Vec<Token>, LemonError> {
    let tokens = lexer::tokenize(code)?;
    Ok(tokens)
}