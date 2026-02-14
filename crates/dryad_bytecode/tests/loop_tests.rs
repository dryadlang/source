// crates/dryad_bytecode/tests/loop_tests.rs
//! Testes para loops (for, foreach, break, continue)

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
fn test_foreach_array() {
    // Programa:
    // var arr = [1, 2, 3];
    // for x in arr {
    //     print x;
    // }
    
    let program = Program {
        statements: vec![
            // var arr = [1, 2, 3];
            Stmt::VarDeclaration(
                Pattern::Identifier("arr".to_string()),
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
            // for x in arr { print x; }
            Stmt::ForEach(
                Pattern::Identifier("x".to_string()),
                Expr::Variable("arr".to_string(), dummy_loc()),
                Box::new(Stmt::Block(
                    vec![
                        Stmt::Print(
                            Expr::Variable("x".to_string(), dummy_loc()),
                            dummy_loc(),
                        ),
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
fn test_break_in_while() {
    // Programa:
    // var i = 0;
    // while (i < 10) {
    //     if (i == 5) break;
    //     print i;
    //     i = i + 1;
    // }
    
    let program = Program {
        statements: vec![
            // var i = 0;
            Stmt::VarDeclaration(
                Pattern::Identifier("i".to_string()),
                None,
                Some(Expr::Literal(Literal::Number(0.0), dummy_loc())),
                dummy_loc(),
            ),
            // while (i < 10) { ... }
            Stmt::While(
                Expr::Binary(
                    Box::new(Expr::Variable("i".to_string(), dummy_loc())),
                    "<".to_string(),
                    Box::new(Expr::Literal(Literal::Number(10.0), dummy_loc())),
                    dummy_loc(),
                ),
                Box::new(Stmt::Block(
                    vec![
                        // if (i == 5) break;
                        Stmt::If(
                            Expr::Binary(
                                Box::new(Expr::Variable("i".to_string(), dummy_loc())),
                                "==".to_string(),
                                Box::new(Expr::Literal(Literal::Number(5.0), dummy_loc())),
                                dummy_loc(),
                            ),
                            Box::new(Stmt::Break(dummy_loc())),
                            dummy_loc(),
                        ),
                        // print i;
                        Stmt::Print(
                            Expr::Variable("i".to_string(), dummy_loc()),
                            dummy_loc(),
                        ),
                        // i = i + 1;
                        Stmt::Assignment(
                            Pattern::Identifier("i".to_string()),
                            Expr::Binary(
                                Box::new(Expr::Variable("i".to_string(), dummy_loc())),
                                "+".to_string(),
                                Box::new(Expr::Literal(Literal::Number(1.0), dummy_loc())),
                                dummy_loc(),
                            ),
                            dummy_loc(),
                        ),
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
fn test_continue_in_for() {
    // Programa:
    // for (var i = 0; i < 5; i = i + 1) {
    //     if (i == 2) continue;
    //     print i;
    // }
    // Deve imprimir: 0, 1, 3, 4
    
    let program = Program {
        statements: vec![
            Stmt::For(
                Some(Box::new(Stmt::VarDeclaration(
                    Pattern::Identifier("i".to_string()),
                    None,
                    Some(Expr::Literal(Literal::Number(0.0), dummy_loc())),
                    dummy_loc(),
                ))),
                Some(Expr::Binary(
                    Box::new(Expr::Variable("i".to_string(), dummy_loc())),
                    "<".to_string(),
                    Box::new(Expr::Literal(Literal::Number(5.0), dummy_loc())),
                    dummy_loc(),
                )),
                Some(Box::new(Stmt::Assignment(
                    Pattern::Identifier("i".to_string()),
                    Expr::Binary(
                        Box::new(Expr::Variable("i".to_string(), dummy_loc())),
                        "+".to_string(),
                        Box::new(Expr::Literal(Literal::Number(1.0), dummy_loc())),
                        dummy_loc(),
                    ),
                    dummy_loc(),
                ))),
                Box::new(Stmt::Block(
                    vec![
                        // if (i == 2) continue;
                        Stmt::If(
                            Expr::Binary(
                                Box::new(Expr::Variable("i".to_string(), dummy_loc())),
                                "==".to_string(),
                                Box::new(Expr::Literal(Literal::Number(2.0), dummy_loc())),
                                dummy_loc(),
                            ),
                            Box::new(Stmt::Continue(dummy_loc())),
                            dummy_loc(),
                        ),
                        // print i;
                        Stmt::Print(
                            Expr::Variable("i".to_string(), dummy_loc()),
                            dummy_loc(),
                        ),
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
