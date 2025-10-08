use std::{
    fmt::{self},
    io,
};

#[derive(Debug)]
pub enum RloxError {
    /// Convert from std::io::Error.
    IOError(io::Error),
    /// Lexical error durning scanning tokens.
    LexicalError(usize, String, String),
    /// Error durning scanning tokens.
    ScannerError,
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
        }
    }
}

/// Report a RloxError.
pub fn report(e: &RloxError) {
    eprintln!("{e}")
}
