use crate::token::TokenType;
use crate::ast::{Statement, Expression};
use super::Parser;
use crate::grammar::*;

impl Parser {
    // === PUNTO DE ENTRADA GENERAL PARA STATEMENTS ===

    pub fn parse_statement(&mut self) -> Result<Statement, String> {
        // Declaración de variable: let x = ...
        if self.match_token(&[TokenType::Keyword(Keyword::Let)]) {
            self.parse_variable_declaration()
        }
        // Declaración de función: fun name(...) { ... }
        else if self.match_token(&[TokenType::Keyword(Keyword::Fn)]) {
            self.parse_function_declaration()
        }
        // Condicional if-else
        else if self.match_token(&[TokenType::Keyword(Keyword::If)]) {
            self.parse_if_statement()
        }
        // Bucle while
        else if self.match_token(&[TokenType::Keyword(Keyword::While)]) {
            self.parse_while_statement()
        }
        // Bucle do-while
        else if self.match_token(&[TokenType::Keyword(Keyword::Do)]) {
            self.parse_do_while_statement()
        }
        // Bucle for (C-style o Python-style)
        else if self.match_token(&[TokenType::Keyword(Keyword::For)]) {
            self.parse_for_statement()
        }
        // Return: back ...
        else if self.match_token(&[TokenType::Keyword(Keyword::Back)]) {
            self.parse_return_statement()
        }
        // Sentencias break y continue
        else if self.match_token(&[TokenType::Keyword(Keyword::Break)]) {
            self.consume(TokenType::Symbol(Symbol::Semicolon), "Se esperaba ';' después de 'break'")?;
            Ok(Statement::Break)
        }
        else if self.match_token(&[TokenType::Keyword(Keyword::Continue)]) {
            self.consume(TokenType::Symbol(Symbol::Semicolon), "Se esperaba ';' después de 'continue'")?;
            Ok(Statement::Continue)
        }
        // Bloques de código: { ... }
        else if self.match_token(&[TokenType::Symbol(Symbol::OpenBrace)]) {
            let block = self.parse_block()?;
            Ok(Statement::Block(block))
        }
        // Expresiones como statements: foo(); o x + 3;
        else {
            let expr = self.parse_expression()?;
            if self.match_token(&[TokenType::Symbol(Symbol::Semicolon)]) {
                Ok(Statement::Expression(expr))
            } else {
                Err("Se esperaba ';' después de la expresión".into())
            }
        }
    }

    // === DECLARACIONES ===

    // Declaración de variable: let nombre = valor?;
    fn parse_variable_declaration(&mut self) -> Result<Statement, String> {
        let name = self.consume_identifier("Se esperaba el nombre de la variable")?;

        // Inicializador opcional
        let initializer = if self.match_token(&[TokenType::Symbol(Symbol::Define)]) {
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.consume(TokenType::Symbol(Symbol::Semicolon), "Se esperaba ';' después de la declaración")?;
        Ok(Statement::Variable { name, initializer })
    }

    // Declaración de función: fun nombre? (param1, ...) { ... }
    fn parse_function_declaration(&mut self) -> Result<Statement, String> {
        // Puede ser anónima si no hay identificador
        let name = if self.check(TokenType::Identifier) {
            Some(self.advance().value.clone())
        } else {
            None
        };

        self.consume(TokenType::Symbol(Symbol::OpenParen), "Se esperaba '(' después de 'fun'")?;

        // Parseo de parámetros
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

    // === CONTROL DE FLUJO ===

    // Condicional: if (...) { ... } else { ... }?
    fn parse_if_statement(&mut self) -> Result<Statement, String> {
        self.consume(TokenType::Symbol(Symbol::OpenParen), "Se esperaba '(' después de 'if'")?;
        let condition = self.parse_expression()?;
        self.consume(TokenType::Symbol(Symbol::CloseParen), "Se esperaba ')' después de la condición")?;

        let then_branch = Box::new(self.parse_statement()?);

        // else opcional
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

    // Bucle while: while (condición) { cuerpo }
    fn parse_while_statement(&mut self) -> Result<Statement, String> {
        self.consume(TokenType::Symbol(Symbol::OpenParen), "Se esperaba '(' después de 'while'")?;
        let condition = self.parse_expression()?;
        self.consume(TokenType::Symbol(Symbol::CloseParen), "Se esperaba ')' después de la condición")?;
        let body = Box::new(self.parse_statement()?);

        Ok(Statement::While { condition, body })
    }

    // Bucle do-while: do { cuerpo } while (condición);
    fn parse_do_while_statement(&mut self) -> Result<Statement, String> {
        let body = Box::new(self.parse_statement()?);

        self.consume(TokenType::Keyword(Keyword::While), "Se esperaba 'while' después de bloque 'do'")?;
        self.consume(TokenType::Symbol(Symbol::OpenParen), "Se esperaba '(' después de 'while'")?;
        let condition = self.parse_expression()?;
        self.consume(TokenType::Symbol(Symbol::CloseParen), "Se esperaba ')' después de la condición")?;
        self.consume(TokenType::Symbol(Symbol::Semicolon), "Se esperaba ';' al final del do-while")?;

        Ok(Statement::DoWhile { condition, body })
    }

    // Bucle for (C-style o Python-style)
    fn parse_for_statement(&mut self) -> Result<Statement, String> {
        self.consume(TokenType::Symbol(Symbol::OpenParen), "Se esperaba '(' después de 'for'")?;

        // Estilo C: for (let i = 0; i < 10; i = i + 1)
        if self.match_token(&[TokenType::Keyword(Keyword::Let)]) {
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
        }
        // Estilo Python: for item in iterable
        else {
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

    // Return opcional: return valor?;
    fn parse_return_statement(&mut self) -> Result<Statement, String> {
        let value = if !self.check(TokenType::Symbol(Symbol::Semicolon)) {
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.consume(TokenType::Symbol(Symbol::Semicolon), "Se esperaba ';' después de 'return'")?;
        Ok(Statement::Return(value))
    }

    // === BLOQUES ===

    // Bloques de código: { sentencia* }
    fn parse_block(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();

        // Agrega sentencias hasta cerrar con '}'
        while !self.check(TokenType::Symbol(Symbol::CloseBrace)) && !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }

        self.consume(TokenType::Symbol(Symbol::CloseBrace), "Se esperaba '}' al final del bloque")?;
        Ok(statements)
    }
}
