// crates/dryad_benchmark/benches/lexer_bench.rs
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use dryad_lexer::{Lexer, Token};
use dryad_benchmark::test_cases::get_lexer_test_cases;

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

fn lexer_benchmarks(c: &mut Criterion) {
    let test_cases = get_lexer_test_cases();
    
    let mut group = c.benchmark_group("lexer");
    
    for test_case in test_cases {
        group.bench_with_input(
            BenchmarkId::new("tokenize", &test_case.name),
            &test_case.code,
            |b, code| {
                b.iter(|| {
                    tokenize_code(code)
                })
            },
        );
    }
    
    group.finish();
}

fn lexer_scaling_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("lexer_scaling");
    
    // Teste de escalabilidade com diferentes tamanhos de código
    let base_code = "var x = 10 + 20 * 30;\n";
    let sizes = vec![10, 100, 1000, 5000];
    
    for size in sizes {
        let large_code = base_code.repeat(size);
        group.bench_with_input(
            BenchmarkId::new("scale", size),
            &large_code,
            |b, code| {
                b.iter(|| {
                    tokenize_code(code)
                })
            },
        );
    }
    
    group.finish();
}

fn lexer_memory_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("lexer_memory");
    
    // Teste com strings muito longas
    let long_string_code = format!(r#"var texto = "{}";"#, "a".repeat(10000));
    
    group.bench_function("long_string", |b| {
        b.iter(|| {
            tokenize_code(&long_string_code)
        })
    });
    
    // Teste com muitos tokens pequenos
    let many_tokens_code = (0..1000)
        .map(|i| format!("var x{} = {};", i, i))
        .collect::<Vec<_>>()
        .join("\n");
    
    group.bench_function("many_tokens", |b| {
        b.iter(|| {
            tokenize_code(&many_tokens_code)
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    lexer_benchmarks,
    lexer_scaling_benchmarks,
    lexer_memory_benchmarks
);
criterion_main!(benches);
