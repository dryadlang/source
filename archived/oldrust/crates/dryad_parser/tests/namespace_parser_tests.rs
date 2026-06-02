// crates/dryad_parser/tests/namespace_parser_tests.rs

use dryad_lexer::{Lexer, Token};
use dryad_parser::{ast::*, Parser};

fn parse_dryad_code(input: &str) -> Result<Program, String> {
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();

    loop {
        match lexer.next_token() {
            Ok(tok) if tok.token == Token::Eof => break,
            Ok(token) => tokens.push(token),
            Err(e) => return Err(format!("Lexer error: {:?}", e)),
        }
    }

    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(program) => Ok(program),
        Err(e) => Err(format!("Parser error: {:?}", e)),
    }
}

#[test]
fn test_basic_namespace() {
    let code = r#"
        namespace MyNamespace {
            let x = 1;
        }
    "#;

    let result = parse_dryad_code(code);
    assert!(
        result.is_ok(),
        "Failed to parse basic namespace: {:?}",
        result.err()
    );

    if let Ok(program) = result {
        assert_eq!(program.statements.len(), 1);

        if let Stmt::Namespace(name, statements, _) = &program.statements[0] {
            assert_eq!(name, "MyNamespace");
            assert_eq!(statements.len(), 1);

            // Check that the statement inside namespace is a variable declaration
            if let Stmt::VarDeclaration(pattern, _, Some(expr), _) = &statements[0] {
                assert_eq!(pattern.identifier_name().unwrap(), "x");
                if let Expr::Literal(Literal::Number(num), _) = expr {
                    assert_eq!(*num, 1.0);
                } else {
                    panic!("Expected number literal");
                }
            } else {
                panic!("Expected variable declaration in namespace");
            }
        } else {
            panic!("Expected namespace statement");
        }
    }
}

#[test]
fn test_empty_namespace() {
    let code = r#"
        namespace Empty {
        }
    "#;

    let result = parse_dryad_code(code);
    assert!(
        result.is_ok(),
        "Failed to parse empty namespace: {:?}",
        result.err()
    );

    if let Ok(program) = result {
        assert_eq!(program.statements.len(), 1);

        if let Stmt::Namespace(name, statements, _) = &program.statements[0] {
            assert_eq!(name, "Empty");
            assert_eq!(statements.len(), 0, "Expected empty namespace");
        } else {
            panic!("Expected namespace statement");
        }
    }
}

#[test]
fn test_namespace_with_multiple_statements() {
    let code = r#"
        namespace Utils {
            let a = 1;
            function foo() {
                return 2;
            }
        }
    "#;

    let result = parse_dryad_code(code);
    assert!(
        result.is_ok(),
        "Failed to parse namespace with multiple statements: {:?}",
        result.err()
    );

    if let Ok(program) = result {
        assert_eq!(program.statements.len(), 1);

        if let Stmt::Namespace(name, statements, _) = &program.statements[0] {
            assert_eq!(name, "Utils");
            assert_eq!(statements.len(), 2, "Expected 2 statements in namespace");

            // First statement: variable declaration
            if let Stmt::VarDeclaration(pattern, _, Some(expr), _) = &statements[0] {
                assert_eq!(pattern.identifier_name().unwrap(), "a");
                if let Expr::Literal(Literal::Number(num), _) = expr {
                    assert_eq!(*num, 1.0);
                } else {
                    panic!("Expected number literal");
                }
            } else {
                panic!("Expected variable declaration as first statement");
            }

            // Second statement: function declaration
            if let Stmt::FunctionDeclaration { name, params, .. } = &statements[1] {
                assert_eq!(name, "foo");
                assert_eq!(params.len(), 0, "Expected no parameters");
            } else {
                panic!("Expected function declaration as second statement");
            }
        } else {
            panic!("Expected namespace statement");
        }
    }
}

#[test]
fn test_namespace_with_function_declaration() {
    let code = r#"
        namespace MyUtils {
            function greet(name) {
                return "Hello, " + name;
            }
        }
    "#;

    let result = parse_dryad_code(code);
    assert!(
        result.is_ok(),
        "Failed to parse namespace with function: {:?}",
        result.err()
    );

    if let Ok(program) = result {
        assert_eq!(program.statements.len(), 1);

        if let Stmt::Namespace(name, statements, _) = &program.statements[0] {
            assert_eq!(name, "MyUtils");
            assert_eq!(statements.len(), 1);

            if let Stmt::FunctionDeclaration { name, params, .. } = &statements[0] {
                assert_eq!(name, "greet");
                assert_eq!(params.len(), 1, "Expected 1 parameter");
                assert_eq!(params[0].0, "name");
            } else {
                panic!("Expected function declaration in namespace");
            }
        } else {
            panic!("Expected namespace statement");
        }
    }
}
