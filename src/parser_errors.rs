use crate::jsonvalue::TokenKind;
use crate::lexer_error::LexerError;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ParserErrorKind {
    UnexpectedToken(TokenKind),
    UnexpectedEOF,
    // you can add "ExpectedButFound" variants for richer errors
    ExpectedToken(TokenKind, TokenKind), // expected, found
    ExpectedObjectEndOrComma(TokenKind), // found
    TrailingComma,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParserError {
    Lexer(LexerError),
    Parser {
        kind: ParserErrorKind,
        line: usize,
        column: usize,
    },
}

impl fmt::Display for ParserErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserErrorKind::UnexpectedToken(tok) => write!(f, "Unexpected token: {}", tok),
            ParserErrorKind::UnexpectedEOF => write!(f, "Unexpected end of input"),
            ParserErrorKind::ExpectedToken(expected, found) => {
                write!(f, "Expected token {}, but found {}", expected, found)
            }
            ParserErrorKind::ExpectedObjectEndOrComma(found) => {
                write!(f, "Expected ',' or '}}' in object, but found {}", found)
            }
            ParserErrorKind::TrailingComma => write!(f, "Trailing comma found"),
        }
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::Lexer(e) => write!(f, "Lexer error: {}", e),
            ParserError::Parser { kind, line, column } => {
                write!(
                    f,
                    "Parse error at line {}, column {}: {}",
                    line, column, kind
                )
            }
        }
    }
}

impl ParserError {
    pub fn pretty_print(&self, input: &str) {
        let lines: Vec<&str> = input.lines().collect();

        match self {
            ParserError::Parser { kind, line, column } => {
                let token_len = match kind {
                    ParserErrorKind::UnexpectedToken(tok) => tok.display_len(),
                    ParserErrorKind::ExpectedToken(_, tok) => tok.display_len(),
                    ParserErrorKind::ExpectedObjectEndOrComma(tok) => tok.display_len(),
                    ParserErrorKind::UnexpectedEOF => 1,
                    ParserErrorKind::TrailingComma => 1,
                };

                eprintln!(
                    "\x1b[31mParse error at line {}, column {}: {}\x1b[0m",
                    line, column, kind
                );

                if let Some(code_line) = lines.get(line - 1) {
                    eprintln!("{:>4} | {}", line, code_line);
                    eprintln!(
                        "     | {:>width$}{}",
                        "",
                        "^".repeat(token_len.max(1)),
                        width = column - 1
                    );
                }
            }
            ParserError::Lexer(err) => {
                eprintln!("\x1b[31mLexer error: {}\x1b[0m", err);

                if let Some(code_line) = lines.get(err.line - 1) {
                    eprintln!("{:>4} | {}", err.line, code_line);
                    eprintln!("     | {:>width$}^", "", width = err.column - 1);
                }
            }
        }
    }
}
