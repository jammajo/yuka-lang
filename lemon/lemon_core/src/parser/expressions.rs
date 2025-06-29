use crate::token::{Token, TokenType};
use crate::ast::{Expression, BinaryOp, UnaryOp};
use super::Parser;

impl Parser {
    pub fn parse_expression(&mut self) -> Result<Expression, String> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<Expression, String> {
        let expr = self.parse_ternary()?;

        if self.match_token(&[TokenType::Equal]) {
            let value = self.parse_assignment()?;

            match expr {
                Expression::Variable(name) => Ok(Expression::Assign {
                    variable: name,
                    value: Box::new(value),
                }),
                Expression::Get { object, name } => Ok(Expression::Set {
                    object,
                    name,
                    value: Box::new(value),
                }),
                _ => Err("Error: solo se puede asignar a una variable o propiedad".into()),
            }
        } else {
            Ok(expr)
        }
    }

    fn parse_ternary(&mut self) -> Result<Expression, String> {
        let condition = self.parse_logical_or()?;

        if self.match_token(&[TokenType::Question]) {
            let true_branch = self.parse_expression()?;
            self.consume(TokenType::Colon, "Se esperaba ':' en expresión ternaria")?;
            let false_branch = self.parse_expression()?;

            return Ok(Expression::Ternary {
                condition: Box::new(condition),
                then_branch: Box::new(true_branch),
                else_branch: Box::new(false_branch),
            });
        }

        Ok(condition)
    }

    fn parse_logical_or(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_logical_and()?;

        while self.match_token(&[TokenType::Or]) {
            let right = self.parse_logical_and()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                op: BinaryOp::Or,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_logical_and(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_equality()?;

        while self.match_token(&[TokenType::And]) {
            let right = self.parse_equality()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                op: BinaryOp::And,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_comparison()?;

        while self.match_token(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let op = match self.previous().token_type {
                TokenType::BangEqual => BinaryOp::Neq,
                TokenType::EqualEqual => BinaryOp::Eq,
                _ => unreachable!(),
            };
            let right = self.parse_comparison()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_term()?;

        while self.match_token(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let op = match self.previous().token_type {
                TokenType::Greater => BinaryOp::Gt,
                TokenType::GreaterEqual => BinaryOp::Gte,
                TokenType::Less => BinaryOp::Lt,
                TokenType::LessEqual => BinaryOp::Lte,
                _ => unreachable!(),
            };
            let right = self.parse_term()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_factor()?;

        while self.match_token(&[TokenType::Plus, TokenType::Minus]) {
            let op = match self.previous().token_type {
                TokenType::Plus => BinaryOp::Add,
                TokenType::Minus => BinaryOp::Sub,
                _ => unreachable!(),
            };
            let right = self.parse_factor()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_unary()?;

        while self.match_token(&[TokenType::Star, TokenType::Slash]) {
            let op = match self.previous().token_type {
                TokenType::Star => BinaryOp::Mul,
                TokenType::Slash => BinaryOp::Div,
                _ => unreachable!(),
            };
            let right = self.parse_unary()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expression, String> {
        if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            let op = match self.previous().token_type {
                TokenType::Bang => UnaryOp::Not,
                TokenType::Minus => UnaryOp::Neg,
                _ => unreachable!(),
            };
            let expr = self.parse_unary()?;
            return Ok(Expression::Unary {
                op,
                expr: Box::new(expr),
            });
        }

        self.parse_call()
    }

    fn parse_call(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_primary()?;

        loop {
            if self.match_token(&[TokenType::LeftParen]) {
                let mut args = Vec::new();

                if !self.check(TokenType::RightParen) {
                    loop {
                        args.push(self.parse_expression()?);
                        if !self.match_token(&[TokenType::Comma]) {
                            break;
                        }
                    }
                }

                self.consume(TokenType::RightParen, "Se esperaba ')' después de los argumentos")?;
                expr = Expression::Call {
                    function: Box::new(expr),
                    args,
                };
            } else if self.match_token(&[TokenType::Dot]) {
                let name = self.consume_identifier("Se esperaba un nombre de propiedad después de '.'")?;
                expr = Expression::Get {
                    object: Box::new(expr),
                    name,
                };
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expression, String> {
        let token = self.peek();

        match token.token_type {
            TokenType::False => {
                self.advance();
                Ok(Expression::Boolean(false))
            }
            TokenType::True => {
                self.advance();
                Ok(Expression::Boolean(true))
            }
            TokenType::None => {
                self.advance();
                Ok(Expression::None)
            }
            TokenType::Number => {
                let value = self.advance().value.parse::<f64>().map_err(|_| "Número inválido")?;
                Ok(Expression::Number(value))
            }
            TokenType::String => {
                let value = self.advance().value.clone();
                Ok(Expression::String(value))
            }
            TokenType::Identifier => {
                let name = self.advance().value.clone();
                Ok(Expression::Variable(name))
            }
            TokenType::LeftParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.consume(TokenType::RightParen, "Se esperaba ')' después de la expresión")?;
                Ok(Expression::Grouping(Box::new(expr)))
            }
            _ => Err(format!("Expresión inesperada: '{}'", token.value)),
        }
    }
}
