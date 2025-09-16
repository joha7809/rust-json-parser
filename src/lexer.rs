use std::{char, str::Chars};

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

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
}

pub struct Lexer<'a> {
    // Define the fields for the Lexer struct
    // e.g., input string, current position, etc.
    input: std::iter::Peekable<Chars<'a>>,
    line: usize,   // Current line Number
    column: usize, // Current column number
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, ()>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Ok(Token {
                kind: TokenKind::EOF,
                ..
            }) => None,
            Ok(token) => Some(Ok(token)),
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
        }
    }

    pub fn next_token(&mut self) -> Result<Token, ()> {
        // Start by skipping whitespace
        self.skip_whitespace();
        let next_char = self.peek();
        let c = match next_char {
            Some(c) => c,
            None => {
                return Ok(Token {
                    kind: TokenKind::EOF,
                    line: self.line,
                    column: self.column,
                });
            }
        };
        match c {
            '{' => Ok(self.new_simple_token(TokenKind::LeftBrace)),
            '}' => Ok(self.new_simple_token(TokenKind::RightBrace)),
            '[' => Ok(self.new_simple_token(TokenKind::LeftBracket)),
            ']' => Ok(self.new_simple_token(TokenKind::RightBracket)),
            ':' => Ok(self.new_simple_token(TokenKind::Colon)),
            ',' => Ok(self.new_simple_token(TokenKind::Comma)),
            '"' => {
                let string_value = self.read_string()?;
                Ok(Token {
                    kind: TokenKind::String(string_value),
                    line: self.line,
                    column: self.column,
                })
            }
            n if n.is_ascii_digit() || n == '-' => {
                let number_value = self.read_number()?;
                Ok(Token {
                    kind: TokenKind::Number(number_value),
                    line: self.line,
                    column: self.column,
                })
            }

            'f' | 't' | 'n' => {
                todo!();
            }

            _ => Err(()), // Handle other characters or errors
        }
    }

    /// Generates a simple token, that is those of one char. Do not use on other kinds, as the
    /// function advances
    fn new_simple_token(&mut self, kind: TokenKind) -> Token {
        self.advance();
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

    // TODO: IMplement error handling for this and above function
    fn read_string(&mut self) -> Result<String, ()> {
        let mut result = String::new();
        self.advance(); // Skip the opening quote

        while let Some(c) = self.peek() {
            if c == '"' {
                self.advance();
                return Ok(result);
            }
            //TODO: IMplement escape characters
            result.push(c);
            self.advance();
        }
        Err(())
    }

    fn read_number(&mut self) -> Result<f64, ()> {
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

        nums.parse::<f64>().map_err(|_| ())
    }

    fn read_integer(&mut self, nums: &mut String) -> Result<(), ()> {
        // Read integers
        match self.peek() {
            Some('0') => {
                nums.push('0');
                self.advance();
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
            _ => return Err(()),
        }
        Ok(())
    }

    fn read_fraction(&mut self, nums: &mut String) -> Result<(), ()> {
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
                return Err(());
            }
        }
        Ok(())
    }

    fn read_exponent(&mut self, nums: &mut String) -> Result<(), ()> {
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
                }
            }

            if !digit_found {
                return Err(());
            }
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
}
