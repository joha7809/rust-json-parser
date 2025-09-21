use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum LexerErrorKinds {
    UnexcpectedChar(char),
    InvalidEscapeChar(char),
    UnclosedString,
    LeadingZero,
    InvalidNumber,
    InvalidLiteral,
    InvalidDecimal,
    InvalidExponent,
    CastingError,
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
        };
        write!(
            f,
            "{} at line {}, column {}",
            description, self.line, self.column
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unexpected_char_display() {
        let err = LexerError {
            kind: LexerErrorKinds::UnexcpectedChar('x'),
            line: 1,
            column: 2,
        };
        assert_eq!(
            format!("{}", err),
            "Unexpected character: 'x' at line 1, column 2"
        );
    }

    #[test]
    fn test_invalid_escape_char_display() {
        let err = LexerError {
            kind: LexerErrorKinds::InvalidEscapeChar('n'),
            line: 3,
            column: 4,
        };
        assert_eq!(
            format!("{}", err),
            "Invalid escape character: '\\n' at line 3, column 4"
        );
    }

    #[test]
    fn test_unclosed_string_display() {
        let err = LexerError {
            kind: LexerErrorKinds::UnclosedString,
            line: 5,
            column: 6,
        };
        assert_eq!(
            format!("{}", err),
            "Unclosed string literal at line 5, column 6"
        );
    }

    #[test]
    fn test_leading_zero_display() {
        let err = LexerError {
            kind: LexerErrorKinds::LeadingZero,
            line: 7,
            column: 8,
        };
        assert_eq!(
            format!("{}", err),
            "Number cannot have leading zeros at line 7, column 8"
        );
    }

    #[test]
    fn test_invalid_number_display() {
        let err = LexerError {
            kind: LexerErrorKinds::InvalidNumber,
            line: 9,
            column: 10,
        };
        assert_eq!(
            format!("{}", err),
            "Invalid number format at line 9, column 10"
        );
    }

    #[test]
    fn test_invalid_decimal_display() {
        let err = LexerError {
            kind: LexerErrorKinds::InvalidDecimal,
            line: 11,
            column: 12,
        };
        assert_eq!(
            format!("{}", err),
            "Invalid decimal format at line 11, column 12"
        );
    }

    #[test]
    fn test_invalid_exponent_display() {
        let err = LexerError {
            kind: LexerErrorKinds::InvalidExponent,
            line: 13,
            column: 14,
        };
        assert_eq!(
            format!("{}", err),
            "Invalid exponent format at line 13, column 14"
        );
    }
}
