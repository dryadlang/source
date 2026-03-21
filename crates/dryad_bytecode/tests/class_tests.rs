// crates/dryad_bytecode/tests/class_tests.rs
//! Testes para classes e objetos no bytecode

use dryad_bytecode::{Compiler, InterpretResult, VM};
use dryad_errors::SourceLocation;
use dryad_parser::ast::{ClassMember, Expr, Literal, Program, Stmt, Type, Visibility};

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
fn test_class_declaration() {
    // Programa:
    // class Pessoa {
    //     fn saudar() {
    //         print "Ola!";
    //     }
    // }

    let program = Program {
        statements: vec![Stmt::ClassDeclaration(
            "Pessoa".to_string(),
            None,
            vec![],
            vec![ClassMember::Method {
                visibility: Visibility::Public,
                is_static: false,
                is_async: false,
                name: "saudar".to_string(),
                params: vec![],
                return_type: None,
                body: Box::new(Stmt::Block(
                    vec![Stmt::Expression(
                        Expr::Call(
                            Box::new(Expr::Variable("print".to_string(), dummy_loc())),
                            vec![Expr::Literal(
                                Literal::String("Ola!".to_string()),
                                dummy_loc(),
                            )],
                            dummy_loc(),
                        ),
                        dummy_loc(),
                    )],
                    dummy_loc(),
                )),
            }],
            dummy_loc(),
        )],
    };

    let mut compiler = Compiler::new();
    let chunk = compiler.compile(program);
    assert!(chunk.is_ok(), "Erro na compilação: {:?}", chunk.err());

    let mut vm = VM::new();
    let result = vm.interpret(chunk.unwrap());
    assert_eq!(result, InterpretResult::Ok);
}

#[test]
fn test_class_with_property() {
    // Programa:
    // class Ponto {
    //     x = 0;
    //     y = 0;
    // }

    let program = Program {
        statements: vec![Stmt::ClassDeclaration(
            "Ponto".to_string(),
            None,
            vec![],
            vec![
                ClassMember::Property(
                    Visibility::Public,
                    false,
                    "x".to_string(),
                    Some(Type::Number),
                    Some(Expr::Literal(Literal::Number(0.0), dummy_loc())),
                ),
                ClassMember::Property(
                    Visibility::Public,
                    false,
                    "y".to_string(),
                    Some(Type::Number),
                    Some(Expr::Literal(Literal::Number(0.0), dummy_loc())),
                ),
            ],
            dummy_loc(),
        )],
    };

    let mut compiler = Compiler::new();
    let chunk = compiler.compile(program);
    assert!(chunk.is_ok(), "Erro na compilação: {:?}", chunk.err());

    let mut vm = VM::new();
    let result = vm.interpret(chunk.unwrap());
    assert_eq!(result, InterpretResult::Ok);
}

#[test]
fn test_class_inheritance_with_super() {
    // Programa:
    // class Animal {
    //     fn falar() {
    //         print "Som genérico";
    //     }
    // }
    //
    // class Cachorro extends Animal {
    //     fn falar() {
    //         super.falar();
    //         print "Au au!";
    //     }
    // }

    let program = Program {
        statements: vec![
            // Classe base: Animal
            Stmt::ClassDeclaration(
                "Animal".to_string(),
                None,
                vec![],
                vec![ClassMember::Method {
                    visibility: Visibility::Public,
                    is_static: false,
                    is_async: false,
                    name: "falar".to_string(),
                    params: vec![],
                    return_type: None,
                    body: Box::new(Stmt::Block(
                        vec![Stmt::Expression(
                            Expr::Call(
                                Box::new(Expr::Variable("print".to_string(), dummy_loc())),
                                vec![Expr::Literal(
                                    Literal::String("Som genérico".to_string()),
                                    dummy_loc(),
                                )],
                                dummy_loc(),
                            ),
                            dummy_loc(),
                        )],
                        dummy_loc(),
                    )),
                }],
                dummy_loc(),
            ),
            // Classe derivada: Cachorro
            Stmt::ClassDeclaration(
                "Cachorro".to_string(),
                Some("Animal".to_string()),
                vec![],
                vec![ClassMember::Method {
                    visibility: Visibility::Public,
                    is_static: false,
                    is_async: false,
                    name: "falar".to_string(),
                    params: vec![],
                    return_type: None,
                    body: Box::new(Stmt::Block(
                        vec![
                            // Chamada a super.falar()
                            Stmt::Expression(
                                Expr::Call(
                                    Box::new(Expr::PropertyAccess(
                                        Box::new(Expr::Super(dummy_loc())),
                                        "falar".to_string(),
                                        dummy_loc(),
                                    )),
                                    vec![],
                                    dummy_loc(),
                                ),
                                dummy_loc(),
                            ),
                            // Imprime mensagem do cão
                            Stmt::Expression(
                                Expr::Call(
                                    Box::new(Expr::Variable("print".to_string(), dummy_loc())),
                                    vec![Expr::Literal(
                                        Literal::String("Au au!".to_string()),
                                        dummy_loc(),
                                    )],
                                    dummy_loc(),
                                ),
                                dummy_loc(),
                            ),
                        ],
                        dummy_loc(),
                    )),
                }],
                dummy_loc(),
            ),
        ],
    };

    let mut compiler = Compiler::new();
    let chunk = compiler.compile(program);
    assert!(chunk.is_ok(), "Erro na compilação: {:?}", chunk.err());

    let mut vm = VM::new();
    let result = vm.interpret(chunk.unwrap());
    // Should compile and run without errors
    assert_eq!(result, InterpretResult::Ok);
}
