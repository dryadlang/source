// crates/dryad_aot/src/backend/liveness.rs
//! Liveness analysis for register allocation
//!
//! Determines which virtual registers are live at each instruction.

use crate::ir::*;
use std::collections::HashSet;

/// Computes live ranges for all virtual registers in a function
pub struct LivenessAnalyzer;

impl LivenessAnalyzer {
    /// Analyze function and return live ranges for each virtual register
    pub fn analyze(func: &IrFunction) -> Vec<crate::backend::register_allocator::LiveRange> {
        let mut ranges = Vec::new();
        let mut first_use: std::collections::HashMap<RegisterId, usize> =
            std::collections::HashMap::new();
        let mut last_use: std::collections::HashMap<RegisterId, usize> =
            std::collections::HashMap::new();

        let mut instruction_index = 0;

        for block in &func.blocks {
            for instr in &block.instructions {
                LivenessAnalyzer::scan_instruction(
                    instr,
                    instruction_index,
                    &mut first_use,
                    &mut last_use,
                );
                instruction_index += 1;
            }

            LivenessAnalyzer::scan_terminator(
                &block.terminator,
                instruction_index,
                &mut first_use,
                &mut last_use,
            );
            instruction_index += 1;
        }

        for (vreg, first) in first_use {
            let last = last_use.get(&vreg).copied().unwrap_or(first);
            ranges.push(crate::backend::register_allocator::LiveRange::new(
                vreg, first, last,
            ));
        }

        ranges
    }

    fn scan_instruction(
        instr: &IrInstruction,
        index: usize,
        first_use: &mut std::collections::HashMap<RegisterId, usize>,
        last_use: &mut std::collections::HashMap<RegisterId, usize>,
    ) {
        match instr {
            IrInstruction::LoadConst { dest, .. } => {
                first_use.entry(*dest).or_insert(index);
                last_use.insert(*dest, index);
            }
            IrInstruction::Move { dest, src } => {
                first_use.entry(*src).or_insert(index);
                last_use.insert(*src, index);
                first_use.entry(*dest).or_insert(index);
                last_use.insert(*dest, index);
            }
            IrInstruction::Load { dest, ptr } => {
                first_use.entry(*ptr).or_insert(index);
                last_use.insert(*ptr, index);
                first_use.entry(*dest).or_insert(index);
                last_use.insert(*dest, index);
            }
            IrInstruction::Store { ptr, value } => {
                first_use.entry(*ptr).or_insert(index);
                last_use.insert(*ptr, index);
                first_use.entry(*value).or_insert(index);
                last_use.insert(*value, index);
            }
            IrInstruction::Add { dest, lhs, rhs }
            | IrInstruction::Sub { dest, lhs, rhs }
            | IrInstruction::Mul { dest, lhs, rhs }
            | IrInstruction::Div { dest, lhs, rhs }
            | IrInstruction::Mod { dest, lhs, rhs }
            | IrInstruction::CmpEq { dest, lhs, rhs }
            | IrInstruction::CmpNe { dest, lhs, rhs }
            | IrInstruction::CmpLt { dest, lhs, rhs }
            | IrInstruction::CmpLe { dest, lhs, rhs }
            | IrInstruction::CmpGt { dest, lhs, rhs }
            | IrInstruction::CmpGe { dest, lhs, rhs }
            | IrInstruction::And { dest, lhs, rhs }
            | IrInstruction::Or { dest, lhs, rhs }
            | IrInstruction::Xor { dest, lhs, rhs }
            | IrInstruction::Shl { dest, lhs, rhs }
            | IrInstruction::Shr { dest, lhs, rhs } => {
                first_use.entry(*lhs).or_insert(index);
                last_use.insert(*lhs, index);
                first_use.entry(*rhs).or_insert(index);
                last_use.insert(*rhs, index);
                first_use.entry(*dest).or_insert(index);
                last_use.insert(*dest, index);
            }
            IrInstruction::Neg { dest, src } | IrInstruction::Not { dest, src } => {
                first_use.entry(*src).or_insert(index);
                last_use.insert(*src, index);
                first_use.entry(*dest).or_insert(index);
                last_use.insert(*dest, index);
            }
            _ => {}
        }
    }

    fn scan_terminator(
        term: &IrTerminator,
        index: usize,
        first_use: &mut std::collections::HashMap<RegisterId, usize>,
        last_use: &mut std::collections::HashMap<RegisterId, usize>,
    ) {
        match term {
            IrTerminator::Return(Some(reg)) => {
                first_use.entry(*reg).or_insert(index);
                last_use.insert(*reg, index);
            }
            IrTerminator::Branch { cond, .. } => {
                first_use.entry(*cond).or_insert(index);
                last_use.insert(*cond, index);
            }
            IrTerminator::Throw(reg) => {
                first_use.entry(*reg).or_insert(index);
                last_use.insert(*reg, index);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_load_const() {
        let mut func = IrFunction::new("test", IrType::I32);
        let block = IrBlock::new(0);
        func.entry_block = 0;
        func.add_block(block);

        if let Some(block_mut) = func.get_block_mut(0) {
            block_mut.add_instruction(IrInstruction::LoadConst {
                dest: 0,
                value: IrValue::Constant(IrConstant::I32(42)),
            });
            block_mut.set_terminator(IrTerminator::Return(Some(0)));
        }

        let ranges = LivenessAnalyzer::analyze(&func);

        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0].vreg, 0);
    }

    #[test]
    fn test_multiple_registers() {
        let mut func = IrFunction::new("test", IrType::I32);
        let block = IrBlock::new(0);
        func.entry_block = 0;
        func.add_block(block);

        if let Some(block_mut) = func.get_block_mut(0) {
            block_mut.add_instruction(IrInstruction::LoadConst {
                dest: 0,
                value: IrValue::Constant(IrConstant::I32(1)),
            });
            block_mut.add_instruction(IrInstruction::LoadConst {
                dest: 1,
                value: IrValue::Constant(IrConstant::I32(2)),
            });
            block_mut.add_instruction(IrInstruction::Add {
                dest: 2,
                lhs: 0,
                rhs: 1,
            });
            block_mut.set_terminator(IrTerminator::Return(Some(2)));
        }

        let ranges = LivenessAnalyzer::analyze(&func);

        assert!(ranges.iter().any(|r| r.vreg == 0));
        assert!(ranges.iter().any(|r| r.vreg == 1));
        assert!(ranges.iter().any(|r| r.vreg == 2));
    }
}
