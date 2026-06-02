// crates/dryad_parser/tests/lambda_parser_tests.rs

use dryad_parser::{Parser, ast::*};
use dryad_lexer::{Lexer, token::Token};

fn parse_expression(input: &str) -> Result<Expr, dryad_errors::DryadError> {
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();
    
    loop {
        let tok = lexer.next_token().unwrap();
        if let Token::Eof = tok.token { break; }
        tokens.push(tok);
    }
    
    let mut parser = Parser::new(tokens);
    // Para testar expressões, vamos criar um statement de expressão e extrair a expressão
    if let Some(Stmt::Expression(expr, _)) = parser.statement()? {
        Ok(expr)
    } else {
        panic!("Expected expression statement");
    }
}

#[test]
fn test_single_param_lambda() {
    let expr = parse_expression("x => x * 2").unwrap();
    
    match expr {
        Expr::Lambda { params, body, .. } => {
            assert_eq!(params[0].0, "x".to_string());
            match *body {
                Expr::Binary(left, op, right, _) => {
                    assert_eq!(op, "*");
                    assert!(matches!(*left, Expr::Variable(name, _) if name == "x"));
                    assert!(matches!(*right, Expr::Literal(Literal::Number(2.0), _)));
                }
                _ => panic!("Expected binary expression")
            }
        }
        _ => panic!("Expected lambda expression")
    }
}

#[test]
fn test_multi_param_lambda() {
    let expr = parse_expression("(a, b) => a + b").unwrap();
    
    match expr {
        Expr::Lambda { params, body, .. } => {
            assert_eq!(params.len(), 2); assert_eq!(params[0].0, "a".to_string()); assert_eq!(params[1].0, "b".to_string());
            match *body {
                Expr::Binary(left, op, right, _) => {
                    assert_eq!(op, "+");
                    assert!(matches!(*left, Expr::Variable(name, _) if name == "a"));
                    assert!(matches!(*right, Expr::Variable(name, _) if name == "b"));
                }
                _ => panic!("Expected binary expression")
            }
        }
        _ => panic!("Expected lambda expression")
    }
}

#[test]
fn test_zero_param_lambda() {
    let expr = parse_expression("() => 42").unwrap();
    
    match expr {
        Expr::Lambda { params, body, .. } => {
            assert!(params.is_empty());
            assert!(matches!(*body, Expr::Literal(Literal::Number(42.0), _)));
        }
        _ => panic!("Expected lambda expression")
    }
}

#[test]
fn test_lambda_assignment() {
    let mut lexer = Lexer::new("let quadrado = x => x * x;");
    let mut tokens = Vec::new();
    
    loop {
        match lexer.next_token().unwrap() {
            tok if tok.token == Token::Eof => break,
            tok => tokens.push(tok),
        }
    }
    
    let mut parser = Parser::new(tokens);
    let stmt = parser.statement().unwrap().unwrap();
    
    match stmt {
        Stmt::VarDeclaration(name, _, Some(expr), _) => {
            assert_eq!(name.identifier_name().unwrap(), "quadrado");
            match expr {
                Expr::Lambda { params, body, .. } => {
                    assert_eq!(params[0].0, "x".to_string());
                    match *body {
                        Expr::Binary(left, op, right, _) => {
                            assert_eq!(op, "*");
                            assert!(matches!(*left, Expr::Variable(name, _) if name == "x"));
                            assert!(matches!(*right, Expr::Variable(name, _) if name == "x"));
                        }
                        _ => panic!("Expected binary expression")
                    }
                }
                _ => panic!("Expected lambda expression")
            }
        }
        _ => panic!("Expected variable declaration")
    }
}

#[test]
fn test_nested_lambdas() {
    let expr = parse_expression("x => y => x + y").unwrap();
    
    match expr {
        Expr::Lambda { params, body, .. } => {
            assert_eq!(params[0].0, "x".to_string());
            match *body {
                Expr::Lambda { params: inner_params, body: inner_body, .. } => {
                    assert_eq!(inner_params[0].0, "y".to_string());
                    match *inner_body {
                        Expr::Binary(left, op, right, _) => {
                            assert_eq!(op, "+");
                            assert!(matches!(*left, Expr::Variable(name, _) if name == "x"));
                            assert!(matches!(*right, Expr::Variable(name, _) if name == "y"));
                        }
                        _ => panic!("Expected binary expression")
                    }
                }
                _ => panic!("Expected inner lambda")
            }
        }
        _ => panic!("Expected lambda expression")
    }
}

#[test]
fn test_lambda_with_complex_expression() {
    let expr = parse_expression("(x, y) => x * 2 + y / 3").unwrap();
    
    match expr {
        Expr::Lambda { params, body, .. } => {
            assert_eq!(params.len(), 2); assert_eq!(params[0].0, "x".to_string()); assert_eq!(params[1].0, "y".to_string());
            // O corpo deve ser: (x * 2) + (y / 3)
            match *body {
                Expr::Binary(_, op, _, _) => {
                    assert_eq!(op, "+");
                    // Não vamos verificar toda a estrutura da árvore aqui,
                    // só confirmar que foi parseado como lambda
                }
                _ => panic!("Expected binary expression")
            }
        }
        _ => panic!("Expected lambda expression")
    }
}
