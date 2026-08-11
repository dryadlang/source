// crates/dryad_lexer/tests/debug_dot_test.rs
use dryad_lexer::{lexer::Lexer, token::Token};

#[test]
fn test_simple_dot() {
    let mut lexer = Lexer::new("a.b");
    
    let token1 = lexer.next_token();
    println!("Token 1: {:?}", token1);
    
    let token2 = lexer.next_token();
    println!("Token 2: {:?}", token2);
    
    let token3 = lexer.next_token();
    println!("Token 3: {:?}", token3);
    
    assert_eq!(token1.unwrap().token, Token::Identifier("a".to_string()));
    assert_eq!(token2.unwrap().token, Token::Symbol('.'));
    assert_eq!(token3.unwrap().token, Token::Identifier("b".to_string()));
}
