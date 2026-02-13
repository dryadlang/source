 // crates/dryad_lexer/tests/native_directive_tests.rs

use dryad_lexer::{Lexer, token::{Token, TokenWithLocation}};

#[test]
fn test_tokenize_simple_native_directive() {
    let input = "#<console_io>";
    let mut lexer = Lexer::new(input);
    
    let token = lexer.next_token().unwrap();
    assert_eq!(token.token, Token::NativeDirective("console_io".to_string()));
    
    let token = lexer.next_token().unwrap();
    assert_eq!(token.token, Token::Eof);
}

#[test]
fn test_tokenize_multiple_native_directives() {
    let input = r#"
        #<console_io>
        #<file_io>
        #<debug>
    "#;
    let mut lexer = Lexer::new(input);
    
    let token1 = lexer.next_token().unwrap();
    assert_eq!(token1.token, Token::NativeDirective("console_io".to_string()));
    
    let token2 = lexer.next_token().unwrap();
    assert_eq!(token2.token, Token::NativeDirective("file_io".to_string()));
    
    let token3 = lexer.next_token().unwrap();
    assert_eq!(token3.token, Token::NativeDirective("debug".to_string()));
    
    let token4 = lexer.next_token().unwrap();
    assert_eq!(token4.token, Token::Eof);
}

#[test]
fn test_native_directive_with_underscores() {
    let input = "#<terminal_ansi>";
    let mut lexer = Lexer::new(input);
    
    let token = lexer.next_token().unwrap();
    assert_eq!(token.token, Token::NativeDirective("terminal_ansi".to_string()));
}

#[test]
fn test_native_directive_with_numbers() {
    let input = "#<module123>";
    let mut lexer = Lexer::new(input);
    
    let token = lexer.next_token().unwrap();
    assert_eq!(token.token, Token::NativeDirective("module123".to_string()));
}

#[test]
fn test_native_directive_mixed_with_code() {
    let input = r#"
        #<console_io>
        let x = 5;
        #<debug>
        native_print("Hello");
    "#;
    let mut lexer = Lexer::new(input);
    
    let token1 = lexer.next_token().unwrap();
    assert_eq!(token1.token, Token::NativeDirective("console_io".to_string()));
    
    let token2 = lexer.next_token().unwrap();
    assert_eq!(token2.token, Token::Keyword("let".to_string()));
    
    let token3 = lexer.next_token().unwrap();
    assert_eq!(token3.token, Token::Identifier("x".to_string()));
    
    let token4 = lexer.next_token().unwrap();
    assert_eq!(token4.token, Token::Symbol('='));
    
    let token5 = lexer.next_token().unwrap();
    assert_eq!(token5.token, Token::Number(5.0));
    
    let token6 = lexer.next_token().unwrap();
    assert_eq!(token6.token, Token::Symbol(';'));
    
    let token7 = lexer.next_token().unwrap();
    assert_eq!(token7.token, Token::NativeDirective("debug".to_string()));
}

#[test]
fn test_error_native_directive_unclosed() {
    let input = "#<console_io";
    let mut lexer = Lexer::new(input);
    
    let result = lexer.next_token();
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.code(), 1006);
    assert!(error.message().contains("não fechada"));
}

#[test]
fn test_error_native_directive_empty() {
    let input = "#<>";
    let mut lexer = Lexer::new(input);
    
    let result = lexer.next_token();
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.code(), 1006);
    assert!(error.message().contains("não pode estar vazio"));
}

#[test]
fn test_error_native_directive_invalid_char() {
    let input = "#<module-name>";
    let mut lexer = Lexer::new(input);
    
    let result = lexer.next_token();
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.code(), 1006);
    assert!(error.message().contains("Caracter inválido"));
}

#[test]
fn test_hash_without_directive() {
    let input = "# comment";
    let mut lexer = Lexer::new(input);
    
    let result = lexer.next_token();
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.code(), 1001);
    assert!(error.message().contains("Caracter inesperado '#'"));
}

#[test]
fn test_double_hash_operator() {
    let input = "##";
    let mut lexer = Lexer::new(input);
    
    let token = lexer.next_token().unwrap();
    assert_eq!(token.token, Token::Operator("##".to_string()));
}

#[test]
fn test_native_directive_case_sensitive() {
    let input = "#<Console_IO>";
    let mut lexer = Lexer::new(input);
    
    let token = lexer.next_token().unwrap();
    assert_eq!(token.token, Token::NativeDirective("Console_IO".to_string()));
}

#[test]
fn test_native_directive_in_expression() {
    let input = r#"
        #<console_io>
        5 + 3
    "#;
    let mut lexer = Lexer::new(input);
    
    let token1 = lexer.next_token().unwrap();
    assert_eq!(token1.token, Token::NativeDirective("console_io".to_string()));
    
    let token2 = lexer.next_token().unwrap();
    assert_eq!(token2.token, Token::Number(5.0));
    
    let token3 = lexer.next_token().unwrap();
    assert_eq!(token3.token, Token::Operator("+".to_string()));
    
    let token4 = lexer.next_token().unwrap();
    assert_eq!(token4.token, Token::Number(3.0));
}
