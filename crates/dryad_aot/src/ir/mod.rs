// crates/dryad_aot/src/ir/mod.rs
//! Intermediate Representation (IR)
//!
//! Representação intermediária de baixo nível, próxima ao código de máquina
//! mas ainda independente de arquitetura específica.

pub mod instructions;
pub mod types;
pub mod values;
pub mod module;

pub use instructions::{IrInstruction, IrBlock, IrTerminator};
pub use types::IrType;
pub use values::{IrValue, IrConstant, IrRegister};
pub use module::{IrModule, IrFunction, IrGlobal};

/// ID único para blocos básicos
type BlockId = u32;

/// ID único para registradores virtuais
type RegisterId = u32;
