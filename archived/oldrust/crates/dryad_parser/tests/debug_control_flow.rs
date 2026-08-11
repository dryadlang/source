use dryad_parser::Parser;
use dryad_lexer::{Lexer, Token};

fn debug_parse(input: &str) {
    println!("Input: {}", input);
    
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();
    
        loop {
            let token_with_loc = lexer.next_token().unwrap();
            match token_with_loc.token {
                Token::Eof => break,
                ref token => {
                    println!("Token: {:?}", token);
                    tokens.push(token_with_loc);
                }
            }
        }
    
    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(program) => println!("Sucesso: {:?}", program),
        Err(e) => println!("Erro: {:?}", e),
    }
}

#[test]
fn debug_simple_if() {
    debug_parse("if true { x = 1; }");
}

#[test]
fn debug_if_with_print() {
    debug_parse("if true { print(\"hello\"); }");
}

#[test]
fn debug_print_alone() {
    debug_parse("print(\"hello\");");
}
