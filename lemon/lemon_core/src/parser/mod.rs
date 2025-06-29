// Reexporta submódulos del parser
pub mod expressions;
pub mod statements;
pub mod helpers;

use crate::token::{Token, TokenType};
use crate::ast::{statements::Statement};

/// Estructura principal del parser: contiene la lista de tokens y la posición actual
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    // ========================
    // 🔧 Constructor principal
    // ========================

    /// Crea un nuevo parser a partir de una lista de tokens generados por el lexer
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    // ========================
    // 🎯 Punto de entrada
    // ========================

    /// Analiza todos los statements del archivo fuente y devuelve el AST completo
    pub fn parse(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            match self.parse_statement() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => return Err(e), // En caso de error, se detiene el análisis
            }
        }

        Ok(statements)
    }

    // ========================
    // 👀 Navegación en tokens
    // ========================

    /// Devuelve `true` si hemos llegado al final del archivo (EOF)
    pub fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    /// Mira el token actual sin consumirlo
    pub fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    /// Devuelve el token anterior al actual
    pub fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    /// Avanza al siguiente token y devuelve el anterior
    pub fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    /// Verifica si el token actual es igual al tipo dado
    pub fn check(&self, token_type: TokenType) -> bool {
        !self.is_at_end() && self.peek().token_type == token_type
    }

    /// Intenta avanzar si el token actual coincide con alguno de los tipos dados
    pub fn match_token(&mut self, types: &[TokenType]) -> bool {
        for &tt in types {
            if self.check(tt) {
                self.advance();
                return true;
            }
        }
        false
    }

    /// Consume un token del tipo esperado o lanza un error con mensaje personalizado
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

    /// Consume y devuelve el valor de un identificador. Lanza error si no lo es.
    pub fn consume_identifier(&mut self, message: &str) -> Result<String, String> {
        let token = self.consume(TokenType::Identifier, message)?;
        Ok(token.value.clone())
    }
}
