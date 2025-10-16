use std::fmt;

#[derive(Debug, Clone)]
pub struct LangError {
    pub message: String,
}

impl LangError {
    pub fn new(message: String) -> Self {
        LangError { message }
    }
}

impl fmt::Display for LangError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\x1b[0;31m[Error]\x1b[0m: {}", self.message)
    }
}

impl std::error::Error for LangError {}