// crates/dryad_aot/src/generator/pe.rs
//! Gerador de arquivos PE
//!
//! Gera executáveis no formato PE/COFF (Windows).

use super::Generator;
use crate::ir::IrModule;

/// Gerador de PE
pub struct PeGenerator {
    /// Subsystem: 1=native, 2=windows, 3=console
    subsystem: u16,
}

impl PeGenerator {
    pub fn new() -> Self {
        Self { subsystem: 3 } // CONSOLE por padrão
    }
    
    /// Define o subsystem
    pub fn set_subsystem(mut self, subsystem: u16) -> Self {
        self.subsystem = subsystem;
        self
    }
}

impl Generator for PeGenerator {
    fn generate_object(&self, _module: &IrModule, _code: &[u8]) -> Result<Vec<u8>, String> {
        // TODO: Implementar gerador PE completo
        // Por enquanto, retorna erro
        Err("Gerador PE ainda não implementado".to_string())
    }
    
    fn format_name(&self) -> &'static str {
        "PE"
    }
    
    fn file_extension(&self) -> &'static str {
        ".exe"
    }
}
