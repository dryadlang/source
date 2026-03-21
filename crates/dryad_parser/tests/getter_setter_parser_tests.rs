// crates/dryad_parser/tests/getter_setter_parser_tests.rs

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
fn test_class_with_getter() {
    let code = r#"
        class Foo {
            get name() {
                return this.name;
            }
        }
    "#;

    let result = parse_dryad_code(code);
    assert!(
        result.is_ok(),
        "Failed to parse class with getter: {:?}",
        result.err()
    );

    if let Ok(program) = result {
        assert_eq!(program.statements.len(), 1);

        if let Stmt::ClassDeclaration(name, parent, _, members, _) = &program.statements[0] {
            assert_eq!(name, "Foo");
            assert!(parent.is_none());
            assert_eq!(members.len(), 1);

            if let ClassMember::Getter {
                visibility,
                is_static,
                name: getter_name,
                body,
            } = &members[0]
            {
                assert!(matches!(visibility, Visibility::Public));
                assert!(!is_static);
                assert_eq!(getter_name, "name");
                // Body is wrapped in a Block containing the return statement
                if let Stmt::Block(stmts, _) = &**body {
                    assert!(!stmts.is_empty());
                    assert!(matches!(stmts[0], Stmt::Return(_, _)));
                } else {
                    panic!("Expected body to be a block statement");
                }
            } else {
                panic!("Expected getter member");
            }
        } else {
            panic!("Expected class declaration");
        }
    }
}

#[test]
fn test_class_with_setter() {
    let code = r#"
        class Foo {
            set name(value) {
                this.name = value;
            }
        }
    "#;

    let result = parse_dryad_code(code);
    assert!(
        result.is_ok(),
        "Failed to parse class with setter: {:?}",
        result.err()
    );

    if let Ok(program) = result {
        assert_eq!(program.statements.len(), 1);

        if let Stmt::ClassDeclaration(name, parent, _, members, _) = &program.statements[0] {
            assert_eq!(name, "Foo");
            assert!(parent.is_none());
            assert_eq!(members.len(), 1);

            if let ClassMember::Setter {
                visibility,
                is_static,
                name: setter_name,
                param,
                body,
            } = &members[0]
            {
                assert!(matches!(visibility, Visibility::Public));
                assert!(!is_static);
                assert_eq!(setter_name, "name");
                assert_eq!(param, "value");
                // Body is wrapped in a Block containing the assignment statement
                if let Stmt::Block(stmts, _) = &**body {
                    assert!(!stmts.is_empty());
                    assert!(matches!(
                        stmts[0],
                        Stmt::PropertyAssignment(_, _, _, _) | Stmt::Assignment(_, _, _)
                    ));
                } else {
                    panic!("Expected body to be a block statement");
                }
            } else {
                panic!("Expected setter member");
            }
        } else {
            panic!("Expected class declaration");
        }
    }
}

#[test]
fn test_class_with_getter_and_setter() {
    let code = r#"
        class Person {
            get fullName() {
                return this.first + " " + this.last;
            }
            set fullName(value) {
                this.first = value;
            }
        }
    "#;

    let result = parse_dryad_code(code);
    assert!(
        result.is_ok(),
        "Failed to parse class with getter and setter: {:?}",
        result.err()
    );

    if let Ok(program) = result {
        assert_eq!(program.statements.len(), 1);

        if let Stmt::ClassDeclaration(name, parent, _, members, _) = &program.statements[0] {
            assert_eq!(name, "Person");
            assert!(parent.is_none());
            assert_eq!(members.len(), 2);

            // Check first member is getter
            if let ClassMember::Getter {
                visibility: vis1,
                is_static: static1,
                name: name1,
                body: body1,
            } = &members[0]
            {
                assert!(matches!(vis1, Visibility::Public));
                assert!(!static1);
                assert_eq!(name1, "fullName");
                if let Stmt::Block(stmts, _) = &**body1 {
                    assert!(!stmts.is_empty());
                    assert!(matches!(stmts[0], Stmt::Return(_, _)));
                } else {
                    panic!("Expected getter body to be a block statement");
                }
            } else {
                panic!("Expected first member to be getter");
            }

            // Check second member is setter
            if let ClassMember::Setter {
                visibility: vis2,
                is_static: static2,
                name: name2,
                param: param2,
                body: body2,
            } = &members[1]
            {
                assert!(matches!(vis2, Visibility::Public));
                assert!(!static2);
                assert_eq!(name2, "fullName");
                assert_eq!(param2, "value");
                if let Stmt::Block(stmts, _) = &**body2 {
                    assert!(!stmts.is_empty());
                    assert!(matches!(
                        stmts[0],
                        Stmt::PropertyAssignment(_, _, _, _) | Stmt::Assignment(_, _, _)
                    ));
                } else {
                    panic!("Expected setter body to be a block statement");
                }
            } else {
                panic!("Expected second member to be setter");
            }
        } else {
            panic!("Expected class declaration");
        }
    }
}

#[test]
fn test_getter_setter_with_visibility_modifiers() {
    let code = r#"
        class Foo {
            public get x() {
                return 1;
            }
            private set x(v) {
            }
        }
    "#;

    let result = parse_dryad_code(code);
    assert!(
        result.is_ok(),
        "Failed to parse getter/setter with visibility modifiers: {:?}",
        result.err()
    );

    if let Ok(program) = result {
        assert_eq!(program.statements.len(), 1);

        if let Stmt::ClassDeclaration(_, _, _, members, _) = &program.statements[0] {
            assert_eq!(members.len(), 2);

            // Check getter visibility is public
            if let ClassMember::Getter {
                visibility: vis1, ..
            } = &members[0]
            {
                assert!(matches!(vis1, Visibility::Public));
            } else {
                panic!("Expected first member to be getter");
            }

            // Check setter visibility is private
            if let ClassMember::Setter {
                visibility: vis2, ..
            } = &members[1]
            {
                assert!(matches!(vis2, Visibility::Private));
            } else {
                panic!("Expected second member to be setter");
            }
        } else {
            panic!("Expected class declaration");
        }
    }
}
