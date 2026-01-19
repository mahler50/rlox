use std::{
    fmt::{self},
    io,
};

#[derive(Debug)]
pub enum RloxError {
    /// Convert from std::io::Error.
    IOError(io::Error),
    /// Lexical error during scanning tokens.
    LexicalError(usize, String, String),
    /// Error during scanning tokens.
    ScannerError,
    /// Syntax error during parsing.
    SyntaxError(usize, String, String),
}

impl From<io::Error> for RloxError {
    fn from(value: io::Error) -> Self {
        RloxError::IOError(value)
    }
}

impl fmt::Display for RloxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RloxError::IOError(e) => write!(f, "IO Error: {e}."),
            RloxError::LexicalError(line, near, message) => write!(
                f,
                "Lexical Error: [line: {line}, near: {near}, message: {message}].",
            ),
            RloxError::ScannerError => write!(f, "Scanner Error."),
            RloxError::SyntaxError(line, near, message) => write!(
                f,
                "Syntax Error: [line: {line}, near: {near}, message: {message}]."
            ),
        }
    }
}

/// Report a RloxError.
pub fn report(e: &RloxError) {
    eprintln!("{e}")
}
