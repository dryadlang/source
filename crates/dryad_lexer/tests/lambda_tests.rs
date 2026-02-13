// crates/dryad_lexer/tests/lambda_tests.rs

use dryad_lexer::{Lexer, token::Token};

#[test]
fn test_arrow_token() {
    let mut lexer = Lexer::new("=>");
    let token = lexer.next_token().unwrap();
    assert_eq!(token.token, Token::Arrow);
}

#[test]
fn test_lambda_single_param() {
    let mut lexer = Lexer::new("x => x * 2");
    
    let tokens: Vec<Token> = std::iter::from_fn(|| {
        match lexer.next_token() {
            Ok(tok) if tok.token == Token::Eof => None,
            Ok(tok) => Some(tok.token),
            Err(_) => None,
        }
    }).collect();
    
    assert_eq!(tokens, vec![
        Token::Identifier("x".to_string()),
        Token::Arrow,
        Token::Identifier("x".to_string()),
        Token::Operator("*".to_string()),
        Token::Number(2.0)
    ]);
}

#[test]
fn test_lambda_multiple_params() {
    let mut lexer = Lexer::new("(a, b) => a + b");
    
    let tokens: Vec<Token> = std::iter::from_fn(|| {
        match lexer.next_token() {
            Ok(tok) if tok.token == Token::Eof => None,
            Ok(tok) => Some(tok.token),
            Err(_) => None,
        }
    }).collect();
    
    assert_eq!(tokens, vec![
        Token::Symbol('('),
        Token::Identifier("a".to_string()),
        Token::Symbol(','),
        Token::Identifier("b".to_string()),
        Token::Symbol(')'),
        Token::Arrow,
        Token::Identifier("a".to_string()),
        Token::Operator("+".to_string()),
        Token::Identifier("b".to_string())
    ]);
}

#[test]
fn test_lambda_assignment() {
    let mut lexer = Lexer::new("let quadrado = x => x * x;");
    
    let tokens: Vec<Token> = std::iter::from_fn(|| {
        match lexer.next_token() {
            Ok(tok) if tok.token == Token::Eof => None,
            Ok(tok) => Some(tok.token),
            Err(_) => None,
        }
    }).collect();
    
    assert_eq!(tokens, vec![
        Token::Keyword("let".to_string()),
        Token::Identifier("quadrado".to_string()),
        Token::Symbol('='),
        Token::Identifier("x".to_string()),
        Token::Arrow,
        Token::Identifier("x".to_string()),
        Token::Operator("*".to_string()),
        Token::Identifier("x".to_string()),
        Token::Symbol(';')
    ]);
}

#[test]
fn test_lambda_zero_params() {
    let mut lexer = Lexer::new("() => 42");
    
    let tokens: Vec<Token> = std::iter::from_fn(|| {
        match lexer.next_token() {
            Ok(tok) if tok.token == Token::Eof => None,
            Ok(tok) => Some(tok.token),
            Err(_) => None,
        }
    }).collect();
    
    assert_eq!(tokens, vec![
        Token::Symbol('('),
        Token::Symbol(')'),
        Token::Arrow,
        Token::Number(42.0)
    ]);
}

#[test]
fn test_lambda_complex_expression() {
    let mut lexer = Lexer::new("(x, y) => x * 2 + y / 3");
    
    let tokens: Vec<Token> = std::iter::from_fn(|| {
        match lexer.next_token() {
            Ok(tok) if tok.token == Token::Eof => None,
            Ok(tok) => Some(tok.token),
            Err(_) => None,
        }
    }).collect();
    
    assert_eq!(tokens, vec![
        Token::Symbol('('),
        Token::Identifier("x".to_string()),
        Token::Symbol(','),
        Token::Identifier("y".to_string()),
        Token::Symbol(')'),
        Token::Arrow,
        Token::Identifier("x".to_string()),
        Token::Operator("*".to_string()),
        Token::Number(2.0),
        Token::Operator("+".to_string()),
        Token::Identifier("y".to_string()),
        Token::Operator("/".to_string()),
        Token::Number(3.0)
    ]);
}
