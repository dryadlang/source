// debug test
use dryad_parser::{Parser, ast::{Stmt, Expr, Literal}};
use dryad_lexer::{Lexer, Token};

fn parse_tokens(input: &str) -> dryad_parser::ast::Program {
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();
    
    loop {
        match lexer.next_token().unwrap() {
            Token::Eof => break,
            token => {
                println!("Token: {:?}", token);
                tokens.push(token);
            }
        }
    }
    
    let mut parser = Parser::new(tokens);
    parser.parse().unwrap()
}

fn main() {
    let program = parse_tokens("for item in [1, 2, 3] { print(item); }");
    println!("Program: {:?}", program);
}
