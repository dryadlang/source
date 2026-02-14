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
        statements: vec![
            Stmt::ClassDeclaration(
                "Pessoa".to_string(),
                None,
                vec![],
                vec![
                    ClassMember::Method {
                        visibility: Visibility::Public,
                        is_static: false,
                        is_async: false,
                        name: "saudar".to_string(),
                        params: vec![],
                        return_type: None,
                        body: Box::new(Stmt::Block(
                            vec![
                                Stmt::Print(
                                    Expr::Literal(Literal::String("Ola!".to_string()), dummy_loc()),
                                    dummy_loc(),
                                ),
                            ],
                            dummy_loc(),
                        )),
                    },
                ],
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
fn test_class_with_property() {
    // Programa:
    // class Ponto {
    //     x = 0;
    //     y = 0;
    // }
    
    let program = Program {
        statements: vec![
            Stmt::ClassDeclaration(
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
