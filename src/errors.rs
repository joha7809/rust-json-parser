use crate::jsonvalue::TokenKind;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ParserErrorKind {
    UnexpectedToken(TokenKind),
    UnexpectedEOF,
    // you can add "ExpectedButFound" variants for richer errors
    ExpectedToken(TokenKind, TokenKind), // expected, found
    ExpectedOneOfTokens(Vec<TokenKind>, TokenKind), // expected, found
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
            ParserErrorKind::UnexpectedToken(tok) => {
                write!(f, "Expected a value, received: {}", tok)
            }
            ParserErrorKind::UnexpectedEOF => write!(f, "Unexpected end of input"),
            ParserErrorKind::ExpectedToken(expected, found) => {
                write!(f, "Expected token {} but found {}", expected, found)
            }
            ParserErrorKind::ExpectedObjectEndOrComma(found) => {
                write!(f, "Expected ',' or '}}' in object but found {}", found)
            }
            ParserErrorKind::TrailingComma => write!(f, "Trailing comma found"),
            ParserErrorKind::ExpectedOneOfTokens(expected, found) => {
                let expected_list = expected
                    .iter()
                    .map(|tok| tok.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "Expected one of {} but found {}", expected_list, found)
            }
        }
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::Lexer(e) => write!(f, "{}", e),
            ParserError::Parser { kind, line, column } => {
                write!(f, "{}:{}: {}", line, column, kind)
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
                    ParserErrorKind::ExpectedOneOfTokens(_, tok) => tok.display_len(),
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

#[derive(Debug, Clone, PartialEq)]
pub enum LexerErrorKinds {
    UnexcpectedChar(char),
    UnescapedControlCharacter,
    InvalidEscapeChar(char),
    UnclosedString,
    LeadingZero,
    InvalidNumber,
    InvalidLiteral,
    InvalidDecimal,
    InvalidExponent,
    CastingError,
    InvalidEscape,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LexerError {
    pub kind: LexerErrorKinds,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for LexerError {
    // TODO: Instead of returning to_string for each case, use write! macro to write directly to the formatter
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = match self.kind {
            LexerErrorKinds::UnexcpectedChar(c) => format!("Unexpected character: '{}'", c),
            LexerErrorKinds::InvalidEscapeChar(c) => format!("Invalid escape character: '\\{}'", c),
            LexerErrorKinds::UnclosedString => "Unclosed string literal".to_string(),
            LexerErrorKinds::LeadingZero => "Number cannot have leading zeros".to_string(),
            LexerErrorKinds::InvalidNumber => "Invalid number format".to_string(),
            LexerErrorKinds::InvalidDecimal => "Invalid decimal format".to_string(),
            LexerErrorKinds::InvalidExponent => "Invalid exponent format".to_string(),
            LexerErrorKinds::CastingError => "Error casting number".to_string(),
            LexerErrorKinds::InvalidLiteral => "Invalid literal".to_string(),
            LexerErrorKinds::InvalidEscape => "Invalid escape sequence".to_string(),
            LexerErrorKinds::UnescapedControlCharacter => {
                "Unescaped control character in string".to_string()
            }
        };
        write!(f, "{}:{}: {}", self.line, self.column, description)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_unexpected_char_display() {
//         let err = LexerError {
//             kind: LexerErrorKinds::UnexcpectedChar('x'),
//             line: 1,
//             column: 2,
//         };
//         assert_eq!(
//             format!("{}", err),
//             "Unexpected character: 'x' at line 1 column 2"
//         );
//     }
//
//     #[test]
//     fn test_invalid_escape_char_display() {
//         let err = LexerError {
//             kind: LexerErrorKinds::InvalidEscapeChar('n'),
//             line: 3,
//             column: 4,
//         };
//         assert_eq!(
//             format!("{}", err),
//             "Invalid escape character: '\\n' at line 3 column 4"
//         );
//     }
//
//     #[test]
//     fn test_unclosed_string_display() {
//         let err = LexerError {
//             kind: LexerErrorKinds::UnclosedString,
//             line: 5,
//             column: 6,
//         };
//         assert_eq!(
//             format!("{}", err),
//             "Unclosed string literal at line 5, column 6"
//         );
//     }
//
//     #[test]
//     fn test_leading_zero_display() {
//         let err = LexerError {
//             kind: LexerErrorKinds::LeadingZero,
//             line: 7,
//             column: 8,
//         };
//         assert_eq!(
//             format!("{}", err),
//             "Number cannot have leading zeros at line 7, column 8"
//         );
//     }
//
//     #[test]
//     fn test_invalid_number_display() {
//         let err = LexerError {
//             kind: LexerErrorKinds::InvalidNumber,
//             line: 9,
//             column: 10,
//         };
//         assert_eq!(
//             format!("{}", err),
//             "Invalid number format at line 9, column 10"
//         );
//     }
//
//     #[test]
//     fn test_invalid_decimal_display() {
//         let err = LexerError {
//             kind: LexerErrorKinds::InvalidDecimal,
//             line: 11,
//             column: 12,
//         };
//         assert_eq!(
//             format!("{}", err),
//             "Invalid decimal format at line 11, column 12"
//         );
//     }
//
//     #[test]
//     fn test_invalid_exponent_display() {
//         let err = LexerError {
//             kind: LexerErrorKinds::InvalidExponent,
//             line: 13,
//             column: 14,
//         };
//         assert_eq!(
//             format!("{}", err),
//             "Invalid exponent format at line 13, column 14"
//         );
//     }
// }
