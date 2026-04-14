use crate::{
    expr::{BinaryOp, Expr, UnaryOp},
    lexer::Token,
};

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken {
        expected: Vec<&'static str>,
        found: Option<Token>,
    },
}

pub fn precedence(token: &Token) -> u8 {
    match token {
        Token::Plus | Token::Minus => 1,
        Token::Star | Token::Slash => 2,
        _ => 0,
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<Token> {
        self.tokens.get(self.pos).cloned()
    }

    fn advance(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.pos)?.clone();
        self.pos += 1;
        Some(token)
    }

    fn is_infix_op(&self, token: &Token) -> bool {
        matches!(
            token,
            Token::Plus | Token::Minus | Token::Star | Token::Slash
        )
    }

    pub fn parse_expr(&mut self, min_prec: u8) -> Result<Expr, ParseError> {
        let mut left = self.parse_prefix()?;

        while let Some(op) = self.peek() {
            if !self.is_infix_op(&op) {
                break;
            }

            let prec = precedence(&op);

            if prec < min_prec {
                break;
            }

            let op_token = self.advance().unwrap();

            let op = match op_token {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Sub,
                Token::Star => BinaryOp::Mul,
                Token::Slash => BinaryOp::Div,
                _ => unreachable!(),
            };

            let right = self.parse_expr(prec + 1)?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            }
        }
        Ok(left)
    }

    fn parse_prefix(&mut self) -> Result<Expr, ParseError> {
        match self.advance() {
            Some(Token::Number(n)) => Ok(Expr::Number(n)),
            Some(Token::Minus) => {
                let expr = self.parse_expr(100)?;
                Ok(Expr::Unary {
                    op: UnaryOp::Neg,
                    right: Box::new(expr),
                })
            }
            Some(Token::LParen) => {
                let expr = self.parse_expr(0)?;
                match self.advance() {
                    Some(Token::RParen) => Ok(expr),
                    _found => Err(ParseError::UnexpectedToken {
                        expected: vec![")"],
                        found: None,
                    }),
                }
            }

            _other => Err(ParseError::UnexpectedToken {
                expected: vec!["number", "(", "expression"],
                found: None,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Token;

    fn parse(tokens: Vec<Token>) -> Expr {
        let mut parser = Parser::new(tokens);
        parser.parse_expr(0).unwrap()
    }

    #[test]
    fn test_single_number() {
        let expr = parse(vec![Token::Number(42.0)]);
        match expr {
            Expr::Number(n) => assert_eq!(n, 42.0),
            _ => panic!("Expected number"),
        }
    }

    #[test]
    fn test_simple_addition() {
        let expr = parse(vec![Token::Number(1.0), Token::Plus, Token::Number(2.0)]);

        match expr {
            Expr::Binary {
                op: BinaryOp::Add, ..
            } => {}
            _ => panic!("Expected addition"),
        }
    }

    #[test]
    fn test_operator_precedence() {
        // 1 + 2 * 3  => 1 + (2 * 3)
        let expr = parse(vec![
            Token::Number(1.0),
            Token::Plus,
            Token::Number(2.0),
            Token::Star,
            Token::Number(3.0),
        ]);

        match expr {
            Expr::Binary {
                op: BinaryOp::Add,
                left: _,
                right,
            } => match *right {
                Expr::Binary {
                    op: BinaryOp::Mul, ..
                } => {}
                _ => panic!("Expected multiplication on right side"),
            },
            _ => panic!("Expected addition at root"),
        }
    }

    #[test]
    fn test_left_associativity() {
        // (1 - 2) - 3
        let expr = parse(vec![
            Token::Number(1.0),
            Token::Minus,
            Token::Number(2.0),
            Token::Minus,
            Token::Number(3.0),
        ]);

        match expr {
            Expr::Binary {
                op: BinaryOp::Sub,
                left,
                right: _,
            } => match *left {
                Expr::Binary {
                    op: BinaryOp::Sub, ..
                } => {}
                _ => panic!("Expected left-associative subtraction"),
            },
            _ => panic!("Expected subtraction"),
        }
    }

    #[test]
    fn test_parentheses_override_precedence() {
        // (1 + 2) * 3
        let expr = parse(vec![
            Token::LParen,
            Token::Number(1.0),
            Token::Plus,
            Token::Number(2.0),
            Token::RParen,
            Token::Star,
            Token::Number(3.0),
        ]);

        match expr {
            Expr::Binary {
                op: BinaryOp::Mul,
                left,
                ..
            } => match *left {
                Expr::Binary {
                    op: BinaryOp::Add, ..
                } => {}
                _ => panic!("Expected addition inside parentheses"),
            },
            _ => panic!("Expected multiplication at root"),
        }
    }

    #[test]
    fn test_missing_rparen_error() {
        let mut parser = Parser::new(vec![
            Token::LParen,
            Token::Number(1.0),
            Token::Plus,
            Token::Number(2.0),
        ]);

        let result = parser.parse_expr(0);
        assert!(result.is_err());
    }

    #[test]
    fn test_unexpected_token_error() {
        let mut parser = Parser::new(vec![Token::Plus]);

        let result = parser.parse_expr(0);
        assert!(result.is_err());
    }
}
