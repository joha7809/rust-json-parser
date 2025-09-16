use crate::lexer::{Lexer, Token};

pub struct Parser<I>
where
    I: Iterator<Item = Result<Token, ()>>,
{
    lexer: std::iter::Peekable<I>,
}

impl<I> Parser<I>
where
    I: Iterator<Item = Result<Token, ()>>,
{
    pub fn new(lexer: I) -> Self {
        Self {
            lexer: lexer.peekable(),
        }
    }

    fn peek(&mut self) -> Result<&Token, &()> {
        match self.lexer.peek() {
            Some(Ok(token)) => Ok(token),
            Some(Err(error)) => Err(error),
            None => Err(&()),
        }
    }
}
