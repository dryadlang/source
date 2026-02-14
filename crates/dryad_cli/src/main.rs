// crates/dryad_cli/src/main.rs
use clap::{Parser, Subcommand};
use dryad_checker::TypeChecker;
use dryad_lexer::Lexer;
use dryad_lexer::Token;
use dryad_parser::Parser as DryadParser;
use dryad_runtime::Interpreter;
use std::fs;
use std::io::{self, Write};

mod oak_adapter;
use oak_adapter::OakModuleResolver;

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
        /// Permite operações inseguras (ex: native_set_env)
        #[arg(long)]
        allow_unsafe: bool,
        /// Permite execução de comandos do sistema (ex: native_exec)
        #[arg(long)]
        allow_exec: bool,
        /// Diretório raiz para o sandbox de arquivos
        #[arg(long)]
        sandbox: Option<String>,
        /// Compila para bytecode antes de executar (mais rápido para execuções repetidas)
        #[arg(long)]
        compile: bool,
        /// Usa compilação JIT para funções quentes (experimental)
        #[arg(long)]
        jit: bool,
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
        Some(Commands::Run {
            file,
            verbose,
            allow_unsafe,
            allow_exec,
            sandbox,
            compile,
            jit,
        }) => {
            if let Err(e) = run_file(
                file,
                *verbose,
                *allow_unsafe,
                *allow_exec,
                sandbox.as_deref(),
                *compile,
                *jit,
            ) {
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
        Some(Commands::Check { file }) => match check_file(file) {
            Ok(_) => println!("✓ Código válido (sintaxe e tipos)"),
            Err(e) => {
                eprintln!("Erro de validação:\n{}", e);
                std::process::exit(1);
            }
        },
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
                if let Err(e) = run_file("main.dryad", false, false, false, None) {
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

fn run_file(
    filename: &str,
    verbose: bool,
    allow_unsafe: bool,
    allow_exec: bool,
    sandbox: Option<&str>,
    compile: bool,
    jit: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let source = fs::read_to_string(filename)
        .map_err(|e| format!("Erro ao ler arquivo '{}': {}", filename, e))?;

    if verbose {
        println!("=== EXECUTANDO: {} ===", filename);
    }

    let mut lexer = Lexer::new(&source);
    let mut tokens = vec![];

    // Tokenização
    loop {
        let token_with_loc = lexer.next_token()?;
        if matches!(token_with_loc.token, Token::Eof) {
            tokens.push(token_with_loc);
            break;
        }
        tokens.push(token_with_loc);
    }

    if verbose {
        println!("\n=== TOKENS ===");
        for (i, token_with_loc) in tokens.iter().enumerate() {
            println!(
                "{:3}: {:?} @ {:?}",
                i, token_with_loc.token, token_with_loc.location
            );
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

    // Configurar o resolver (Oak)
    interpreter.set_resolver(Box::new(OakModuleResolver));

    // Configurar flags de segurança
    interpreter.set_allow_unsafe(allow_unsafe);
    interpreter.set_allow_exec(allow_exec);

    if let Some(sb) = sandbox {
        interpreter.set_sandbox_root(std::path::PathBuf::from(sb));
    }

    // Definir o arquivo atual para resolução de imports relativos
    interpreter.set_current_file(std::path::PathBuf::from(filename));

    // Configurar modo de execução
    if compile {
        println!("Modo: Bytecode Compiler");
        interpreter.set_compile_mode(true);
    }
    if jit {
        println!("Modo: JIT Compiler (experimental)");
        interpreter.set_jit_mode(true);
    }

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
    // Configurar o resolver (Oak)
    interpreter.set_resolver(Box::new(OakModuleResolver));

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
                interpreter.set_resolver(Box::new(OakModuleResolver));
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

fn process_repl_input(
    input: &str,
    interpreter: &mut Interpreter,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut lexer = Lexer::new(input);
    let mut tokens = vec![];

    loop {
        let token = lexer.next_token()?;
        if matches!(token.token, Token::Eof) {
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
        if matches!(token.token, Token::Eof) {
            tokens.push(token);
            break;
        }
        tokens.push(token);
    }

    // Parsing
    let mut parser = DryadParser::new(tokens);
    let program = parser.parse()?;

    // Type Checking
    let mut checker = TypeChecker::new();
    if let Err(errors) = checker.check(&program) {
        let mut error_msg = String::new();
        for err in errors {
            error_msg.push_str(&format!("- {}\n", err));
        }
        return Err(error_msg.into());
    }

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

        if matches!(token.token, Token::Eof) {
            break;
        }
    }

    println!("\nTotal de tokens: {}", token_count);
    Ok(())
}
