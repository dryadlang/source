// crates/dryad_benchmark/benches/end_to_end_bench.rs
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use dryad_lexer::{Lexer, Token};
use dryad_parser::Parser;
use dryad_runtime::Interpreter;
use dryad_benchmark::test_cases::get_end_to_end_test_cases;

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

fn end_to_end_benchmarks(c: &mut Criterion) {
    let test_cases = get_end_to_end_test_cases();
    
    let mut group = c.benchmark_group("end_to_end");
    
    for test_case in test_cases {
        group.bench_with_input(
            BenchmarkId::new("full_pipeline", &test_case.name),
            &test_case.code,
            |b, code| {
                b.iter(|| {
                    // Lexer
                    if let Ok(tokens) = tokenize_code(code) {
                        // Parser
                        let mut parser = Parser::new(tokens);
                        if let Ok(ast) = parser.parse() {
                            // Runtime
                            let mut interpreter = Interpreter::new();
                            let _ = interpreter.execute(&ast);
                        }
                    }
                })
            },
        );
    }
    
    group.finish();
}

fn pipeline_stage_benchmarks(c: &mut Criterion) {
    let sample_code = r#"
        classe Contador {
            construtor(inicial) {
                este.valor = inicial;
            }
            
            incrementar() {
                este.valor = este.valor + 1;
                retorna este.valor;
            }
            
            decrementar() {
                este.valor = este.valor - 1;
                retorna este.valor;
            }
        }
        
        var contador = novo Contador(0);
        
        para (var i = 0; i < 100; i++) {
            contador.incrementar();
        }
        
        para (var j = 0; j < 50; j++) {
            contador.decrementar();
        }
        
        imprimir("Valor final: " + contador.valor);
    "#;
    
    let mut group = c.benchmark_group("pipeline_stages");
    
    // Benchmark apenas lexer
    group.bench_function("lexer_only", |b| {
        b.iter(|| {
            tokenize_code(sample_code)
        })
    });
    
    // Benchmark lexer + parser
    group.bench_function("lexer_parser", |b| {
        b.iter(|| {
            if let Ok(tokens) = tokenize_code(sample_code) {
                let mut parser = Parser::new(tokens);
                let _ = parser.parse();
            }
        })
    });
    
    // Benchmark pipeline completo
    group.bench_function("full_pipeline", |b| {
        b.iter(|| {
            if let Ok(tokens) = tokenize_code(sample_code) {
                let mut parser = Parser::new(tokens);
                if let Ok(ast) = parser.parse() {
                    let mut interpreter = Interpreter::new();
                    let _ = interpreter.execute(&ast);
                }
            }
        })
    });
    
    group.finish();
}

fn real_world_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("real_world");
    
    // Simulação de um programa de calculadora
    let calculator_program = r#"
        classe Calculadora {
            construtor() {
                este.memoria = 0;
            }
            
            somar(a, b) {
                var resultado = a + b;
                este.memoria = resultado;
                retorna resultado;
            }
            
            subtrair(a, b) {
                var resultado = a - b;
                este.memoria = resultado;
                retorna resultado;
            }
            
            multiplicar(a, b) {
                var resultado = a * b;
                este.memoria = resultado;
                retorna resultado;
            }
            
            dividir(a, b) {
                se (b == 0) {
                    imprimir("Erro: divisão por zero");
                    retorna nulo;
                }
                var resultado = a / b;
                este.memoria = resultado;
                retorna resultado;
            }
            
            calcularExpressao(numeros, operacoes) {
                var resultado = numeros[0];
                
                para (var i = 0; i < operacoes.tamanho; i++) {
                    var op = operacoes[i];
                    var num = numeros[i + 1];
                    
                    se (op == "+") {
                        resultado = este.somar(resultado, num);
                    } senao se (op == "-") {
                        resultado = este.subtrair(resultado, num);
                    } senao se (op == "*") {
                        resultado = este.multiplicar(resultado, num);
                    } senao se (op == "/") {
                        resultado = este.dividir(resultado, num);
                    }
                }
                
                retorna resultado;
            }
        }
        
        var calc = novo Calculadora();
        
        // Teste de expressões múltiplas
        para (var i = 0; i < 50; i++) {
            var numeros = [i, i + 1, i + 2, i + 3];
            var ops = ["+", "*", "-"];
            calc.calcularExpressao(numeros, ops);
        }
    "#;
    
    group.bench_function("calculator_simulation", |b| {
        b.iter(|| {
            if let Ok(tokens) = tokenize_code(calculator_program) {
                let mut parser = Parser::new(tokens);
                if let Ok(ast) = parser.parse() {
                    let mut interpreter = Interpreter::new();
                    let _ = interpreter.execute(&ast);
                }
            }
        })
    });
    
    // Simulação de um gerenciador de tarefas
    let task_manager_program = r#"
        classe Tarefa {
            construtor(id, descricao, prioridade) {
                este.id = id;
                este.descricao = descricao;
                este.prioridade = prioridade;
                este.concluida = falso;
            }
            
            concluir() {
                este.concluida = verdadeiro;
            }
        }
        
        classe GerenciadorTarefas {
            construtor() {
                este.tarefas = [];
            }
            
            adicionarTarefa(tarefa) {
                este.tarefas.adicionar(tarefa);
            }
            
            buscarPorPrioridade(prioridade) {
                var resultado = [];
                paracada (tarefa em este.tarefas) {
                    se (tarefa.prioridade == prioridade) {
                        resultado.adicionar(tarefa);
                    }
                }
                retorna resultado;
            }
            
            contarConcluidas() {
                var contador = 0;
                paracada (tarefa em este.tarefas) {
                    se (tarefa.concluida) {
                        contador = contador + 1;
                    }
                }
                retorna contador;
            }
        }
        
        var gerenciador = novo GerenciadorTarefas();
        
        // Criar muitas tarefas
        para (var i = 0; i < 200; i++) {
            var tarefa = novo Tarefa(i, "Tarefa " + i, i % 5 + 1);
            gerenciador.adicionarTarefa(tarefa);
            
            // Concluir algumas tarefas aleatoriamente
            se (i % 3 == 0) {
                tarefa.concluir();
            }
        }
        
        // Buscar tarefas por prioridade
        para (var p = 1; p <= 5; p++) {
            var tarefas_prioritarias = gerenciador.buscarPorPrioridade(p);
        }
        
        var total_concluidas = gerenciador.contarConcluidas();
    "#;
    
    group.bench_function("task_manager_simulation", |b| {
        b.iter(|| {
            if let Ok(tokens) = tokenize_code(task_manager_program) {
                let mut parser = Parser::new(tokens);
                if let Ok(ast) = parser.parse() {
                    let mut interpreter = Interpreter::new();
                    let _ = interpreter.execute(&ast);
                }
            }
        })
    });
    
    group.finish();
}

fn scaling_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("scaling");
    
    // Teste de escalabilidade com diferentes tamanhos de programa
    let sizes = vec![10, 50, 100, 500];
    
    for size in sizes {
        let large_program = generate_large_program(size);
        
        group.bench_with_input(
            BenchmarkId::new("program_size", size),
            &large_program,
            |b, code| {
                b.iter(|| {
                    if let Ok(tokens) = tokenize_code(code) {
                        let mut parser = Parser::new(tokens);
                        if let Ok(ast) = parser.parse() {
                            let mut interpreter = Interpreter::new();
                            let _ = interpreter.execute(&ast);
                        }
                    }
                })
            },
        );
    }
    
    group.finish();
}

fn generate_large_program(functions_count: usize) -> String {
    let mut program = String::new();
    
    for i in 0..functions_count {
        program.push_str(&format!(r#"
            funcao funcao_{}(x) {{
                var resultado = 0;
                para (var j = 0; j < x; j++) {{
                    resultado = resultado + j * {};
                }}
                retorna resultado;
            }}
        "#, i, i + 1));
    }
    
    program.push_str("\n// Chamadas das funções\n");
    for i in 0..functions_count {
        program.push_str(&format!("var resultado_{} = funcao_{}({});\n", i, i, i * 2 + 1));
    }
    
    program
}

criterion_group!(
    benches,
    end_to_end_benchmarks,
    pipeline_stage_benchmarks,
    real_world_benchmarks,
    scaling_benchmarks
);
criterion_main!(benches);
