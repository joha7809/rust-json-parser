//! JSON lexer for tokenizing input.
//!
//! This module provides a lexer that breaks JSON input into tokens
//! for easier parsing.

use crate::error::{Error, Result};

/// Represents a JSON token.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// Left brace `{`
    LeftBrace,
    /// Right brace `}`
    RightBrace,
    /// Left bracket `[`
    LeftBracket,
    /// Right bracket `]`
    RightBracket,
    /// Colon `:`
    Colon,
    /// Comma `,`
    Comma,
    /// String literal
    String(String),
    /// Number literal
    Number(f64),
    /// Boolean literal
    Bool(bool),
    /// Null literal
    Null,
    /// End of input
    EOF,
}

/// A lexer for JSON input.
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    /// Creates a new lexer for the given input.
    pub fn new(input: &str) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let current_char = chars.get(0).copied();
        
        Self {
            input: chars,
            position: 0,
            current_char,
        }
    }

    /// Returns the current position in the input.
    pub fn position(&self) -> usize {
        self.position
    }

    /// Advances to the next character.
    fn advance(&mut self) {
        self.position += 1;
        self.current_char = self.input.get(self.position).copied();
    }



    /// Skips whitespace characters.
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Reads a string literal.
    fn read_string(&mut self) -> Result<String> {
        let start_pos = self.position;
        self.advance(); // Skip opening quote
        
        let mut value = String::new();
        
        while let Some(ch) = self.current_char {
            match ch {
                '"' => {
                    self.advance(); // Skip closing quote
                    return Ok(value);
                }
                '\\' => {
                    self.advance();
                    match self.current_char {
                        Some('"') => value.push('"'),
                        Some('\\') => value.push('\\'),
                        Some('/') => value.push('/'),
                        Some('b') => value.push('\u{0008}'),
                        Some('f') => value.push('\u{000C}'),
                        Some('n') => value.push('\n'),
                        Some('r') => value.push('\r'),
                        Some('t') => value.push('\t'),
                        Some('u') => {
                            // TODO: Handle unicode escapes
                            return Err(Error::InvalidEscape { ch: 'u', position: self.position });
                        }
                        Some(escape_ch) => {
                            return Err(Error::InvalidEscape { ch: escape_ch, position: self.position });
                        }
                        None => return Err(Error::UnexpectedEndOfInput),
                    }
                    self.advance();
                }
                _ => {
                    value.push(ch);
                    self.advance();
                }
            }
        }
        
        Err(Error::InvalidString { position: start_pos })
    }

    /// Reads a number literal.
    fn read_number(&mut self) -> Result<f64> {
        let start_pos = self.position;
        let mut number_str = String::new();
        
        // Handle negative sign
        if self.current_char == Some('-') {
            number_str.push('-');
            self.advance();
        }
        
        // Read digits
        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() || ch == '.' || ch == 'e' || ch == 'E' || ch == '+' || ch == '-' {
                number_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        number_str.parse::<f64>()
            .map_err(|_| Error::InvalidNumber { position: start_pos })
    }

    /// Reads an identifier (true, false, null).
    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();
        
        while let Some(ch) = self.current_char {
            if ch.is_ascii_alphabetic() {
                identifier.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        identifier
    }

    /// Returns the next token.
    pub fn next_token(&mut self) -> Result<Token> {
        self.skip_whitespace();
        
        match self.current_char {
            None => Ok(Token::EOF),
            Some('{') => {
                self.advance();
                Ok(Token::LeftBrace)
            }
            Some('}') => {
                self.advance();
                Ok(Token::RightBrace)
            }
            Some('[') => {
                self.advance();
                Ok(Token::LeftBracket)
            }
            Some(']') => {
                self.advance();
                Ok(Token::RightBracket)
            }
            Some(':') => {
                self.advance();
                Ok(Token::Colon)
            }
            Some(',') => {
                self.advance();
                Ok(Token::Comma)
            }
            Some('"') => {
                let string = self.read_string()?;
                Ok(Token::String(string))
            }
            Some(ch) if ch.is_ascii_digit() || ch == '-' => {
                let number = self.read_number()?;
                Ok(Token::Number(number))
            }
            Some(ch) if ch.is_ascii_alphabetic() => {
                let identifier = self.read_identifier();
                match identifier.as_str() {
                    "true" => Ok(Token::Bool(true)),
                    "false" => Ok(Token::Bool(false)),
                    "null" => Ok(Token::Null),
                    _ => Err(Error::UnexpectedCharacter { ch, position: self.position }),
                }
            }
            Some(ch) => Err(Error::UnexpectedCharacter { ch, position: self.position }),
        }
    }
}