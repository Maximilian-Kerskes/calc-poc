use std::{iter::Peekable, str::Chars};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
}

#[derive(Debug)]
pub enum LexError {
    InvalidChar(char),
}

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        let ch = self.chars.next()?;

        match ch {
            ' ' | '\t' | '\n' => self.next(),
            '0'..='9' => {
                let mut num = ch.to_string();

                while let Some('0'..='9') = self.chars.peek() {
                    num.push(self.chars.next().unwrap());
                }

                Some(Ok(Token::Number(num.parse().unwrap())))
            }
            '+' => Some(Ok(Token::Plus)),
            '-' => Some(Ok(Token::Minus)),
            '*' => Some(Ok(Token::Star)),
            '/' => Some(Ok(Token::Slash)),
            '(' => Some(Ok(Token::LParen)),
            ')' => Some(Ok(Token::RParen)),

            _ => Some(Err(LexError::InvalidChar(ch))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_lexer_should_succeed() {
        let input = "1 + 2 * (3 - 4)";
        let lexer = Lexer::new(input);
        let tokens: Vec<Token> = lexer.collect::<Result<Vec<_>, _>>().unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Number(1.0),
                Token::Plus,
                Token::Number(2.0),
                Token::Star,
                Token::LParen,
                Token::Number(3.0),
                Token::Minus,
                Token::Number(4.0),
                Token::RParen,
            ]
        );
    }

    #[test]
    pub fn test_lexer_should_fail() {
        let input = "!";
        let result = Lexer::new(input).collect::<Result<Vec<_>, _>>();

        assert!(result.is_err());
    }
}
