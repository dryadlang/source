// crates/dryad_bytecode/tests/function_tests.rs
//! Testes para funções no bytecode

use dryad_bytecode::{Compiler, InterpretResult, VM};
use dryad_errors::SourceLocation;
use dryad_parser::ast::{Expr, Literal, Program, Stmt, Type};

fn dummy_loc() -> SourceLocation {
    SourceLocation {
        line: 1,
        column: 1,
        file: None,
    }
}

#[test]
fn test_simple_function_declaration() {
    let program = Program {
        statements: vec![
            Stmt::FunctionDeclaration {
                name: "add".to_string(),
                params: vec![
                    ("a".to_string(), Some(Type::Number)),
                    ("b".to_string(), Some(Type::Number)),
                ],
                return_type: Some(Type::Number),
                body: Box::new(Stmt::Block(
                    vec![
                        Stmt::Return(
                            Some(Expr::Binary(
                                Box::new(Expr::Variable("a".to_string(), dummy_loc())),
                                "+".to_string(),
                                Box::new(Expr::Variable("b".to_string(), dummy_loc())),
                                dummy_loc(),
                            )),
                            dummy_loc(),
                        ),
                    ],
                    dummy_loc(),
                )),
                location: dummy_loc(),
                is_async: false,
            },
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
fn test_function_call() {
    // Programa:
    // fn add(a, b) { return a + b; }
    // print add(1, 2);
    
    let program = Program {
        statements: vec![
            // Declaração da função
            Stmt::FunctionDeclaration {
                name: "add".to_string(),
                params: vec![
                    ("a".to_string(), Some(Type::Number)),
                    ("b".to_string(), Some(Type::Number)),
                ],
                return_type: Some(Type::Number),
                body: Box::new(Stmt::Block(
                    vec![
                        Stmt::Return(
                            Some(Expr::Binary(
                                Box::new(Expr::Variable("a".to_string(), dummy_loc())),
                                "+".to_string(),
                                Box::new(Expr::Variable("b".to_string(), dummy_loc())),
                                dummy_loc(),
                            )),
                            dummy_loc(),
                        ),
                    ],
                    dummy_loc(),
                )),
                location: dummy_loc(),
                is_async: false,
            },
            // Chamada da função
            Stmt::Print(
                Expr::Call(
                    Box::new(Expr::Variable("add".to_string(), dummy_loc())),
                    vec![
                        Expr::Literal(Literal::Number(1.0), dummy_loc()),
                        Expr::Literal(Literal::Number(2.0), dummy_loc()),
                    ],
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
fn test_function_with_local_variables() {
    // Programa:
    // fn multiply(x, y) {
    //     var result = x * y;
    //     return result;
    // }
    // print multiply(3, 4);
    
    let program = Program {
        statements: vec![
            Stmt::FunctionDeclaration {
                name: "multiply".to_string(),
                params: vec![
                    ("x".to_string(), Some(Type::Number)),
                    ("y".to_string(), Some(Type::Number)),
                ],
                return_type: Some(Type::Number),
                body: Box::new(Stmt::Block(
                    vec![
                        // var result = x * y;
                        Stmt::VarDeclaration(
                            dryad_parser::ast::Pattern::Identifier("result".to_string()),
                            Some(Type::Number),
                            Some(Expr::Binary(
                                Box::new(Expr::Variable("x".to_string(), dummy_loc())),
                                "*".to_string(),
                                Box::new(Expr::Variable("y".to_string(), dummy_loc())),
                                dummy_loc(),
                            )),
                            dummy_loc(),
                        ),
                        // return result;
                        Stmt::Return(
                            Some(Expr::Variable("result".to_string(), dummy_loc())),
                            dummy_loc(),
                        ),
                    ],
                    dummy_loc(),
                )),
                location: dummy_loc(),
                is_async: false,
            },
            // print multiply(3, 4);
            Stmt::Print(
                Expr::Call(
                    Box::new(Expr::Variable("multiply".to_string(), dummy_loc())),
                    vec![
                        Expr::Literal(Literal::Number(3.0), dummy_loc()),
                        Expr::Literal(Literal::Number(4.0), dummy_loc()),
                    ],
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
