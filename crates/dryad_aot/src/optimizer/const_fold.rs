// crates/dryad_aot/src/optimizer/const_fold.rs
//! Constant Folding
//!
//! Performs compile-time evaluation of constant expressions.

use super::Optimizer;
use crate::ir::{IrBlock, IrConstant, IrFunction, IrInstruction, IrModule, IrTerminator, IrValue};
use std::collections::HashMap;

/// Constant Folder
pub struct ConstantFolder;

impl Optimizer for ConstantFolder {
    fn optimize(&self, module: IrModule) -> IrModule {
        let functions = module
            .functions
            .into_iter()
            .map(|func| Self::fold_constants_in_function(func))
            .collect();

        IrModule {
            name: module.name,
            functions,
            globals: module.globals,
            metadata: module.metadata,
            next_register_id: module.next_register_id,
            next_block_id: module.next_block_id,
        }
    }
}

impl ConstantFolder {
    /// Performs constant folding within a function
    fn fold_constants_in_function(func: IrFunction) -> IrFunction {
        let blocks = func
            .blocks
            .into_iter()
            .map(|block| Self::fold_constants_in_block(block))
            .collect();

        IrFunction {
            name: func.name,
            params: func.params,
            return_type: func.return_type,
            blocks,
            entry_block: func.entry_block,
            locals: func.locals,
            is_external: func.is_external,
            is_exported: func.is_exported,
        }
    }

    /// Performs constant folding within a basic block
    fn fold_constants_in_block(mut block: IrBlock) -> IrBlock {
        let mut const_values: HashMap<u32, i64> = HashMap::new();

        block.instructions = block
            .instructions
            .into_iter()
            .filter_map(|instr| match &instr {
                IrInstruction::LoadConst { dest, value } => {
                    if let IrValue::Constant(IrConstant::I64(n)) = value {
                        const_values.insert(*dest, *n);
                    }
                    Some(instr)
                }
                IrInstruction::Add { dest, lhs, rhs } => {
                    let lhs_const = const_values.get(lhs).copied();
                    let rhs_const = const_values.get(rhs).copied();
                    if let (Some(lhs_val), Some(rhs_val)) = (lhs_const, rhs_const) {
                        const_values.insert(*dest, lhs_val + rhs_val);
                        Some(IrInstruction::LoadConst {
                            dest: *dest,
                            value: IrValue::Constant(IrConstant::I64(lhs_val + rhs_val)),
                        })
                    } else {
                        Some(instr)
                    }
                }
                _ => Some(instr),
            })
            .collect();

        block
    }
}
