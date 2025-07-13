// crates/dryad_benchmark/benches/parser_bench.rs
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use dryad_lexer::{Lexer, Token};
use dryad_parser::Parser;
use dryad_benchmark::test_cases::get_parser_test_cases;

fn tokenize_code(code: &str) -> Result<Vec<Token>, Box<dyn std::error::Error>> {
    let mut lexer = Lexer::new(code);
    let mut tokens = Vec::new();
    
    loop {
        let token = lexer.next_token()?;
        let is_eof = matches!(token, Token::Eof);
        tokens.push(token);
        if is_eof {
            break;
        }
    }
    
    Ok(tokens)
}

fn parser_benchmarks(c: &mut Criterion) {
    let test_cases = get_parser_test_cases();
    
    let mut group = c.benchmark_group("parser");
    
    for test_case in test_cases {
        // Pre-tokenize para medir apenas o parsing
        if let Ok(tokens) = tokenize_code(&test_case.code) {
            group.bench_with_input(
                BenchmarkId::new("parse", &test_case.name),
                &tokens,
                |b, tokens| {
                    b.iter(|| {
                        let mut parser = Parser::new(tokens.clone());
                        parser.parse()
                    })
                },
            );
        }
    }
    
    group.finish();
}

fn parser_complexity_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("parser_complexity");
    
    // Teste com expressões aninhadas
    let nested_levels = vec![5, 10, 20, 50];
    
    for level in nested_levels {
        let nested_expr = generate_nested_expression(level);
        
        if let Ok(tokens) = tokenize_code(&nested_expr) {
            group.bench_with_input(
                BenchmarkId::new("nested_expr", level),
                &tokens,
                |b, tokens| {
                    b.iter(|| {
                        let mut parser = Parser::new(tokens.clone());
                        parser.parse()
                    })
                },
            );
        }
    }
    
    group.finish();
}

fn parser_statement_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("parser_statements");
    
    // Diferentes tipos de statements
    let statements = vec![
        ("variable_declaration", "var x = 10;"),
        ("function_declaration", "funcao teste() { retorna 42; }"),
        ("if_statement", "se (verdadeiro) { imprimir('oi'); }"),
        ("for_loop", "para (var i = 0; i < 10; i++) { imprimir(i); }"),
        ("while_loop", "enquanto (verdadeiro) { parar; }"),
        ("class_declaration", "classe Test { construtor() { este.x = 1; } }"),
    ];
    
    for (name, code) in statements {
        if let Ok(tokens) = tokenize_code(code) {
            group.bench_function(name, |b| {
                b.iter(|| {
                    let mut parser = Parser::new(tokens.clone());
                    parser.parse()
                })
            });
        }
    }
    
    group.finish();
}

fn generate_nested_expression(depth: usize) -> String {
    if depth == 0 {
        "1".to_string()
    } else {
        format!("({} + {})", generate_nested_expression(depth - 1), generate_nested_expression(depth - 1))
    }
}

criterion_group!(
    benches,
    parser_benchmarks,
    parser_complexity_benchmarks,
    parser_statement_benchmarks
);
criterion_main!(benches);
