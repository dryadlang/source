// crates/dryad_bytecode/tests/exception_tests.rs
//! Testes para exceções (try/catch/throw)

use dryad_bytecode::{Compiler, InterpretResult, VM};
use dryad_errors::SourceLocation;
use dryad_parser::ast::{Expr, Literal, Pattern, Program, Stmt};

fn dummy_loc() -> SourceLocation {
    SourceLocation {
        line: 1,
        column: 1,
        file: None,
    }
}

#[test]
fn test_try_catch() {
    // Programa:
    // try {
    //     throw "Erro!";
    // } catch (e) {
    //     print "Capturado:";
    //     print e;
    // }
    
    let program = Program {
        statements: vec![
            Stmt::Try(
                Box::new(Stmt::Block(
                    vec![
                        Stmt::Throw(
                            Expr::Literal(Literal::String("Erro!".to_string()), dummy_loc()),
                            dummy_loc(),
                        ),
                    ],
                    dummy_loc(),
                )),
                Some(("e".to_string(), Box::new(Stmt::Block(
                    vec![
                        Stmt::Print(
                            Expr::Literal(Literal::String("Capturado:".to_string()), dummy_loc()),
                            dummy_loc(),
                        ),
                        Stmt::Print(
                            Expr::Variable("e".to_string(), dummy_loc()),
                            dummy_loc(),
                        ),
                    ],
                    dummy_loc(),
                )))),
                None,
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
fn test_try_catch_finally() {
    // Programa:
    // try {
    //     throw "Erro!";
    // } catch (e) {
    //     print "Capturado";
    // } finally {
    //     print "Sempre executa";
    // }
    
    let program = Program {
        statements: vec![
            Stmt::Try(
                Box::new(Stmt::Block(
                    vec![
                        Stmt::Throw(
                            Expr::Literal(Literal::String("Erro!".to_string()), dummy_loc()),
                            dummy_loc(),
                        ),
                    ],
                    dummy_loc(),
                )),
                Some(("e".to_string(), Box::new(Stmt::Block(
                    vec![
                        Stmt::Print(
                            Expr::Literal(Literal::String("Capturado".to_string()), dummy_loc()),
                            dummy_loc(),
                        ),
                    ],
                    dummy_loc(),
                )))),
                Some(Box::new(Stmt::Block(
                    vec![
                        Stmt::Print(
                            Expr::Literal(Literal::String("Sempre executa".to_string()), dummy_loc()),
                            dummy_loc(),
                        ),
                    ],
                    dummy_loc(),
                ))),
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
