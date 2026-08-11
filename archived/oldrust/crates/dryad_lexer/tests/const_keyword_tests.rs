// crates/dryad_lexer/tests/const_keyword_tests.rs
use dryad_lexer::{Lexer, token::Token};

#[test]
fn test_const_keyword_recognition() {
    let mut lexer = Lexer::new("const");
    let token = lexer.next_token().unwrap();
    assert_eq!(token.token, Token::Keyword("const".to_string()));
}

#[test]
fn test_const_variable_declaration_lexing() {
    let mut lexer = Lexer::new("const PI = 3.14159;");
    
    // const
    let token1 = lexer.next_token().unwrap();
    assert_eq!(token1.token, Token::Keyword("const".to_string()));
    
    // PI
    let token2 = lexer.next_token().unwrap();
    assert_eq!(token2.token, Token::Identifier("PI".to_string()));
    
    // =
    let token3 = lexer.next_token().unwrap();
    assert_eq!(token3.token, Token::Symbol('='));
    
    // 3.14159
    let token4 = lexer.next_token().unwrap();
    assert_eq!(token4.token, Token::Number(3.14159));
    
    // ;
    let token5 = lexer.next_token().unwrap();
    assert_eq!(token5.token, Token::Symbol(';'));
}

#[test]
fn test_const_vs_let_distinction() {
    let mut lexer1 = Lexer::new("const x = 5;");
    let mut lexer2 = Lexer::new("let x = 5;");
    
    // const vs let
    let token1 = lexer1.next_token().unwrap();
    let token2 = lexer2.next_token().unwrap();
    
    assert_eq!(token1.token, Token::Keyword("const".to_string()));
    assert_eq!(token2.token, Token::Keyword("let".to_string()));
    assert_ne!(token1, token2);
}

#[test]
fn test_const_not_identifier() {
    let mut lexer = Lexer::new("const");
    let token = lexer.next_token().unwrap();
    
    // Deve ser uma palavra-chave, não um identificador
    match token.token {
        Token::Keyword(k) => assert_eq!(k, "const"),
        Token::Identifier(_) => panic!("const deve ser reconhecido como palavra-chave, não identificador"),
        _ => panic!("Token inesperado: {:?}", token),
    }
}

#[test]
fn test_const_case_sensitive() {
    let test_cases = ["CONST", "Const", "cOnSt", "consT"];
    
    for case in &test_cases {
        let mut lexer = Lexer::new(case);
        let token = lexer.next_token().unwrap();
        
        // Todas as variações devem ser identificadores, não palavras-chave
        match token.token {
            Token::Identifier(id) => assert_eq!(id, *case),
            _ => panic!("'{}' deve ser identificador, não palavra-chave", case),
        }
    }
}

#[test]
fn test_const_in_complex_expression() {
    let code = "const MAX_SIZE = 100; let size = MAX_SIZE * 2;";
    let mut lexer = Lexer::new(code);
    
    let expected_tokens = vec![
        Token::Keyword("const".to_string()),
        Token::Identifier("MAX_SIZE".to_string()),
        Token::Symbol('='),
        Token::Number(100.0),
        Token::Symbol(';'),
        Token::Keyword("let".to_string()),
        Token::Identifier("size".to_string()),
        Token::Symbol('='),
        Token::Identifier("MAX_SIZE".to_string()),
        Token::Operator("*".to_string()),
        Token::Number(2.0),
        Token::Symbol(';'),
        Token::Eof,
    ];
    
    for expected_token in expected_tokens {
        let token = lexer.next_token().unwrap();
        assert_eq!(token.token, expected_token);
    }
}

#[test]
fn test_const_with_string_literal() {
    let mut lexer = Lexer::new("const MESSAGE = \"Hello, World!\";");
    
    let tokens: Vec<Token> = std::iter::repeat_with(|| lexer.next_token())
        .take_while(|result| result.is_ok())
        .map(|result| result.unwrap().token)
        .take_while(|token| !matches!(token, Token::Eof))
        .collect();
    
    let expected = vec![
        Token::Keyword("const".to_string()),
        Token::Identifier("MESSAGE".to_string()),
        Token::Symbol('='),
        Token::String("Hello, World!".to_string()),
        Token::Symbol(';'),
    ];
    
    assert_eq!(tokens, expected);
}