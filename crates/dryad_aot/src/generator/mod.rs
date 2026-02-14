// crates/dryad_aot/src/generator/mod.rs
//! Geradores de formato de executável
//!
//! Geram arquivos objeto ou executáveis nos formatos ELF, PE, etc.

pub mod elf;
pub mod pe;

use crate::ir::IrModule;

/// Trait para geradores de formato
trait Generator {
    /// Gera um arquivo objeto a partir do módulo IR
    fn generate_object(&self, module: &IrModule, code: &[u8]) -> Result<Vec<u8>, String>;
    
    /// Retorna o nome do formato
    fn format_name(&self) -> &'static str;
    
    /// Retorna a extensão de arquivo padrão
    fn file_extension(&self) -> &'static str;
}
