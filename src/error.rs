//! Error types for the JSON parser.

use std::fmt;

/// A specialized Result type for JSON parsing operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Represents errors that can occur during JSON parsing.
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    /// An unexpected character was encountered.
    UnexpectedCharacter { ch: char, position: usize },
    /// An unexpected end of input was reached.
    UnexpectedEndOfInput,
    /// An invalid number format was encountered.
    InvalidNumber { position: usize },
    /// An invalid string format was encountered.
    InvalidString { position: usize },
    /// An invalid escape sequence was encountered.
    InvalidEscape { ch: char, position: usize },
    /// A syntax error was encountered.
    SyntaxError { message: String, position: usize },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::UnexpectedCharacter { ch, position } => {
                write!(f, "Unexpected character '{}' at position {}", ch, position)
            }
            Error::UnexpectedEndOfInput => {
                write!(f, "Unexpected end of input")
            }
            Error::InvalidNumber { position } => {
                write!(f, "Invalid number format at position {}", position)
            }
            Error::InvalidString { position } => {
                write!(f, "Invalid string format at position {}", position)
            }
            Error::InvalidEscape { ch, position } => {
                write!(f, "Invalid escape sequence '\\{}' at position {}", ch, position)
            }
            Error::SyntaxError { message, position } => {
                write!(f, "Syntax error: {} at position {}", message, position)
            }
        }
    }
}

impl std::error::Error for Error {}