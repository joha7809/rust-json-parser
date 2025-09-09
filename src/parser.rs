//! JSON parser for converting tokens into values.
//!
//! This module provides a recursive descent parser that converts
//! tokens from the lexer into JSON values.

use std::collections::HashMap;

use crate::error::{Error, Result};
use crate::lexer::{Lexer, Token};
use crate::value::Value;

/// A JSON parser.
pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    /// Creates a new parser for the given input.
    pub fn new(input: &str) -> Result<Self> {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token()?;
        
        Ok(Self {
            lexer,
            current_token,
        })
    }

    /// Advances to the next token.
    fn advance(&mut self) -> Result<()> {
        self.current_token = self.lexer.next_token()?;
        Ok(())
    }

    /// Parses a JSON value.
    fn parse_value(&mut self) -> Result<Value> {
        match &self.current_token {
            Token::String(s) => {
                let value = Value::String(s.clone());
                self.advance()?;
                Ok(value)
            }
            Token::Number(n) => {
                let value = Value::Number(*n);
                self.advance()?;
                Ok(value)
            }
            Token::Bool(b) => {
                let value = Value::Bool(*b);
                self.advance()?;
                Ok(value)
            }
            Token::Null => {
                self.advance()?;
                Ok(Value::Null)
            }
            Token::LeftBrace => self.parse_object(),
            Token::LeftBracket => self.parse_array(),
            _ => Err(Error::SyntaxError {
                message: format!("Unexpected token: {:?}", self.current_token),
                position: self.lexer.position(),
            }),
        }
    }

    /// Parses a JSON object.
    fn parse_object(&mut self) -> Result<Value> {
        let mut object = HashMap::new();
        
        self.advance()?; // Skip '{'
        
        // Handle empty object
        if matches!(self.current_token, Token::RightBrace) {
            self.advance()?;
            return Ok(Value::Object(object));
        }
        
        loop {
            // Parse key
            let key = match &self.current_token {
                Token::String(s) => s.clone(),
                _ => return Err(Error::SyntaxError {
                    message: "Expected string key in object".to_string(),
                    position: self.lexer.position(),
                }),
            };
            self.advance()?;
            
            // Expect colon
            if !matches!(self.current_token, Token::Colon) {
                return Err(Error::SyntaxError {
                    message: "Expected ':' after object key".to_string(),
                    position: self.lexer.position(),
                });
            }
            self.advance()?;
            
            // Parse value
            let value = self.parse_value()?;
            object.insert(key, value);
            
            // Check for continuation or end
            match &self.current_token {
                Token::Comma => {
                    self.advance()?;
                    continue;
                }
                Token::RightBrace => {
                    self.advance()?;
                    break;
                }
                _ => return Err(Error::SyntaxError {
                    message: "Expected ',' or '}' in object".to_string(),
                    position: self.lexer.position(),
                }),
            }
        }
        
        Ok(Value::Object(object))
    }

    /// Parses a JSON array.
    fn parse_array(&mut self) -> Result<Value> {
        let mut array = Vec::new();
        
        self.advance()?; // Skip '['
        
        // Handle empty array
        if matches!(self.current_token, Token::RightBracket) {
            self.advance()?;
            return Ok(Value::Array(array));
        }
        
        loop {
            // Parse value
            let value = self.parse_value()?;
            array.push(value);
            
            // Check for continuation or end
            match &self.current_token {
                Token::Comma => {
                    self.advance()?;
                    continue;
                }
                Token::RightBracket => {
                    self.advance()?;
                    break;
                }
                _ => return Err(Error::SyntaxError {
                    message: "Expected ',' or ']' in array".to_string(),
                    position: self.lexer.position(),
                }),
            }
        }
        
        Ok(Value::Array(array))
    }

    /// Parses the input and returns a JSON value.
    pub fn parse(&mut self) -> Result<Value> {
        let value = self.parse_value()?;
        
        // Ensure we've consumed all input
        if !matches!(self.current_token, Token::EOF) {
            return Err(Error::SyntaxError {
                message: "Unexpected trailing content".to_string(),
                position: self.lexer.position(),
            });
        }
        
        Ok(value)
    }
}

/// Parses a JSON string and returns a Value.
///
/// # Example
///
/// ```rust
/// use rust_json_parser::{parse, Value};
///
/// let json = r#"{"name": "Alice", "age": 30}"#;
/// let value = parse(json).expect("Failed to parse JSON");
/// 
/// assert!(value.is_object());
/// ```
pub fn parse(input: &str) -> Result<Value> {
    let mut parser = Parser::new(input)?;
    parser.parse()
}