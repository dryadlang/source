// crates/dryad_parser/tests/class_parser_tests.rs

use dryad_parser::{Parser, ast::*};
use dryad_lexer::{Lexer, Token};

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
    assert!(result.is_ok(), "Failed to parse class declaration: {:?}", result.err());
    
    if let Ok(program) = result {
        assert_eq!(program.statements.len(), 1);
        
        if let Stmt::ClassDeclaration(name, parent, members, _) = &program.statements[0] {
            assert_eq!(name, "Pessoa");
            assert!(parent.is_none());
            assert_eq!(members.len(), 1);
            
            if let ClassMember::Method(visibility, is_static, method_name, params, _) = &members[0] {
                assert!(matches!(visibility, Visibility::Public));
                assert!(!is_static);
                assert_eq!(method_name, "init");
                assert_eq!(params.len(), 2);
                assert_eq!(params[0], "nome");
                assert_eq!(params[1], "idade");
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
    assert!(result.is_ok(), "Failed to parse class with inheritance: {:?}", result.err());
    
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
    assert!(result.is_ok(), "Failed to parse class with visibility modifiers: {:?}", result.err());
    
    if let Ok(program) = result {
        if let Stmt::ClassDeclaration(_, _, members, _) = &program.statements[0] {
            assert_eq!(members.len(), 3);
            
            // Check visibility modifiers
            for (i, expected_visibility) in [Visibility::Public, Visibility::Private, Visibility::Protected].iter().enumerate() {
                if let ClassMember::Method(visibility, _, _, _, _) = &members[i] {
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
    assert!(result.is_ok(), "Failed to parse class with static methods: {:?}", result.err());
    
    if let Ok(program) = result {
        if let Stmt::ClassDeclaration(_, _, members, _) = &program.statements[0] {
            assert_eq!(members.len(), 2);
            
            for member in members {
                if let ClassMember::Method(_, is_static, _, _, _) = member {
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
    assert!(result.is_ok(), "Failed to parse class with properties: {:?}", result.err());
    
    if let Ok(program) = result {
        if let Stmt::ClassDeclaration(_, _, members, _) = &program.statements[0] {
            assert_eq!(members.len(), 3);
            
            // Check first property
            if let ClassMember::Property(visibility, is_static, name, default_value) = &members[0] {
                assert!(matches!(visibility, Visibility::Public));
                assert!(!is_static);
                assert_eq!(name, "publicProp");
                assert!(default_value.is_some());
            } else {
                panic!("Expected property member");
            }
            
            // Check second property
            if let ClassMember::Property(visibility, is_static, name, _) = &members[1] {
                assert!(matches!(visibility, Visibility::Private));
                assert!(!is_static);
                assert_eq!(name, "privateProp");
            } else {
                panic!("Expected property member");
            }
            
            // Check third property (static)
            if let ClassMember::Property(_, is_static, name, _) = &members[2] {
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
fn test_this_and_super_expressions() {
    let code = r#"
        class Test {
            function testThis() {
                return this.value;
            }
            
            function testSuper() {
                return super.method();
            }
        }
    "#;
    
    let result = parse_dryad_code(code);
    assert!(result.is_ok(), "Failed to parse this and super expressions: {:?}", result.err());
}

#[test]
fn test_method_call_parsing() {
    let code = r#"
        let obj = SomeClass();
        obj.method(1, 2, 3);
        obj.property;
    "#;
    
    let result = parse_dryad_code(code);
    assert!(result.is_ok(), "Failed to parse method calls and property access: {:?}", result.err());
}
