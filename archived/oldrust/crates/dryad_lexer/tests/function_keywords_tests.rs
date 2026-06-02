// crates/dryad_lexer/tests/function_keywords_tests.rs
use dryad_lexer::{Lexer, token::{Token, TokenWithLocation}};

#[test]
fn test_function_keyword_recognition() {
    let mut lexer = Lexer::new("function");
    let tokens = collect_tokens(&mut lexer);
    
    assert_eq!(tokens.len(), 1);
    assert!(matches!(tokens[0], Token::Keyword(_)));
}

#[test]
fn test_return_keyword_recognition() {
    let mut lexer = Lexer::new("return");
    let tokens = collect_tokens(&mut lexer);
    
    assert_eq!(tokens.len(), 1);
    assert!(matches!(tokens[0], Token::Keyword(_)));
}

#[test]
fn test_function_declaration_syntax() {
    let mut lexer = Lexer::new("function saudacao(nome) { return \"Olá, \" + nome + \"!\"; }");
    let tokens = collect_tokens(&mut lexer);
    
    // Deve ter pelo menos: function, identifier, (, identifier, ), {, return, ..., }
    assert!(tokens.len() >= 8);
    assert!(matches!(tokens[0], Token::Keyword(_))); // function
    assert!(matches!(tokens[1], Token::Identifier(_))); // saudacao
    assert!(matches!(tokens[2], Token::Symbol('('))); // (
    assert!(matches!(tokens[3], Token::Identifier(_))); // nome
    assert!(matches!(tokens[4], Token::Symbol(')'))); // )
    assert!(matches!(tokens[5], Token::Symbol('{'))); // {
    assert!(matches!(tokens[6], Token::Keyword(_))); // return
}

#[test]
fn test_function_call_syntax() {
    let mut lexer = Lexer::new("saudacao(\"Maria\")");
    let tokens = collect_tokens(&mut lexer);
    
    // Deve ter: identifier, (, string, )
    assert_eq!(tokens.len(), 4);
    assert!(matches!(tokens[0], Token::Identifier(_))); // saudacao
    assert!(matches!(tokens[1], Token::Symbol('('))); // (
    assert!(matches!(tokens[2], Token::String(_))); // "Maria"
    assert!(matches!(tokens[3], Token::Symbol(')'))); // )
}

#[test]
fn test_function_with_multiple_parameters() {
    let mut lexer = Lexer::new("function calcular(x, y, z)");
    let tokens = collect_tokens(&mut lexer);
    
    // function, identifier, (, id, ,, id, ,, id, )
    assert_eq!(tokens.len(), 9);
    assert!(matches!(tokens[0], Token::Keyword(_))); // function
    assert!(matches!(tokens[1], Token::Identifier(_))); // calcular
    assert!(matches!(tokens[2], Token::Symbol('('))); // (
    assert!(matches!(tokens[3], Token::Identifier(_))); // x
    assert!(matches!(tokens[4], Token::Symbol(','))); // ,
    assert!(matches!(tokens[5], Token::Identifier(_))); // y
    assert!(matches!(tokens[6], Token::Symbol(','))); // ,
    assert!(matches!(tokens[7], Token::Identifier(_))); // z
    assert!(matches!(tokens[8], Token::Symbol(')'))); // )
}

#[test]
fn test_function_with_no_parameters() {
    let mut lexer = Lexer::new("function semParametros()");
    let tokens = collect_tokens(&mut lexer);
    
    assert_eq!(tokens.len(), 4);
    assert!(matches!(tokens[0], Token::Keyword(_))); // function
    assert!(matches!(tokens[1], Token::Identifier(_))); // semParametros
    assert!(matches!(tokens[2], Token::Symbol('('))); // (
    assert!(matches!(tokens[3], Token::Symbol(')'))); // )
}

#[test]
fn test_function_call_with_expressions() {
    let mut lexer = Lexer::new("calcular(2 + 3, x * 4, \"test\")");
    let tokens = collect_tokens(&mut lexer);
    
    // Deve ter pelo menos: identifier, (, number, +, number, ,, identifier, *, number, ,, string, )
    assert!(tokens.len() >= 12);
    assert!(matches!(tokens[0], Token::Identifier(_))); // calcular
    assert!(matches!(tokens[1], Token::Symbol('('))); // (
    assert!(matches!(tokens[2], Token::Number(_))); // 2
}

#[test]
fn test_nested_function_calls() {
    let mut lexer = Lexer::new("print(saudacao(\"João\"))");
    let tokens = collect_tokens(&mut lexer);
    
    // print, (, saudacao, (, "João", ), )
    assert_eq!(tokens.len(), 7);
    assert!(matches!(tokens[0], Token::Identifier(_))); // print
    assert!(matches!(tokens[1], Token::Symbol('('))); // (
    assert!(matches!(tokens[2], Token::Identifier(_))); // saudacao
    assert!(matches!(tokens[3], Token::Symbol('('))); // (
    assert!(matches!(tokens[4], Token::String(_))); // "João"
    assert!(matches!(tokens[5], Token::Symbol(')'))); // )
    assert!(matches!(tokens[6], Token::Symbol(')'))); // )
}

#[test]
fn test_case_sensitivity_functions() {
    let mut lexer = Lexer::new("Function FUNCTION fUnCtIoN");
    let tokens = collect_tokens(&mut lexer);
    
    // Apenas "function" minúsculo deve ser reconhecido como keyword
    assert_eq!(tokens.len(), 3);
    assert!(matches!(tokens[0], Token::Identifier(_))); // Function
    assert!(matches!(tokens[1], Token::Identifier(_))); // FUNCTION
    assert!(matches!(tokens[2], Token::Identifier(_))); // fUnCtIoN
}

fn collect_tokens(lexer: &mut Lexer) -> Vec<Token> {
    let mut tokens = Vec::new();
    loop {
        match lexer.next_token().unwrap() {
            tok if tok.token == Token::Eof => break,
            tok => tokens.push(tok.token),
        }
    }
    tokens
}
