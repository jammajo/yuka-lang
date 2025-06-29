// Importaciones necesarias para acceder a los tokens, expresiones del AST y definiciones del parser
use crate::token::{Token, TokenType};
use crate::ast::{Expression, BinaryOp, UnaryOp};
use super::Parser;
use crate::grammar::*;

// Implementación de los métodos del parser
impl Parser {
    // Punto de entrada para parsear cualquier expresión
    pub fn parse_expression(&mut self) -> Result<Expression, String> {
        self.parse_assignment()
    }

    // Parseo de asignaciones (ej. x = y;)
    fn parse_assignment(&mut self) -> Result<Expression, String> {
        let expr = self.parse_ternary()?; // Parte izquierda de la asignación

        // Verifica si el siguiente token es '='
        if self.match_token(&[TokenType::Symbol(Symbol::Define)]) {
            let value = self.parse_assignment()?; // Recursivo para permitir asignaciones encadenadas

            match expr {
                // Solo se puede asignar a una variable (ej. x = 5)
                Expression::Variable(name) => Ok(Expression::Assign {
                    variable: name,
                    value: Box::new(value),
                }),
                // También se permite asignar a una propiedad de un objeto (ej. obj.prop = 10)
                Expression::Get { object, name } => Ok(Expression::Set {
                    object,
                    name,
                    value: Box::new(value),
                }),
                // Si no es una variable ni propiedad, es un error
                _ => Err("Error: solo se puede asignar a una variable o propiedad".into()),
            }
        } else {
            Ok(expr) // Si no es asignación, se retorna la expresión original
        }
    }

    // Operador ternario: cond ? expr1 : expr2
    fn parse_ternary(&mut self) -> Result<Expression, String> {
        let condition = self.parse_logical_or()?; // Se parsea la condición

        // Verifica si hay '?'
        if self.match_token(&[TokenType::Symbol(Symbol::Question)]) {
            let true_branch = self.parse_expression()?; // Rama verdadera
            self.consume(TokenType::Symbol(Symbol::Colon), "Se esperaba ':' en expresión ternaria")?;
            let false_branch = self.parse_expression()?; // Rama falsa

            return Ok(Expression::Ternary {
                condition: Box::new(condition),
                then_branch: Box::new(true_branch),
                else_branch: Box::new(false_branch),
            });
        }

        Ok(condition) // Si no hay '?', solo se devuelve la condición
    }

    // Operador lógico OR (`or`)
    fn parse_logical_or(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_logical_and()?; // Se evalúa primero el AND por precedencia

        while self.match_token(&[TokenType::Logical(Logical::Or)]) {
            let right = self.parse_logical_and()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                op: BinaryOp::Or,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    // Operador lógico AND (`and`)
    fn parse_logical_and(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_equality()?; // Se evalúan primero las igualdades

        while self.match_token(&[TokenType::Logical(Logical::And)]) {
            let right = self.parse_equality()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                op: BinaryOp::And,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    // Comparadores de igualdad (`==`, `!=`)
    fn parse_equality(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_comparison()?; // Primero se evalúan comparaciones relacionales

        while self.match_token(&[TokenType::Comparator(Comparator::NotEqual), TokenType::Comparator(Comparator::Equal)]) {
            let op = match self.previous().token_type {
                TokenType::Comparator(Comparator::NotEqual) => BinaryOp::Neq,
                TokenType::Comparator(Comparator::Equal) => BinaryOp::Eq,
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

    // Comparaciones relacionales (`<`, `>`, `<=`, `>=`)
    fn parse_comparison(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_term()?; // Suma/resta se evalúan luego

        while self.match_token(&[
            TokenType::Comparator(Comparator::Greater),
            TokenType::Comparator(Comparator::GreaterEqual),
            TokenType::Comparator(Comparator::Less),
            TokenType::Comparator(Comparator::LessEqual),
        ]) {
            let op = match self.previous().token_type {
                TokenType::Comparator(Comparator::Greater) => BinaryOp::Gt,
                TokenType::Comparator(Comparator::GreaterEqual) => BinaryOp::Gte,
                TokenType::Comparator(Comparator::Less) => BinaryOp::Lt,
                TokenType::Comparator(Comparator::LessEqual) => BinaryOp::Lte,
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

    // Suma y resta (`+`, `-`)
    fn parse_term(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_factor()?; // Multiplicación/división se evalúan primero

        while self.match_token(&[TokenType::Operator(Operator::Add), TokenType::Operator(Operator::Subtract)]) {
            let op = match self.previous().token_type {
                TokenType::Operator(Operator::Add) => BinaryOp::Add,
                TokenType::Operator(Operator::Subtract) => BinaryOp::Sub,
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

    // Multiplicación y división (`*`, `/`)
    fn parse_factor(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_unary()?; // Los unarios se evalúan antes

        while self.match_token(&[TokenType::Operator(Operator::Multiply), TokenType::Operator(Operator::Divide)]) {
            let op = match self.previous().token_type {
                TokenType::Operator(Operator::Multiply) => BinaryOp::Mul,
                TokenType::Operator(Operator::Divide) => BinaryOp::Div,
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

    // Operadores unarios (`!`, `-`)
    fn parse_unary(&mut self) -> Result<Expression, String> {
        if self.match_token(&[TokenType::Logical(Logical::Not), TokenType::Operator(Operator::Subtract)]) {
            let op = match self.previous().token_type {
                TokenType::Logical(Logical::Not) => UnaryOp::Not,
                TokenType::Operator(Operator::Subtract) => UnaryOp::Neg,
                _ => unreachable!(),
            };
            let expr = self.parse_unary()?; // Recursividad permite múltiples operadores unarios
            return Ok(Expression::Unary {
                op,
                expr: Box::new(expr),
            });
        }

        self.parse_call() // Si no hay unario, intenta parsear llamadas o propiedades
    }

    // Llamadas a funciones y acceso a propiedades (`foo()`, `obj.prop`)
    fn parse_call(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_primary()?; // Punto de partida

        loop {
            // Parseo de llamadas: `func(args...)`
            if self.match_token(&[TokenType::Symbol(Symbol::OpenParen)]) {
                let mut args = Vec::new();

                if !self.check(TokenType::Symbol(Symbol::CloseParen)) {
                    loop {
                        args.push(self.parse_expression()?);
                        if !self.match_token(&[TokenType::Symbol(Symbol::Comma)]) {
                            break;
                        }
                    }
                }

                self.consume(TokenType::Symbol(Symbol::CloseParen), "Se esperaba ')' después de los argumentos")?;
                expr = Expression::Call {
                    function: Box::new(expr),
                    args,
                };
            }
            // Parseo de acceso a propiedad: `obj.prop`
            else if self.match_token(&[TokenType::Symbol(Symbol::Dot)]) {
                let name = self.consume_identifier("Se esperaba un nombre de propiedad después de '.'")?;
                expr = Expression::Get {
                    object: Box::new(expr),
                    name,
                };
            }
            else {
                break;
            }
        }

        Ok(expr)
    }

    // Parseo de literales, variables y agrupaciones (`(expr)`)
    fn parse_primary(&mut self) -> Result<Expression, String> {
        let token = self.peek();

        match token.token_type {
            TokenType::Keyword(Keyword::False) => {
                self.advance();
                Ok(Expression::Boolean(false))
            }
            TokenType::Keyword(Keyword::True) => {
                self.advance();
                Ok(Expression::Boolean(true))
            }
            TokenType::Type(Type::Null) => {
                self.advance();
                Ok(Expression::None)
            }
            TokenType::Number => {
                let value = self.advance().value.parse::<f64>().map_err(|_| "Número inválido")?;
                Ok(Expression::Number(value))
            }
            TokenType::Type(Type::String) => {
                let value = self.advance().value.clone();
                Ok(Expression::String(value))
            }
            TokenType::Identifier => {
                let name = self.advance().value.clone();
                Ok(Expression::Variable(name))
            }
            TokenType::Symbol(Symbol::OpenParen) => {
                self.advance();
                let expr = self.parse_expression()?; // Se permite agrupación con paréntesis
                self.consume(TokenType::Symbol(Symbol::CloseParen), "Se esperaba ')' después de la expresión")?;
                Ok(Expression::Grouping(Box::new(expr)))
            }
            _ => Err(format!("Expresión inesperada: '{}'", token.value)),
        }
    }
}
