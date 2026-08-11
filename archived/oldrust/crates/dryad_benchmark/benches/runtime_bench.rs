// crates/dryad_benchmark/benches/runtime_bench.rs
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use dryad_lexer::{Lexer, token::{Token, TokenWithLocation}};
use dryad_parser::Parser;
use dryad_runtime::Interpreter;
use dryad_benchmark::test_cases::get_all_test_cases;
use dryad_benchmark::test_cases::TestCategory;

fn tokenize_code(code: &str) -> Result<Vec<TokenWithLocation>, Box<dyn std::error::Error>> {
    let mut lexer = Lexer::new(code);
    let mut tokens = Vec::new();
    
    loop {
        let token = lexer.next_token()?;
        let is_eof = matches!(token.token, Token::Eof);
        tokens.push(token);
        if is_eof {
            break;
        }
    }
    
    Ok(tokens)
}

fn runtime_benchmarks(c: &mut Criterion) {
    let all_cases = get_all_test_cases();
    let test_cases = all_cases.get(&TestCategory::Runtime).unwrap_or(&vec![]);
    
    let mut group = c.benchmark_group("runtime");
    
    for test_case in test_cases {
        // Pre-parse para medir apenas a execução
        if let Ok(tokens) = tokenize_code(&test_case.code) {
            let mut parser = Parser::new(tokens);
            if let Ok(ast) = parser.parse() {
                group.bench_with_input(
                    BenchmarkId::new("execute", &test_case.name),
                    &ast,
                    |b, ast| {
                        b.iter(|| {
                            let mut interpreter = Interpreter::new();
                            interpreter.execute(ast)
                        })
                    },
                );
            }
        }
    }
    
    group.finish();
}

fn runtime_arithmetic_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("runtime_arithmetic");
    
    let arithmetic_operations = vec![
        ("addition", "var result = 0; para (var i = 0; i < 1000; i++) { result = result + i; }"),
        ("multiplication", "var result = 1; para (var i = 1; i < 100; i++) { result = result * 2; }"),
        ("division", "var result = 1000000; para (var i = 0; i < 100; i++) { result = result / 2; }"),
        ("modulo", "var result = 0; para (var i = 0; i < 1000; i++) { result = i % 17; }"),
    ];
    
    for (name, code) in arithmetic_operations {
        if let Ok(ast) = compile_code(code) {
            group.bench_function(name, |b| {
                b.iter(|| {
                    let mut interpreter = Interpreter::new();
                    interpreter.execute(&ast)
                })
            });
        }
    }
    
    group.finish();
}

fn runtime_control_flow_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("runtime_control_flow");
    
    let control_flow_tests = vec![
        ("simple_loop", r#"
            para (var i = 0; i < 1000; i++) {
                // loop simples
            }
        "#),
        ("nested_loops", r#"
            para (var i = 0; i < 100; i++) {
                para (var j = 0; j < 10; j++) {
                    // loop aninhado
                }
            }
        "#),
        ("conditional_heavy", r#"
            para (var i = 0; i < 1000; i++) {
                se (i % 2 == 0) {
                    se (i % 4 == 0) {
                        // par e múltiplo de 4
                    } senao {
                        // par mas não múltiplo de 4
                    }
                } senao {
                    se (i % 3 == 0) {
                        // ímpar e múltiplo de 3
                    }
                }
            }
        "#),
        ("function_calls", r#"
            funcao fibonacci(n) {
                se (n <= 1) {
                    retorna n;
                }
                retorna fibonacci(n - 1) + fibonacci(n - 2);
            }
            
            fibonacci(20);
        "#),
    ];
    
    for (name, code) in control_flow_tests {
        if let Ok(ast) = compile_code(code) {
            group.bench_function(name, |b| {
                b.iter(|| {
                    let mut interpreter = Interpreter::new();
                    interpreter.execute(&ast)
                })
            });
        }
    }
    
    group.finish();
}

fn runtime_data_structure_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("runtime_data_structures");
    
    let data_structure_tests = vec![
        ("array_operations", r#"
            var arr = [];
            para (var i = 0; i < 1000; i++) {
                arr.adicionar(i);
            }
            
            var soma = 0;
            paracada (item em arr) {
                soma = soma + item;
            }
        "#),
        ("object_creation", r#"
            var objetos = [];
            para (var i = 0; i < 500; i++) {
                var obj = {
                    id: i,
                    nome: "objeto" + i,
                    ativo: verdadeiro
                };
                objetos.adicionar(obj);
            }
        "#),
        ("string_operations", r#"
            var texto = "";
            para (var i = 0; i < 100; i++) {
                texto = texto + "linha " + i + "\n";
            }
        "#),
    ];
    
    for (name, code) in data_structure_tests {
        if let Ok(ast) = compile_code(code) {
            group.bench_function(name, |b| {
                b.iter(|| {
                    let mut interpreter = Interpreter::new();
                    interpreter.execute(&ast)
                })
            });
        }
    }
    
    group.finish();
}

fn runtime_memory_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("runtime_memory");
    
    // Teste de pressão de memória
    let memory_pressure_tests = vec![
        ("large_array", r#"
            var grandes_arrays = [];
            para (var i = 0; i < 10; i++) {
                var arr = [];
                para (var j = 0; j < 1000; j++) {
                    arr.adicionar(j * i);
                }
                grandes_arrays.adicionar(arr);
            }
        "#),
        ("deep_recursion", r#"
            funcao profunda(n) {
                se (n <= 0) {
                    retorna 0;
                }
                retorna n + profunda(n - 1);
            }
            
            profunda(100);
        "#),
        ("object_chain", r#"
            var primeiro = { valor: 0, proximo: nulo };
            var atual = primeiro;
            
            para (var i = 1; i < 1000; i++) {
                atual.proximo = { valor: i, proximo: nulo };
                atual = atual.proximo;
            }
        "#),
    ];
    
    for (name, code) in memory_pressure_tests {
        if let Ok(ast) = compile_code(code) {
            group.bench_function(name, |b| {
                b.iter(|| {
                    let mut interpreter = Interpreter::new();
                    interpreter.execute(&ast)
                })
            });
        }
    }
    
    group.finish();
}

fn compile_code(code: &str) -> Result<dryad_parser::ast::Program, Box<dyn std::error::Error>> {
    let tokens = tokenize_code(code)?;
    let mut parser = Parser::new(tokens);
    Ok(parser.parse()?)
}

criterion_group!(
    benches,
    runtime_benchmarks,
    runtime_arithmetic_benchmarks,
    runtime_control_flow_benchmarks,
    runtime_data_structure_benchmarks,
    runtime_memory_benchmarks
);
criterion_main!(benches);
