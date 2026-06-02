use dryad_lexer::{Lexer, token::{Token, TokenWithLocation}};

fn tokenize_all(input: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();
    
    loop {
        match lexer.next_token().unwrap() {
            tok if tok.token == Token::Eof => break,
            tok => tokens.push(tok.token),
        }
    }
    
    tokens
}

#[test]
fn test_tokenize_for_keyword() {
    let input = "for";
    let tokens = tokenize_all(input);
    
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0], Token::Keyword("for".to_string()));
}

#[test]
fn test_for_simple_statement() {
    let input = r#"
    for i = 0; i < 5; i = i + 1 {
        print(i);
    }
    "#;
    
    let tokens = tokenize_all(input);
    
    // Verifica que "for" é tokenizado como keyword
    let for_token = tokens.iter().find(|t| matches!(t, Token::Keyword(k) if k == "for"));
    assert!(for_token.is_some());
    
    // Verifica que outros elementos estão presentes
    assert!(tokens.iter().any(|t| matches!(t, Token::Identifier(_))));
    assert!(tokens.iter().any(|t| matches!(t, Token::Symbol('='))));
    assert!(tokens.iter().any(|t| matches!(t, Token::Symbol(';'))));
    assert!(tokens.iter().any(|t| matches!(t, Token::Operator(_))));
    assert!(tokens.iter().any(|t| matches!(t, Token::Symbol('{'))));
    assert!(tokens.iter().any(|t| matches!(t, Token::Symbol('}'))));
}

#[test]
fn test_for_with_complex_condition() {
    let input = r#"
    for count = 1; count <= 10 && active; count = count + 2 {
        result = result + count;
    }
    "#;
    
    let tokens = tokenize_all(input);
    
    // Verifica que "for" é reconhecido
    let for_token = tokens.iter().find(|t| matches!(t, Token::Keyword(k) if k == "for"));
    assert!(for_token.is_some());
    
    // Verifica operadores complexos
    assert!(tokens.iter().any(|t| matches!(t, Token::Operator(op) if op == "<=")));
    assert!(tokens.iter().any(|t| matches!(t, Token::Operator(op) if op == "&&")));
}

#[test]
fn test_differentiate_for_from_identifiers() {
    let input = r#"
    let forValue = 42;
    let format = "text";
    for i = 0; i < 3; i = i + 1 {
        let force = i * 2;
    }
    "#;
    
    let tokens = tokenize_all(input);
    
    // "for" deve ser keyword
    let for_keyword = tokens.iter().find(|t| matches!(t, Token::Keyword(k) if k == "for"));
    assert!(for_keyword.is_some());
    
    // "forValue", "format", "force" devem ser identificadores
    assert!(tokens.iter().any(|t| matches!(t, Token::Identifier(id) if id == "forValue")));
    assert!(tokens.iter().any(|t| matches!(t, Token::Identifier(id) if id == "format")));
    assert!(tokens.iter().any(|t| matches!(t, Token::Identifier(id) if id == "force")));
}

#[test]
fn test_for_in_strings() {
    let input = r#"
    let text = "for loop example";
    for i = 0; i < 3; i = i + 1 {
        print("iteration for " + i);
    }
    "#;
    
    let tokens = tokenize_all(input);
    
    // "for" dentro de strings não deve ser keyword
    let string_tokens: Vec<_> = tokens.iter().filter(|t| matches!(t, Token::String(_))).collect();
    assert_eq!(string_tokens.len(), 2);
    
    // Mas "for" fora de strings deve ser keyword
    let for_keyword = tokens.iter().find(|t| matches!(t, Token::Keyword(k) if k == "for"));
    assert!(for_keyword.is_some());
}

#[test]
fn test_for_with_break_continue() {
    let input = r#"
    for i = 0; i < 10; i = i + 1 {
        if i == 3 {
            continue;
        }
        if i == 7 {
            break;
        }
        print(i);
    }
    "#;
    
    let tokens = tokenize_all(input);
    
    // Verifica keywords
    assert!(tokens.iter().any(|t| matches!(t, Token::Keyword(k) if k == "for")));
    assert!(tokens.iter().any(|t| matches!(t, Token::Keyword(k) if k == "if")));
    assert!(tokens.iter().any(|t| matches!(t, Token::Keyword(k) if k == "continue")));
    assert!(tokens.iter().any(|t| matches!(t, Token::Keyword(k) if k == "break")));
}

#[test]
fn test_nested_for_statements() {
    let input = r#"
    for i = 0; i < 3; i = i + 1 {
        for j = 0; j < 2; j = j + 1 {
            result = i * j;
        }
    }
    "#;
    
    let tokens = tokenize_all(input);
    
    // Deve ter duas keywords "for"
    let for_count = tokens.iter().filter(|t| matches!(t, Token::Keyword(k) if k == "for")).count();
    assert_eq!(for_count, 2);
    
    // Verifica identificadores diferentes
    assert!(tokens.iter().any(|t| matches!(t, Token::Identifier(id) if id == "i")));
    assert!(tokens.iter().any(|t| matches!(t, Token::Identifier(id) if id == "j")));
}

#[test]
fn test_for_with_whitespace_and_newlines() {
    let input = r#"
    for
        counter = 0;
        counter < 5;
        counter = counter + 1
    {
        sum = sum + counter;
    }
    "#;
    
    let tokens = tokenize_all(input);
    
    // "for" deve ser reconhecido mesmo com quebras de linha
    let for_token = tokens.iter().find(|t| matches!(t, Token::Keyword(k) if k == "for"));
    assert!(for_token.is_some());
    
    // Verifica estrutura básica
    assert!(tokens.iter().any(|t| matches!(t, Token::Identifier(id) if id == "counter")));
    assert!(tokens.iter().any(|t| matches!(t, Token::Symbol(';'))));
}

#[test]
fn test_exact_syntax_md_example() {
    let input = r#"
    for i = 0; i < 5; i = i + 1 {
        print(i);
    }
    "#;
    
    let tokens = tokenize_all(input);
    
    // Verifica estrutura exata do exemplo da documentação
    assert!(tokens.iter().any(|t| matches!(t, Token::Keyword(k) if k == "for")));
    assert!(tokens.iter().any(|t| matches!(t, Token::Identifier(id) if id == "i")));
    assert!(tokens.iter().any(|t| matches!(t, Token::Symbol('='))));
    assert!(tokens.iter().any(|t| matches!(t, Token::Number(n) if *n == 0.0)));
    assert!(tokens.iter().any(|t| matches!(t, Token::Symbol(';'))));
    assert!(tokens.iter().any(|t| matches!(t, Token::Operator(op) if op == "<")));
    assert!(tokens.iter().any(|t| matches!(t, Token::Number(n) if *n == 5.0)));
    assert!(tokens.iter().any(|t| matches!(t, Token::Operator(op) if op == "+")));
    assert!(tokens.iter().any(|t| matches!(t, Token::Number(n) if *n == 1.0)));
}




