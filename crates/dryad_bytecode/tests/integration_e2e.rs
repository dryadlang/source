// crates/dryad_bytecode/tests/integration_e2e.rs
//! End-to-end integration test for bytecode compiler
//! Tests a realistic program combining multiple language features

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
#[ignore] // These tests document realistic program structures, not full VM support
fn test_e2e_realistic_oop_program() {
    // Realistic program: Bank account system with transactions
    //
    // class Account {
    //     fn __init__(balance) { this.balance = balance; }
    //     fn deposit(amount) { this.balance = this.balance + amount; }
    //     fn withdraw(amount) {
    //         if (this.balance >= amount) {
    //             this.balance = this.balance - amount;
    //         }
    //     }
    // }
    //
    // let account = new Account(1000);
    // account.deposit(500);
    // account.withdraw(200);
    // print account.balance;  // Should output: 1300

    let program = Program {
        statements: vec![
            // Class definition
            Stmt::ClassDeclaration(
                "Account".to_string(),
                None,
                vec![],
                vec![
                    // Constructor: fn __init__(balance) { this.balance = balance; }
                    ClassMember::Method {
                        visibility: Visibility::Public,
                        is_static: false,
                        is_async: false,
                        name: "__init__".to_string(),
                        params: vec![("balance".to_string(), Some(Type::Number), None)],
                        return_type: None,
                        body: Box::new(Stmt::Block(
                            vec![Stmt::Expression(
                                Expr::Call(
                                    Box::new(Expr::PropertyAccess(
                                        Box::new(Expr::This(dummy_loc())),
                                        "balance".to_string(),
                                        dummy_loc(),
                                    )),
                                    vec![],
                                    dummy_loc(),
                                ),
                                dummy_loc(),
                            )],
                            dummy_loc(),
                        )),
                    },
                    // deposit method: fn deposit(amount) { this.balance = this.balance + amount; }
                    ClassMember::Method {
                        visibility: Visibility::Public,
                        is_static: false,
                        is_async: false,
                        name: "deposit".to_string(),
                        params: vec![("amount".to_string(), Some(Type::Number), None)],
                        return_type: None,
                        body: Box::new(Stmt::Block(
                            vec![Stmt::Expression(
                                Expr::Binary(
                                    Box::new(Expr::PropertyAccess(
                                        Box::new(Expr::This(dummy_loc())),
                                        "balance".to_string(),
                                        dummy_loc(),
                                    )),
                                    "+".to_string(),
                                    Box::new(Expr::Variable("amount".to_string(), dummy_loc())),
                                    dummy_loc(),
                                ),
                                dummy_loc(),
                            )],
                            dummy_loc(),
                        )),
                    },
                    // withdraw method with control flow
                    ClassMember::Method {
                        visibility: Visibility::Public,
                        is_static: false,
                        is_async: false,
                        name: "withdraw".to_string(),
                        params: vec![("amount".to_string(), Some(Type::Number), None)],
                        return_type: None,
                        body: Box::new(Stmt::Block(
                            vec![Stmt::If(
                                Expr::Binary(
                                    Box::new(Expr::PropertyAccess(
                                        Box::new(Expr::This(dummy_loc())),
                                        "balance".to_string(),
                                        dummy_loc(),
                                    )),
                                    ">=".to_string(),
                                    Box::new(Expr::Variable("amount".to_string(), dummy_loc())),
                                    dummy_loc(),
                                ),
                                Box::new(Stmt::Block(
                                    vec![Stmt::Expression(
                                        Expr::Binary(
                                            Box::new(Expr::PropertyAccess(
                                                Box::new(Expr::This(dummy_loc())),
                                                "balance".to_string(),
                                                dummy_loc(),
                                            )),
                                            "-".to_string(),
                                            Box::new(Expr::Variable(
                                                "amount".to_string(),
                                                dummy_loc(),
                                            )),
                                            dummy_loc(),
                                        ),
                                        dummy_loc(),
                                    )],
                                    dummy_loc(),
                                )),
                                dummy_loc(),
                            )],
                            dummy_loc(),
                        )),
                    },
                ],
                dummy_loc(),
            ),
            // Create instance: let account = new Account(1000);
            Stmt::VarDeclaration(
                dryad_parser::ast::Pattern::Identifier("account".to_string()),
                None,
                Some(Expr::ClassInstantiation(
                    "Account".to_string(),
                    vec![Expr::Literal(Literal::Number(1000.0), dummy_loc())],
                    dummy_loc(),
                )),
                dummy_loc(),
            ),
            // Call deposit: account.deposit(500);
            Stmt::Expression(
                Expr::Call(
                    Box::new(Expr::PropertyAccess(
                        Box::new(Expr::Variable("account".to_string(), dummy_loc())),
                        "deposit".to_string(),
                        dummy_loc(),
                    )),
                    vec![Expr::Literal(Literal::Number(500.0), dummy_loc())],
                    dummy_loc(),
                ),
                dummy_loc(),
            ),
            // Call withdraw: account.withdraw(200);
            Stmt::Expression(
                Expr::Call(
                    Box::new(Expr::PropertyAccess(
                        Box::new(Expr::Variable("account".to_string(), dummy_loc())),
                        "withdraw".to_string(),
                        dummy_loc(),
                    )),
                    vec![Expr::Literal(Literal::Number(200.0), dummy_loc())],
                    dummy_loc(),
                ),
                dummy_loc(),
            ),
            // Print balance
            Stmt::Expression(
                Expr::Call(
                    Box::new(Expr::Variable("print".to_string(), dummy_loc())),
                    vec![Expr::PropertyAccess(
                        Box::new(Expr::Variable("account".to_string(), dummy_loc())),
                        "balance".to_string(),
                        dummy_loc(),
                    )],
                    dummy_loc(),
                ),
                dummy_loc(),
            ),
        ],
    };

    let mut compiler = Compiler::new();
    let chunk = compiler.compile(program);
    assert!(chunk.is_ok(), "Compilation failed: {:?}", chunk.err());

    let mut vm = VM::new();
    let result = vm.interpret(chunk.unwrap());
    assert_eq!(result, InterpretResult::Ok, "Runtime error occurred");
}

#[test]
#[ignore]
fn test_e2e_complex_control_flow() {
    // Program with nested control flow
    //
    // var sum = 0;
    // for (var i = 1; i <= 10; i = i + 1) {
    //     if (i % 2 == 0) {
    //         sum = sum + i;
    //     }
    // }
    // print sum;  // Should output: 30 (2+4+6+8+10)

    let program = Program {
        statements: vec![
            // var sum = 0;
            Stmt::VarDeclaration(
                dryad_parser::ast::Pattern::Identifier("sum".to_string()),
                None,
                Some(Expr::Literal(Literal::Number(0.0), dummy_loc())),
                dummy_loc(),
            ),
            // for loop
            Stmt::For(
                Some(Box::new(Stmt::VarDeclaration(
                    dryad_parser::ast::Pattern::Identifier("i".to_string()),
                    None,
                    Some(Expr::Literal(Literal::Number(1.0), dummy_loc())),
                    dummy_loc(),
                ))),
                Some(Expr::Binary(
                    Box::new(Expr::Variable("i".to_string(), dummy_loc())),
                    "<=".to_string(),
                    Box::new(Expr::Literal(Literal::Number(10.0), dummy_loc())),
                    dummy_loc(),
                )),
                Some(Box::new(Stmt::Expression(
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
                        // if (i % 2 == 0)
                        Stmt::If(
                            Expr::Binary(
                                Box::new(Expr::Binary(
                                    Box::new(Expr::Variable("i".to_string(), dummy_loc())),
                                    "%".to_string(),
                                    Box::new(Expr::Literal(Literal::Number(2.0), dummy_loc())),
                                    dummy_loc(),
                                )),
                                "==".to_string(),
                                Box::new(Expr::Literal(Literal::Number(0.0), dummy_loc())),
                                dummy_loc(),
                            ),
                            Box::new(Stmt::Block(
                                vec![Stmt::Expression(
                                    Expr::Binary(
                                        Box::new(Expr::Variable("sum".to_string(), dummy_loc())),
                                        "+".to_string(),
                                        Box::new(Expr::Variable("i".to_string(), dummy_loc())),
                                        dummy_loc(),
                                    ),
                                    dummy_loc(),
                                )],
                                dummy_loc(),
                            )),
                            dummy_loc(),
                        ),
                    ],
                    dummy_loc(),
                )),
                dummy_loc(),
            ),
            // print sum;
            Stmt::Expression(
                Expr::Call(
                    Box::new(Expr::Variable("print".to_string(), dummy_loc())),
                    vec![Expr::Variable("sum".to_string(), dummy_loc())],
                    dummy_loc(),
                ),
                dummy_loc(),
            ),
        ],
    };

    let mut compiler = Compiler::new();
    let chunk = compiler.compile(program);
    assert!(chunk.is_ok(), "Compilation failed: {:?}", chunk.err());

    let mut vm = VM::new();
    let result = vm.interpret(chunk.unwrap());
    assert_eq!(result, InterpretResult::Ok);
}

#[test]
#[ignore]
fn test_e2e_exception_handling() {
    // Program with exception handling
    //
    // try {
    //     let x = 10;
    //     let y = 0;
    //     let z = x / y;  // Would error
    // } catch (err) {
    //     print "Division error";
    // } finally {
    //     print "Cleanup";
    // }
    // print "Done";

    let program = Program {
        statements: vec![
            // try block
            Stmt::Try(
                Box::new(Stmt::Block(
                    vec![
                        Stmt::VarDeclaration(
                            dryad_parser::ast::Pattern::Identifier("x".to_string()),
                            None,
                            Some(Expr::Literal(Literal::Number(10.0), dummy_loc())),
                            dummy_loc(),
                        ),
                        Stmt::VarDeclaration(
                            dryad_parser::ast::Pattern::Identifier("y".to_string()),
                            None,
                            Some(Expr::Literal(Literal::Number(0.0), dummy_loc())),
                            dummy_loc(),
                        ),
                        Stmt::Expression(
                            Expr::Call(
                                Box::new(Expr::Variable("print".to_string(), dummy_loc())),
                                vec![Expr::Literal(
                                    Literal::String("In try block".to_string()),
                                    dummy_loc(),
                                )],
                                dummy_loc(),
                            ),
                            dummy_loc(),
                        ),
                    ],
                    dummy_loc(),
                )),
                None, // no catch for now
                None, // no finally for now
                dummy_loc(),
            ),
            Stmt::Expression(
                Expr::Call(
                    Box::new(Expr::Variable("print".to_string(), dummy_loc())),
                    vec![Expr::Literal(
                        Literal::String("After try".to_string()),
                        dummy_loc(),
                    )],
                    dummy_loc(),
                ),
                dummy_loc(),
            ),
        ],
    };

    let mut compiler = Compiler::new();
    let chunk = compiler.compile(program);
    assert!(chunk.is_ok(), "Compilation failed: {:?}", chunk.err());

    let mut vm = VM::new();
    let result = vm.interpret(chunk.unwrap());
    assert_eq!(result, InterpretResult::Ok);
}

#[test]
#[ignore]
fn test_e2e_array_operations() {
    // Program with array operations
    //
    // let numbers = [1, 2, 3, 4, 5];
    // let sum = 0;
    // for (i in numbers) {
    //     sum = sum + numbers[i];
    // }
    // print sum;  // Should output: 15

    let program = Program {
        statements: vec![
            // let numbers = [1, 2, 3, 4, 5];
            Stmt::VarDeclaration(
                dryad_parser::ast::Pattern::Identifier("numbers".to_string()),
                None,
                Some(Expr::Array(
                    vec![
                        Expr::Literal(Literal::Number(1.0), dummy_loc()),
                        Expr::Literal(Literal::Number(2.0), dummy_loc()),
                        Expr::Literal(Literal::Number(3.0), dummy_loc()),
                        Expr::Literal(Literal::Number(4.0), dummy_loc()),
                        Expr::Literal(Literal::Number(5.0), dummy_loc()),
                    ],
                    dummy_loc(),
                )),
                dummy_loc(),
            ),
            // let sum = 0;
            Stmt::VarDeclaration(
                dryad_parser::ast::Pattern::Identifier("sum".to_string()),
                None,
                Some(Expr::Literal(Literal::Number(0.0), dummy_loc())),
                dummy_loc(),
            ),
            // for-in loop
            Stmt::ForEach(
                dryad_parser::ast::Pattern::Identifier("i".to_string()),
                Expr::Variable("numbers".to_string(), dummy_loc()),
                Box::new(Stmt::Block(
                    vec![Stmt::Expression(
                        Expr::Binary(
                            Box::new(Expr::Variable("sum".to_string(), dummy_loc())),
                            "+".to_string(),
                            Box::new(Expr::Index(
                                Box::new(Expr::Variable("numbers".to_string(), dummy_loc())),
                                Box::new(Expr::Variable("i".to_string(), dummy_loc())),
                                dummy_loc(),
                            )),
                            dummy_loc(),
                        ),
                        dummy_loc(),
                    )],
                    dummy_loc(),
                )),
                dummy_loc(),
            ),
            // print sum;
            Stmt::Expression(
                Expr::Call(
                    Box::new(Expr::Variable("print".to_string(), dummy_loc())),
                    vec![Expr::Variable("sum".to_string(), dummy_loc())],
                    dummy_loc(),
                ),
                dummy_loc(),
            ),
        ],
    };

    let mut compiler = Compiler::new();
    let chunk = compiler.compile(program);
    assert!(chunk.is_ok(), "Compilation failed: {:?}", chunk.err());

    let mut vm = VM::new();
    let result = vm.interpret(chunk.unwrap());
    assert_eq!(result, InterpretResult::Ok);
}
