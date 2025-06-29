use crate::token::{Token, TokenType};
use crate::grammar::*;
use crate::ast::{BinaryOp, UnaryOp};

/// Verifica si el token representa un identificador válido
pub fn is_identifier(token: &Token) -> bool {
    matches!(token.token_type, TokenType::Identifier)
}

/// Verifica si el token es un literal: booleano, número, string o None
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

/// Intenta convertir un token en un operador binario del AST
pub fn token_to_binary_op(token: &Token) -> Option<BinaryOp> {
    match token.token_type {
        TokenType::Operator(Operator::Add) => Some(BinaryOp::Add),
        TokenType::Operator(Operator::Subtract) => Some(BinaryOp::Sub),
        TokenType::Operator(Operator::Multiply) => Some(BinaryOp::Mul),
        TokenType::Operator(Operator::Divide) => Some(BinaryOp::Div),
        TokenType::Comparator(Comparator::Equal) => Some(BinaryOp::Eq),
        TokenType::Comparator(Comparator::NotEqual) => Some(BinaryOp::Neq),
        TokenType::Comparator(Comparator::Greater) => Some(BinaryOp::Gt),
        TokenType::Comparator(Comparator::GreaterEqual) => Some(BinaryOp::Gte),
        TokenType::Comparator(Comparator::Less) => Some(BinaryOp::Lt),
        TokenType::Comparator(Comparator::LessEqual) => Some(BinaryOp::Lte),
        TokenType::Logical(Logical::And) => Some(BinaryOp::And),
        TokenType::Logical(Logical::Or) => Some(BinaryOp::Or),
        _ => None,
    }
}

/// Intenta convertir un token en un operador unario del AST
pub fn token_to_unary_op(token: &Token) -> Option<UnaryOp> {
    match token.token_type {
        TokenType::Logical(Logical::Not) => Some(UnaryOp::Not),
        TokenType::Operator(Operator::Subtract) => Some(UnaryOp::Neg),
        _ => None,
    }
}

/// Utilidad para imprimir los tokens para depuración
pub fn debug_token_stream(tokens: &[Token]) {
    for token in tokens {
        println!(
            "Línea {:>3}: {:?} => '{}'",
            token.line, token.token_type, token.value
        );
    }
}
