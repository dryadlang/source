// crates/dryad_lexer/tests/async_threading_tests.rs
use dryad_lexer::{Lexer, token::Token};

#[test]
fn test_async_keyword() {
    let mut lexer = Lexer::new("async");
    let token = lexer.next_token().expect("Token esperado");
    assert!(matches!(token.token, Token::Keyword(ref keyword) if keyword == "async"));
}

#[test]
fn test_await_keyword() {
    let mut lexer = Lexer::new("await");
    let token = lexer.next_token().expect("Token esperado");
    assert!(matches!(token.token, Token::Keyword(ref keyword) if keyword == "await"));
}

#[test]
fn test_thread_keyword() {
    let mut lexer = Lexer::new("thread");
    let token = lexer.next_token().expect("Token esperado");
    assert!(matches!(token.token, Token::Keyword(ref keyword) if keyword == "thread"));
}

#[test]
fn test_mutex_keyword() {
    let mut lexer = Lexer::new("mutex");
    let token = lexer.next_token().expect("Token esperado");
    assert!(matches!(token.token, Token::Keyword(ref keyword) if keyword == "mutex"));
}

#[test]
fn test_join_keyword() {
    let mut lexer = Lexer::new("join");
    let token = lexer.next_token().expect("Token esperado");
    assert!(matches!(token.token, Token::Identifier(ref id) if id == "join"));
}

#[test]
fn test_lock_keyword() {
    let mut lexer = Lexer::new("lock");
    let token = lexer.next_token().expect("Token esperado");
    assert!(matches!(token.token, Token::Identifier(ref id) if id == "lock"));
}

#[test]
fn test_unlock_keyword() {
    let mut lexer = Lexer::new("unlock");
    let token = lexer.next_token().expect("Token esperado");
    assert!(matches!(token.token, Token::Identifier(ref id) if id == "unlock"));
}

#[test]
fn test_async_function_declaration() {
    let mut lexer = Lexer::new("async function testFunc() { }");
    
    let tokens: Vec<Token> = (0..6)
        .map(|_| lexer.next_token().expect("Token esperado").token)
        .collect();
    
    assert!(matches!(tokens[0], Token::Keyword(ref keyword) if keyword == "async"));
    assert!(matches!(tokens[1], Token::Keyword(ref keyword) if keyword == "function"));
    assert!(matches!(tokens[2], Token::Identifier(ref id) if id == "testFunc"));
    assert!(matches!(tokens[3], Token::Symbol('(')));
    assert!(matches!(tokens[4], Token::Symbol(')')));
    assert!(matches!(tokens[5], Token::Symbol('{')));
}

#[test]
fn test_await_expression() {
    let mut lexer = Lexer::new("await getData()");
    
    let tokens: Vec<Token> = (0..4)
        .map(|_| lexer.next_token().expect("Token esperado").token)
        .collect();
    
    assert!(matches!(tokens[0], Token::Keyword(ref keyword) if keyword == "await"));
    assert!(matches!(tokens[1], Token::Identifier(ref id) if id == "getData"));
    assert!(matches!(tokens[2], Token::Symbol('(')));
    assert!(matches!(tokens[3], Token::Symbol(')')));
}

#[test]
fn test_thread_function_declaration() {
    let mut lexer = Lexer::new("thread function backgroundTask() { }");
    
    let tokens: Vec<Token> = (0..6)
        .map(|_| lexer.next_token().expect("Token esperado").token)
        .collect();
    
    assert!(matches!(tokens[0], Token::Keyword(ref keyword) if keyword == "thread"));
    assert!(matches!(tokens[1], Token::Keyword(ref keyword) if keyword == "function"));
    assert!(matches!(tokens[2], Token::Identifier(ref id) if id == "backgroundTask"));
    assert!(matches!(tokens[3], Token::Symbol('(')));
    assert!(matches!(tokens[4], Token::Symbol(')')));
    assert!(matches!(tokens[5], Token::Symbol('{')));
}

#[test]
fn test_mutex_operations() {
    let mut lexer = Lexer::new("let lock = mutex(); lock.lock(); lock.unlock();");
    
    let mut tokens = Vec::new();
    loop {
        match lexer.next_token().expect("Token esperado") {
            tok if tok.token == Token::Eof => break,
            tok => tokens.push(tok.token)
        }
    }
    
    assert!(tokens.len() >= 13);
    assert!(matches!(tokens[0], Token::Keyword(ref keyword) if keyword == "let"));
    assert!(matches!(tokens[1], Token::Identifier(ref id) if id == "lock"));
    assert!(matches!(tokens[2], Token::Symbol('=')));
    assert!(matches!(tokens[3], Token::Keyword(ref keyword) if keyword == "mutex"));
    assert!(matches!(tokens[4], Token::Symbol('(')));
    assert!(matches!(tokens[5], Token::Symbol(')')));
    assert!(matches!(tokens[6], Token::Symbol(';')));
    assert!(matches!(tokens[7], Token::Identifier(ref id) if id == "lock"));
    assert!(matches!(tokens[8], Token::Symbol('.')));
    assert!(matches!(tokens[9], Token::Identifier(ref id) if id == "lock"));
    assert!(matches!(tokens[10], Token::Symbol('(')));
    assert!(matches!(tokens[11], Token::Symbol(')')));
    assert!(matches!(tokens[12], Token::Symbol(';')));
}

#[test]
fn test_thread_join() {
    let mut lexer = Lexer::new("thread.join()");
    
    let mut tokens = Vec::new();
    loop {
        match lexer.next_token().expect("Token esperado") {
            tok if tok.token == Token::Eof => break,
            tok => tokens.push(tok.token)
        }
    }
    
    assert!(tokens.len() >= 5);
    assert!(matches!(tokens[0], Token::Keyword(ref keyword) if keyword == "thread"));
    assert!(matches!(tokens[1], Token::Symbol('.')));
    assert!(matches!(tokens[2], Token::Identifier(ref id) if id == "join"));
    assert!(matches!(tokens[3], Token::Symbol('(')));
    assert!(matches!(tokens[4], Token::Symbol(')')));
}