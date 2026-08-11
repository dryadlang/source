// crates/dryad_aot/examples/simple_compile.rs
//! Exemplo simples de compilação AOT

use dryad_aot::{AotCompiler, Target};

fn main() {
    // Criar compilador para Linux x86_64
    let compiler = AotCompiler::new(Target::X86_64Linux);
    
    // Compilar arquivo
    match compiler.compile_file("hello.dryad", "hello") {
        Ok(()) => println!("Compilação bem-sucedida!"),
        Err(e) => eprintln!("Erro: {}", e),
    }
}
