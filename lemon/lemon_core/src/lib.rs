mod lexer;
mod parser;
mod ast;
mod error;
mod eval;
mod token;

use error::LemonError;

pub fn interpret(code: &str) -> Result<String, LemonError> {
    let tokens = lexer::tokenize(code)?;

    // (A futuro) Parsear los tokens y ejecutar el código
    // let ast = parser::parse(tokens)?;
    // let result = eval::evaluate(ast)?;

    // Por ahora, devolvemos los tokens como debug
    Ok(format!("{:?}", tokens))
}
