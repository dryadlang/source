// crates/dryad_parser/tests/const_parser_tests.rs
use dryad_parser::{Parser, ast::{Stmt, Expr, Literal}};
use dryad_lexer::{Lexer, token::Token};

fn parse_tokens(input: &str) -> dryad_parser::ast::Program {
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();
    
    loop {
        let tok = lexer.next_token().unwrap();
        if let Token::Eof = tok.token { break; }
        tokens.push(tok);
    }
    
    let mut parser = Parser::new(tokens);
    parser.parse().unwrap()
}

#[test]
fn test_parse_simple_const_declaration() {
    let program = parse_tokens("const PI = 3.14159;");
    assert_eq!(program.statements.len(), 1);
    
    match &program.statements[0] {
        Stmt::ConstDeclaration(name, _, expr, _) => {
            assert_eq!(name.identifier_name().unwrap(), "PI");
            match expr {
                Expr::Literal(Literal::Number(n), _) => assert_eq!(*n, 3.14159),
                _ => panic!("Esperado número literal"),
            }
        }
        _ => panic!("Esperado ConstDeclaration"),
    }
}

#[test]
fn test_parse_const_string_declaration() {
    let program = parse_tokens("const APP_NAME = \"Dryad Language\";");
    assert_eq!(program.statements.len(), 1);
    
    match &program.statements[0] {
        Stmt::ConstDeclaration(name, _, expr, _) => {
            assert_eq!(name.identifier_name().unwrap(), "APP_NAME");
            match expr {
                Expr::Literal(Literal::String(s), _) => assert_eq!(s, "Dryad Language"),
                _ => panic!("Esperado string literal"),
            }
        }
        _ => panic!("Esperado ConstDeclaration"),
    }
}

#[test]
fn test_parse_const_boolean_declaration() {
    let program = parse_tokens("const DEBUG_MODE = true;");
    assert_eq!(program.statements.len(), 1);
    
    match &program.statements[0] {
        Stmt::ConstDeclaration(name, _, expr, _) => {
            assert_eq!(name.identifier_name().unwrap(), "DEBUG_MODE");
            match expr {
                Expr::Literal(Literal::Bool(b), _) => assert_eq!(*b, true),
                _ => panic!("Esperado boolean literal"),
            }
        }
        _ => panic!("Esperado ConstDeclaration"),
    }
}

#[test]
fn test_parse_const_with_expression() {
    let program = parse_tokens("const MAX_SIZE = 100 * 2;");
    assert_eq!(program.statements.len(), 1);
    
    match &program.statements[0] {
        Stmt::ConstDeclaration(name, _, expr, _) => {
            assert_eq!(name.identifier_name().unwrap(), "MAX_SIZE");
            match expr {
                Expr::Binary { .. } => {}, // Verificação simples que é uma expressão binária
                _ => panic!("Esperado expressão binária"),
            }
        }
        _ => panic!("Esperado ConstDeclaration"),
    }
}

#[test]
fn test_const_without_value_error() {
    let mut lexer = Lexer::new("const PI;");
    let mut tokens = Vec::new();
    
    loop {
        match lexer.next_token().unwrap() {
            tok if tok.token == Token::Eof => break,
            tok => tokens.push(tok),
        }
    }
    
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(result.is_err());
}

#[test]
fn test_multiple_const_declarations() {
    let program = parse_tokens(r#"
        const PI = 3.14159;
        const E = 2.71828;
        const NAME = "Dryad";
    "#);
    
    assert_eq!(program.statements.len(), 3);
    
    // Verifica primeira constante
    match &program.statements[0] {
        Stmt::ConstDeclaration(name, _, _, _) => assert_eq!(name.identifier_name().unwrap(), "PI"),
        _ => panic!("Esperado ConstDeclaration"),
    }
    
    // Verifica segunda constante
    match &program.statements[1] {
        Stmt::ConstDeclaration(name, _, _, _) => assert_eq!(name.identifier_name().unwrap(), "E"),
        _ => panic!("Esperado ConstDeclaration"),
    }
    
    // Verifica terceira constante
    match &program.statements[2] {
        Stmt::ConstDeclaration(name, _, _, _) => assert_eq!(name.identifier_name().unwrap(), "NAME"),
        _ => panic!("Esperado ConstDeclaration"),
    }
}