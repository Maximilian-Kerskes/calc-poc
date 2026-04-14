use std::io::stdin;

use crate::lexer::Lexer;

pub mod expr;
pub mod lexer;
pub mod parser;

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).expect("failed to read line");

    let lexer = Lexer::new(&input);
    let tokens: Vec<lexer::Token> = lexer.collect::<Result<Vec<_>, _>>().expect("failed to lex");

    let mut parser = parser::Parser::new(tokens);
    let ast = parser.parse_expr(0).unwrap();

    let result = ast.eval().expect("failed to evaluate");
    println!("{result}");
}
