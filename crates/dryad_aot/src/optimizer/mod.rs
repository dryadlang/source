// crates/dryad_aot/src/optimizer/mod.rs
//! Otimização de IR
//!
//! Passes de otimização que transformam o IR para melhor performance.

pub mod const_fold;
pub mod dce;

use crate::ir::IrModule;

/// Trait para passes de otimização
pub trait Optimizer {
    /// Otimiza um módulo IR
    fn optimize(&self, module: IrModule) -> IrModule;
}

/// Pipeline de otimização
pub struct OptimizationPipeline {
    passes: Vec<Box<dyn Optimizer>>,
}

impl OptimizationPipeline {
    /// Cria um novo pipeline com todos os passes padrão
    pub fn new() -> Self {
        Self {
            passes: vec![
                Box::new(dce::DeadCodeEliminator),
                Box::new(const_fold::ConstantFolder),
            ],
        }
    }

    /// Executa todos os passes no módulo
    pub fn run(&self, mut module: IrModule) -> IrModule {
        for pass in &self.passes {
            module = pass.optimize(module);
        }
        module
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::*;
    use std::collections::HashMap;

    #[test]
    fn test_optimization_pipeline_runs() {
        let module = IrModule {
            name: "test".to_string(),
            functions: vec![],
            globals: vec![],
            metadata: HashMap::new(),
            next_register_id: 0,
            next_block_id: 0,
        };

        let pipeline = OptimizationPipeline::new();
        let _optimized = pipeline.run(module);
    }

    #[test]
    fn test_dead_code_elimination() {
        use std::collections::HashMap;

        let block1 = IrBlock {
            id: 0,
            instructions: vec![],
            terminator: IrTerminator::Jump(1),
        };

        let block2 = IrBlock {
            id: 1,
            instructions: vec![],
            terminator: IrTerminator::Return(None),
        };

        let func = IrFunction {
            name: "test_func".to_string(),
            params: vec![],
            return_type: IrType::I64,
            blocks: vec![block1, block2],
            entry_block: 0,
            locals: vec![],
            is_external: false,
            is_exported: false,
        };

        let module = IrModule {
            name: "test".to_string(),
            functions: vec![func],
            globals: vec![],
            metadata: HashMap::new(),
            next_register_id: 0,
            next_block_id: 2,
        };

        let pipeline = OptimizationPipeline::new();
        let optimized = pipeline.run(module);

        assert_eq!(optimized.functions[0].blocks.len(), 2);
    }
}
