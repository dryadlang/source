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
fn test_tokenize_if_keyword() {
    let input = "if";
    let tokens = tokenize_all(input);
    
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0], Token::Keyword("if".to_string()));
}

#[test]
fn test_tokenize_else_keyword() {
    let input = "else";
    let tokens = tokenize_all(input);
    
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0], Token::Keyword("else".to_string()));
}

#[test]
fn test_if_simple_statement() {
    let input = "if idade >= 18 { print(\"Maior de idade\"); }";
    let tokens = tokenize_all(input);
    
    assert_eq!(tokens[0], Token::Keyword("if".to_string()));
    assert_eq!(tokens[1], Token::Identifier("idade".to_string()));
    assert_eq!(tokens[2], Token::Operator(">=".to_string()));
    assert_eq!(tokens[3], Token::Number(18.0));
    assert_eq!(tokens[4], Token::Symbol('{'));
    // ... resto dos tokens
    assert!(tokens.iter().any(|t| matches!(t, Token::Symbol('}'))));
}

#[test]
fn test_if_else_statement() {
    let input = "if nota >= 7.0 { print(\"Aprovado\"); } else { print(\"Reprovado\"); }";
    let tokens = tokenize_all(input);
    
    assert_eq!(tokens[0], Token::Keyword("if".to_string()));
    assert!(tokens.iter().any(|t| matches!(t, Token::Keyword(k) if k == "else")));
}

#[test]
fn test_else_if_chained() {
    let input = "if pontuacao >= 90 { print(\"Excelente\"); } else if pontuacao >= 80 { print(\"Bom\"); }";
    let tokens = tokenize_all(input);
    
    let if_count = tokens.iter().filter(|t| matches!(t, Token::Keyword(k) if k == "if")).count();
    let else_count = tokens.iter().filter(|t| matches!(t, Token::Keyword(k) if k == "else")).count();
    
    assert_eq!(if_count, 2); // "if" e "else if"
    assert_eq!(else_count, 1); // "else"
}

#[test]
fn test_nested_if_statements() {
    let input = r#"
    if x > 0 {
        if y > 0 {
            print("Ambos positivos");
        }
    }
    "#;
    let tokens = tokenize_all(input);
    
    let if_count = tokens.iter().filter(|t| matches!(t, Token::Keyword(k) if k == "if")).count();
    assert_eq!(if_count, 2);
}

#[test]
fn test_if_with_complex_condition() {
    let input = "if (idade >= 18 && idade <= 65) && ativo == true { print(\"Elegível\"); }";
    let tokens = tokenize_all(input);
    
    assert_eq!(tokens[0], Token::Keyword("if".to_string()));
    assert!(tokens.iter().any(|t| matches!(t, Token::Operator(s) if s == "&&")));
    assert!(tokens.iter().any(|t| matches!(t, Token::Operator(s) if s == "==")));
}

#[test]
fn test_exact_syntax_md_examples() {
    // Teste do if simples
    let input1 = r#"let idade = 18;
if idade >= 18 {
    print("Maior de idade");
}"#;
    let tokens1 = tokenize_all(input1);
    assert!(tokens1.iter().any(|t| matches!(t, Token::Keyword(k) if k == "if")));
    
    // Teste do if-else
    let input2 = r#"let nota = 7.5;
if nota >= 7.0 {
    print("Aprovado");
} else {
    print("Reprovado");
}"#;
    let tokens2 = tokenize_all(input2);
    assert!(tokens2.iter().any(|t| matches!(t, Token::Keyword(k) if k == "if")));
    assert!(tokens2.iter().any(|t| matches!(t, Token::Keyword(k) if k == "else")));
    
    // Teste do if-else encadeado
    let input3 = r#"let pontuacao = 85;
if pontuacao >= 90 {
    print("Excelente");
} else if pontuacao >= 80 {
    print("Bom");
} else if pontuacao >= 70 {
    print("Regular");
} else {
    print("Insuficiente");
}"#;
    let tokens3 = tokenize_all(input3);
    let if_count = tokens3.iter().filter(|t| matches!(t, Token::Keyword(k) if k == "if")).count();
    let else_count = tokens3.iter().filter(|t| matches!(t, Token::Keyword(k) if k == "else")).count();
    assert_eq!(if_count, 3); // "if" + 2 "else if"
    assert_eq!(else_count, 3); // 3 "else"
}

#[test]
fn test_if_else_with_whitespace_and_newlines() {
    let input = r#"
    if     condition    {
        statement1;
    }    else    {
        statement2;
    }
    "#;
    let tokens = tokenize_all(input);
    
    assert!(tokens.iter().any(|t| matches!(t, Token::Keyword(k) if k == "if")));
    assert!(tokens.iter().any(|t| matches!(t, Token::Keyword(k) if k == "else")));
}

#[test]
fn test_differentiate_if_else_from_identifiers() {
    let input = "ifvar elsevar if_var else_var if else";
    let tokens = tokenize_all(input);
    
    assert_eq!(tokens[0], Token::Identifier("ifvar".to_string()));
    assert_eq!(tokens[1], Token::Identifier("elsevar".to_string()));
    assert_eq!(tokens[2], Token::Identifier("if_var".to_string()));
    assert_eq!(tokens[3], Token::Identifier("else_var".to_string()));
    assert_eq!(tokens[4], Token::Keyword("if".to_string()));
    assert_eq!(tokens[5], Token::Keyword("else".to_string()));
}

#[test]
fn test_if_else_in_strings() {
    let input = r#"let message = "if this else that"; if true { print("if else"); }"#;
    let tokens = tokenize_all(input);
    
    // Verificar que "if" e "else" dentro de strings não são tokenizados como keywords
    assert!(tokens.iter().any(|t| matches!(t, Token::String(s) if s.contains("if this else that"))));
    // Mas o "if" fora da string deve ser keyword
    assert!(tokens.iter().any(|t| matches!(t, Token::Keyword(k) if k == "if")));
}



