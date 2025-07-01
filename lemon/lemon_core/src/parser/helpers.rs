use crate::token::{Token, TokenType};
use crate::grammar::*;
use crate::ast::{BinaryOp, UnaryOp};
use crate::parser::Parser;

/// Determina si un token representa un identificador válido.
/// Un identificador es típicamente un nombre de variable, función, etc.
pub fn is_identifier(token: &Token) -> bool {
    matches!(token.token_type, TokenType::Identifier)
}

/// Verifica si un token representa un valor literal en el lenguaje Yuka.
/// Los literales son: booleanos (`true`, `false`), números, strings y `null`.
pub fn is_literal(token: &Token) -> bool {
    matches!(
        token.token_type,
        TokenType::Keyword(Keyword::True)
            | TokenType::Keyword(Keyword::False)
            | TokenType::Type(Type::Null)
            | TokenType::Number
            | TokenType::Type(Type::String)
    )
}

/// Convierte un token en su operador binario correspondiente en el AST, si aplica.
/// Retorna `Some(BinaryOp)` si el token es un operador válido, o `None` si no lo es.
pub fn token_to_binary_op(token: &Token) -> Option<BinaryOp> {
    match token.token_type {
        TokenType::Operator(Operator::Add) => Some(BinaryOp::Add),     // +
        TokenType::Operator(Operator::Subtract) => Some(BinaryOp::Sub),// -
        TokenType::Operator(Operator::Multiply) => Some(BinaryOp::Mul),// *
        TokenType::Operator(Operator::Divide) => Some(BinaryOp::Div),  // /
        TokenType::Comparator(Comparator::Equal) => Some(BinaryOp::Eq),         // ==
        TokenType::Comparator(Comparator::NotEqual) => Some(BinaryOp::Neq),     // !=
        TokenType::Comparator(Comparator::Greater) => Some(BinaryOp::Gt),       // >
        TokenType::Comparator(Comparator::GreaterEqual) => Some(BinaryOp::Gte), // >=
        TokenType::Comparator(Comparator::Less) => Some(BinaryOp::Lt),          // <
        TokenType::Comparator(Comparator::LessEqual) => Some(BinaryOp::Lte),    // <=
        TokenType::Logical(Logical::And) => Some(BinaryOp::And), // &&, and
        TokenType::Logical(Logical::Or) => Some(BinaryOp::Or),   // ||, or
        _ => None,
    }
}

/// Convierte un token en su operador unario correspondiente en el AST, si aplica.
/// Retorna `Some(UnaryOp)` si el token es un operador unario válido, o `None` si no lo es.
pub fn token_to_unary_op(token: &Token) -> Option<UnaryOp> {
    match token.token_type {
        TokenType::Logical(Logical::Not) => Some(UnaryOp::Not),     // !, not
        TokenType::Operator(Operator::Subtract) => Some(UnaryOp::Neg), // - (negativo)
        _ => None,
    }
}

/// Imprime todos los tokens recibidos, útil para depurar el flujo del lexer o parser.
/// Muestra la línea, tipo y valor de cada token.
pub fn debug_token_stream(tokens: &[Token]) {
    for token in tokens {
        println!(
            "Línea {:>3}: {:?} => '{}'",
            token.line, token.token_type, token.value
        );
    }
}
