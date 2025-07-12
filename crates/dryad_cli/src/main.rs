// crates/dryad_cli/src/main.rs
use clap::{Parser, Subcommand};
use dryad_lexer::Lexer;
use dryad_parser::Parser as DryadParser;
use dryad_runtime::Interpreter;
use dryad_lexer::Token;
use std::fs;
use std::io::{self, Write};

#[derive(Parser)]
#[command(name = "dryad")]
#[command(about = "Dryad Programming Language CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Executa um arquivo Dryad
    Run {
        /// Arquivo .dryad para executar
        file: String,
        /// Modo verboso (mostra tokens e AST)
        #[arg(short, long)]
        verbose: bool,
    },
    /// Inicia o modo interativo (REPL)
    Repl,
    /// Valida a sintaxe de um arquivo sem executar
    Check {
        /// Arquivo .dryad para validar
        file: String,
    },
    /// Mostra os tokens de um arquivo (debug)
    Tokens {
        /// Arquivo .dryad para tokenizar
        file: String,
    },
    /// Mostra informações sobre a versão
    Version,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Run { file, verbose }) => {
            if let Err(e) = run_file(file, *verbose) {
                eprintln!("Erro: {}", e);
                std::process::exit(1);
            }
        }
        Some(Commands::Repl) => {
            if let Err(e) = run_repl() {
                eprintln!("Erro no REPL: {}", e);
                std::process::exit(1);
            }
        }
        Some(Commands::Check { file }) => {
            if let Err(e) = check_file(file) {
                eprintln!("Erro de sintaxe: {}", e);
                std::process::exit(1);
            } else {
                println!("✓ Sintaxe válida");
            }
        }
        Some(Commands::Tokens { file }) => {
            if let Err(e) = show_tokens(file) {
                eprintln!("Erro: {}", e);
                std::process::exit(1);
            }
        }
        Some(Commands::Version) => {
            println!("Dryad v{}", env!("CARGO_PKG_VERSION"));
            println!("Linguagem de programação moderna e expressiva");
        }
        None => {
            // Se não houver subcomando, tenta executar main.dryad
            if std::path::Path::new("main.dryad").exists() {
                if let Err(e) = run_file("main.dryad", false) {
                    eprintln!("Erro: {}", e);
                    std::process::exit(1);
                }
            } else {
                eprintln!("Uso: dryad <comando>");
                eprintln!("Tente 'dryad --help' para mais informações.");
                std::process::exit(1);
            }
        }
    }
}

fn run_file(filename: &str, verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
    let source = fs::read_to_string(filename)
        .map_err(|e| format!("Erro ao ler arquivo '{}': {}", filename, e))?;

    if verbose {
        println!("=== EXECUTANDO: {} ===", filename);
    }

    let mut lexer = Lexer::new(&source);
    let mut tokens = vec![];

    // Tokenização
    loop {
        let token = lexer.next_token()?;
        if matches!(token, Token::Eof) {
            tokens.push(token);
            break;
        }
        tokens.push(token);
    }

    if verbose {
        println!("\n=== TOKENS ===");
        for (i, token) in tokens.iter().enumerate() {
            println!("{:3}: {:?}", i, token);
        }
    }

    // Parsing
    let mut parser = DryadParser::new(tokens);
    let program = parser.parse()?;

    if verbose {
        println!("\n=== AST ===");
        println!("{:#?}", program);
    }

    // Execução
    let mut interpreter = Interpreter::new();
    
    // Definir o arquivo atual para resolução de imports relativos
    interpreter.set_current_file(std::path::PathBuf::from(filename));
    
    let result = interpreter.execute(&program)?;

    if verbose {
        println!("\n=== RESULTADO ===");
        println!("{}", result);
    } else if result != "null" {
        println!("{}", result);
    }

    Ok(())
}

fn run_repl() -> Result<(), Box<dyn std::error::Error>> {
    println!("Dryad v{} - REPL Interativo", env!("CARGO_PKG_VERSION"));
    println!("Digite 'exit' para sair, 'help' para ajuda");
    
    let mut interpreter = Interpreter::new();
    
    loop {
        print!("dryad> ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        let input = input.trim();
        
        match input {
            "exit" | "quit" => {
                println!("Tchau!");
                break;
            }
            "help" => {
                println!("Comandos disponíveis:");
                println!("  exit, quit - Sair do REPL");
                println!("  help       - Mostrar esta ajuda");
                println!("  clear      - Limpar variáveis");
                println!("\nDigite código Dryad para executar.");
                continue;
            }
            "clear" => {
                interpreter = Interpreter::new();
                println!("Variáveis limpas.");
                continue;
            }
            "" => continue,
            _ => {}
        }
        
        // Processa o código
        match process_repl_input(input, &mut interpreter) {
            Ok(result) => {
                if !result.is_empty() && result != "null" {
                    println!("=> {}", result);
                }
            }
            Err(e) => println!("Erro: {}", e),
        }
    }
    
    Ok(())
}

fn process_repl_input(input: &str, interpreter: &mut Interpreter) -> Result<String, Box<dyn std::error::Error>> {
    let mut lexer = Lexer::new(input);
    let mut tokens = vec![];

    loop {
        let token = lexer.next_token()?;
        if matches!(token, Token::Eof) {
            tokens.push(token);
            break;
        }
        tokens.push(token);
    }

    let mut parser = DryadParser::new(tokens);
    let program = parser.parse()?;
    
    let result = interpreter.execute(&program)?;
    Ok(result)
}

fn check_file(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let source = fs::read_to_string(filename)
        .map_err(|e| format!("Erro ao ler arquivo '{}': {}", filename, e))?;

    let mut lexer = Lexer::new(&source);
    let mut tokens = vec![];

    // Tokenização
    loop {
        let token = lexer.next_token()?;
        if matches!(token, Token::Eof) {
            tokens.push(token);
            break;
        }
        tokens.push(token);
    }

    // Parsing (apenas validação)
    let mut parser = DryadParser::new(tokens);
    parser.parse()?;

    Ok(())
}

fn show_tokens(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let source = fs::read_to_string(filename)
        .map_err(|e| format!("Erro ao ler arquivo '{}': {}", filename, e))?;

    let mut lexer = Lexer::new(&source);
    let mut token_count = 0;

    println!("=== TOKENS DE: {} ===", filename);
    
    loop {
        let token = lexer.next_token()?;
        println!("{:3}: {:?}", token_count, token);
        token_count += 1;
        
        if matches!(token, Token::Eof) {
            break;
        }
    }
    
    println!("\nTotal de tokens: {}", token_count);
    Ok(())
}
