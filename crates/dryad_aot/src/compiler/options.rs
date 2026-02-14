// crates/dryad_aot/src/compiler/options.rs
//! Opções de compilação

use crate::backend::{x86_64::X86_64Backend, Backend};
use crate::generator::{elf::ElfGenerator, pe::PeGenerator, Generator};

/// Alvo de compilação
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    /// Linux x86_64
    X86_64Linux,
    /// Linux ARM64
    Arm64Linux,
    /// Windows x86_64
    X86_64Windows,
    /// Windows ARM64
    Arm64Windows,
    /// macOS x86_64
    X86_64MacOS,
    /// macOS ARM64 (Apple Silicon)
    Arm64MacOS,
}

impl Target {
    /// Retorna o triple do target
    pub fn triple(&self) -> &'static str {
        match self {
            Target::X86_64Linux => "x86_64-unknown-linux-gnu",
            Target::Arm64Linux => "aarch64-unknown-linux-gnu",
            Target::X86_64Windows => "x86_64-pc-windows-gnu",
            Target::Arm64Windows => "aarch64-pc-windows-msvc",
            Target::X86_64MacOS => "x86_64-apple-darwin",
            Target::Arm64MacOS => "aarch64-apple-darwin",
        }
    }
    
    /// Cria o backend apropriado
    pub fn create_backend(&self) -> Box<dyn Backend> {
        match self {
            Target::X86_64Linux | Target::X86_64Windows | Target::X86_64MacOS => {
                Box::new(X86_64Backend::new())
            }
            _ => {
                // TODO: Implementar ARM64
                unimplemented!("ARM64 ainda não suportado");
            }
        }
    }
    
    /// Cria o gerador apropriado
    pub fn create_generator(&self) -> Box<dyn Generator> {
        match self {
            Target::X86_64Linux | Target::Arm64Linux => {
                Box::new(ElfGenerator::new())
            }
            Target::X86_64Windows | Target::Arm64Windows => {
                Box::new(PeGenerator::new())
            }
            Target::X86_64MacOS | Target::Arm64MacOS => {
                // macOS usa Mach-O, mas podemos começar com ELF
                Box::new(ElfGenerator::new())
            }
        }
    }
    
    /// Retorna o linker padrão
    pub fn default_linker(&self) -> &'static str {
        match self {
            Target::X86_64Linux | Target::Arm64Linux => "gcc",
            Target::X86_64Windows | Target::Arm64Windows => "gcc",
            Target::X86_64MacOS | Target::Arm64MacOS => "clang",
        }
    }
    
    /// Verifica se é Windows
    pub fn is_windows(&self) -> bool {
        matches!(self, Target::X86_64Windows | Target::Arm64Windows)
    }
    
    /// Verifica se é Linux
    pub fn is_linux(&self) -> bool {
        matches!(self, Target::X86_64Linux | Target::Arm64Linux)
    }
    
    /// Verifica se é macOS
    pub fn is_macos(&self) -> bool {
        matches!(self, Target::X86_64MacOS | Target::Arm64MacOS)
    }
}

/// Nível de otimização
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationLevel {
    /// Sem otimização (compilação rápida)
    None,
    /// Otimizações básicas
    Basic,
    /// Otimizações agressivas
    Aggressive,
    /// Otimização para tamanho
    Size,
}

impl OptimizationLevel {
    /// Retorna a flag correspondente para o linker
    pub fn as_flag(&self) -> &'static str {
        match self {
            OptimizationLevel::None => "-O0",
            OptimizationLevel::Basic => "-O2",
            OptimizationLevel::Aggressive => "-O3",
            OptimizationLevel::Size => "-Os",
        }
    }
}

/// Opções de compilação
#[derive(Debug, Clone)]
pub struct CompileOptions {
    /// Alvo de compilação
    pub target: Target,
    
    /// Nível de otimização
    pub optimization: OptimizationLevel,
    
    /// Linker a usar
    pub linker: String,
    
    /// Bibliotecas a linkar
    pub libraries: Vec<String>,
    
    /// Caminhos de busca de bibliotecas
    pub library_paths: Vec<String>,
    
    /// Flags adicionais para o linker
    pub linker_flags: Vec<String>,
    
    /// Linkagem estática
    pub static_linking: bool,
    
    /// Remover arquivo objeto após linkagem
    pub cleanup_object: bool,
    
    /// Incluir símbolos de debug
    pub debug_symbols: bool,
    
    /// Stripar símbolos do executável final
    pub strip_symbols: bool,
}

impl CompileOptions {
    /// Cria opções padrão para um alvo
    pub fn new(target: Target) -> Self {
        Self {
            target,
            optimization: OptimizationLevel::Basic,
            linker: target.default_linker().to_string(),
            libraries: vec!["dryad_runtime".to_string()],
            library_paths: vec![],
            linker_flags: vec![],
            static_linking: false,
            cleanup_object: true,
            debug_symbols: false,
            strip_symbols: false,
        }
    }
    
    /// Define o linker
    pub fn set_linker(&mut self, linker: impl Into<String>) -> &mut Self {
        self.linker = linker.into();
        self
    }
    
    /// Adiciona uma biblioteca
    pub fn add_library(&mut self, lib: impl Into<String>) -> &mut Self {
        self.libraries.push(lib.into());
        self
    }
    
    /// Adiciona um caminho de biblioteca
    pub fn add_library_path(&mut self, path: impl Into<String>) -> &mut Self {
        self.library_paths.push(path.into());
        self
    }
    
    /// Adiciona uma flag ao linker
    pub fn add_linker_flag(&mut self, flag: impl Into<String>) -> &mut Self {
        self.linker_flags.push(flag.into());
        self
    }
    
    /// Ativa linkagem estática
    pub fn set_static(&mut self) -> &mut Self {
        self.static_linking = true;
        self
    }
    
    /// Ativa símbolos de debug
    pub fn set_debug(&mut self) -> &mut Self {
        self.debug_symbols = true;
        self
    }
    
    /// Ativa stripping de símbolos
    pub fn set_strip(&mut self) -> &mut Self {
        self.strip_symbols = true;
        self
    }
}

impl Default for CompileOptions {
    fn default() -> Self {
        Self::new(Target::X86_64Linux)
    }
}
