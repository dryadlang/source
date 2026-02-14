// crates/dryad_bytecode/tests/array_tests.rs
//! Testes para arrays e coleções no bytecode

use dryad_bytecode::{Compiler, InterpretResult, VM};
use dryad_errors::SourceLocation;
use dryad_parser::ast::{Expr, Literal, Program, Stmt};

fn dummy_loc() -> SourceLocation {
    SourceLocation {
        line: 1,
        column: 1,
        file: None,
    }
}

#[test]
fn test_array_creation() {
    // Programa: var arr = [1, 2, 3];
    let program = Program {
        statements: vec![
            Stmt::VarDeclaration(
                dryad_parser::ast::Pattern::Identifier("arr".to_string()),
                None,
                Some(Expr::Array(
                    vec![
                        Expr::Literal(Literal::Number(1.0), dummy_loc()),
                        Expr::Literal(Literal::Number(2.0), dummy_loc()),
                        Expr::Literal(Literal::Number(3.0), dummy_loc()),
                    ],
                    dummy_loc(),
                )),
                dummy_loc(),
            ),
        ],
    };

    let mut compiler = Compiler::new();
    let chunk = compiler.compile(program);
    assert!(chunk.is_ok(), "Erro na compilação: {:?}", chunk.err());

    let mut vm = VM::new();
    let result = vm.interpret(chunk.unwrap());
    assert_eq!(result, InterpretResult::Ok);
}

#[test]
fn test_array_indexing() {
    // Programa:
    // var arr = [10, 20, 30];
    // print arr[1];  # Deve imprimir 20
    
    let program = Program {
        statements: vec![
            // var arr = [10, 20, 30];
            Stmt::VarDeclaration(
                dryad_parser::ast::Pattern::Identifier("arr".to_string()),
                None,
                Some(Expr::Array(
                    vec![
                        Expr::Literal(Literal::Number(10.0), dummy_loc()),
                        Expr::Literal(Literal::Number(20.0), dummy_loc()),
                        Expr::Literal(Literal::Number(30.0), dummy_loc()),
                    ],
                    dummy_loc(),
                )),
                dummy_loc(),
            ),
            // print arr[1];
            Stmt::Print(
                Expr::Index(
                    Box::new(Expr::Variable("arr".to_string(), dummy_loc())),
                    Box::new(Expr::Literal(Literal::Number(1.0), dummy_loc())),
                    dummy_loc(),
                ),
                dummy_loc(),
            ),
        ],
    };

    let mut compiler = Compiler::new();
    let chunk = compiler.compile(program);
    assert!(chunk.is_ok(), "Erro na compilação: {:?}", chunk.err());

    let mut vm = VM::new();
    let result = vm.interpret(chunk.unwrap());
    assert_eq!(result, InterpretResult::Ok);
}

#[test]
fn test_array_mutation() {
    // Programa:
    // var arr = [1, 2, 3];
    // arr[0] = 100;
    // print arr[0];  # Deve imprimir 100
    
    let program = Program {
        statements: vec![
            // var arr = [1, 2, 3];
            Stmt::VarDeclaration(
                dryad_parser::ast::Pattern::Identifier("arr".to_string()),
                None,
                Some(Expr::Array(
                    vec![
                        Expr::Literal(Literal::Number(1.0), dummy_loc()),
                        Expr::Literal(Literal::Number(2.0), dummy_loc()),
                        Expr::Literal(Literal::Number(3.0), dummy_loc()),
                    ],
                    dummy_loc(),
                )),
                dummy_loc(),
            ),
            // arr[0] = 100;
            Stmt::IndexAssignment(
                Expr::Variable("arr".to_string(), dummy_loc()),
                Expr::Literal(Literal::Number(0.0), dummy_loc()),
                Expr::Literal(Literal::Number(100.0), dummy_loc()),
                dummy_loc(),
            ),
            // print arr[0];
            Stmt::Print(
                Expr::Index(
                    Box::new(Expr::Variable("arr".to_string(), dummy_loc())),
                    Box::new(Expr::Literal(Literal::Number(0.0), dummy_loc())),
                    dummy_loc(),
                ),
                dummy_loc(),
            ),
        ],
    };

    let mut compiler = Compiler::new();
    let chunk = compiler.compile(program);
    assert!(chunk.is_ok(), "Erro na compilação: {:?}", chunk.err());

    let mut vm = VM::new();
    let result = vm.interpret(chunk.unwrap());
    assert_eq!(result, InterpretResult::Ok);
}

#[test]
fn test_tuple_creation() {
    // Programa: var t = (1, 2, 3);
    let program = Program {
        statements: vec![
            Stmt::VarDeclaration(
                dryad_parser::ast::Pattern::Identifier("t".to_string()),
                None,
                Some(Expr::Tuple(
                    vec![
                        Expr::Literal(Literal::Number(1.0), dummy_loc()),
                        Expr::Literal(Literal::Number(2.0), dummy_loc()),
                        Expr::Literal(Literal::Number(3.0), dummy_loc()),
                    ],
                    dummy_loc(),
                )),
                dummy_loc(),
            ),
        ],
    };

    let mut compiler = Compiler::new();
    let chunk = compiler.compile(program);
    assert!(chunk.is_ok(), "Erro na compilação: {:?}", chunk.err());

    let mut vm = VM::new();
    let result = vm.interpret(chunk.unwrap());
    assert_eq!(result, InterpretResult::Ok);
}

#[test]
fn test_tuple_access() {
    // Programa:
    // var t = (10, 20, 30);
    // print t.1;  # Deve imprimir 20
    
    let program = Program {
        statements: vec![
            // var t = (10, 20, 30);
            Stmt::VarDeclaration(
                dryad_parser::ast::Pattern::Identifier("t".to_string()),
                None,
                Some(Expr::Tuple(
                    vec![
                        Expr::Literal(Literal::Number(10.0), dummy_loc()),
                        Expr::Literal(Literal::Number(20.0), dummy_loc()),
                        Expr::Literal(Literal::Number(30.0), dummy_loc()),
                    ],
                    dummy_loc(),
                )),
                dummy_loc(),
            ),
            // print t.1;
            Stmt::Print(
                Expr::TupleAccess(
                    Box::new(Expr::Variable("t".to_string(), dummy_loc())),
                    1,
                    dummy_loc(),
                ),
                dummy_loc(),
            ),
        ],
    };

    let mut compiler = Compiler::new();
    let chunk = compiler.compile(program);
    assert!(chunk.is_ok(), "Erro na compilação: {:?}", chunk.err());

    let mut vm = VM::new();
    let result = vm.interpret(chunk.unwrap());
    assert_eq!(result, InterpretResult::Ok);
}
