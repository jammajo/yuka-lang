use crate::token::TokenType;
use crate::ast::{Statement, Expression};
use super::Parser;
use crate::grammar::*;

impl Parser {
    // === PUNTO DE ENTRADA GENERAL PARA STATEMENTS ===

    // Determina qué tipo de sentencia se está analizando y delega al método correspondiente.
    pub fn parse_statement(&mut self) -> Result<Statement, String> {
        if self.match_token(&[TokenType::Keyword(Keyword::Let)]) {
            self.parse_variable_declaration()
        } else if self.match_token(&[TokenType::Keyword(Keyword::Fn)]) {
            self.parse_function_declaration()
        } else if self.match_token(&[TokenType::Keyword(Keyword::If)]) {
            self.parse_if_statement()
        } else if self.match_token(&[TokenType::Keyword(Keyword::While)]) {
            self.parse_while_statement()
        } else if self.match_token(&[TokenType::Keyword(Keyword::Do)]) {
            self.parse_do_while_statement()
        } else if self.match_token(&[TokenType::Keyword(Keyword::For)]) {
            self.parse_for_statement()
        } else if self.match_token(&[TokenType::Keyword(Keyword::Return)]) {
            self.parse_return_statement()
        } else if self.match_token(&[TokenType::Keyword(Keyword::Break)]) {
            // 'break;' requiere punto y coma
            self.consume(TokenType::Symbol(Symbol::Semicolon), "Se esperaba ';' después de 'break'")?;
            Ok(Statement::Break)
        } else if self.match_token(&[TokenType::Keyword(Keyword::Continue)]) {
            // 'continue;' requiere punto y coma
            self.consume(TokenType::Symbol(Symbol::Semicolon), "Se esperaba ';' después de 'continue'")?;
            Ok(Statement::Continue)
        } else if self.match_token(&[TokenType::Symbol(Symbol::OpenBrace)]) {
            // Bloques: { ... }
            let block = self.parse_block()?;
            Ok(Statement::Block(block))
        } else {
            // Expresiones sueltas como statements (ej: `call();`)
            let expr = self.parse_expression()?;
            self.consume(TokenType::Symbol(Symbol::Semicolon), "Se esperaba ';' después de la expresión")?;
            Ok(Statement::Expression(expr))
        }
    }

    // === DECLARACIÓN DE VARIABLES ===
    fn parse_variable_declaration(&mut self) -> Result<Statement, String> {
        let name = self.consume_identifier("Se esperaba el nombre de la variable")?;

        // Permite inicialización opcional: let x; o let x = expr;
        let initializer = if self.match_token(&[TokenType::Symbol(Symbol::Define)]) {
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.consume(TokenType::Symbol(Symbol::Semicolon), "Se esperaba ';' después de la declaración")?;
        Ok(Statement::Variable { name, initializer })
    }

    // === DECLARACIÓN DE FUNCIONES ===
    fn parse_function_declaration(&mut self) -> Result<Statement, String> {
        // Puede ser anónima o nombrada
        let name = if self.check(TokenType::Identifier) {
            Some(self.advance().value.clone())
        } else {
            None
        };

        self.consume(TokenType::Symbol(Symbol::OpenParen), "Se esperaba '(' después de 'fun'")?;

        // Parámetros
        let mut params = Vec::new();
        if !self.check(TokenType::Symbol(Symbol::CloseParen)) {
            loop {
                params.push(self.consume_identifier("Se esperaba nombre del parámetro")?);
                if !self.match_token(&[TokenType::Symbol(Symbol::Comma)]) {
                    break;
                }
            }
        }

        self.consume(TokenType::Symbol(Symbol::CloseParen), "Se esperaba ')' después de los parámetros")?;
        self.consume(TokenType::Symbol(Symbol::OpenBrace), "Se esperaba '{' para el cuerpo de la función")?;
        let body = self.parse_block()?;

        Ok(Statement::Function { name, params, body })
    }

    // === SENTENCIA IF ===
    fn parse_if_statement(&mut self) -> Result<Statement, String> {
        self.consume(TokenType::Symbol(Symbol::OpenParen), "Se esperaba '(' después de 'if'")?;
        let condition = self.parse_expression()?;
        self.consume(TokenType::Symbol(Symbol::CloseParen), "Se esperaba ')' después de la condición")?;

        let then_branch = Box::new(self.parse_statement()?);

        // Opcional: else
        let else_branch = if self.match_token(&[TokenType::Keyword(Keyword::Else)]) {
            Some(Box::new(self.parse_statement()?))
        } else {
            None
        };

        Ok(Statement::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    // === BUCLE WHILE ===
    fn parse_while_statement(&mut self) -> Result<Statement, String> {
        self.consume(TokenType::Symbol(Symbol::OpenParen), "Se esperaba '(' después de 'while'")?;
        let condition = self.parse_expression()?;
        self.consume(TokenType::Symbol(Symbol::CloseParen), "Se esperaba ')' después de la condición")?;
        let body = Box::new(self.parse_statement()?);

        Ok(Statement::While { condition, body })
    }

    // === BUCLE DO-WHILE ===
    fn parse_do_while_statement(&mut self) -> Result<Statement, String> {
        let body = Box::new(self.parse_statement()?);

        self.consume(TokenType::Keyword(Keyword::While), "Se esperaba 'while' después de bloque 'do'")?;
        self.consume(TokenType::Symbol(Symbol::OpenParen), "Se esperaba '(' después de 'while'")?;
        let condition = self.parse_expression()?;
        self.consume(TokenType::Symbol(Symbol::CloseParen), "Se esperaba ')' después de la condición")?;
        self.consume(TokenType::Symbol(Symbol::Semicolon), "Se esperaba ';' al final del do-while")?;

        Ok(Statement::DoWhile { condition, body })
    }

    // === BUCLE FOR (C y estilo Python) ===
    fn parse_for_statement(&mut self) -> Result<Statement, String> {
        self.consume(TokenType::Symbol(Symbol::OpenParen), "Se esperaba '(' después de 'for'")?;

        if self.match_token(&[TokenType::Keyword(Keyword::Let)]) {
            // Estilo C: for (let i = 0; i < 10; i++) { ... }
            let decl = self.parse_variable_declaration()?;
            let condition = self.parse_expression()?;
            self.consume(TokenType::Symbol(Symbol::Semicolon), "Se esperaba ';' después de condición")?;
            let increment = self.parse_expression()?;
            self.consume(TokenType::Symbol(Symbol::CloseParen), "Se esperaba ')' después del incremento")?;
            let body = Box::new(self.parse_statement()?);

            Ok(Statement::ForCStyle {
                init: Box::new(decl),
                condition,
                increment,
                body,
            })
        } else {
            // Estilo Python: for (item in iterable) { ... }
            let iterator = self.consume_identifier("Se esperaba nombre de la variable")?;
            self.consume(TokenType::Keyword(Keyword::In), "Se esperaba 'in' en bucle for estilo Python")?;
            let iterable = self.parse_expression()?;
            self.consume(TokenType::Symbol(Symbol::CloseParen), "Se esperaba ')' después de expresión iterable")?;
            let body = Box::new(self.parse_statement()?);

            Ok(Statement::ForIn {
                variable: iterator,
                iterable,
                body,
            })
        }
    }

    // === RETURN ===
    fn parse_return_statement(&mut self) -> Result<Statement, String> {
        // Valor opcional: return; o return expr;
        let value = if !self.check(TokenType::Symbol(Symbol::Semicolon)) {
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.consume(TokenType::Symbol(Symbol::Semicolon), "Se esperaba ';' después de 'return'")?;
        Ok(Statement::Return(value))
    }

    // === BLOQUE ===
    fn parse_block(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();

        while !self.check(TokenType::Symbol(Symbol::CloseBrace)) && !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }

        self.consume(TokenType::Symbol(Symbol::CloseBrace), "Se esperaba '}' al final del bloque")?;
        Ok(statements)
    }
}
