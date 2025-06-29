use crate::token::TokenType;
use crate::ast::{Statement, Expression};
use super::Parser;

impl Parser {
    pub fn parse_statement(&mut self) -> Result<Statement, String> {
        if self.match_token(&[TokenType::Let]) {
            self.parse_variable_declaration()
        } else if self.match_token(&[TokenType::Fun]) {
            self.parse_function_declaration()
        } else if self.match_token(&[TokenType::If]) {
            self.parse_if_statement()
        } else if self.match_token(&[TokenType::While]) {
            self.parse_while_statement()
        } else if self.match_token(&[TokenType::Do]) {
            self.parse_do_while_statement()
        } else if self.match_token(&[TokenType::For]) {
            self.parse_for_statement()
        } else if self.match_token(&[TokenType::Return]) {
            self.parse_return_statement()
        } else if self.match_token(&[TokenType::Break]) {
            self.consume(TokenType::Semicolon, "Se esperaba ';' después de 'break'")?;
            Ok(Statement::Break)
        } else if self.match_token(&[TokenType::Continue]) {
            self.consume(TokenType::Semicolon, "Se esperaba ';' después de 'continue'")?;
            Ok(Statement::Continue)
        } else if self.match_token(&[TokenType::LeftBrace]) {
            let block = self.parse_block()?;
            Ok(Statement::Block(block))
        } else {
            let expr = self.parse_expression()?;
            if self.match_token(&[TokenType::Semicolon]) {
                Ok(Statement::Expression(expr))
            } else {
                Err("Se esperaba ';' después de la expresión".into())
            }
        }
    }

    fn parse_variable_declaration(&mut self) -> Result<Statement, String> {
        let name = self.consume_identifier("Se esperaba el nombre de la variable")?;

        let initializer = if self.match_token(&[TokenType::Equal]) {
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.consume(TokenType::Semicolon, "Se esperaba ';' después de la declaración")?;
        Ok(Statement::Variable { name, initializer })
    }

    fn parse_function_declaration(&mut self) -> Result<Statement, String> {
        let name = if self.check(TokenType::Identifier) {
            Some(self.advance().value.clone())
        } else {
            None
        };

        self.consume(TokenType::LeftParen, "Se esperaba '(' después de 'fun'")?;
        let mut params = Vec::new();

        if !self.check(TokenType::RightParen) {
            loop {
                params.push(self.consume_identifier("Se esperaba nombre del parámetro")?);
                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        self.consume(TokenType::RightParen, "Se esperaba ')' después de los parámetros")?;
        self.consume(TokenType::LeftBrace, "Se esperaba '{' para el cuerpo de la función")?;
        let body = self.parse_block()?;

        Ok(Statement::Function { name, params, body })
    }

    fn parse_if_statement(&mut self) -> Result<Statement, String> {
        self.consume(TokenType::LeftParen, "Se esperaba '(' después de 'if'")?;
        let condition = self.parse_expression()?;
        self.consume(TokenType::RightParen, "Se esperaba ')' después de la condición")?;

        let then_branch = Box::new(self.parse_statement()?);
        let else_branch = if self.match_token(&[TokenType::Else]) {
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

    fn parse_while_statement(&mut self) -> Result<Statement, String> {
        self.consume(TokenType::LeftParen, "Se esperaba '(' después de 'while'")?;
        let condition = self.parse_expression()?;
        self.consume(TokenType::RightParen, "Se esperaba ')' después de la condición")?;
        let body = Box::new(self.parse_statement()?);
        Ok(Statement::While { condition, body })
    }

    fn parse_do_while_statement(&mut self) -> Result<Statement, String> {
        let body = Box::new(self.parse_statement()?);
        self.consume(TokenType::While, "Se esperaba 'while' después de bloque 'do'")?;
        self.consume(TokenType::LeftParen, "Se esperaba '(' después de 'while'")?;
        let condition = self.parse_expression()?;
        self.consume(TokenType::RightParen, "Se esperaba ')' después de la condición")?;
        self.consume(TokenType::Semicolon, "Se esperaba ';' al final del do-while")?;
        Ok(Statement::DoWhile { condition, body })
    }

    fn parse_for_statement(&mut self) -> Result<Statement, String> {
        self.consume(TokenType::LeftParen, "Se esperaba '(' después de 'for'")?;

        if self.match_token(&[TokenType::Let]) {
            let decl = self.parse_variable_declaration()?;
            let condition = self.parse_expression()?;
            self.consume(TokenType::Semicolon, "Se esperaba ';' después de condición")?;
            let increment = self.parse_expression()?;
            self.consume(TokenType::RightParen, "Se esperaba ')' después del incremento")?;
            let body = Box::new(self.parse_statement()?);

            Ok(Statement::ForCStyle {
                init: Box::new(decl),
                condition,
                increment,
                body,
            })
        } else {
            let iterator = self.consume_identifier("Se esperaba nombre de la variable")?;
            self.consume(TokenType::In, "Se esperaba 'in' en bucle for estilo Python")?;
            let iterable = self.parse_expression()?;
            self.consume(TokenType::RightParen, "Se esperaba ')' después de expresión iterable")?;
            let body = Box::new(self.parse_statement()?);

            Ok(Statement::ForIn {
                variable: iterator,
                iterable,
                body,
            })
        }
    }

    fn parse_return_statement(&mut self) -> Result<Statement, String> {
        let value = if !self.check(crate::token::TokenType::Semicolon) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        self.consume(TokenType::Semicolon, "Se esperaba ';' después de 'return'")?;
        Ok(Statement::Return(value))
    }

    fn parse_block(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }

        self.consume(TokenType::RightBrace, "Se esperaba '}' al final del bloque")?;
        Ok(statements)
    }
}
