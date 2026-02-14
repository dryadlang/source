// crates/dryad_aot/src/backend/mod.rs
//! Backends de geração de código
//!
//! Backends convertem a IR para código de máquina específico de arquitetura.

pub mod x86_64;
pub mod arm64;

use crate::ir::IrModule;

/// Trait para backends de compilação
trait Backend {
    /// Compila um módulo IR para código de máquina
    fn compile_module(&self, module: &IrModule) -> Result<Vec<u8>, String>;
    
    /// Retorna o nome do backend
    fn name(&self) -> &'static str;
    
    /// Retorna o triple do target
    fn target_triple(&self) -> &'static str;
}
