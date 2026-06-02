// crates/dryad_aot/src/optimizer/dce.rs
//! Dead Code Elimination
//!
//! Removes unreachable basic blocks using reachability analysis.

use super::Optimizer;
use crate::ir::{IrFunction, IrModule, IrTerminator};
use std::collections::HashSet;

/// Dead Code Eliminator
pub struct DeadCodeEliminator;

impl Optimizer for DeadCodeEliminator {
    fn optimize(&self, module: IrModule) -> IrModule {
        let functions = module
            .functions
            .into_iter()
            .map(|func| Self::eliminate_dead_blocks(func))
            .collect();

        IrModule {
            name: module.name,
            functions,
            globals: module.globals,
            metadata: module.metadata,
            next_register_id: module.next_register_id,
            next_block_id: module.next_block_id,
            locals: module.locals,
            next_local_id: module.next_local_id,
            current_stack_offset: module.current_stack_offset,
        }
    }
}

impl DeadCodeEliminator {
    /// Eliminates unreachable blocks using reachability analysis
    fn eliminate_dead_blocks(func: IrFunction) -> IrFunction {
        if func.blocks.is_empty() {
            return func;
        }

        // Mark reachable blocks using BFS
        let mut reachable = HashSet::new();
        let mut queue = vec![func.blocks[0].id];

        while let Some(block_id) = queue.pop() {
            if reachable.contains(&block_id) {
                continue;
            }
            reachable.insert(block_id);

            if let Some(block) = func.blocks.iter().find(|b| b.id == block_id) {
                match &block.terminator {
                    IrTerminator::Jump(target) => queue.push(*target),
                    IrTerminator::Branch {
                        then_block,
                        else_block,
                        ..
                    } => {
                        queue.push(*then_block);
                        queue.push(*else_block);
                    }
                    IrTerminator::Return(_) => {}
                    _ => {}
                }
            }
        }

        // Filter to keep only reachable blocks
        let blocks = func
            .blocks
            .into_iter()
            .filter(|b| reachable.contains(&b.id))
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
}
