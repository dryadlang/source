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
fn test_tokenize_do_keyword() {
    let input = "do";
    let tokens = tokenize_all(input);
    
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0], Token::Keyword("do".to_string()));
}

#[test]
fn test_do_while_simple_statement() {
    let input = r#"
    do {
        i = i + 1;
    } while i < 5;
    "#;
    
    let tokens = tokenize_all(input);
    
    // Verifica se 'do' e 'while' são reconhecidos como keywords
    assert!(tokens.contains(&Token::Keyword("do".to_string())));
    assert!(tokens.contains(&Token::Keyword("while".to_string())));
    assert!(tokens.contains(&Token::Identifier("i".to_string())));
    assert!(tokens.contains(&Token::Operator("<".to_string())));
    assert!(tokens.contains(&Token::Number(5.0)));
    assert!(tokens.contains(&Token::Symbol('{')));
    assert!(tokens.contains(&Token::Symbol('}')));
}

#[test]
fn test_do_while_with_complex_condition() {
    let input = r#"
    do {
        x = x - 1;
        y = y + 1;
    } while x > 0 && y < 10;
    "#;
    
    let tokens = tokenize_all(input);
    
    assert!(tokens.contains(&Token::Keyword("do".to_string())));
    assert!(tokens.contains(&Token::Keyword("while".to_string())));
    assert!(tokens.contains(&Token::Identifier("x".to_string())));
    assert!(tokens.contains(&Token::Operator(">".to_string())));
    assert!(tokens.contains(&Token::Operator("&&".to_string())));
    assert!(tokens.contains(&Token::Identifier("y".to_string())));
    assert!(tokens.contains(&Token::Operator("<".to_string())));
}

#[test]
fn test_nested_do_while_statements() {
    let input = r#"
    do {
        do {
            inner = inner + 1;
        } while inner < 2;
        outer = outer + 1;
    } while outer < 3;
    "#;
    
    let tokens = tokenize_all(input);
    
    // Deve haver duas keywords 'do' e duas 'while'
    let do_count = tokens.iter()
        .filter(|token| matches!(*token, Token::Keyword(s) if s == "do"))
        .count();
    let while_count = tokens.iter()
        .filter(|token| matches!(*token, Token::Keyword(s) if s == "while"))
        .count();
    assert_eq!(do_count, 2);
    assert_eq!(while_count, 2);
}

#[test]
fn test_do_while_with_break_continue() {
    let input = r#"
    do {
        if condition {
            break;
        } else {
            continue;
        }
    } while true;
    "#;
    
    let tokens = tokenize_all(input);
    
    assert!(tokens.contains(&Token::Keyword("do".to_string())));
    assert!(tokens.contains(&Token::Keyword("while".to_string())));
    assert!(tokens.contains(&Token::Boolean(true)));
    assert!(tokens.contains(&Token::Keyword("if".to_string())));
    assert!(tokens.contains(&Token::Keyword("break".to_string())));
    assert!(tokens.contains(&Token::Keyword("continue".to_string())));
}

#[test]
fn test_differentiate_do_from_identifiers() {
    let input = r#"
    let doVar = "not keyword";
    do {
        doAction = doSomething;
    } while doCondition;
    "#;
    
    let tokens = tokenize_all(input);
    
    // Apenas um 'do' deve ser keyword, outros são identifiers
    let do_keywords = tokens.iter()
        .filter(|token| matches!(*token, Token::Keyword(s) if s == "do"))
        .count();
    assert_eq!(do_keywords, 1);
    
    assert!(tokens.contains(&Token::Identifier("doVar".to_string())));
    assert!(tokens.contains(&Token::Identifier("doAction".to_string())));
    assert!(tokens.contains(&Token::Identifier("doSomething".to_string())));
    assert!(tokens.contains(&Token::Identifier("doCondition".to_string())));
}

#[test]
fn test_do_while_in_strings() {
    let input = r#"
    let texto = "this do while is not keywords";
    do {
        print("do while inside string");
    } while condition;
    "#;
    
    let tokens = tokenize_all(input);
    
    // Apenas um 'do' e um 'while' devem ser keywords (fora da string)
    let do_keywords = tokens.iter()
        .filter(|token| matches!(*token, Token::Keyword(s) if s == "do"))
        .count();
    let while_keywords = tokens.iter()
        .filter(|token| matches!(*token, Token::Keyword(s) if s == "while"))
        .count();
    assert_eq!(do_keywords, 1);
    assert_eq!(while_keywords, 1);
    
    assert!(tokens.contains(&Token::String("this do while is not keywords".to_string())));
    assert!(tokens.contains(&Token::String("do while inside string".to_string())));
}

#[test]
fn test_do_while_with_whitespace_and_newlines() {
    let input = r#"
        do   
        {
            
            statement;
            
        }   
        while   
            condition;
    "#;
    
    let tokens = tokenize_all(input);
    
    assert!(tokens.contains(&Token::Keyword("do".to_string())));
    assert!(tokens.contains(&Token::Symbol('{')));
    assert!(tokens.contains(&Token::Identifier("statement".to_string())));
    assert!(tokens.contains(&Token::Symbol('}')));
    assert!(tokens.contains(&Token::Keyword("while".to_string())));
    assert!(tokens.contains(&Token::Identifier("condition".to_string())));
}

#[test]
fn test_exact_syntax_md_example() {
    let input = r#"
    let i = 0;
    do {
        print(i);
        i = i + 1;
    } while i < 5;
    "#;
    
    let tokens = tokenize_all(input);
    
    // Verifica elementos principais do exemplo do SYNTAX.md
    assert!(tokens.contains(&Token::Keyword("let".to_string())));
    assert!(tokens.contains(&Token::Identifier("i".to_string())));
    assert!(tokens.contains(&Token::Symbol('=')));
    assert!(tokens.contains(&Token::Number(0.0)));
    assert!(tokens.contains(&Token::Keyword("do".to_string())));
    assert!(tokens.contains(&Token::Symbol('{')));
    assert!(tokens.contains(&Token::Identifier("print".to_string())));
    assert!(tokens.contains(&Token::Operator("+".to_string())));
    assert!(tokens.contains(&Token::Number(1.0)));
    assert!(tokens.contains(&Token::Symbol('}')));
    assert!(tokens.contains(&Token::Keyword("while".to_string())));
    assert!(tokens.contains(&Token::Operator("<".to_string())));
    assert!(tokens.contains(&Token::Number(5.0)));
}
