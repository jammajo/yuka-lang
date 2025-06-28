use crate::token::{Token, TokenType};
use crate::error::LemonError;

pub fn tokenize(code: &str) -> Result<Vec<Token>, LemonError> {
    let mut tokens = Vec::new();
    let mut current = String::new();

    let mut chars = code.chars().peekable();

    while let Some(&ch) = chars.peek() {
        if ch.is_whitespace() {
            flush_current(&mut current, &mut tokens);
            chars.next();
        } else if let Some(symbol) = crate::grammar::symbols::Symbol::from_char(ch) {
            flush_current(&mut current, &mut tokens);
            tokens.push(Token {
                token_type: TokenType::Symbol(symbol.clone()),
                value: symbol.to_char().to_string(),
            });
            chars.next();
        } else {
            current.push(ch);
            chars.next();
        }
    }

    flush_current(&mut current, &mut tokens);
    Ok(tokens)
}

fn flush_current(current: &mut String, tokens: &mut Vec<Token>) {
    if current.is_empty() {
        return;
    }

    tokens.push(classify(current));
    current.clear();
}

fn classify(word: &str) -> Token {
    use TokenType::*;

    // Detectar números (con coma como decimal opcional)
    if let Ok(_) = word.replace(",", ".").parse::<f64>() {
        return Token {
            token_type: Number,
            value: word.to_string(),
        };
    }

    // Keyword
    if let Some(kw) = crate::grammar::keywords::Keyword::from_str(word) {
        return Token {
            token_type: Keyword(kw),
            value: word.to_string(),
        };
    }

    // Tipo (como number.int, number.float, string, etc.)
    if let Some(ty) = crate::grammar::types::Type::from_str(word) {
        return Token {
            token_type: Type(ty),
            value: word.to_string(),
        };
    }

    // Operador
    if let Some(op) = crate::grammar::operators::Operator::from_str(word) {
        return Token {
            token_type: Operator(op),
            value: word.to_string(),
        };
    }

    // Comparador
    if let Some(cmp) = crate::grammar::comparators::Comparator::from_str(word) {
        return Token {
            token_type: Comparator(cmp),
            value: word.to_string(),
        };
    }

    // Lógico
    if let Some(log) = crate::grammar::logicals::Logical::from_str(word) {
        return Token {
            token_type: Logical(log),
            value: word.to_string(),
        };
    }

    // Comentario (si es línea completa o bloque)
    if let Some(comment) = crate::grammar::comments::Comment::from_str(word) {
        return Token {
            token_type: Comment(comment),
            value: word.to_string(),
        };
    }

    // Identificador (letras, números, guion bajo)
    if word.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Token {
            token_type: Identifier,
            value: word.to_string(),
        };
    }

    // Si no se reconoce, se marca como desconocido
    Token {
        token_type: Unknown,
        value: word.to_string(),
    }
}
