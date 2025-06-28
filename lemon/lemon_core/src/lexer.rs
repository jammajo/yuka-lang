use crate::token::{Token, TokenType};
use crate::error::LemonError;

pub fn tokenize(code: &str) -> Result<Vec<Token>, LemonError> {
    let mut tokens = Vec::new();
    let mut current = String::new();

    for ch in code.chars() {
        if ch.is_whitespace() {
            if !current.is_empty() {
                tokens.push(classify(&current));
                current.clear();
            }
        } else if let Some(symbol) = crate::grammar::symbols::Symbol::from_char(ch) {
            if !current.is_empty() {
                tokens.push(classify(&current));
                current.clear();
            }
            tokens.push(Token {
                token_type: TokenType::Symbol(symbol.clone()),
                value: symbol.to_char().to_string(), // puedes cambiar esto a symbol.to_char().to_string() si usas el método `to_char()`
            });
        } else {
            current.push(ch);
        }
    }

    if !current.is_empty() {
        tokens.push(classify(&current));
    }

    Ok(tokens)
}

fn classify(word: &str) -> Token {
    use TokenType::*;

    if let Ok(_) = word.parse::<f64>() {
        Token {
            token_type: Number,
            value: word.to_string(),
        }
    } else if let Some(kw) = crate::grammar::keywords::Keyword::from_str(word) {
        Token {
            token_type: Keyword(kw),
            value: word.to_string(),
        }
    } else if word.chars().all(|c| c.is_alphanumeric() || c == '_') {
        Token {
            token_type: Identifier,
            value: word.to_string(),
        }
    } else {
        Token {
            token_type: Unknown,
            value: word.to_string(),
        }
    }
}
