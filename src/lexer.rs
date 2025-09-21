use std::{char, str::Chars};

use crate::{
    jsonvalue::TokenKind,
    lexer_error::{LexerError, LexerErrorKinds},
};

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
}

pub(crate) struct Lexer<'a> {
    // Define the fields for the Lexer struct
    // e.g., input string, current position, etc.
    input: std::iter::Peekable<Chars<'a>>,
    line: usize,   // Current line Number
    column: usize, // Current column number
    finished: bool,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, LexerError>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        match self.next_token() {
            Ok(token) => {
                if let TokenKind::EOF = token.kind {
                    // Emit EOF once, then mark finished so subsequent next() returns None
                    self.finished = true;
                    Some(Ok(token))
                } else {
                    Some(Ok(token))
                }
            }
            Err(e) => Some(Err(e)),
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let chars = input.chars().peekable();

        Self {
            input: chars,
            line: 1,
            column: 1,
            finished: false,
        }
    }

    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        // Start by skipping whitespace
        self.skip_whitespace();
        let next_char = self.peek();
        let c = match next_char {
            Some(c) => c,
            None => {
                return Ok(self.new_token(TokenKind::EOF));
            }
        };
        match c {
            '{' => Ok(self.new_token_advance(TokenKind::LeftBrace)),
            '}' => Ok(self.new_token_advance(TokenKind::RightBrace)),
            '[' => Ok(self.new_token_advance(TokenKind::LeftBracket)),
            ']' => Ok(self.new_token_advance(TokenKind::RightBracket)),
            ':' => Ok(self.new_token_advance(TokenKind::Colon)),
            ',' => Ok(self.new_token_advance(TokenKind::Comma)),
            '"' => {
                let start_column = self.column;
                let string_value = self.read_string()?;
                Ok(Token {
                    kind: TokenKind::String(string_value),
                    line: self.line,
                    column: start_column,
                })
            }
            n if n.is_ascii_digit() || n == '-' => {
                let number_value = self.read_number()?;
                Ok(self.new_token(TokenKind::Number(number_value)))
            }

            n @ ('f' | 't' | 'n') => {
                let token_kind = self.read_literal(n)?;
                Ok(self.new_token(token_kind))
            }
            c => {
                let err = self.return_error(LexerErrorKinds::UnexcpectedChar(c));
                self.advance(); // skip it!
                Err(err)
            }
        }
    }

    /// Generates a simple token, that is those of one char. Do not use on other kinds, as the
    /// function advances
    fn new_token_advance(&mut self, kind: TokenKind) -> Token {
        let token = Token {
            kind,
            line: self.line,
            column: self.column,
        };
        self.advance();
        token
    }

    fn new_token(&mut self, kind: TokenKind) -> Token {
        Token {
            kind,
            line: self.line,
            column: self.column,
        }
    }

    fn advance(&mut self) {
        match self.input.next() {
            Some('\n') => {
                self.line += 1;
                self.column = 1; // Reset column on new line
            }
            Some(_) => self.column += 1,
            None => {} // End of input
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.input.peek().copied()
    }

    fn skip_whitespace(&mut self) {
        while let Some(char) = self.peek() {
            if char.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_string(&mut self) -> Result<String, LexerError> {
        let mut result = String::new();
        self.advance(); // Skip the opening quote

        while let Some(c) = self.peek() {
            if c == '"' {
                self.advance();
                return Ok(result);
            }
            if c == '\n' || c == '\r' {
                return Err(self.return_error(LexerErrorKinds::UnclosedString));
            }
            if c < '\u{20}' {
                return Err(self.return_error(LexerErrorKinds::UnescapedControlCharacter));
            }
            if c == '\\' {
                self.read_escape_sequence(&mut result)?;
            } else {
                result.push(c);
                self.advance();
            }
        }
        Err(self.return_error(LexerErrorKinds::UnclosedString))
    }

    fn read_number(&mut self) -> Result<f64, LexerError> {
        let mut nums = String::new();

        // Check for optional minus
        if let Some('-') = self.peek() {
            nums.push('-');
            self.advance(); // consume it
        }

        // Read integers
        self.read_integer(&mut nums)?;

        // Optional fraction
        self.read_fraction(&mut nums)?;

        // Optional exponent
        self.read_exponent(&mut nums)?;

        nums.parse::<f64>()
            .map_err(|_| self.return_error(LexerErrorKinds::CastingError))
    }

    fn read_integer(&mut self, nums: &mut String) -> Result<(), LexerError> {
        // Read integers
        match self.peek() {
            Some('0') => {
                nums.push('0');
                self.advance();
                // 0 cannot be followed by other digits
                if let Some(c) = self.peek() {
                    if c.is_ascii_digit() {
                        return Err(self.return_error(LexerErrorKinds::LeadingZero));
                    }
                }
            }
            Some(c) if c.is_ascii_digit() => {
                while let Some(d) = self.peek() {
                    if d.is_ascii_digit() {
                        nums.push(d);
                        self.advance()
                    } else {
                        break;
                    }
                }
            }
            _ => return Err(self.return_error(LexerErrorKinds::InvalidNumber)),
        }
        Ok(())
    }

    fn read_fraction(&mut self, nums: &mut String) -> Result<(), LexerError> {
        if let Some('.') = self.peek() {
            let mut num_found = false;
            nums.push('.');
            self.advance();

            while let Some(d) = self.peek() {
                // We continue appending numeric values
                if d.is_ascii_digit() {
                    nums.push(d);
                    self.advance();
                    num_found = true;
                } else {
                    break;
                }
            }

            if !num_found {
                return Err(self.return_error(LexerErrorKinds::InvalidDecimal));
            }
        }
        Ok(())
    }

    fn read_exponent(&mut self, nums: &mut String) -> Result<(), LexerError> {
        if let Some('e') | Some('E') = self.peek() {
            nums.push('e');
            self.advance();

            // Exponent can be followed by optional sign + or -
            if let Some(sign @ ('+' | '-')) = self.peek() {
                nums.push(sign);
                self.advance();
            }

            // If we have an exponent, atleast a digit must follow
            let mut digit_found = false;
            while let Some(d) = self.peek() {
                if d.is_ascii_digit() {
                    nums.push(d);
                    self.advance();
                    digit_found = true;
                } else {
                    break;
                }
            }

            if !digit_found {
                return Err(self.return_error(LexerErrorKinds::InvalidExponent));
            }
        }
        Ok(())
    }

    fn return_error(&self, kind: LexerErrorKinds) -> LexerError {
        LexerError {
            kind,
            line: self.line,
            column: self.column,
        }
    }

    fn read_literal(&mut self, b_char: char) -> Result<TokenKind, LexerError> {
        let literal = match b_char {
            't' => "true",
            'f' => "false",
            'n' => "null",
            _ => unreachable!(),
        };
        for expected_char in literal.chars() {
            match self.peek() {
                Some(c) if c == expected_char => self.advance(),
                _ => return Err(self.return_error(LexerErrorKinds::InvalidLiteral)),
            }
        }
        if self.peek().is_some_and(|c| c.is_ascii_alphanumeric()) {
            return Err(self.return_error(LexerErrorKinds::InvalidLiteral));
        }
        Ok(match b_char {
            't' => TokenKind::True,
            'f' => TokenKind::False,
            'n' => TokenKind::Null,
            _ => unreachable!(),
        })
    }

    fn read_escape_sequence(&mut self, result: &mut String) -> Result<(), LexerError> {
        self.advance(); // Skip the backslash
        match self.peek() {
            Some('\"') => {
                result.push('\"');
                self.advance();
            }
            Some('\\') => {
                result.push('\\');
                self.advance();
            }
            Some('/') => {
                result.push('/');
                self.advance();
            }
            Some('b') => {
                result.push('\u{0008}'); // Backspace
                self.advance();
            }
            Some('f') => {
                result.push('\u{000C}'); // Formfeed
                self.advance();
            }
            Some('n') => {
                result.push('\n');
                self.advance();
            }
            Some('r') => {
                result.push('\r');
                self.advance();
            }
            Some('t') => {
                result.push('\t');
                self.advance();
            }
            Some('u') => {
                self.advance(); // Skip 'u'
                let mut hex = String::new();
                for _ in 0..4 {
                    match self.peek() {
                        Some(c) if c.is_ascii_hexdigit() => {
                            hex.push(c);
                            self.advance();
                        }
                        _ => return Err(self.return_error(LexerErrorKinds::InvalidEscape)),
                    }
                }
                let code_point = u32::from_str_radix(&hex, 16)
                    .map_err(|_| self.return_error(LexerErrorKinds::InvalidEscape))?;
                if let Some(ch) = char::from_u32(code_point) {
                    result.push(ch);
                } else {
                    return Err(self.return_error(LexerErrorKinds::InvalidEscape));
                }
            }
            Some(c) => return Err(self.return_error(LexerErrorKinds::InvalidEscapeChar(c))),
            None => return Err(self.return_error(LexerErrorKinds::InvalidEscape)),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lex_all(input: &str) -> Vec<TokenKind> {
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        loop {
            let token = lexer.next_token().unwrap();
            let Token { kind, .. } = token.clone();
            {
                if kind == TokenKind::EOF {
                    break;
                }
                tokens.push(kind);
            }
        }
        tokens
    }

    #[test]
    fn test_single_char_tokens() {
        let input = "{\"hello\" } [ ] : ,";
        let expected = vec![
            TokenKind::LeftBrace,
            TokenKind::String("hello".into()),
            TokenKind::RightBrace,
            TokenKind::LeftBracket,
            TokenKind::RightBracket,
            TokenKind::Colon,
            TokenKind::Comma,
        ];
        assert_eq!(lex_all(input), expected);
    }

    #[test]
    fn test_simple_string() {
        let input = r#""hello" "world""#;
        let expected = vec![
            TokenKind::String("hello".into()),
            TokenKind::String("world".into()),
        ];
        assert_eq!(lex_all(input), expected);
    }

    #[test]
    fn test_simple_numbers() {
        let input = "0 123 -456 -0.2e2";
        let expected = vec![
            TokenKind::Number(0.0),
            TokenKind::Number(123.0),
            TokenKind::Number(-456.0),
            TokenKind::Number(-20.0),
        ];
        assert_eq!(lex_all(input), expected);
    }

    #[test]
    fn test_mixed_input() {
        let input = r#"{ "key": -42 }"#;
        let expected = vec![
            TokenKind::LeftBrace,
            TokenKind::String("key".into()),
            TokenKind::Colon,
            TokenKind::Number(-42.0),
            TokenKind::RightBrace,
        ];
        assert_eq!(lex_all(input), expected);
    }

    #[test]
    fn test_whitespace_skipping() {
        let input = "   { \n\t: , }  ";
        let expected = vec![
            TokenKind::LeftBrace,
            TokenKind::Colon,
            TokenKind::Comma,
            TokenKind::RightBrace,
        ];
        assert_eq!(lex_all(input), expected);
    }
    #[test]
    fn test_unexpected_char_error() {
        let mut lexer = Lexer::new("@");
        let result = lexer.next_token();
        assert!(matches!(
            result,
            Err(LexerError {
                kind: LexerErrorKinds::UnexcpectedChar('@'),
                ..
            })
        ));
    }

    #[test]
    fn test_unclosed_string_error() {
        let mut lexer = Lexer::new("\"unterminated");
        let result = lexer.next_token();
        assert!(matches!(
            result,
            Err(LexerError {
                kind: LexerErrorKinds::UnclosedString,
                ..
            })
        ));
    }

    #[test]
    fn test_leading_zero_error() {
        let mut lexer = Lexer::new("0123");
        let result = lexer.next_token();
        assert!(matches!(
            result,
            Err(LexerError {
                kind: LexerErrorKinds::LeadingZero,
                ..
            })
        ));
    }

    #[test]
    fn test_invalid_number_error() {
        let mut lexer = Lexer::new("-");
        let result = lexer.next_token();
        assert!(matches!(
            result,
            Err(LexerError {
                kind: LexerErrorKinds::InvalidNumber,
                ..
            })
        ));
    }

    #[test]
    fn test_invalid_decimal_error() {
        let mut lexer = Lexer::new("1.");
        let result = lexer.next_token();
        assert!(matches!(
            result,
            Err(LexerError {
                kind: LexerErrorKinds::InvalidDecimal,
                ..
            })
        ));
    }

    #[test]
    fn test_invalid_exponent_error() {
        let mut lexer = Lexer::new("1e");
        let result = lexer.next_token();
        assert!(matches!(
            result,
            Err(LexerError {
                kind: LexerErrorKinds::InvalidExponent,
                ..
            })
        ));
    }
}
