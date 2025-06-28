#[derive(Debug)]
pub struct LemonError {
    pub message: String,
}

impl LemonError {
    pub fn new(message: &str, line: usize, column: usize) -> Self {
        LemonError {
            message: format!("{} at line {}, column {}", message, line, column),
        }
    }

    pub fn with_message(message: &str) -> Self {
        LemonError {
            message: message.to_string(),
        }
    }
}

impl std::fmt::Display for LemonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for LemonError {}

impl From<&str> for LemonError {
    fn from(message: &str) -> Self {
        LemonError {
            message: message.to_string(),
        }
    }
}
