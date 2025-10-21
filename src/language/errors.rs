use std::fmt;

#[derive(Debug, Clone)]
pub struct LangError {
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct ProgramError {
    pub message: String,
    pub line_number: usize,
    pub line_text: String,
}

impl ProgramError {
    pub fn new(message: String, line_number: usize, line_text: String) -> Self {
        ProgramError {
            message,
            line_number,
            line_text,
        }
    }
}

impl LangError {
    pub fn new(message: String) -> Self {
        LangError { message }
    }
}

impl fmt::Display for LangError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\x1b[1;31m[Error]:\x1b[0m {}", self.message)
    }
}

impl fmt::Display for ProgramError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\x1b[1;31m[Error]:\x1b[0m {}\n| On line \x1b[1;33m[{}]:\x1b[0m \"\x1b[1;35m{}\x1b[0m\"", self.message, self.line_number, self.line_text)
    }
}

impl std::error::Error for LangError {}