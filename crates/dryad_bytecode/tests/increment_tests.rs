// crates/dryad_bytecode/tests/increment_tests.rs
//! Testes para incremento/decremento

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
fn test_post_increment() {
    // Programa:
    // var x = 5;
    // print x++;  # Deve imprimir 5
    // print x;     # Deve imprimir 6
    
    let program = Program {
        statements: vec![
            Stmt::VarDeclaration(
                Pattern::Identifier("x".to_string()),
                None,
                Some(Expr::Literal(Literal::Number(5.0), dummy_loc())),
                dummy_loc(),
            ),
            Stmt::Print(
                Expr::PostIncrement(
                    Box::new(Expr::Variable("x".to_string(), dummy_loc())),
                    dummy_loc(),
                ),
                dummy_loc(),
            ),
            Stmt::Print(
                Expr::Variable("x".to_string(), dummy_loc()),
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
fn test_pre_increment() {
    // Programa:
    // var x = 5;
    // print ++x;  # Deve imprimir 6
    // print x;     # Deve imprimir 6
    
    let program = Program {
        statements: vec![
            Stmt::VarDeclaration(
                Pattern::Identifier("x".to_string()),
                None,
                Some(Expr::Literal(Literal::Number(5.0), dummy_loc())),
                dummy_loc(),
            ),
            Stmt::Print(
                Expr::PreIncrement(
                    Box::new(Expr::Variable("x".to_string(), dummy_loc())),
                    dummy_loc(),
                ),
                dummy_loc(),
            ),
            Stmt::Print(
                Expr::Variable("x".to_string(), dummy_loc()),
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
fn test_post_decrement() {
    // Programa:
    // var x = 5;
    // print x--;  # Deve imprimir 5
    // print x;     # Deve imprimir 4
    
    let program = Program {
        statements: vec![
            Stmt::VarDeclaration(
                Pattern::Identifier("x".to_string()),
                None,
                Some(Expr::Literal(Literal::Number(5.0), dummy_loc())),
                dummy_loc(),
            ),
            Stmt::Print(
                Expr::PostDecrement(
                    Box::new(Expr::Variable("x".to_string(), dummy_loc())),
                    dummy_loc(),
                ),
                dummy_loc(),
            ),
            Stmt::Print(
                Expr::Variable("x".to_string(), dummy_loc()),
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
fn test_increment_in_loop() {
    // Programa:
    // var i = 0;
    // while (i < 3) {
    //     print i++;
    // }
    
    let program = Program {
        statements: vec![
            Stmt::VarDeclaration(
                Pattern::Identifier("i".to_string()),
                None,
                Some(Expr::Literal(Literal::Number(0.0), dummy_loc())),
                dummy_loc(),
            ),
            Stmt::While(
                Expr::Binary(
                    Box::new(Expr::Variable("i".to_string(), dummy_loc())),
                    "<".to_string(),
                    Box::new(Expr::Literal(Literal::Number(3.0), dummy_loc())),
                    dummy_loc(),
                ),
                Box::new(Stmt::Block(
                    vec![
                        Stmt::Print(
                            Expr::PostIncrement(
                                Box::new(Expr::Variable("i".to_string(), dummy_loc())),
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
