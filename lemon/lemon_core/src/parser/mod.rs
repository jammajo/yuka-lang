// Reexporta los submódulos que forman el parser completo
pub mod expressions;  // Módulo que maneja el parsing de expresiones
pub mod helpers;      // Utilidades auxiliares para parsing
pub mod statements;   // Módulo que maneja el parsing de statements (instrucciones)

pub use crate::token::{Token, TokenType};           // Reexporta tipos relacionados con los tokens
pub use crate::ast::{statements::Statement};        // Reexporta el tipo Statement del AST

/// Estructura principal del parser.
/// Contiene la lista de tokens generados por el lexer y un índice (`current`)
/// que indica cuál es el token que se está procesando actualmente.
pub struct Parser {
    tokens: Vec<Token>,  // Lista completa de tokens de entrada
    current: usize,      // Índice actual dentro del vector de tokens
}

impl Parser {
    // ========================
    // Constructor principal
    // ========================

    /// Crea una nueva instancia del parser a partir de una lista de tokens.
    /// Este parser recorrerá y analizará los tokens para producir un AST.
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    // ========================
    // Punto de entrada
    // ========================

    /// Inicia el proceso de parsing completo.
    /// Recorre todos los tokens mientras no se llegue al final,
    /// intentando generar una lista de statements válidos (el AST).
    pub fn parse(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            match self.parse_statement() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => return Err(e), // Si ocurre un error, el análisis se detiene inmediatamente
            }
        }

        Ok(statements) // AST final construido con éxito
    }

    // ========================
    // Navegación en tokens
    // ========================

    /// Verifica si ya se ha llegado al final de los tokens (EOF).
    pub fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    /// Retorna el token actual sin consumirlo.
    pub fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    /// Retorna el token que acaba de ser consumido (el anterior al actual).
    pub fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    /// Avanza al siguiente token y retorna el token anterior.
    /// No avanza si ya está en EOF.
    pub fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    /// Verifica si el token actual coincide exactamente con el tipo dado.
    pub fn check(&self, token_type: TokenType) -> bool {
        !self.is_at_end() && self.peek().token_type == token_type
    }

    /// Intenta avanzar si el token actual coincide con alguno de los tipos proporcionados.
    /// Si coincide con alguno, lo consume y retorna `true`; si no, retorna `false`.
    pub fn match_token(&mut self, types: &[TokenType]) -> bool {
        for &tt in types {
            if self.check(tt) {
                self.advance();
                return true;
            }
        }
        false
    }

    /// Consume un token del tipo esperado. Si no coincide, lanza un error con el mensaje dado.
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

    /// Consume un token que debe ser un identificador válido y retorna su valor (texto).
    /// Si no es un identificador, lanza un error con el mensaje proporcionado.
    pub fn consume_identifier(&mut self, message: &str) -> Result<String, String> {
        let token = self.consume(TokenType::Identifier, message)?;
        Ok(token.value.clone())
    }
}
