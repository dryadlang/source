// crates/dryad_parser/tests/interface_parser_tests.rs

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
fn test_basic_interface_declaration() {
    // Test: interface Drawable { function draw(); }
    let code = r#"
        interface Drawable {
            function draw();
        }
    "#;

    let result = parse_dryad_code(code);
    assert!(
        result.is_ok(),
        "Failed to parse basic interface: {:?}",
        result.err()
    );

    if let Ok(program) = result {
        assert_eq!(program.statements.len(), 1);

        if let Stmt::InterfaceDeclaration(name, members, _) = &program.statements[0] {
            assert_eq!(name, "Drawable");
            assert_eq!(members.len(), 1);

            let InterfaceMember::Method(method) = &members[0];
            assert_eq!(method.name, "draw");
            assert_eq!(method.params.len(), 0);
            assert!(method.return_type.is_none());
        } else {
            panic!("Expected interface declaration");
        }
    }
}

#[test]
fn test_interface_with_multiple_methods() {
    // Test: interface Shape { function area(); function perimeter(); }
    let code = r#"
        interface Shape {
            function area();
            function perimeter();
        }
    "#;

    let result = parse_dryad_code(code);
    assert!(
        result.is_ok(),
        "Failed to parse interface with multiple methods: {:?}",
        result.err()
    );

    if let Ok(program) = result {
        assert_eq!(program.statements.len(), 1);

        if let Stmt::InterfaceDeclaration(name, members, _) = &program.statements[0] {
            assert_eq!(name, "Shape");
            assert_eq!(members.len(), 2);

            // Check first method
            let InterfaceMember::Method(method1) = &members[0];
            assert_eq!(method1.name, "area");
            assert_eq!(method1.params.len(), 0);
            assert!(method1.return_type.is_none());

            // Check second method
            let InterfaceMember::Method(method2) = &members[1];
            assert_eq!(method2.name, "perimeter");
            assert_eq!(method2.params.len(), 0);
            assert!(method2.return_type.is_none());
        } else {
            panic!("Expected interface declaration");
        }
    }
}

#[test]
fn test_interface_with_typed_parameters() {
    // Test: interface Comparable { function compareTo(other: any); }
    let code = r#"
        interface Comparable {
            function compareTo(other: any);
        }
    "#;

    let result = parse_dryad_code(code);
    assert!(
        result.is_ok(),
        "Failed to parse interface with typed parameters: {:?}",
        result.err()
    );

    if let Ok(program) = result {
        assert_eq!(program.statements.len(), 1);

        if let Stmt::InterfaceDeclaration(name, members, _) = &program.statements[0] {
            assert_eq!(name, "Comparable");
            assert_eq!(members.len(), 1);

            let InterfaceMember::Method(method) = &members[0];
            assert_eq!(method.name, "compareTo");
            assert_eq!(method.params.len(), 1);
            assert_eq!(method.params[0].0, "other");
            assert!(method.params[0].1.is_some());
            assert!(method.return_type.is_none());
        } else {
            panic!("Expected interface declaration");
        }
    }
}

#[test]
fn test_class_with_implements_clause() {
    // Test: class Circle implements Drawable { }
    let code = r#"
        class Circle implements Drawable {
        }
    "#;

    let result = parse_dryad_code(code);
    assert!(
        result.is_ok(),
        "Failed to parse class with implements: {:?}",
        result.err()
    );

    if let Ok(program) = result {
        assert_eq!(program.statements.len(), 1);

        if let Stmt::ClassDeclaration(name, parent, interfaces, _, _) = &program.statements[0] {
            assert_eq!(name, "Circle");
            assert!(parent.is_none());
            assert_eq!(interfaces.len(), 1);
            assert_eq!(interfaces[0], "Drawable");
        } else {
            panic!("Expected class declaration");
        }
    }
}

#[test]
fn test_class_with_extends_and_implements() {
    // Test: class Circle extends Shape implements Drawable { }
    let code = r#"
        class Circle extends Shape implements Drawable {
        }
    "#;

    let result = parse_dryad_code(code);
    assert!(
        result.is_ok(),
        "Failed to parse class with extends and implements: {:?}",
        result.err()
    );

    if let Ok(program) = result {
        assert_eq!(program.statements.len(), 1);

        if let Stmt::ClassDeclaration(name, parent, interfaces, _, _) = &program.statements[0] {
            assert_eq!(name, "Circle");
            assert_eq!(parent.as_ref().unwrap(), "Shape");
            assert_eq!(interfaces.len(), 1);
            assert_eq!(interfaces[0], "Drawable");
        } else {
            panic!("Expected class declaration");
        }
    }
}

#[test]
fn test_class_with_multiple_implements() {
    // Test: class MyClass implements Interface1, Interface2 { }
    let code = r#"
        class MyClass implements Interface1, Interface2 {
        }
    "#;

    let result = parse_dryad_code(code);
    assert!(
        result.is_ok(),
        "Failed to parse class with multiple implements: {:?}",
        result.err()
    );

    if let Ok(program) = result {
        assert_eq!(program.statements.len(), 1);

        if let Stmt::ClassDeclaration(name, parent, interfaces, _, _) = &program.statements[0] {
            assert_eq!(name, "MyClass");
            assert!(parent.is_none());
            assert_eq!(interfaces.len(), 2);
            assert_eq!(interfaces[0], "Interface1");
            assert_eq!(interfaces[1], "Interface2");
        } else {
            panic!("Expected class declaration");
        }
    }
}

#[test]
fn test_interface_with_return_type() {
    // Test: interface Calculator { function add(a: number, b: number): number; }
    let code = r#"
        interface Calculator {
            function add(a: number, b: number): number;
        }
    "#;

    let result = parse_dryad_code(code);
    assert!(
        result.is_ok(),
        "Failed to parse interface with return type: {:?}",
        result.err()
    );

    if let Ok(program) = result {
        assert_eq!(program.statements.len(), 1);

        if let Stmt::InterfaceDeclaration(name, members, _) = &program.statements[0] {
            assert_eq!(name, "Calculator");
            assert_eq!(members.len(), 1);

            let InterfaceMember::Method(method) = &members[0];
            assert_eq!(method.name, "add");
            assert_eq!(method.params.len(), 2);
            assert_eq!(method.params[0].0, "a");
            assert_eq!(method.params[1].0, "b");
            assert!(method.return_type.is_some());
        } else {
            panic!("Expected interface declaration");
        }
    }
}

#[test]
fn test_interface_with_multiple_params() {
    // Test: interface Logger { function log(msg: string, level: string); }
    let code = r#"
        interface Logger {
            function log(msg: string, level: string);
        }
    "#;

    let result = parse_dryad_code(code);
    assert!(
        result.is_ok(),
        "Failed to parse interface with multiple params: {:?}",
        result.err()
    );

    if let Ok(program) = result {
        assert_eq!(program.statements.len(), 1);

        if let Stmt::InterfaceDeclaration(name, members, _) = &program.statements[0] {
            assert_eq!(name, "Logger");
            assert_eq!(members.len(), 1);

            let InterfaceMember::Method(method) = &members[0];
            assert_eq!(method.name, "log");
            assert_eq!(method.params.len(), 2);
            assert_eq!(method.params[0].0, "msg");
            assert_eq!(method.params[1].0, "level");
        } else {
            panic!("Expected interface declaration");
        }
    }
}
