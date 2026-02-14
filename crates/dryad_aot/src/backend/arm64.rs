// crates/dryad_aot/src/backend/arm64.rs
//! Backend ARM64
//!
//! Gera código de máquina ARM64/AArch64 a partir da IR.

use super::Backend;
use crate::ir::IrModule;

/// Backend para ARM64
pub struct Arm64Backend;

impl Arm64Backend {
    pub fn new() -> Self {
        Self
    }
}

impl Backend for Arm64Backend {
    fn compile_module(&self, _module: &IrModule) -> Result<Vec<u8>, String> {
        // TODO: Implementar backend ARM64
        Err("Backend ARM64 ainda não implementado".to_string())
    }
    
    fn name(&self) -> &'static str {
        "arm64"
    }
    
    fn target_triple(&self) -> &'static str {
        "aarch64-unknown-linux-gnu"
    }
}
