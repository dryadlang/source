// crates/dryad_bytecode/tests/class_tests.rs
//! Testes para classes com getters/setters no bytecode

use dryad_bytecode::Compiler;
use dryad_lexer::Lexer;
use dryad_parser::Parser;

#[test]
fn test_class_getter_compiles() {
    let source = r#"
        class Circle {
            get radius() {
                return 5;
            }
        }
    "#;

    let mut lexer = Lexer::new(source);
    let mut parser = Parser::new_from_lexer(&mut lexer).expect("Parser creation failed");
    let program = parser.parse().expect("Parse failed");

    let mut compiler = Compiler::new();
    let chunk = compiler.compile(program);
    assert!(chunk.is_ok(), "Compilation failed: {:?}", chunk.err());
}

#[test]
fn test_class_setter_compiles() {
    let source = r#"
        class Circle {
            set radius(value) {
                // Body
            }
        }
    "#;

    let mut lexer = Lexer::new(source);
    let mut parser = Parser::new_from_lexer(&mut lexer).expect("Parser creation failed");
    let program = parser.parse().expect("Parse failed");

    let mut compiler = Compiler::new();
    let chunk = compiler.compile(program);
    assert!(chunk.is_ok(), "Compilation failed: {:?}", chunk.err());
}

#[test]
fn test_class_getter_and_setter_compile() {
    let source = r#"
        class Circle {
            get radius() {
                return 5;
            }
            
            set radius(value) {
                // Body
            }
        }
    "#;

    let mut lexer = Lexer::new(source);
    let mut parser = Parser::new_from_lexer(&mut lexer).expect("Parser creation failed");
    let program = parser.parse().expect("Parse failed");

    let mut compiler = Compiler::new();
    let chunk = compiler.compile(program);
    assert!(chunk.is_ok(), "Compilation failed: {:?}", chunk.err());
}

#[test]
fn test_static_getter_compiles() {
    let source = r#"
        class Config {
            static get version() {
                return "1.0.0";
            }
        }
    "#;

    let mut lexer = Lexer::new(source);
    let mut parser = Parser::new_from_lexer(&mut lexer).expect("Parser creation failed");
    let program = parser.parse().expect("Parse failed");

    let mut compiler = Compiler::new();
    let chunk = compiler.compile(program);
    assert!(chunk.is_ok(), "Compilation failed: {:?}", chunk.err());
}
