// crates/dryad_lexer/tests/array_tests.rs
use dryad_lexer::{lexer::Lexer, token::Token};

#[test]
fn test_array_literal_empty() {
    let mut lexer = Lexer::new("[]");
    
    assert_eq!(lexer.next_token().unwrap().token, Token::Symbol('['));
    assert_eq!(lexer.next_token().unwrap().token, Token::Symbol(']'));
    assert_eq!(lexer.next_token().unwrap().token, Token::Eof);
}

#[test]
fn test_array_literal_numbers() {
    let mut lexer = Lexer::new("[1, 2, 3]");
    
    assert_eq!(lexer.next_token().unwrap().token, Token::Symbol('['));
    assert_eq!(lexer.next_token().unwrap().token, Token::Number(1.0));
    assert_eq!(lexer.next_token().unwrap().token, Token::Symbol(','));
    assert_eq!(lexer.next_token().unwrap().token, Token::Number(2.0));
    assert_eq!(lexer.next_token().unwrap().token, Token::Symbol(','));
    assert_eq!(lexer.next_token().unwrap().token, Token::Number(3.0));
    assert_eq!(lexer.next_token().unwrap().token, Token::Symbol(']'));
    assert_eq!(lexer.next_token().unwrap().token, Token::Eof);
}

#[test]
fn test_array_literal_strings() {
    let mut lexer = Lexer::new(r#"["hello", "world"]"#);
    
    assert_eq!(lexer.next_token().unwrap().token, Token::Symbol('['));
    assert_eq!(lexer.next_token().unwrap().token, Token::String("hello".to_string()));
    assert_eq!(lexer.next_token().unwrap().token, Token::Symbol(','));
    assert_eq!(lexer.next_token().unwrap().token, Token::String("world".to_string()));
    assert_eq!(lexer.next_token().unwrap().token, Token::Symbol(']'));
    assert_eq!(lexer.next_token().unwrap().token, Token::Eof);
}

#[test]
fn test_array_access() {
    let mut lexer = Lexer::new("arr[0]");
    
    assert_eq!(lexer.next_token().unwrap().token, Token::Identifier("arr".to_string()));
    assert_eq!(lexer.next_token().unwrap().token, Token::Symbol('['));
    assert_eq!(lexer.next_token().unwrap().token, Token::Number(0.0));
    assert_eq!(lexer.next_token().unwrap().token, Token::Symbol(']'));
    assert_eq!(lexer.next_token().unwrap().token, Token::Eof);
}

#[test]
fn test_nested_array() {
    let mut lexer = Lexer::new("[[1, 2], [3, 4]]");
    
    assert_eq!(lexer.next_token().unwrap().token, Token::Symbol('['));
    assert_eq!(lexer.next_token().unwrap().token, Token::Symbol('['));
    assert_eq!(lexer.next_token().unwrap().token, Token::Number(1.0));
    assert_eq!(lexer.next_token().unwrap().token, Token::Symbol(','));
    assert_eq!(lexer.next_token().unwrap().token, Token::Number(2.0));
    assert_eq!(lexer.next_token().unwrap().token, Token::Symbol(']'));
    assert_eq!(lexer.next_token().unwrap().token, Token::Symbol(','));
    assert_eq!(lexer.next_token().unwrap().token, Token::Symbol('['));
    assert_eq!(lexer.next_token().unwrap().token, Token::Number(3.0));
    assert_eq!(lexer.next_token().unwrap().token, Token::Symbol(','));
    assert_eq!(lexer.next_token().unwrap().token, Token::Number(4.0));
    assert_eq!(lexer.next_token().unwrap().token, Token::Symbol(']'));
    assert_eq!(lexer.next_token().unwrap().token, Token::Symbol(']'));
    assert_eq!(lexer.next_token().unwrap().token, Token::Eof);
}

#[test]
fn test_tuple_literal() {
    let mut lexer = Lexer::new("(1, \"dois\", 3.0)");
    
    assert_eq!(lexer.next_token().unwrap().token, Token::Symbol('('));
    assert_eq!(lexer.next_token().unwrap().token, Token::Number(1.0));
    assert_eq!(lexer.next_token().unwrap().token, Token::Symbol(','));
    assert_eq!(lexer.next_token().unwrap().token, Token::String("dois".to_string()));
    assert_eq!(lexer.next_token().unwrap().token, Token::Symbol(','));
    assert_eq!(lexer.next_token().unwrap().token, Token::Number(3.0));
    assert_eq!(lexer.next_token().unwrap().token, Token::Symbol(')'));
    assert_eq!(lexer.next_token().unwrap().token, Token::Eof);
}

#[test]
fn test_tuple_access() {
    let mut lexer = Lexer::new("tupla.1");
    
    assert_eq!(lexer.next_token().unwrap().token, Token::Identifier("tupla".to_string()));
    assert_eq!(lexer.next_token().unwrap().token, Token::Symbol('.'));
    assert_eq!(lexer.next_token().unwrap().token, Token::Number(1.0));
    assert_eq!(lexer.next_token().unwrap().token, Token::Eof);
}

#[test]
fn test_empty_tuple() {
    let mut lexer = Lexer::new("()");
    
    assert_eq!(lexer.next_token().unwrap().token, Token::Symbol('('));
    assert_eq!(lexer.next_token().unwrap().token, Token::Symbol(')'));
    assert_eq!(lexer.next_token().unwrap().token, Token::Eof);
}

#[test]
fn test_matrix_access() {
    let mut lexer = Lexer::new("matriz[1][0]");
    
    assert_eq!(lexer.next_token().unwrap().token, Token::Identifier("matriz".to_string()));
    assert_eq!(lexer.next_token().unwrap().token, Token::Symbol('['));
    assert_eq!(lexer.next_token().unwrap().token, Token::Number(1.0));
    assert_eq!(lexer.next_token().unwrap().token, Token::Symbol(']'));
    assert_eq!(lexer.next_token().unwrap().token, Token::Symbol('['));
    assert_eq!(lexer.next_token().unwrap().token, Token::Number(0.0));
    assert_eq!(lexer.next_token().unwrap().token, Token::Symbol(']'));
    assert_eq!(lexer.next_token().unwrap().token, Token::Eof);
}

#[test]
fn test_decimal_number_with_dot() {
    // Teste para garantir que números decimais ainda funcionam
    let mut lexer = Lexer::new("3.14");
    
    assert_eq!(lexer.next_token().unwrap().token, Token::Number(3.14));
    assert_eq!(lexer.next_token().unwrap().token, Token::Eof);
}
