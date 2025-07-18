use crate::token::{Token, TokenType};
use crate::error::LemonError;

/// Función principal: convierte el código fuente en una lista de tokens
pub fn tokenize(code: &str) -> Result<Vec<Token>, LemonError> {
    let mut tokens = Vec::new();            // Vector donde se guardan los tokens generados
    let mut current = String::new();        // Acumula caracteres para identificadores y palabras clave
    let mut chars = code.chars().peekable(); // Permite mirar el siguiente carácter sin consumirlo
    let mut line = 1;
    let mut column = 1;
    let mut start_column = column;

    while let Some(&ch) = chars.peek() {
        // =========================
        // Comentario de una línea
        // =========================
        if ch == '/' && chars.clone().nth(1) == Some('/') {
            while let Some(&c) = chars.peek() {
                chars.next();
                column += 1;
                if c == '\n' {
                    break;
                }
            }
            line += 1;
            column = 1;
            continue;
        }

        // =========================
        // Comentario multilínea
        // =========================
        if ch == '/' && chars.clone().nth(1) == Some('*') {
            chars.next(); // consume '/'
            chars.next(); // consume '*'
            column += 2;
            let mut closed = false;
            while let Some(c) = chars.next() {
                if c == '*' && chars.peek() == Some(&'/') {
                    chars.next(); // consume '/'
                    column += 2;
                    closed = true;
                    break;
                }
                if c == '\n' {
                    line += 1;
                    column = 1;
                } else {
                    column += 1;
                }
            }
            if !closed {
                return Err(LemonError::new("Unclosed multiline comment", line, column));
            }
            continue;
        }

        // =========================
        // Literales de cadena
        // =========================
        if ch == '"' {
            flush_current(&mut current, &mut tokens, line, start_column);
            let mut string_value = String::new();
            chars.next(); // consume '"'
            column += 1;
            let mut closed = false;

            while let Some(c) = chars.next() {
                column += 1;
                if c == '\\' {
                    // Manejo de secuencias de escape
                    if let Some(escaped) = chars.next() {
                        column += 1;
                        match escaped {
                            'n' => string_value.push('\n'),
                            't' => string_value.push('\t'),
                            '\\' => string_value.push('\\'),
                            '"' => string_value.push('"'),
                            _ => return Err(LemonError::new("Invalid escape character", line, column)),
                        }
                    }
                } else if c == '"' {
                    closed = true;
                    break;
                } else {
                    string_value.push(c);
                }
            }

            if !closed {
                return Err(LemonError::new("Unclosed string literal", line, column));
            }

            tokens.push(Token {
                token_type: TokenType::StringLiteral,
                value: string_value,
                line,
                column: start_column,
            });

            continue;
        }

        // =========================
        // Lógicos dobles: &&, ||
        // =========================
        if let Some(double_log) = try_double_char_logical(&mut chars) {
            flush_current(&mut current, &mut tokens, line, start_column);
            tokens.push(Token {
                token_type: TokenType::Logical(double_log),
                value: double_log.to_string(),
                line,
                column,
            });
            column += 2;
            continue;
        }

        // =========================
        // Operador lógico unitario: !
        // =========================
        if let Some(logical) = crate::grammar::logicals::Logical::from_str(&ch.to_string()) {
            flush_current(&mut current, &mut tokens, line, start_column);
            tokens.push(Token {
                token_type: TokenType::Logical(logical),
                value: ch.to_string(),
                line,
                column,
            });
            chars.next();
            column += 1;
            continue;
        }

        // =========================
        // Operadores dobles (==, +=, etc.)
        // =========================
        if let Some(double_op) = try_double_char_operator(&mut chars) {
            flush_current(&mut current, &mut tokens, line, start_column);
            tokens.push(Token {
                token_type: TokenType::Operator(double_op.clone()),
                value: double_op.to_string(),
                line,
                column,
            });
            column += 2;
            continue;
        }

        // =========================
        // Fin de línea
        // =========================
        if ch == '\n' {
            flush_current(&mut current, &mut tokens, line, start_column);
            chars.next();
            line += 1;
            column = 1;
        }

        // =========================
        // Espacios y tabulaciones
        // =========================
        else if ch.is_whitespace() {
            flush_current(&mut current, &mut tokens, line, start_column);
            chars.next();
            column += 1;
        }

        // =========================
        // Símbolos simples: ( ) { } , ; ...
        // =========================
        else if let Some(symbol) = crate::grammar::symbols::Symbol::from_char(ch) {
            flush_current(&mut current, &mut tokens, line, start_column);
            tokens.push(Token {
                token_type: TokenType::Symbol(symbol.clone()),
                value: symbol.to_char().to_string(),
                line,
                column,
            });
            chars.next();
            column += 1;
        }

        // =========================
        // Caracteres inválidos
        // =========================
        else if ch.is_control() {
            return Err(LemonError::new("Illegal control character", line, column));
        } else if !ch.is_ascii_graphic() && !ch.is_whitespace() {
            return Err(LemonError::new("Illegal character in input", line, column));
        }

        // =========================
        // Identificadores o palabras clave
        // =========================
        else {
            if current.is_empty() {
                start_column = column;
            }
            current.push(ch);
            chars.next();
            column += 1;
        }
    }

    // Procesar la última palabra acumulada (si hay)
    flush_current(&mut current, &mut tokens, line, start_column);

    // Agregar token de fin de archivo
    tokens.push(Token {
        token_type: TokenType::EOF,
        value: "<EOF>".to_string(),
        line,
        column,
    });

    Ok(tokens)
}

/// Si hay texto acumulado, clasifícalo como un token y agrégalo al vector
fn flush_current(current: &mut String, tokens: &mut Vec<Token>, line: usize, column: usize) {
    if current.is_empty() {
        return;
    }
    tokens.push(classify(current, line, column));
    current.clear();
}

/// Clasifica una palabra como keyword, tipo, operador, identificador, número, etc.
fn classify(word: &str, line: usize, column: usize) -> Token {
    use TokenType::*;
    let value = word.to_string();

    if let Ok(_) = word.replace(",", ".").parse::<f64>() {
        return Token { token_type: Number, value, line, column };
    }

    if let Some(kw) = crate::grammar::keywords::Keyword::from_str(word) {
        return Token { token_type: Keyword(kw), value, line, column };
    }

    if let Some(ty) = crate::grammar::types::Type::from_str(word) {
        return Token { token_type: Type(ty), value, line, column };
    }

    if let Some(op) = crate::grammar::operators::Operator::from_str(word) {
        return Token { token_type: Operator(op), value, line, column };
    }

    if let Some(cmp) = crate::grammar::comparators::Comparator::from_str(word) {
        return Token { token_type: Comparator(cmp), value, line, column };
    }

    if let Some(log) = crate::grammar::logicals::Logical::from_str(word) {
        return Token { token_type: Logical(log), value, line, column };
    }

    if let Some(comment) = crate::grammar::comments::Comment::from_str(word) {
        return Token { token_type: Comment(comment), value, line, column };
    }

    // Número mal formado (empieza con dígito pero tiene basura)
    if word.starts_with(|c: char| c.is_ascii_digit()) {
        return Token { token_type: Unknown, value, line, column };
    }

    // Identificador válido
    if word.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Token { token_type: Identifier, value, line, column };
    }

    // Token desconocido
    Token { token_type: Unknown, value, line, column }
}

/// Detecta operadores de dos caracteres (==, +=, <=, etc.)
fn try_double_char_operator(
    chars: &mut std::iter::Peekable<std::str::Chars>,
) -> Option<crate::grammar::operators::Operator> {
    let ch1 = chars.peek().copied()?;
    let ch2 = chars.clone().nth(1)?;
    let pair = format!("{}{}", ch1, ch2);

    crate::grammar::operators::Operator::from_str(&pair).map(|op| {
        chars.next(); // consume ch1
        chars.next(); // consume ch2
        op
    })
}

/// Detecta operadores lógicos dobles: &&, ||
fn try_double_char_logical(
    chars: &mut std::iter::Peekable<std::str::Chars>,
) -> Option<crate::grammar::logicals::Logical> {
    let ch1 = chars.peek().copied()?;
    let ch2 = chars.clone().nth(1)?;
    let pair = format!("{}{}", ch1, ch2);

    crate::grammar::logicals::Logical::from_str(&pair).map(|log| {
        chars.next();
        chars.next();
        log
    })
}
