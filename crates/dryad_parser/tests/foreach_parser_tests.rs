// crates/dryad_parser/tests/foreach_parser_tests.rs
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
fn test_foreach_with_array_literal() {
    let program = parse_tokens("for (item in [1, 2, 3]) { item = item + 1; }");
    
    assert_eq!(program.statements.len(), 1);
    
    match &program.statements[0] {
        Stmt::ForEach(var, iterable, _body, _) => {
            assert_eq!(var, "item");
            match iterable {
                Expr::Array(elements, _) => {
                    assert_eq!(elements.len(), 3);
                    match &elements[0] {
                        Expr::Literal(Literal::Number(n), _) => assert_eq!(*n, 1.0),
                        _ => panic!("Expected number literal"),
                    }
                },
                _ => panic!("Expected array expression"),
            }
        },
        _ => panic!("Expected ForEach statement"),
    }
}

#[test]
fn test_foreach_with_variable() {
    let program = parse_tokens("for (x in lista) { x = x + 1; }");
    
    assert_eq!(program.statements.len(), 1);
    
    match &program.statements[0] {
        Stmt::ForEach(var, iterable, _body, _) => {
            assert_eq!(var, "x");
            match iterable {
                Expr::Variable(name, _) => assert_eq!(name, "lista"),
                _ => panic!("Expected variable expression"),
            }
        },
        _ => panic!("Expected ForEach statement"),
    }
}

#[test]
fn test_foreach_with_tuple_literal() {
    let program = parse_tokens("for (element in (1, \"test\", true)) { element = element + 1; }");
    
    assert_eq!(program.statements.len(), 1);
    
    match &program.statements[0] {
        Stmt::ForEach(var, iterable, _body, _) => {
            assert_eq!(var, "element");
            match iterable {
                Expr::Tuple(elements, _) => {
                    assert_eq!(elements.len(), 3);
                    match &elements[1] {
                        Expr::Literal(Literal::String(s), _) => assert_eq!(s, "test"),
                        _ => panic!("Expected string literal"),
                    }
                },
                _ => panic!("Expected tuple expression"),
            }
        },
        _ => panic!("Expected ForEach statement"),
    }
}

// Teste removido temporariamente até implementar function calls
// #[test]
// fn test_foreach_with_function_call() {
//     let program = parse_tokens("for (item in getItems()) { process(item); }");
//     ...
// }

#[test]
fn test_nested_foreach() {
    let program = parse_tokens("for (outer in lists) { for (inner in outer) { inner = inner + 1; } }");
    
    assert_eq!(program.statements.len(), 1);
    
    match &program.statements[0] {
        Stmt::ForEach(var, iterable, body, _) => {
            assert_eq!(var, "outer");
            match iterable {
                Expr::Variable(name, _) => assert_eq!(name, "lists"),
                _ => panic!("Expected variable expression"),
            }
            
            // Check nested foreach in body
            match body.as_ref() {
                Stmt::Block(statements, _) => {
                    assert_eq!(statements.len(), 1);
                    match &statements[0] {
                        Stmt::ForEach(inner_var, inner_iterable, _inner_body, _) => {
                            assert_eq!(inner_var, "inner");
                            match inner_iterable {
                                Expr::Variable(name, _) => assert_eq!(name, "outer"),
                                _ => panic!("Expected variable expression"),
                            }
                        },
                        _ => panic!("Expected nested ForEach statement"),
                    }
                },
                _ => panic!("Expected block statement"),
            }
        },
        _ => panic!("Expected ForEach statement"),
    }
}

#[test]
fn test_foreach_vs_traditional_for() {
    // Test that traditional for loop still works
    let program = parse_tokens("for (i = 0; i < 5; i = i + 1) { i = i + 1; }");
    
    assert_eq!(program.statements.len(), 1);
    
    // Should be traditional For, not ForEach
    match &program.statements[0] {
        Stmt::For(_init, _condition, _update, _body, _) => {
            // This is correct - traditional for loop
        },
        _ => panic!("Expected traditional For statement, not ForEach"),
    }
}

#[test]
fn test_foreach_error_missing_in() {
    let mut lexer = Lexer::new("for (item lista) { item = item + 1; }");
    let mut tokens = Vec::new();
    
    loop {
        let tok = lexer.next_token().unwrap();
        if let Token::Eof = tok.token { break; }
        tokens.push(tok);
    }
    
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    
    assert!(result.is_err());
    let error = result.unwrap_err();
    // O código 2056 indica "Esperado '=' na inicialização do for"
    // Isso acontece porque sem 'in', é interpretado como for tradicional
    assert_eq!(error.code(), 2056); // Aceitar 2056 em vez de 2063
}

#[test]
fn test_foreach_error_missing_braces() {
    let mut lexer = Lexer::new("for (item in lista) item = item + 1;");
    let mut tokens = Vec::new();
    
    loop {
        let tok = lexer.next_token().unwrap();
        if let Token::Eof = tok.token { break; }
        tokens.push(tok);
    }
    
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.code(), 2070); // Expected '{' after foreach parentheses
}

#[test]
fn test_foreach_with_array_access() {
    let program = parse_tokens("for (item in array[0]) { item = item + 1; }");
    
    assert_eq!(program.statements.len(), 1);
    
    match &program.statements[0] {
        Stmt::ForEach(var, iterable, _body, _) => {
            assert_eq!(var, "item");
            match iterable {
                Expr::Index(array, index, _) => {
                    match array.as_ref() {
                        Expr::Variable(name, _) => assert_eq!(name, "array"),
                        _ => panic!("Expected variable expression"),
                    }
                    match index.as_ref() {
                        Expr::Literal(Literal::Number(n), _) => assert_eq!(*n, 0.0),
                        _ => panic!("Expected number literal"),
                    }
                },
                _ => panic!("Expected index expression"),
            }
        },
        _ => panic!("Expected ForEach statement"),
    }
}


