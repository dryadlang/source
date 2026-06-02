// crates/dryad_parser/tests/debug_foreach_test.rs
use dryad_parser::Parser;
use dryad_lexer::{Lexer, Token};

fn parse_tokens_debug(input: &str) -> Result<dryad_parser::ast::Program, dryad_errors::DryadError> {
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();
    
    println!("Parsing: {}", input);
    loop {
            let tok = lexer.next_token().unwrap();
            if let Token::Eof = tok.token { break; }
            println!("Token: {:?}", tok);
            tokens.push(tok);
    }
    
    let mut parser = Parser::new(tokens);
    parser.parse()
}

#[test]
fn debug_simple_foreach() {
    let result = parse_tokens_debug("for (x in lista) { x = x + 1; }");
    assert!(result.is_ok());
    println!("SUCCESS: simple foreach worked");
}

#[test]
fn debug_array_foreach() {
    let result = parse_tokens_debug("for (item in [1, 2, 3]) { print(item); }");
    match result {
        Ok(program) => {
            println!("SUCCESS: array foreach worked");
            println!("Program: {:?}", program);
        },
        Err(e) => {
            println!("FAILED: array foreach failed with error: {:?}", e);
            panic!("Test failed");
        }
    }
}
