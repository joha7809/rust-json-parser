use std::collections::HashMap;

use crate::{
    errors::{LexerError, ParserError, ParserErrorKind},
    jsonvalue::{JSONValue, TokenKind},
    lexer::{Lexer, Token},
};

pub fn parse_json(input: &str) -> Result<JSONValue, ParserError> {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    parser.parse()
}

type PResult<T> = Result<T, ParserError>;

pub struct Parser<I>
where
    I: Iterator<Item = Result<Token, LexerError>>,
{
    lexer: std::iter::Peekable<I>,
}

impl<I> Parser<I>
where
    I: Iterator<Item = Result<Token, LexerError>>,
{
    pub fn new(lexer: I) -> Self {
        Self {
            lexer: lexer.peekable(),
        }
    }

    fn peek(&mut self) -> Result<&Token, ParserError> {
        match self.lexer.peek() {
            Some(Ok(token)) => Ok(token),
            Some(Err(error)) => Err(ParserError::Lexer(error.clone())),
            None => unreachable!("Lexer should always produce EOF token"),
        }
    }

    fn peek_kind(&mut self) -> Result<&TokenKind, ParserError> {
        Ok(&self.peek()?.kind)
    }

    fn advance(&mut self) -> Result<Token, ParserError> {
        match self.lexer.next() {
            Some(Ok(token)) => Ok(token),
            Some(Err(error)) => Err(ParserError::Lexer(error)),
            None => unreachable!("Lexer should always produce EOF token"),
        }
    }

    fn advance_kind(&mut self) -> Result<TokenKind, ParserError> {
        Ok(self.advance()?.kind)
    }

    fn expect(&mut self, kind: TokenKind) -> Result<Token, ParserError> {
        // Uses advance_kind to advance and look at the kind returned
        let k = self.advance()?;
        if k.kind != kind {
            return Err(ParserError::Parser {
                kind: ParserErrorKind::ExpectedToken(kind, k.kind),
                line: k.line,
                column: k.column,
            });
        }
        Ok(k)
    }

    fn expect_either(&mut self, kinds: &[TokenKind]) -> Result<Token, ParserError> {
        // TODO: Pretty hacky but works i guess?
        let k = self.advance()?;
        if !kinds.contains(&k.kind) {
            return Err(ParserError::Parser {
                kind: ParserErrorKind::ExpectedOneOfTokens(kinds.to_vec(), k.kind),
                line: k.line,
                column: k.column,
            });
        }
        Ok(k)
    }

    pub fn parse(&mut self) -> PResult<JSONValue> {
        let value = self.parse_value()?;
        // After parsing the value, we expect an EOF token
        self.expect(TokenKind::EOF)?;

        Ok(value)
    }

    fn parse_value(&mut self) -> PResult<JSONValue> {
        // This function should look at the next token and decide which parse_* function to call
        let token = self.peek()?;
        match &token.kind {
            TokenKind::LeftBrace => self.parse_object(),
            TokenKind::LeftBracket => self.parse_array(),
            TokenKind::String(_) => {
                if let TokenKind::String(s) = self.advance_kind()? {
                    Ok(JSONValue::String(s))
                } else {
                    unreachable!()
                }
            }
            TokenKind::Number(_) => {
                if let TokenKind::Number(n) = self.advance_kind()? {
                    Ok(JSONValue::Number(n))
                } else {
                    unreachable!()
                }
            }
            TokenKind::True => {
                self.advance_kind()?;
                Ok(JSONValue::Bool(true))
            }
            TokenKind::False => {
                self.advance_kind()?;
                Ok(JSONValue::Bool(false))
            }
            TokenKind::Null => {
                self.advance_kind()?;
                Ok(JSONValue::Null)
            }
            unexpected => Err(ParserError::Parser {
                kind: ParserErrorKind::UnexpectedToken(unexpected.clone()),
                line: (token.line),
                column: (token.column),
            }),
        }
    }

    fn parse_object(&mut self) -> PResult<JSONValue> {
        //TODO: low cost lookahead and count commas for capacity
        // Exoect the left bracket and consume it, propogating error if another type
        // Lets initialize a HashMap to hold the key-value pairs
        let mut map: HashMap<String, JSONValue> = std::collections::HashMap::with_capacity(8);
        self.expect(TokenKind::LeftBrace)?;

        if *self.peek_kind()? != TokenKind::RightBrace {
            loop {
                // We expect a string
                let token = self.advance()?;
                let key = match token.kind {
                    TokenKind::String(s) => s,
                    other => {
                        return Err(ParserError::Parser {
                            kind: ParserErrorKind::ExpectedToken(
                                TokenKind::String("".to_string()),
                                other,
                            ),
                            line: token.line,
                            column: token.column,
                        });
                    } //TODO: Imrpove error handling here
                };

                // After string we expect a colon
                self.expect(TokenKind::Colon)?;
                // Next we can pass the value
                let value = self.parse_value()?;

                map.insert(key, value);
                // next we peek at the next kind, if comma we do nothing, if right brace we break
                match self.peek_kind()? {
                    TokenKind::Comma => {
                        self.advance()?;
                    }
                    TokenKind::RightBrace => break,
                    other => {
                        return Err(ParserError::Parser {
                            kind: ParserErrorKind::ExpectedObjectEndOrComma(other.clone()),
                            line: self.peek()?.line,
                            column: self.peek()?.column,
                        });
                    }
                }
            }
        }

        self.expect(TokenKind::RightBrace)?;
        Ok(JSONValue::Object(map))
    }

    fn parse_array(&mut self) -> PResult<JSONValue> {
        //TODO: low cost lookahead and count commas for capacity
        // Consume left bracket
        self.expect(TokenKind::LeftBracket)?;
        let mut arr: Vec<JSONValue> = Vec::with_capacity(8);

        while *self.peek_kind()? != TokenKind::RightBracket {
            // While we dont see the RightBracket, we pass the current JSON value, and expect a
            // comma to be followed
            arr.push(self.parse_value()?);
            if *self.peek_kind()? != TokenKind::RightBracket {
                let token = self.expect_either(&[TokenKind::Comma, TokenKind::RightBracket])?;
                if *self.peek_kind()? == TokenKind::RightBracket {
                    return Err(ParserError::Parser {
                        kind: ParserErrorKind::TrailingComma,
                        line: token.line,
                        column: token.column,
                    });
                }
            }
        }
        // Next token is RightBracket which we can safely consume
        self.advance()?;

        Ok(JSONValue::Array(arr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn parses_simple_object() {
        let input = r#"{
            "a": "hello",
            "b": 123,
            "c": true,
            "d": false,
            "e": null
        }"#;
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let result = parser.parse();
        assert!(result.is_ok(), "Parser failed: {:?}", result);
        let json = result.unwrap();
        match json {
            JSONValue::Object(map) => {
                assert_eq!(map["a"], JSONValue::String("hello".to_string()));
                assert_eq!(map["b"], JSONValue::Number(123.0));
                assert_eq!(map["c"], JSONValue::Bool(true));
                assert_eq!(map["d"], JSONValue::Bool(false));
                assert_eq!(map["e"], JSONValue::Null);
            }
            _ => panic!("Expected JSON object"),
        }
    }
}
