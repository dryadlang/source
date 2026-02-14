// crates/dryad_aot/src/lib.rs
//! # Dryad AOT Compiler
//!
//! Compilador Ahead-of-Time que converte bytecode Dryad para executáveis nativos.
//!
//! ## Arquitetura
//!
//! ```text
//! Bytecode (.dryad)
//!     ↓
//! IR (Intermediate Representation)
//!     ↓
//! Backend (x86_64, ARM64)
//!     ↓
//! Object File (.o)
//!     ↓
//! Linker
//!     ↓
//! Executable (ELF/PE)
//! ```
//!
//! ## Uso
//!
//! ```rust,no_run
//! use dryad_aot::{AotCompiler, Target};
//!
//! let compiler = AotCompiler::new(Target::X86_64Linux);
//! compiler.compile_file("script.dryad", "output")?;
//! ```

pub mod ir;
pub mod backend;
pub mod generator;
pub mod linker;
pub mod compiler;

pub use compiler::{AotCompiler, CompileOptions, Target};
pub use ir::{IrModule, IrFunction, IrInstruction, IrType, IrValue};
pub use backend::x86_64::X86_64Backend;
pub use generator::elf::ElfGenerator;

/// Versão da crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Verifica se o compilador AOT está disponível
pub fn is_available() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
