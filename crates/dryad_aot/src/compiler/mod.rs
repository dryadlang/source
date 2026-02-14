// crates/dryad_aot/src/compiler/mod.rs
//! Compilador AOT principal
//!
//! Orquestra o processo de compilação: Bytecode → IR → Código de Máquina → Executável

pub mod converter;
pub mod options;

pub use converter::BytecodeToIrConverter;
pub use options::{CompileOptions, OptimizationLevel, Target};

use crate::backend::Backend;
use crate::generator::Generator;
use crate::ir::IrModule;
use dryad_bytecode::{Chunk, VM};

/// Compilador AOT principal
pub struct AotCompiler {
    /// Opções de compilação
    options: CompileOptions,
    
    /// Backend de código (x86_64, ARM64, etc.)
    backend: Box<dyn Backend>,
    
    /// Gerador de formato (ELF, PE, etc.)
    generator: Box<dyn Generator>,
}

impl AotCompiler {
    /// Cria um novo compilador com as opções padrão
    pub fn new(target: Target) -> Self {
        let options = CompileOptions::new(target);
        let backend = target.create_backend();
        let generator = target.create_generator();
        
        Self {
            options,
            backend,
            generator,
        }
    }
    
    /// Cria um compilador com opções personalizadas
    pub fn with_options(options: CompileOptions) -> Self {
        let backend = options.target.create_backend();
        let generator = options.target.create_generator();
        
        Self {
            options,
            backend,
            generator,
        }
    }
    
    /// Define o nível de otimização
    pub fn set_optimization(&mut self, level: OptimizationLevel) {
        self.options.optimization = level;
    }
    
    /// Compila um arquivo .dryad
    pub fn compile_file(&self, input: &str, output: &str) -> Result<(), String> {
        // 1. Parse do arquivo
        let source = std::fs::read_to_string(input)
            .map_err(|e| format!("Erro ao ler arquivo: {}", e))?;
        
        // 2. Compilar para bytecode
        let bytecode = self.compile_to_bytecode(&source)?;
        
        // 3. Compilar para executável
        self.compile_bytecode(&bytecode, output)
    }
    
    /// Compila código fonte para bytecode
    fn compile_to_bytecode(&self, source: &str) -> Result<Chunk, String> {
        use dryad_bytecode::Compiler;
        use dryad_parser::Parser;
        use dryad_lexer::Lexer;
        
        // Tokenizar
        let lexer = Lexer::new(source);
        let tokens = lexer.tokenize()
            .map_err(|e| format!("Erro de lexing: {:?}", e))?;
        
        // Parse
        let mut parser = Parser::new(tokens);
        let program = parser.parse()
            .map_err(|e| format!("Erro de parsing: {:?}", e))?;
        
        // Compilar para bytecode
        let mut compiler = Compiler::new();
        compiler.compile(program)
            .map_err(|e| format!("Erro de compilação: {}", e))
    }
    
    /// Compila bytecode para executável nativo
    pub fn compile_bytecode(&self, bytecode: &Chunk, output: &str) -> Result<(), String> {
        // 1. Bytecode → IR
        let mut converter = BytecodeToIrConverter::new();
        let ir_module = converter.convert(bytecode)?;
        
        // 2. Otimizar IR (se necessário)
        let ir_module = self.optimize_ir(ir_module);
        
        // 3. IR → Código de máquina
        let object_code = self.backend.compile_module(&ir_module)?;
        
        // 4. Gerar arquivo objeto
        let object_file = format!("{}.o", output);
        std::fs::write(&object_file, object_code)
            .map_err(|e| format!("Erro ao escrever arquivo objeto: {}", e))?;
        
        // 5. Linkar executável
        self.link(&object_file, output)?;
        
        // 6. Limpar arquivo objeto (opcional)
        if self.options.cleanup_object {
            let _ = std::fs::remove_file(&object_file);
        }
        
        Ok(())
    }
    
    /// Otimiza o módulo IR
    fn optimize_ir(&self, module: IrModule) -> IrModule {
        match self.options.optimization {
            OptimizationLevel::None => module,
            OptimizationLevel::Basic => self.run_basic_optimizations(module),
            OptimizationLevel::Aggressive => self.run_aggressive_optimizations(module),
            OptimizationLevel::Size => self.run_size_optimizations(module),
        }
    }
    
    /// Otimizações básicas
    fn run_basic_optimizations(&self, mut module: IrModule) -> IrModule {
        // TODO: Implementar otimizações básicas
        // - Constant folding
        // - Dead code elimination
        module
    }
    
    /// Otimizações agressivas
    fn run_aggressive_optimizations(&self, mut module: IrModule) -> IrModule {
        let module = self.run_basic_optimizations(module);
        // TODO: Implementar otimizações avançadas
        // - Inlining
        // - Loop optimizations
        module
    }
    
    /// Otimizações para tamanho
    fn run_size_optimizations(&self, mut module: IrModule) -> IrModule {
        // TODO: Implementar otimizações de tamanho
        module
    }
    
    /// Linka o arquivo objeto para criar executável
    fn link(&self, object_file: &str, output: &str) -> Result<(), String> {
        use std::process::Command;
        
        let mut cmd = Command::new(&self.options.linker);
        
        // Flags de linkagem
        cmd.arg(object_file)
           .arg("-o")
           .arg(output);
        
        // Bibliotecas
        for lib in &self.options.libraries {
            cmd.arg(format!("-l{}", lib));
        }
        
        // Caminhos de busca
        for path in &self.options.library_paths {
            cmd.arg(format!("-L{}", path));
        }
        
        // Linkagem estática/dinâmica
        if self.options.static_linking {
            cmd.arg("-static");
        }
        
        // Flags adicionais
        cmd.args(&self.options.linker_flags);
        
        // Executar
        let output = cmd.output()
            .map_err(|e| format!("Erro ao executar linker: {}", e))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Erro de linkagem: {}", stderr));
        }
        
        Ok(())
    }
}
