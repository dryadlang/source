use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use dryad_bytecode::{Compiler, VM};
use dryad_errors::SourceLocation;
use dryad_parser::ast::{Expr, Literal, Program, Stmt};

fn dummy_loc() -> SourceLocation {
    SourceLocation {
        line: 1,
        column: 1,
        file: None,
        position: 0,
        source_line: None,
    }
}

fn simple_arithmetic_program() -> Program {
    Program {
        statements: vec![Stmt::Expression(
            Expr::Binary(
                Box::new(Expr::Literal(Literal::Number(1.0), dummy_loc())),
                "+".to_string(),
                Box::new(Expr::Literal(Literal::Number(2.0), dummy_loc())),
                dummy_loc(),
            ),
            dummy_loc(),
        )],
    }
}

fn nested_arithmetic_program() -> Program {
    Program {
        statements: vec![Stmt::Expression(
            Expr::Binary(
                Box::new(Expr::Binary(
                    Box::new(Expr::Literal(Literal::Number(1.0), dummy_loc())),
                    "+".to_string(),
                    Box::new(Expr::Literal(Literal::Number(2.0), dummy_loc())),
                    dummy_loc(),
                )),
                "*".to_string(),
                Box::new(Expr::Binary(
                    Box::new(Expr::Literal(Literal::Number(3.0), dummy_loc())),
                    "-".to_string(),
                    Box::new(Expr::Literal(Literal::Number(4.0), dummy_loc())),
                    dummy_loc(),
                )),
                dummy_loc(),
            ),
            dummy_loc(),
        )],
    }
}

fn function_declaration_program() -> Program {
    Program {
        statements: vec![
            Stmt::FunctionDeclaration {
                name: "add".to_string(),
                params: vec![("a".to_string(), None, None), ("b".to_string(), None, None)],
                rest_param: None,
                return_type: None,
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
            },
            Stmt::Expression(
                Expr::Call(
                    Box::new(Expr::Variable("add".to_string(), dummy_loc())),
                    vec![
                        Expr::Literal(Literal::Number(5.0), dummy_loc()),
                        Expr::Literal(Literal::Number(3.0), dummy_loc()),
                    ],
                    dummy_loc(),
                ),
                dummy_loc(),
            ),
        ],
    }
}

fn loop_program() -> Program {
    Program {
        statements: vec![
            Stmt::VarDeclaration(
                dryad_parser::ast::Pattern::Identifier("i".to_string()),
                None,
                Some(Expr::Literal(Literal::Number(0.0), dummy_loc())),
                dummy_loc(),
            ),
            Stmt::While(
                Expr::Binary(
                    Box::new(Expr::Variable("i".to_string(), dummy_loc())),
                    "<".to_string(),
                    Box::new(Expr::Literal(Literal::Number(10.0), dummy_loc())),
                    dummy_loc(),
                ),
                Box::new(Stmt::Block(
                    vec![Stmt::Assignment(
                        dryad_parser::ast::Pattern::Identifier("i".to_string()),
                        Expr::Binary(
                            Box::new(Expr::Variable("i".to_string(), dummy_loc())),
                            "+".to_string(),
                            Box::new(Expr::Literal(Literal::Number(1.0), dummy_loc())),
                            dummy_loc(),
                        ),
                        dummy_loc(),
                    )],
                    dummy_loc(),
                )),
                dummy_loc(),
            ),
        ],
    }
}

fn array_access_program() -> Program {
    Program {
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
            Stmt::Expression(
                Expr::Index(
                    Box::new(Expr::Variable("arr".to_string(), dummy_loc())),
                    Box::new(Expr::Literal(Literal::Number(1.0), dummy_loc())),
                    dummy_loc(),
                ),
                dummy_loc(),
            ),
        ],
    }
}

fn compiler_compilation(c: &mut Criterion) {
    let mut group = c.benchmark_group("compilation");

    group.bench_function("simple_arithmetic", |b| {
        b.iter(|| {
            let program = black_box(simple_arithmetic_program());
            let mut compiler = Compiler::new();
            compiler.compile(program)
        })
    });

    group.bench_function("nested_arithmetic", |b| {
        b.iter(|| {
            let program = black_box(nested_arithmetic_program());
            let mut compiler = Compiler::new();
            compiler.compile(program)
        })
    });

    group.bench_function("function_declaration", |b| {
        b.iter(|| {
            let program = black_box(function_declaration_program());
            let mut compiler = Compiler::new();
            compiler.compile(program)
        })
    });

    group.bench_function("loop_program", |b| {
        b.iter(|| {
            let program = black_box(loop_program());
            let mut compiler = Compiler::new();
            compiler.compile(program)
        })
    });

    group.bench_function("array_access", |b| {
        b.iter(|| {
            let program = black_box(array_access_program());
            let mut compiler = Compiler::new();
            compiler.compile(program)
        })
    });

    group.finish();
}

fn vm_execution(c: &mut Criterion) {
    let mut group = c.benchmark_group("execution");

    group.bench_function("simple_arithmetic_execute", |b| {
        b.iter_batched(
            || {
                let program = simple_arithmetic_program();
                let mut compiler = Compiler::new();
                compiler.compile(program).unwrap()
            },
            |chunk| {
                let mut vm = VM::new();
                vm.interpret(black_box(chunk))
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("nested_arithmetic_execute", |b| {
        b.iter_batched(
            || {
                let program = nested_arithmetic_program();
                let mut compiler = Compiler::new();
                compiler.compile(program).unwrap()
            },
            |chunk| {
                let mut vm = VM::new();
                vm.interpret(black_box(chunk))
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("function_call_execute", |b| {
        b.iter_batched(
            || {
                let program = function_declaration_program();
                let mut compiler = Compiler::new();
                compiler.compile(program).unwrap()
            },
            |chunk| {
                let mut vm = VM::new();
                vm.interpret(black_box(chunk))
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("loop_execute", |b| {
        b.iter_batched(
            || {
                let program = loop_program();
                let mut compiler = Compiler::new();
                compiler.compile(program).unwrap()
            },
            |chunk| {
                let mut vm = VM::new();
                vm.interpret(black_box(chunk))
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("array_access_execute", |b| {
        b.iter_batched(
            || {
                let program = array_access_program();
                let mut compiler = Compiler::new();
                compiler.compile(program).unwrap()
            },
            |chunk| {
                let mut vm = VM::new();
                vm.interpret(black_box(chunk))
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn end_to_end_pipeline(c: &mut Criterion) {
    let mut group = c.benchmark_group("end_to_end");

    let programs = vec![
        ("simple", simple_arithmetic_program()),
        ("nested", nested_arithmetic_program()),
        ("function", function_declaration_program()),
        ("loop", loop_program()),
        ("array", array_access_program()),
    ];

    for (name, program) in programs {
        group.bench_with_input(BenchmarkId::from_parameter(name), &program, |b, program| {
            b.iter(|| {
                let mut compiler = Compiler::new();
                let chunk = compiler.compile(black_box(program.clone())).unwrap();
                let mut vm = VM::new();
                vm.interpret(chunk)
            })
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    compiler_compilation,
    vm_execution,
    end_to_end_pipeline
);
criterion_main!(benches);
