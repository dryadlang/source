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
fn test_tokenize_while_keyword() {
    let input = "while";
    let tokens = tokenize_all(input);
    
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0], Token::Keyword("while".to_string()));
}

#[test]
fn test_while_simple_statement() {
    let input = r#"
    while i < 5 {
        print(i);
    }
    "#;
    
    let tokens = tokenize_all(input);
    
    // Verifica se 'while' é reconhecido como keyword
    assert!(tokens.contains(&Token::Keyword("while".to_string())));
    assert!(tokens.contains(&Token::Identifier("i".to_string())));
    assert!(tokens.contains(&Token::Operator("<".to_string())));
    assert!(tokens.contains(&Token::Number(5.0)));
    assert!(tokens.contains(&Token::Symbol('{')));
    assert!(tokens.contains(&Token::Symbol('}')));
}

#[test]
fn test_while_with_complex_condition() {
    let input = r#"
    while x > 0 && y < 10 {
        x = x - 1;
        y = y + 1;
    }
    "#;
    
    let tokens = tokenize_all(input);
    
    assert!(tokens.contains(&Token::Keyword("while".to_string())));
    assert!(tokens.contains(&Token::Identifier("x".to_string())));
    assert!(tokens.contains(&Token::Operator(">".to_string())));
    assert!(tokens.contains(&Token::Operator("&&".to_string())));
    assert!(tokens.contains(&Token::Identifier("y".to_string())));
    assert!(tokens.contains(&Token::Operator("<".to_string())));
}

#[test]
fn test_nested_while_statements() {
    let input = r#"
    while outer < 3 {
        while inner < 2 {
            result = result + 1;
        }
    }
    "#;
    
    let tokens = tokenize_all(input);
    
    // Deve haver duas keywords 'while'
    let while_count = tokens.iter()
        .filter(|token| matches!(*token, Token::Keyword(s) if s == "while"))
        .count();
    assert_eq!(while_count, 2);
}

#[test]
fn test_while_with_break_continue() {
    let input = r#"
    while true {
        if condition {
            break;
        } else {
            continue;
        }
    }
    "#;
    
    let tokens = tokenize_all(input);
    
    assert!(tokens.contains(&Token::Keyword("while".to_string())));
    assert!(tokens.contains(&Token::Boolean(true)));
    assert!(tokens.contains(&Token::Keyword("if".to_string())));
    assert!(tokens.contains(&Token::Keyword("break".to_string())));
    assert!(tokens.contains(&Token::Keyword("continue".to_string())));
}

#[test]
fn test_differentiate_while_from_identifiers() {
    let input = r#"
    let whileVar = "not keyword";
    while whileCondition {
        whileAction();
    }
    "#;
    
    let tokens = tokenize_all(input);
    
    // Apenas um 'while' deve ser keyword, outros são identifiers
    let while_keywords = tokens.iter()
        .filter(|token| matches!(*token, Token::Keyword(s) if s == "while"))
        .count();
    assert_eq!(while_keywords, 1);
    
    assert!(tokens.contains(&Token::Identifier("whileVar".to_string())));
    assert!(tokens.contains(&Token::Identifier("whileCondition".to_string())));
    assert!(tokens.contains(&Token::Identifier("whileAction".to_string())));
}

#[test]
fn test_while_in_strings() {
    let input = r#"
    let texto = "this while is not a keyword";
    while condition {
        print("while inside string");
    }
    "#;
    
    let tokens = tokenize_all(input);
    
    // Apenas um 'while' deve ser keyword (fora da string)
    let while_keywords = tokens.iter()
        .filter(|token| matches!(*token, Token::Keyword(s) if s == "while"))
        .count();
    assert_eq!(while_keywords, 1);
    
    assert!(tokens.contains(&Token::String("this while is not a keyword".to_string())));
    assert!(tokens.contains(&Token::String("while inside string".to_string())));
}

#[test]
fn test_while_with_whitespace_and_newlines() {
    let input = r#"
        while   
            condition   
        {
            
            statement;
            
        }
    "#;
    
    let tokens = tokenize_all(input);
    
    assert!(tokens.contains(&Token::Keyword("while".to_string())));
    assert!(tokens.contains(&Token::Identifier("condition".to_string())));
    assert!(tokens.contains(&Token::Symbol('{')));
    assert!(tokens.contains(&Token::Identifier("statement".to_string())));
    assert!(tokens.contains(&Token::Symbol('}')));
}

#[test]
fn test_exact_syntax_md_example() {
    let input = r#"
    let i = 0;
    while i < 5 {
        print(i);
        i = i + 1;
    }
    "#;
    
    let tokens = tokenize_all(input);
    
    // Verifica elementos principais do exemplo do SYNTAX.md
    assert!(tokens.contains(&Token::Keyword("let".to_string())));
    assert!(tokens.contains(&Token::Identifier("i".to_string())));
    assert!(tokens.contains(&Token::Symbol('=')));
    assert!(tokens.contains(&Token::Number(0.0)));
    assert!(tokens.contains(&Token::Keyword("while".to_string())));
    assert!(tokens.contains(&Token::Operator("<".to_string())));
    assert!(tokens.contains(&Token::Number(5.0)));
    assert!(tokens.contains(&Token::Symbol('{')));
    assert!(tokens.contains(&Token::Identifier("print".to_string())));
    assert!(tokens.contains(&Token::Operator("+".to_string())));
    assert!(tokens.contains(&Token::Number(1.0)));
    assert!(tokens.contains(&Token::Symbol('}')));
}



