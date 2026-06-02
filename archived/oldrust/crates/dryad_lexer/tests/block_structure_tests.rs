use dryad_lexer::{Lexer, token::{Token, TokenWithLocation}};

#[test]
fn test_tokenize_left_brace() {
    let mut lexer = Lexer::new("{");
    let token = lexer.next_token().unwrap();
    assert!(matches!(token.token, Token::Symbol('{')));
    
    let token = lexer.next_token().unwrap();
    assert!(matches!(token.token, Token::Eof));
}

#[test]
fn test_tokenize_right_brace() {
    let mut lexer = Lexer::new("}");
    let token = lexer.next_token().unwrap();
    assert!(matches!(token.token, Token::Symbol('}')));
    
    let token = lexer.next_token().unwrap();
    assert!(matches!(token.token, Token::Eof));
}

#[test]
fn test_braces_together() {
    let mut lexer = Lexer::new("{}");
    let token1 = lexer.next_token().unwrap();
    assert!(matches!(token1.token, Token::Symbol('{')));
    
    let token2 = lexer.next_token().unwrap();
    assert!(matches!(token2.token, Token::Symbol('}')));
    
    let token3 = lexer.next_token().unwrap();
    assert!(matches!(token3.token, Token::Eof));
}

#[test]
fn test_empty_block() {
    let mut lexer = Lexer::new("{ }");
    let tokens = collect_tokens(&mut lexer);
    
    assert!(matches!(tokens[0], Token::Symbol('{')));
    assert!(matches!(tokens[1], Token::Symbol('}')));
    assert!(matches!(tokens[2], Token::Eof));
}

#[test]
fn test_block_with_content() {
    let mut lexer = Lexer::new("{ let x = 5; }");
    let tokens = collect_tokens(&mut lexer);
    
    assert!(matches!(tokens[0], Token::Symbol('{')));
    assert!(matches!(tokens[1], Token::Keyword(_))); // let
    assert!(matches!(tokens[2], Token::Identifier(_))); // x
    assert!(matches!(tokens[3], Token::Symbol('='))); // =
    assert!(matches!(tokens[4], Token::Number(_))); // 5
    assert!(matches!(tokens[5], Token::Symbol(';')));
    assert!(matches!(tokens[6], Token::Symbol('}')));
}

#[test]
fn test_nested_blocks() {
    let mut lexer = Lexer::new("{ { let x = 1; } { let y = 2; } }");
    let tokens = collect_tokens(&mut lexer);
    
    // Should have 3 opening braces and 3 closing braces
    let open_count = tokens.iter().filter(|t| matches!(*t, Token::Symbol('{'))).count();
    let close_count = tokens.iter().filter(|t| matches!(*t, Token::Symbol('}'))).count();
    
    assert_eq!(open_count, 3);
    assert_eq!(close_count, 3);
}

#[test]
fn test_deeply_nested_blocks() {
    let mut lexer = Lexer::new("{ { { { } } } }");
    let tokens = collect_tokens(&mut lexer);
    
    let open_count = tokens.iter().filter(|t| matches!(*t, Token::Symbol('{'))).count();
    let close_count = tokens.iter().filter(|t| matches!(*t, Token::Symbol('}'))).count();
    
    assert_eq!(open_count, 4);
    assert_eq!(close_count, 4);
}

#[test]
fn test_block_with_multiple_statements() {
    let mut lexer = Lexer::new("{ let a = 1; let b = 2; let c = a + b; }");
    let tokens = collect_tokens(&mut lexer);
    
    assert!(matches!(tokens[0], Token::Symbol('{')));
    
    // Count semicolons (should be 3)
    let semicolon_count = tokens.iter().filter(|t| matches!(*t, Token::Symbol(';'))).count();
    assert_eq!(semicolon_count, 3);
    
    assert!(matches!(tokens[tokens.len() - 2], Token::Symbol('}')));
}

#[test]
fn test_block_with_function() {
    let mut lexer = Lexer::new("function test() { return 42; }");
    let tokens = collect_tokens(&mut lexer);
    
    assert!(matches!(tokens[0], Token::Keyword(_))); // function
    assert!(matches!(tokens[1], Token::Identifier(_))); // test
    assert!(matches!(tokens[2], Token::Symbol('(')));
    assert!(matches!(tokens[3], Token::Symbol(')')));
    assert!(matches!(tokens[4], Token::Symbol('{')));
    // ... function body ...
    assert!(tokens.iter().any(|t| matches!(*t, Token::Symbol('}'))));
}

#[test]
fn test_block_with_if_statement() {
    let mut lexer = Lexer::new("if x > 0 { print(\"positive\"); }");
    let tokens = collect_tokens(&mut lexer);
    
    assert!(matches!(tokens[0], Token::Keyword(_))); // if
    // ... condition ...
    assert!(tokens.iter().any(|t| matches!(*t, Token::Symbol('{'))));
    assert!(tokens.iter().any(|t| matches!(*t, Token::Symbol('}'))));
}

#[test]
fn test_block_with_class() {
    let mut lexer = Lexer::new("class Person { function init() { } }");
    let tokens = collect_tokens(&mut lexer);
    
    assert!(matches!(tokens[0], Token::Keyword(_))); // class
    
    // Should have 2 opening and 2 closing braces (class and method)
    let open_count = tokens.iter().filter(|t| matches!(*t, Token::Symbol('{'))).count();
    let close_count = tokens.iter().filter(|t| matches!(*t, Token::Symbol('}'))).count();
    
    assert_eq!(open_count, 2);
    assert_eq!(close_count, 2);
}

#[test]
fn test_block_with_whitespace_and_newlines() {
    let mut lexer = Lexer::new("{\n  let x = 5;\n  let y = 10;\n}");
    let tokens = collect_tokens(&mut lexer);
    
    assert!(matches!(tokens[0], Token::Symbol('{')));
    assert!(matches!(tokens[tokens.len() - 2], Token::Symbol('}')));
    
    // Should still parse correctly despite whitespace
    let let_count = tokens.iter().filter(|t| {
        if let Token::Keyword(k) = t {
            k == "let"
        } else {
            false
        }
    }).count();
    assert_eq!(let_count, 2);
}

#[test]
fn test_block_with_comments() {
    let mut lexer = Lexer::new("{ // comment\n let x = 5; /* block comment */ }");
    let tokens = collect_tokens(&mut lexer);
    
    assert!(matches!(tokens[0], Token::Symbol('{')));
    assert!(matches!(tokens[tokens.len() - 2], Token::Symbol('}')));
    
    // Comments should be ignored, only tokens should remain
    assert!(tokens.iter().any(|t| {
        if let Token::Keyword(k) = t {
            k == "let"
        } else {
            false
        }
    }));
}

#[test]
fn test_unmatched_braces_detection() {
    // This test ensures we can detect unmatched braces
    // (even though lexer doesn't validate matching, parser will)
    
    let mut lexer = Lexer::new("{ { }");
    let tokens = collect_tokens(&mut lexer);
    
    let open_count = tokens.iter().filter(|t| matches!(*t, Token::Symbol('{'))).count();
    let close_count = tokens.iter().filter(|t| matches!(*t, Token::Symbol('}'))).count();
    
    assert_eq!(open_count, 2);
    assert_eq!(close_count, 1);
    // Parser should catch this mismatch
}

#[test]
fn test_braces_with_strings() {
    let mut lexer = Lexer::new("{ let message = \"Hello {world}\"; }");
    let tokens = collect_tokens(&mut lexer);
    
    assert!(matches!(tokens[0], Token::Symbol('{')));
    
    // String should contain braces as content, not tokens
    let string_tokens: Vec<_> = tokens.iter().filter(|t| matches!(*t, Token::String(_))).collect();
    assert_eq!(string_tokens.len(), 1);
    
    if let Token::String(s) = &string_tokens[0] {
        assert!(s.contains("{world}"));
    }
    
    assert!(matches!(tokens[tokens.len() - 2], Token::Symbol('}')));
}

#[test]
fn test_exact_syntax_md_examples() {
    // Test examples from SYNTAX.md that use blocks
    
    // Function example
    let mut lexer = Lexer::new("function saudacao(nome) { return \"Olá, \" + nome + \"!\"; }");
    let tokens = collect_tokens(&mut lexer);
    assert!(tokens.iter().any(|t| matches!(*t, Token::Symbol('{'))));
    assert!(tokens.iter().any(|t| matches!(*t, Token::Symbol('}'))));
    
    // Class example
    let mut lexer = Lexer::new("class Pessoa { function init(nome) { } }");
    let tokens = collect_tokens(&mut lexer);
    let open_count = tokens.iter().filter(|t| matches!(*t, Token::Symbol('{'))).count();
    let close_count = tokens.iter().filter(|t| matches!(*t, Token::Symbol('}'))).count();
    assert_eq!(open_count, 2);
    assert_eq!(close_count, 2);
    
    // If example
    let mut lexer = Lexer::new("if idade >= 18 { print(\"Maior de idade\"); }");
    let tokens = collect_tokens(&mut lexer);
    assert!(tokens.iter().any(|t| matches!(*t, Token::Symbol('{'))));
    assert!(tokens.iter().any(|t| matches!(*t, Token::Symbol('}'))));
}

fn collect_tokens(lexer: &mut Lexer) -> Vec<Token> {
    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token().unwrap();
        if matches!(token.token, Token::Eof) {
            tokens.push(token.token);
            break;
        }
        tokens.push(token.token);
    }
    tokens
}
