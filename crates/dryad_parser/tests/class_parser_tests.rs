// crates/dryad_parser/tests/class_parser_tests.rs

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
fn test_class_declaration_parsing() {
    let code = r#"
        class Pessoa {
            function init(nome, idade) {
                this.nome = nome;
                this.idade = idade;
            }
        }
    "#;

    let result = parse_dryad_code(code);
    assert!(
        result.is_ok(),
        "Failed to parse class declaration: {:?}",
        result.err()
    );

    if let Ok(program) = result {
        assert_eq!(program.statements.len(), 1);

        if let Stmt::ClassDeclaration(name, parent, members, _) = &program.statements[0] {
            assert_eq!(name, "Pessoa");
            assert!(parent.is_none());
            assert_eq!(members.len(), 1);

            if let ClassMember::Method {
                visibility,
                is_static,
                name: method_name,
                params,
                ..
            } = &members[0]
            {
                assert!(matches!(visibility, Visibility::Public));
                assert!(!is_static);
                assert_eq!(method_name, "init");
                assert_eq!(params.len(), 2);
                assert_eq!(params[0].0, "nome");
                assert_eq!(params[1].0, "idade");
            } else {
                panic!("Expected method member");
            }
        } else {
            panic!("Expected class declaration");
        }
    }
}

#[test]
fn test_class_with_inheritance() {
    let code = r#"
        class Estudante extends Pessoa {
            function init(nome, idade, curso) {
                super.init(nome, idade);
                this.curso = curso;
            }
        }
    "#;

    let result = parse_dryad_code(code);
    assert!(
        result.is_ok(),
        "Failed to parse class with inheritance: {:?}",
        result.err()
    );

    if let Ok(program) = result {
        if let Stmt::ClassDeclaration(name, parent, _, _) = &program.statements[0] {
            assert_eq!(name, "Estudante");
            assert_eq!(parent.as_ref().unwrap(), "Pessoa");
        } else {
            panic!("Expected class declaration");
        }
    }
}

#[test]
fn test_class_with_visibility_modifiers() {
    let code = r#"
        class TestClass {
            public function publicMethod() {
                return "public";
            }
            
            private function privateMethod() {
                return "private";
            }
            
            protected function protectedMethod() {
                return "protected";
            }
        }
    "#;

    let result = parse_dryad_code(code);
    assert!(
        result.is_ok(),
        "Failed to parse class with visibility modifiers: {:?}",
        result.err()
    );

    if let Ok(program) = result {
        if let Stmt::ClassDeclaration(_, _, members, _) = &program.statements[0] {
            assert_eq!(members.len(), 3);

            // Check visibility modifiers
            for (i, expected_visibility) in [
                Visibility::Public,
                Visibility::Private,
                Visibility::Protected,
            ]
            .iter()
            .enumerate()
            {
                if let ClassMember::Method { visibility, .. } = &members[i] {
                    assert!(matches!(visibility, expected_visibility));
                } else {
                    panic!("Expected method member at index {}", i);
                }
            }
        } else {
            panic!("Expected class declaration");
        }
    }
}

#[test]
fn test_class_with_static_methods() {
    let code = r#"
        class MathUtils {
            static function pi() {
                return 3.14159;
            }
            
            static function add(a, b) {
                return a + b;
            }
        }
    "#;

    let result = parse_dryad_code(code);
    assert!(
        result.is_ok(),
        "Failed to parse class with static methods: {:?}",
        result.err()
    );

    if let Ok(program) = result {
        if let Stmt::ClassDeclaration(_, _, members, _) = &program.statements[0] {
            assert_eq!(members.len(), 2);

            for member in members {
                if let ClassMember::Method { is_static, .. } = member {
                    assert!(*is_static, "Expected static method");
                } else {
                    panic!("Expected method member");
                }
            }
        } else {
            panic!("Expected class declaration");
        }
    }
}

#[test]
fn test_class_with_properties() {
    let code = r#"
        class TestClass {
            public let publicProp = "public";
            private let privateProp = 42;
            static let staticProp = true;
        }
    "#;

    let result = parse_dryad_code(code);
    assert!(
        result.is_ok(),
        "Failed to parse class with properties: {:?}",
        result.err()
    );

    if let Ok(program) = result {
        if let Stmt::ClassDeclaration(_, _, members, _) = &program.statements[0] {
            assert_eq!(members.len(), 3);

            // Check first property
            if let ClassMember::Property(visibility, is_static, name, _, default_value) =
                &members[0]
            {
                assert!(matches!(visibility, Visibility::Public));
                assert!(!is_static);
                assert_eq!(name, "publicProp");
                assert!(default_value.is_some());
            } else {
                panic!("Expected property member");
            }

            // Check second property
            if let ClassMember::Property(visibility, is_static, name, _, _) = &members[1] {
                assert!(matches!(visibility, Visibility::Private));
                assert!(!is_static);
                assert_eq!(name, "privateProp");
            } else {
                panic!("Expected property member");
            }

            // Check third property (static)
            if let ClassMember::Property(_, is_static, name, _, _) = &members[2] {
                assert!(*is_static);
                assert_eq!(name, "staticProp");
            } else {
                panic!("Expected property member");
            }
        } else {
            panic!("Expected class declaration");
        }
    }
}

#[test]
fn test_empty_class() {
    let code = r#"
        class EmptyClass {
        }
    "#;

    let result = parse_dryad_code(code);
    assert!(
        result.is_ok(),
        "Failed to parse empty class: {:?}",
        result.err()
    );

    if let Ok(program) = result {
        if let Stmt::ClassDeclaration(name, parent, members, _) = &program.statements[0] {
            assert_eq!(name, "EmptyClass");
            assert!(parent.is_none());
            assert!(members.is_empty());
        } else {
            panic!("Expected class declaration");
        }
    }
}

#[test]
fn test_class_with_multiple_members() {
    let code = r#"
        class Example {
            public let value = 0;
            
            function init(val) {
                this.value = val;
            }
            
            public function getValue() {
                return this.value;
            }
            
            static function create() {
                return new Example(0);
            }
        }
    "#;

    let result = parse_dryad_code(code);
    assert!(
        result.is_ok(),
        "Failed to parse class with multiple members: {:?}",
        result.err()
    );

    if let Ok(program) = result {
        if let Stmt::ClassDeclaration(_, _, members, _) = &program.statements[0] {
            assert_eq!(members.len(), 4);

            // First: property
            assert!(matches!(members[0], ClassMember::Property(..)));

            // Second: init method
            if let ClassMember::Method { name, .. } = &members[1] {
                assert_eq!(name, "init");
            } else {
                panic!("Expected init method");
            }

            // Third: getValue method
            if let ClassMember::Method { name, .. } = &members[2] {
                assert_eq!(name, "getValue");
            } else {
                panic!("Expected getValue method");
            }

            // Fourth: static create method
            if let ClassMember::Method {
                name, is_static, ..
            } = &members[3]
            {
                assert_eq!(name, "create");
                assert!(*is_static);
            } else {
                panic!("Expected static create method");
            }
        } else {
            panic!("Expected class declaration");
        }
    }
}

#[test]
fn test_class_instantiation_parsing() {
    let code = r#"
        let pessoa = new Pessoa("João", 25);
    "#;

    let result = parse_dryad_code(code);
    assert!(
        result.is_ok(),
        "Failed to parse class instantiation: {:?}",
        result.err()
    );

    if let Ok(program) = result {
        assert_eq!(program.statements.len(), 1);

        if let Stmt::VarDeclaration(pattern, _, Some(expr), _) = &program.statements[0] {
            assert_eq!(pattern.identifier_name().unwrap(), "pessoa");

            if let Expr::ClassInstantiation(class_name, args, _) = expr {
                assert_eq!(class_name, "Pessoa");
                assert_eq!(args.len(), 2);
            } else {
                panic!("Expected class instantiation expression");
            }
        } else {
            panic!("Expected variable declaration");
        }
    }
}

#[test]
fn test_class_property_access() {
    let code = r#"
        let nome = pessoa.nome;
        pessoa.idade = 26;
    "#;

    let result = parse_dryad_code(code);
    assert!(
        result.is_ok(),
        "Failed to parse property access: {:?}",
        result.err()
    );

    if let Ok(program) = result {
        assert_eq!(program.statements.len(), 2);

        // First: property access in variable declaration
        if let Stmt::VarDeclaration(pattern, _, Some(expr), _) = &program.statements[0] {
            assert_eq!(pattern.identifier_name().unwrap(), "nome");

            if let Expr::PropertyAccess(obj, prop, _) = expr {
                if let Expr::Variable(var_name, _) = obj.as_ref() {
                    assert_eq!(var_name, "pessoa");
                } else {
                    panic!("Expected variable expression");
                }
                assert_eq!(prop, "nome");
            } else {
                panic!("Expected property access expression");
            }
        } else {
            panic!("Expected variable declaration");
        }

        // Second: property assignment
        if let Stmt::PropertyAssignment(obj, prop, _, _) = &program.statements[1] {
            if let Expr::Variable(var_name, _) = obj {
                assert_eq!(var_name, "pessoa");
            } else {
                panic!("Expected variable expression");
            }
            assert_eq!(prop, "idade");
        } else {
            panic!("Expected property assignment");
        }
    }
}

#[test]
fn test_class_method_call() {
    let code = r#"
        pessoa.sayHello();
    "#;

    let result = parse_dryad_code(code);
    assert!(
        result.is_ok(),
        "Failed to parse method call: {:?}",
        result.err()
    );

    if let Ok(program) = result {
        assert_eq!(program.statements.len(), 1);

        if let Stmt::Expression(expr, _) = &program.statements[0] {
            if let Expr::MethodCall(obj, method_name, args, _) = expr {
                if let Expr::Variable(var_name, _) = obj.as_ref() {
                    assert_eq!(var_name, "pessoa");
                } else {
                    panic!("Expected variable expression");
                }
                assert_eq!(method_name, "sayHello");
                assert!(args.is_empty());
            } else {
                panic!("Expected method call expression");
            }
        } else {
            panic!("Expected expression statement");
        }
    }
}

#[test]
fn test_class_with_async_methods() {
    let code = r#"
        class AsyncExample {
            async function fetchData() {
                return await getData();
            }
        }
    "#;

    let result = parse_dryad_code(code);
    assert!(
        result.is_ok(),
        "Failed to parse class with async methods: {:?}",
        result.err()
    );

    if let Ok(program) = result {
        if let Stmt::ClassDeclaration(_, _, members, _) = &program.statements[0] {
            assert_eq!(members.len(), 1);

            if let ClassMember::Method { is_async, name, .. } = &members[0] {
                assert!(*is_async, "Expected async method");
                assert_eq!(name, "fetchData");
            } else {
                panic!("Expected method member");
            }
        } else {
            panic!("Expected class declaration");
        }
    }
}

#[test]
fn test_super_keyword() {
    let code = r#"
        class Child extends Parent {
            function init() {
                super.init();
            }
        }
    "#;

    let result = parse_dryad_code(code);
    assert!(
        result.is_ok(),
        "Failed to parse super keyword: {:?}",
        result.err()
    );

    if let Ok(program) = result {
        if let Stmt::ClassDeclaration(_, _, members, _) = &program.statements[0] {
            if let ClassMember::Method { body, .. } = &members[0] {
                if let Stmt::Block(statements, _) = body.as_ref() {
                    if let Stmt::Expression(expr, _) = &statements[0] {
                        if let Expr::MethodCall(obj, method_name, args, _) = expr {
                            assert!(
                                matches!(obj.as_ref(), Expr::Super(_)),
                                "Expected super as object"
                            );
                            assert_eq!(method_name, "init");
                            assert!(args.is_empty(), "Expected no arguments");
                        } else {
                            panic!("Expected method call expression");
                        }
                    } else {
                        panic!("Expected expression statement");
                    }
                } else {
                    panic!("Expected block");
                }
            } else {
                panic!("Expected method");
            }
        } else {
            panic!("Expected class declaration");
        }
    }
}
