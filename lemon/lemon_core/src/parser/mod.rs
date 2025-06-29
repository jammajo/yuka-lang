pub mod expressions;
pub mod statements;
pub mod helpers;

use crate::token::{Token, TokenType};
use crate::ast::{statements::Statement, expressions::Expression};
use expressions::*;
use statements::*;
use helpers::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    /// Crea un nuevo parser con la lista de tokens
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    /// Punto de entrada del parser: analiza todos los statements del programa
    pub fn parse(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            match self.parse_statement() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => return Err(e),
            }
        }

        Ok(statements)
    }

    // ========================
    // 👀 Navegación en tokens
    // ========================

    /// Revisa si llegamos al final
    pub fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    /// Devuelve el token actual sin avanzar
    pub fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    /// Devuelve el token anterior al actual
    pub fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    /// Avanza al siguiente token y lo devuelve
    pub fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    /// Verifica si el token actual es del tipo esperado
    pub fn check(&self, token_type: TokenType) -> bool {
        !self.is_at_end() && self.peek().token_type == token_type
    }

    /// Avanza si el token actual coincide con alguno de los tipos dados
pub fn match_token(&mut self, types: &[TokenType]) -> bool {
    for &tt in types {
        if self.check(tt) {
            self.advance();
            return true;
        }
    }
    false
}
    /// Consume un token del tipo esperado o lanza error con mensaje personalizado
    pub fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, String> {
        if self.check(token_type) {
            Ok(self.advance().clone())
        } else {
            Err(format!(
                "[Línea {}] Error de sintaxis: {} (en '{}')",
                self.peek().line, message, self.peek().value
            ))
        }
    }

    /// Intenta consumir un identificador y devuelve su nombre
    pub fn consume_identifier(&mut self, message: &str) -> Result<String, String> {
        let token = self.consume(TokenType::Identifier, message)?;
        Ok(token.value.clone())
    }

    // ==============================
    // 🧱 Métodos principales a definir
    // ==============================

    /// Analiza un statement (instrucción)
    pub fn parse_statement(&mut self) -> Result<Statement, String> {
        // Esto se delega a statements.rs
        Err("parse_statement no implementado aún".into())
    }

    /// Analiza una expresión
    pub fn parse_expression(&mut self) -> Result<Expression, String> {
        // Esto se delega a expressions.rs
        Err("parse_expression no implementado aún".into())
    }
}
