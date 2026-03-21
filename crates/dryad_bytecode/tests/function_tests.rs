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
        position: 0,
        source_line: None,
    }
}

#[test]
fn test_simple_function_declaration() {
    let program = Program {
        statements: vec![Stmt::FunctionDeclaration {
            name: "add".to_string(),
            params: vec![
                ("a".to_string(), Some(Type::Number), None),
                ("b".to_string(), Some(Type::Number), None),
            ],
            return_type: Some(Type::Number),
            body: Box::new(Stmt::Block(
                vec![Stmt::Return(
                    Some(Expr::Binary(
                        Box::new(Expr::Variable("a".to_string(), dummy_loc())),
                        "+".to_string(),
                        Box::new(Expr::Variable("b".to_string(), dummy_loc())),
                        dummy_loc(),
                    )),
                    dummy_loc(),
                )],
                dummy_loc(),
            )),
            location: dummy_loc(),
            is_async: false,
            rest_param: None,
        }],
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
    // var x = add(1, 2);

    let program = Program {
        statements: vec![
            // Declaração da função
            Stmt::FunctionDeclaration {
                name: "add".to_string(),
                params: vec![
                    ("a".to_string(), Some(Type::Number), None),
                    ("b".to_string(), Some(Type::Number), None),
                ],
                return_type: Some(Type::Number),
                body: Box::new(Stmt::Block(
                    vec![Stmt::Return(
                        Some(Expr::Binary(
                            Box::new(Expr::Variable("a".to_string(), dummy_loc())),
                            "+".to_string(),
                            Box::new(Expr::Variable("b".to_string(), dummy_loc())),
                            dummy_loc(),
                        )),
                        dummy_loc(),
                    )],
                    dummy_loc(),
                )),
                location: dummy_loc(),
                is_async: false,
                rest_param: None,
            },
            // var x = add(1, 2);
            Stmt::VarDeclaration(
                dryad_parser::ast::Pattern::Identifier("x".to_string()),
                None,
                Some(Expr::Call(
                    Box::new(Expr::Variable("add".to_string(), dummy_loc())),
                    vec![
                        Expr::Literal(Literal::Number(1.0), dummy_loc()),
                        Expr::Literal(Literal::Number(2.0), dummy_loc()),
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
fn test_function_with_local_variables() {
    // Programa:
    // fn multiply(x, y) {
    //     var result = x * y;
    //     return result;
    // }
    // var z = multiply(3, 4);

    let program = Program {
        statements: vec![
            Stmt::FunctionDeclaration {
                name: "multiply".to_string(),
                params: vec![
                    ("x".to_string(), Some(Type::Number), None),
                    ("y".to_string(), Some(Type::Number), None),
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
                rest_param: None,
            },
            // var z = multiply(3, 4);
            Stmt::VarDeclaration(
                dryad_parser::ast::Pattern::Identifier("z".to_string()),
                None,
                Some(Expr::Call(
                    Box::new(Expr::Variable("multiply".to_string(), dummy_loc())),
                    vec![
                        Expr::Literal(Literal::Number(3.0), dummy_loc()),
                        Expr::Literal(Literal::Number(4.0), dummy_loc()),
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
fn test_recursive_function_sum_to() {
    // Programa:
    // fn sum_to(n) {
    //     if (n <= 1) {
    //         return n;
    //     }
    //     return n + sum_to(n - 1);
    // }
    // var x = sum_to(3);

    let program = Program {
        statements: vec![
            Stmt::FunctionDeclaration {
                name: "sum_to".to_string(),
                params: vec![("n".to_string(), Some(Type::Number), None)],
                return_type: Some(Type::Number),
                body: Box::new(Stmt::Block(
                    vec![
                        Stmt::If(
                            Expr::Binary(
                                Box::new(Expr::Variable("n".to_string(), dummy_loc())),
                                "<=".to_string(),
                                Box::new(Expr::Literal(Literal::Number(1.0), dummy_loc())),
                                dummy_loc(),
                            ),
                            Box::new(Stmt::Block(
                                vec![Stmt::Return(
                                    Some(Expr::Variable("n".to_string(), dummy_loc())),
                                    dummy_loc(),
                                )],
                                dummy_loc(),
                            )),
                            dummy_loc(),
                        ),
                        Stmt::Return(
                            Some(Expr::Binary(
                                Box::new(Expr::Variable("n".to_string(), dummy_loc())),
                                "+".to_string(),
                                Box::new(Expr::Call(
                                    Box::new(Expr::Variable("sum_to".to_string(), dummy_loc())),
                                    vec![Expr::Binary(
                                        Box::new(Expr::Variable("n".to_string(), dummy_loc())),
                                        "-".to_string(),
                                        Box::new(Expr::Literal(Literal::Number(1.0), dummy_loc())),
                                        dummy_loc(),
                                    )],
                                    dummy_loc(),
                                )),
                                dummy_loc(),
                            )),
                            dummy_loc(),
                        ),
                    ],
                    dummy_loc(),
                )),
                location: dummy_loc(),
                is_async: false,
                rest_param: None,
            },
            Stmt::VarDeclaration(
                dryad_parser::ast::Pattern::Identifier("x".to_string()),
                None,
                Some(Expr::Call(
                    Box::new(Expr::Variable("sum_to".to_string(), dummy_loc())),
                    vec![Expr::Literal(Literal::Number(3.0), dummy_loc())],
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
fn test_nested_function_calls() {
    // Programa:
    // fn multiply(x, y) { return x * y; }
    // fn add(a, b) { return a + b; }
    // var result = add(multiply(2, 3), multiply(4, 5));
    // Expected: add(6, 20) = 26

    let program = Program {
        statements: vec![
            Stmt::FunctionDeclaration {
                name: "multiply".to_string(),
                params: vec![
                    ("x".to_string(), Some(Type::Number), None),
                    ("y".to_string(), Some(Type::Number), None),
                ],
                return_type: Some(Type::Number),
                body: Box::new(Stmt::Block(
                    vec![Stmt::Return(
                        Some(Expr::Binary(
                            Box::new(Expr::Variable("x".to_string(), dummy_loc())),
                            "*".to_string(),
                            Box::new(Expr::Variable("y".to_string(), dummy_loc())),
                            dummy_loc(),
                        )),
                        dummy_loc(),
                    )],
                    dummy_loc(),
                )),
                location: dummy_loc(),
                is_async: false,
                rest_param: None,
            },
            Stmt::FunctionDeclaration {
                name: "add".to_string(),
                params: vec![
                    ("a".to_string(), Some(Type::Number), None),
                    ("b".to_string(), Some(Type::Number), None),
                ],
                return_type: Some(Type::Number),
                body: Box::new(Stmt::Block(
                    vec![Stmt::Return(
                        Some(Expr::Binary(
                            Box::new(Expr::Variable("a".to_string(), dummy_loc())),
                            "+".to_string(),
                            Box::new(Expr::Variable("b".to_string(), dummy_loc())),
                            dummy_loc(),
                        )),
                        dummy_loc(),
                    )],
                    dummy_loc(),
                )),
                location: dummy_loc(),
                is_async: false,
                rest_param: None,
            },
            Stmt::VarDeclaration(
                dryad_parser::ast::Pattern::Identifier("result".to_string()),
                None,
                Some(Expr::Call(
                    Box::new(Expr::Variable("add".to_string(), dummy_loc())),
                    vec![
                        Expr::Call(
                            Box::new(Expr::Variable("multiply".to_string(), dummy_loc())),
                            vec![
                                Expr::Literal(Literal::Number(2.0), dummy_loc()),
                                Expr::Literal(Literal::Number(3.0), dummy_loc()),
                            ],
                            dummy_loc(),
                        ),
                        Expr::Call(
                            Box::new(Expr::Variable("multiply".to_string(), dummy_loc())),
                            vec![
                                Expr::Literal(Literal::Number(4.0), dummy_loc()),
                                Expr::Literal(Literal::Number(5.0), dummy_loc()),
                            ],
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
