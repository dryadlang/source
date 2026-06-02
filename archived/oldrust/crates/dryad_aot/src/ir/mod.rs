// crates/dryad_aot/src/ir/mod.rs
//! Intermediate Representation (IR)
//!
//! Representação intermediária de baixo nível, próxima ao código de máquina
//! mas ainda independente de arquitetura específica.

pub mod instructions;
pub mod module;
pub mod types;
pub mod values;

pub use instructions::{IrBlock, IrInstruction, IrTerminator};
pub use module::{IrFunction, IrGlobal, IrModule};
pub use types::IrType;
pub use values::{IrConstant, IrRegister, IrValue};

/// ID único para blocos básicos
pub type BlockId = u32;

/// ID único para registradores virtuais
pub type RegisterId = u32;
