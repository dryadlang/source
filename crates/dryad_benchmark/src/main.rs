// crates/dryad_benchmark/src/main.rs
use clap::{Parser, Subcommand};
use dryad_benchmark::{
    BenchmarkRunner, BenchmarkConfig, OutputFormat,
    test_cases::{get_all_test_cases, TestCategory, get_test_cases_by_category},
    reports::ReportGenerator,
    profiler::Profiler,
};
use dryad_lexer::Lexer;
use dryad_parser::Parser as DryadParser;
use dryad_runtime::Interpreter;
use std::fs;
use std::path::Path;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Parser)]
#[command(name = "benchmark")]
#[command(about = "Ferramenta de benchmark para a linguagem Dryad", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Executa todos os benchmarks
    All {
        /// Número de iterações por teste
        #[arg(short, long, default_value = "100")]
        iterations: u64,
        /// Número de iterações de aquecimento
        #[arg(short, long, default_value = "10")]
        warmup: u64,
        /// Formato de saída (console, json, html, csv, markdown)
        #[arg(short, long, default_value = "console")]
        format: String,
        /// Arquivo para salvar os resultados
        #[arg(short, long)]
        output: Option<String>,
        /// Inclui profiling detalhado
        #[arg(short, long)]
        profile: bool,
    },
    /// Executa benchmarks do lexer
    Lexer {
        #[arg(short, long, default_value = "100")]
        iterations: u64,
        #[arg(short, long, default_value = "console")]
        format: String,
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Executa benchmarks do parser
    Parser {
        #[arg(short, long, default_value = "100")]
        iterations: u64,
        #[arg(short, long, default_value = "console")]
        format: String,
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Executa benchmarks do runtime
    Runtime {
        #[arg(short, long, default_value = "100")]
        iterations: u64,
        #[arg(short, long, default_value = "console")]
        format: String,
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Executa benchmark de um arquivo específico
    File {
        /// Caminho para o arquivo .dryad
        file: String,
        #[arg(short, long, default_value = "10")]
        iterations: u64,
        #[arg(short, long)]
        profile: bool,
    },
    /// Compara performance entre duas versões
    Compare {
        /// Diretório com resultados anteriores
        baseline: String,
        /// Executa novos testes para comparação
        #[arg(short, long)]
        run_new: bool,
    },
    /// Gera relatório de stress test
    Stress {
        /// Duração do teste em segundos
        #[arg(short, long, default_value = "60")]
        duration: u64,
        /// Número de threads paralelas
        #[arg(short, long, default_value = "1")]
        threads: usize,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::All { iterations, warmup, format, output, profile } => {
            run_all_benchmarks(iterations, warmup, format, output, profile)?;
        }
        Commands::Lexer { iterations, format, output } => {
            run_category_benchmark(TestCategory::Lexer, iterations, format, output)?;
        }
        Commands::Parser { iterations, format, output } => {
            run_category_benchmark(TestCategory::Parser, iterations, format, output)?;
        }
        Commands::Runtime { iterations, format, output } => {
            run_category_benchmark(TestCategory::Runtime, iterations, format, output)?;
        }
        Commands::File { file, iterations, profile } => {
            run_file_benchmark(&file, iterations, profile)?;
        }
        Commands::Compare { baseline, run_new } => {
            run_comparison(&baseline, run_new)?;
        }
        Commands::Stress { duration, threads } => {
            run_stress_test(duration, threads)?;
        }
    }

    Ok(())
}

fn run_all_benchmarks(
    iterations: u64,
    warmup: u64,
    format: String,
    output: Option<String>,
    with_profile: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "🚀 Iniciando benchmark completo da linguagem Dryad...".cyan().bold());
    
    let output_format = parse_output_format(&format)?;
    let config = BenchmarkConfig {
        iterations,
        warmup_iterations: warmup,
        measure_memory: true,
        output_format: output_format.clone(),
        save_to_file: output.clone(),
    };

    let mut runner = BenchmarkRunner::new(config);
    let mut profiler = if with_profile { Some(Profiler::new()) } else { None };
    
    let all_cases = get_all_test_cases();
    let total_tests: usize = all_cases.values().map(|v| v.len()).sum();
    
    let pb = ProgressBar::new(total_tests as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap()
        .progress_chars("#>-"));

    for (category, test_cases) in all_cases {
        println!("\n{} {:?}...", "📂 Executando categoria".blue(), category);
        
        for test_case in test_cases {
            pb.set_message(format!("Executando: {}", test_case.name));
            
            let result = match category {
                TestCategory::Lexer => {
                    run_lexer_benchmark(&mut runner, &test_case, None)
                }
                TestCategory::Parser => {
                    run_parser_benchmark(&mut runner, &test_case, None)
                }
                TestCategory::Runtime => {
                    run_runtime_benchmark(&mut runner, &test_case, None)
                }
                TestCategory::EndToEnd => {
                    run_end_to_end_benchmark(&mut runner, &test_case, None)
                }
            };

            if let Err(e) = result {
                eprintln!("⚠️  Erro no teste {}: {}", test_case.name, e);
            }
            
            pb.inc(1);
        }
    }

    pb.finish_with_message("Todos os testes concluídos!");

    // Gerar e exibir relatório
    generate_and_display_report(&runner, &output_format, &output)?;

    // Gerar relatório de profiling se habilitado
    if let Some(profiler) = profiler {
        println!("\n{}", "📊 RELATÓRIO DE PROFILING".yellow().bold());
        println!("{}", profiler.generate_report());
        
        if let Some(output_file) = &output {
            let profile_file = format!("{}.profile.json", output_file);
            if let Ok(json) = profiler.export_json() {
                fs::write(&profile_file, json)?;
                println!("💾 Relatório de profiling salvo em: {}", profile_file);
            }
        }
    }

    Ok(())
}

fn run_category_benchmark(
    category: TestCategory,
    iterations: u64,
    format: String,
    output: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{} {:?}...", "🎯 Executando benchmarks da categoria".cyan().bold(), category);
    
    let output_format = parse_output_format(&format)?;
    let config = BenchmarkConfig {
        iterations,
        warmup_iterations: 10,
        measure_memory: true,
        output_format: output_format.clone(),
        save_to_file: output.clone(),
    };

    let mut runner = BenchmarkRunner::new(config);
    let all_cases = get_all_test_cases();
    
    if let Some(test_cases) = all_cases.get(&category) {
        let pb = ProgressBar::new(test_cases.len() as u64);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("#>-"));

        for test_case in test_cases {
            pb.set_message(format!("Executando: {}", test_case.name));
            
            let result = match category {
                TestCategory::Lexer => run_lexer_benchmark(&mut runner, test_case, None),
                TestCategory::Parser => run_parser_benchmark(&mut runner, test_case, None),
                TestCategory::Runtime => run_runtime_benchmark(&mut runner, test_case, None),
                TestCategory::EndToEnd => run_end_to_end_benchmark(&mut runner, test_case, None),
            };

            if let Err(e) = result {
                eprintln!("⚠️  Erro no teste {}: {}", test_case.name, e);
            }
            
            pb.inc(1);
        }

        pb.finish_with_message("Categoria concluída!");
        generate_and_display_report(&runner, &output_format, &output)?;
    } else {
        println!("❌ Categoria não encontrada: {:?}", category);
    }

    Ok(())
}

fn run_lexer_benchmark(
    runner: &mut BenchmarkRunner,
    test_case: &dryad_benchmark::test_cases::TestCase,
    profiler_name: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    runner.run_with_input(&test_case.name, &test_case.code, |code| {
        let tokens = tokenize_code(code)?;
        
        // Verificar se há tokens válidos
        if tokens.is_empty() {
            return Err("Nenhum token gerado".into());
        }
        
        Ok(())
    });

    Ok(())
}

fn run_parser_benchmark(
    runner: &mut BenchmarkRunner,
    test_case: &dryad_benchmark::test_cases::TestCase,
    profiler_name: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    runner.run_with_input(&test_case.name, &test_case.code, |code| {
        let tokens = tokenize_code(code)?;
        let mut parser = DryadParser::new(tokens);
        let _ast = parser.parse()?;
        
        Ok(())
    });

    Ok(())
}

fn run_runtime_benchmark(
    runner: &mut BenchmarkRunner,
    test_case: &dryad_benchmark::test_cases::TestCase,
    profiler_name: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    runner.run_with_input(&test_case.name, &test_case.code, |code| {
        let tokens = tokenize_code(code)?;
        let mut parser = DryadParser::new(tokens);
        let ast = parser.parse()?;
        let mut interpreter = Interpreter::new();
        let _result = interpreter.execute(&ast)?;
        
        Ok(())
    });

    Ok(())
}

fn run_end_to_end_benchmark(
    runner: &mut BenchmarkRunner,
    test_case: &dryad_benchmark::test_cases::TestCase,
    profiler_name: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    runner.run_with_input(&test_case.name, &test_case.code, |code| {
        // Lexer
        let tokens = tokenize_code(code)?;
        
        // Parser
        let mut parser = DryadParser::new(tokens);
        let ast = parser.parse()?;
        
        // Runtime
        let mut interpreter = Interpreter::new();
        let _result = interpreter.execute(&ast)?;
        
        Ok(())
    });

    Ok(())
}

fn run_file_benchmark(
    file_path: &str,
    iterations: u64,
    _with_profile: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{} {}...", "📄 Executando benchmark do arquivo".cyan().bold(), file_path);
    
    if !Path::new(file_path).exists() {
        return Err(format!("Arquivo não encontrado: {}", file_path).into());
    }

    let code = fs::read_to_string(file_path)?;
    let config = BenchmarkConfig {
        iterations,
        warmup_iterations: 5,
        measure_memory: true,
        output_format: OutputFormat::Console,
        save_to_file: None,
    };

    let mut runner = BenchmarkRunner::new(config);

    let file_name = Path::new(file_path)
        .file_name()
        .unwrap_or_else(|| std::ffi::OsStr::new("unknown"))
        .to_string_lossy();

    println!("🏃 Executando {} iterações...", iterations);

    runner.run_with_input(&file_name, &code, |code| {
        let tokens = tokenize_code(code)?;
        let mut parser = DryadParser::new(tokens);
        let ast = parser.parse()?;
        let mut interpreter = Interpreter::new();
        let _result = interpreter.execute(&ast)?;
        
        Ok(())
    });

    generate_and_display_report(&runner, &OutputFormat::Console, &None)?;

    Ok(())
}

fn run_comparison(
    baseline_path: &str,
    run_new: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "🔍 Executando análise comparativa...".cyan().bold());
    
    if run_new {
        println!("Executando novos benchmarks...");
        run_all_benchmarks(50, 5, "json".to_string(), Some("current_results.json".to_string()), false)?;
    }

    println!("Comparando com baseline em: {}", baseline_path);
    // Implementar lógica de comparação aqui
    println!("⚠️  Funcionalidade de comparação em desenvolvimento");
    
    Ok(())
}

fn run_stress_test(
    duration: u64,
    threads: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{} por {} segundos com {} threads...", 
        "🔥 Executando stress test".red().bold(), duration, threads);
    
    // Implementar stress test aqui
    println!("⚠️  Funcionalidade de stress test em desenvolvimento");
    
    Ok(())
}

fn parse_output_format(format: &str) -> Result<OutputFormat, Box<dyn std::error::Error>> {
    match format.to_lowercase().as_str() {
        "console" => Ok(OutputFormat::Console),
        "json" => Ok(OutputFormat::Json),
        "html" => Ok(OutputFormat::Html),
        "csv" => Ok(OutputFormat::Csv),
        "markdown" | "md" => Ok(OutputFormat::Markdown),
        _ => Err(format!("Formato de saída não suportado: {}. Use: console, json, html, csv, markdown", format).into()),
    }
}

fn generate_and_display_report(
    runner: &BenchmarkRunner,
    format: &OutputFormat,
    output_file: &Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let results = runner.get_results();

    match format {
        OutputFormat::Console => {
            ReportGenerator::generate_console_report(results);
        }
        OutputFormat::Json => {
            let json = ReportGenerator::generate_json_report(results)?;
            if let Some(file) = output_file {
                fs::write(file, &json)?;
                println!("💾 Relatório JSON salvo em: {}", file);
            } else {
                println!("{}", json);
            }
        }
        OutputFormat::Html => {
            let html = ReportGenerator::generate_html_report(results);
            if let Some(file) = output_file {
                fs::write(file, &html)?;
                println!("💾 Relatório HTML salvo em: {}", file);
            } else {
                println!("{}", html);
            }
        }
        OutputFormat::Csv => {
            let csv = ReportGenerator::generate_csv_report(results);
            if let Some(file) = output_file {
                fs::write(file, &csv)?;
                println!("💾 Relatório CSV salvo em: {}", file);
            } else {
                println!("{}", csv);
            }
        }
        OutputFormat::Markdown => {
            let markdown = ReportGenerator::generate_markdown_report(results);
            if let Some(file) = output_file {
                fs::write(file, &markdown)?;
                println!("💾 Relatório Markdown salvo em: {}", file);
            } else {
                println!("{}", markdown);
            }
        }
    }

    // Salvar resultados se configurado
    runner.save_results()?;

    Ok(())
}

// Função auxiliar para tokenizar código completo
fn tokenize_code(code: &str) -> Result<Vec<dryad_lexer::Token>, Box<dyn std::error::Error>> {
    let mut lexer = Lexer::new(code);
    let mut tokens = Vec::new();
    
    loop {
        let token = lexer.next_token()?;
        let is_eof = matches!(token, dryad_lexer::Token::Eof);
        tokens.push(token);
        if is_eof {
            break;
        }
    }
    
    Ok(tokens)
}
