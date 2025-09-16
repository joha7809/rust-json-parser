use std::collections::HashMap;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum JSONValue {
    Array(Vec<JSONValue>),
    /// Classic boolean - true, false
    Bool(bool),
    /// Used to represent null value in JSON
    Null,
    /// floating point number for decimal numbers in JSON
    Number(f64),
    Object(HashMap<String, JSONValue>),
    String(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Colon,        // :
    Comma,        // ,
    String(String),
    Number(f64),
    True,
    False,
    Null,
    EOF,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::LeftBrace => write!(f, "{{"),
            TokenKind::RightBrace => write!(f, "}}"),
            TokenKind::LeftBracket => write!(f, "["),
            TokenKind::RightBracket => write!(f, "]"),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::String(s) => write!(f, "string: \"{}\"", s),
            TokenKind::Number(n) => write!(f, "number({})", n),
            TokenKind::True => write!(f, "true"),
            TokenKind::False => write!(f, "false"),
            TokenKind::Null => write!(f, "null"),
            TokenKind::EOF => write!(f, "EOF"),
        }
    }
}

impl TokenKind {
    pub fn display_len(&self) -> usize {
        match self {
            TokenKind::LeftBrace
            | TokenKind::RightBrace
            | TokenKind::LeftBracket
            | TokenKind::RightBracket
            | TokenKind::Colon
            | TokenKind::Comma => 1,
            TokenKind::String(s) => s.len() + 2, // include quotes
            TokenKind::Number(n) => n.to_string().len(),
            TokenKind::True => 4,
            TokenKind::False => 5,
            TokenKind::Null => 4,
            TokenKind::EOF => 1,
        }
    }
}
