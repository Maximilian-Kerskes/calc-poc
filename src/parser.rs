use crate::{expr::{BinaryOp, Expr}, lexer::Token};

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken {
        expected: Vec<&'static str>,
        found: Option<Token>,
    },
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

    fn expect(&mut self, token: Token) -> Result<(), ParseError> {
        match self.advance() {
            Some(t) if t == token => Ok(()),
            other => Err(ParseError::UnexpectedToken {
                expected: vec!["specific token"],
                found: other,
            }),
        }
    }

    fn parse_factor(&mut self) -> Result<Expr, ParseError> {
        match self.peek() {
            Some(Token::Number(n)) => {
                let expr = Expr::Number(n);
                self.advance();
                Ok(expr)
            }

            Some(Token::LParen) => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(Token::RParen)?;
                Ok(expr)
            }

            other => Err(ParseError::UnexpectedToken {
                expected: vec!["num", "("],
                found: other,
            })?,
        }
    }

    fn parse_term(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_factor()?;

        while matches!(self.peek(), Some(Token::Star) | Some(Token::Slash)) {
            let op = match self.advance() {
                Some(op) => op,
                None => Err(ParseError::UnexpectedToken {
                    expected: vec!["specific token"],
                    found: None,
                })?,
            };

            let right = self.parse_factor()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: match op {
                    Token::Star => BinaryOp::Mul,
                    Token::Slash => BinaryOp::Div,
                    _ => Err(ParseError::UnexpectedToken {
                        expected: vec!["*", "/"],
                        found: Some(op),
                    })?,
                },
                right: Box::new(right),
            }
        }

        Ok(left)
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_term()?;

        while matches!(self.peek(), Some(Token::Plus) | Some(Token::Minus)) {
            let op = match self.advance() {
                Some(op) => op,
                None => Err(ParseError::UnexpectedToken {
                    expected: vec!["specific token"],
                    found: None,
                })?,
            };

            let right = self.parse_term()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: match op {
                    Token::Plus => BinaryOp::Add,
                    Token::Minus => BinaryOp::Sub,
                    _ => Err(ParseError::UnexpectedToken {
                        expected: vec!["+", "-"],
                        found: Some(op),
                    })?,
                },
                right: Box::new(right),
            }
        }

        Ok(left)
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        let expr = self.parse_expr()?;

        if self.peek().is_some() {
            return Err(ParseError::UnexpectedToken {
                expected: vec!["end of input"],
                found: self.peek(),
            });
        }
        Ok(expr)
    }
}

#[test]
fn test_simple_addition() {
    let ast = Parser::new(vec![Token::Number(1.0), Token::Plus, Token::Number(2.0)])
        .parse()
        .unwrap();

    match ast {
        Expr::Binary { left, op, right } => {
            assert!(matches!(op, BinaryOp::Add));

            assert!(matches!(*left, Expr::Number(1.0)));
            assert!(matches!(*right, Expr::Number(2.0)));
        }
        _ => panic!("expected binary expression"),
    }
}

#[test]
fn test_precedence_mul_before_add() {
    let ast = Parser::new(vec![
        Token::Number(1.0),
        Token::Plus,
        Token::Number(2.0),
        Token::Star,
        Token::Number(3.0),
    ])
    .parse()
    .unwrap();

    match ast {
        Expr::Binary { op, left, right } => {
            assert!(matches!(op, BinaryOp::Add));
            assert!(matches!(*left, Expr::Number(1.0)));

            match *right {
                Expr::Binary { op, left, right } => {
                    assert!(matches!(op, BinaryOp::Mul));
                    assert!(matches!(*left, Expr::Number(2.0)));
                    assert!(matches!(*right, Expr::Number(3.0)));
                }
                _ => panic!("expected multiplication on right side"),
            }
        }
        _ => panic!("expected binary expression"),
    }
}

#[test]
fn test_parentheses_override_precedence() {
    let ast = Parser::new(vec![
        Token::LParen,
        Token::Number(1.0),
        Token::Plus,
        Token::Number(2.0),
        Token::RParen,
        Token::Star,
        Token::Number(3.0),
    ])
    .parse()
    .unwrap();

    match ast {
        Expr::Binary { op, left, right } => {
            assert!(matches!(op, BinaryOp::Mul));

            match *left {
                Expr::Binary { op, left, right } => {
                    assert!(matches!(op, BinaryOp::Add));
                    assert!(matches!(*left, Expr::Number(1.0)));
                    assert!(matches!(*right, Expr::Number(2.0)));
                }
                _ => panic!("expected addition inside parentheses"),
            }

            assert!(matches!(*right, Expr::Number(3.0)));
        }
        _ => panic!("expected multiplication expression"),
    }
}

#[test]
fn test_single_number() {
    let tokens = vec![Token::Number(42.0)];

    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();

    assert!(matches!(ast, Expr::Number(42.0)));
}

#[test]
fn test_parser_should_fail() {
    let tokens = vec![Token::Number(42.0), Token::Plus];

    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_err());
}
