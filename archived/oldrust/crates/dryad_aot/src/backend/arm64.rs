// crates/dryad_aot/src/backend/arm64.rs
//! Backend ARM64
//!
//! Gera código de máquina ARM64 a partir da IR.

use super::{
    liveness::LivenessAnalyzer,
    register_allocator::{AllocationResult, LinearScanAllocator, PhysicalReg},
    Backend,
};
use crate::ir::*;
use std::collections::HashMap;

/// ARM64 Register Encoding (0-30 = X0-X30, 31 = SP/ZR)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Arm64Reg {
    X0,
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    X8,
    X9,
    X10,
    X11,
    X12,
    X13,
    X14,
    X15,
    X16,
    X17,
    X18,
    X19,
    X20,
    X21,
    X22,
    X23,
    X24,
    X25,
    X26,
    X27,
    X28,
    X29,
    X30,
    SP,
    ZR,
}

impl Arm64Reg {
    fn encoding(&self) -> u8 {
        match self {
            Arm64Reg::X0 => 0,
            Arm64Reg::X1 => 1,
            Arm64Reg::X2 => 2,
            Arm64Reg::X3 => 3,
            Arm64Reg::X4 => 4,
            Arm64Reg::X5 => 5,
            Arm64Reg::X6 => 6,
            Arm64Reg::X7 => 7,
            Arm64Reg::X8 => 8,
            Arm64Reg::X9 => 9,
            Arm64Reg::X10 => 10,
            Arm64Reg::X11 => 11,
            Arm64Reg::X12 => 12,
            Arm64Reg::X13 => 13,
            Arm64Reg::X14 => 14,
            Arm64Reg::X15 => 15,
            Arm64Reg::X16 => 16,
            Arm64Reg::X17 => 17,
            Arm64Reg::X18 => 18,
            Arm64Reg::X19 => 19,
            Arm64Reg::X20 => 20,
            Arm64Reg::X21 => 21,
            Arm64Reg::X22 => 22,
            Arm64Reg::X23 => 23,
            Arm64Reg::X24 => 24,
            Arm64Reg::X25 => 25,
            Arm64Reg::X26 => 26,
            Arm64Reg::X27 => 27,
            Arm64Reg::X28 => 28,
            Arm64Reg::X29 => 29,
            Arm64Reg::X30 => 30,
            Arm64Reg::SP => 31,
            Arm64Reg::ZR => 31,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CallingConvention {
    /// ARM64 EABI (Linux, macOS)
    SystemV,
}

/// Backend para ARM64
pub struct Arm64Backend {
    /// Convenção de chamada
    calling_conv: CallingConvention,
}

impl Arm64Backend {
    pub fn new() -> Self {
        Self {
            calling_conv: CallingConvention::SystemV,
        }
    }

    /// Define a convenção de chamada
    pub fn with_calling_conv(mut self, conv: CallingConvention) -> Self {
        self.calling_conv = conv;
        self
    }

    /// Compila uma função
    fn compile_function(&self, func: &IrFunction) -> Result<Vec<u8>, String> {
        let live_ranges = LivenessAnalyzer::analyze(func);
        let allocation = LinearScanAllocator::allocate(&live_ranges);

        let mut codegen = Arm64Codegen::new(self.calling_conv, allocation);

        // Prologue
        let locals_size = self.calculate_locals_size(func);
        let total_stack_needed = locals_size + codegen.alloc.total_spill_size as u32;
        let aligned_stack = if total_stack_needed % 16 != 0 {
            total_stack_needed + (16 - (total_stack_needed % 16))
        } else {
            total_stack_needed
        };

        codegen.emit_prologue(aligned_stack);

        // Compilar cada bloco
        for block in &func.blocks {
            self.compile_block(block, &mut codegen)?;
        }

        // Resolver saltos após compilar todos os blocos
        codegen.resolve_labels();

        // Epilogue
        codegen.emit_epilogue();

        Ok(codegen.finish())
    }

    /// Compila um bloco básico
    fn compile_block(&self, block: &IrBlock, codegen: &mut Arm64Codegen) -> Result<(), String> {
        // Label do bloco
        codegen.emit_label(block.id);

        // Compilar instruções
        for instr in &block.instructions {
            self.compile_instruction(instr, codegen)?;
        }

        // Compilar terminador
        self.compile_terminator(&block.terminator, codegen)?;

        Ok(())
    }

    /// Compila uma instrução
    fn compile_instruction(
        &self,
        instr: &IrInstruction,
        codegen: &mut Arm64Codegen,
    ) -> Result<(), String> {
        match instr {
            IrInstruction::LoadConst { dest, value } => {
                let dest_reg = codegen.get_phys_reg(*dest)?;
                match value {
                    IrValue::Constant(IrConstant::I32(n)) => {
                        codegen.emit_mov_imm64(dest_reg, *n as i64);
                    }
                    IrValue::Constant(IrConstant::I64(n)) => {
                        codegen.emit_mov_imm64(dest_reg, *n);
                    }
                    _ => return Err(format!("Constante não suportada: {:?}", value)),
                }
            }

            IrInstruction::Add { dest, lhs, rhs } => {
                let dest_reg = codegen.get_phys_reg(*dest)?;
                let lhs_reg = codegen.get_phys_reg(*lhs)?;
                let rhs_reg = codegen.get_phys_reg(*rhs)?;

                if dest_reg != lhs_reg {
                    codegen.emit_mov_reg_reg(dest_reg, lhs_reg);
                }
                codegen.emit_add_reg_reg(dest_reg, dest_reg, rhs_reg);
            }

            IrInstruction::Sub { dest, lhs, rhs } => {
                let dest_reg = codegen.get_phys_reg(*dest)?;
                let lhs_reg = codegen.get_phys_reg(*lhs)?;
                let rhs_reg = codegen.get_phys_reg(*rhs)?;

                if dest_reg != lhs_reg {
                    codegen.emit_mov_reg_reg(dest_reg, lhs_reg);
                }
                codegen.emit_sub_reg_reg(dest_reg, dest_reg, rhs_reg);
            }

            IrInstruction::Mul { dest, lhs, rhs } => {
                let dest_reg = codegen.get_phys_reg(*dest)?;
                let lhs_reg = codegen.get_phys_reg(*lhs)?;
                let rhs_reg = codegen.get_phys_reg(*rhs)?;

                if dest_reg != lhs_reg {
                    codegen.emit_mov_reg_reg(dest_reg, lhs_reg);
                }
                codegen.emit_mul_reg_reg(dest_reg, dest_reg, rhs_reg);
            }

            IrInstruction::CmpEq { dest, lhs, rhs } => {
                let dest_reg = codegen.get_phys_reg(*dest)?;
                let lhs_reg = codegen.get_phys_reg(*lhs)?;
                let rhs_reg = codegen.get_phys_reg(*rhs)?;

                codegen.emit_cmp_reg_reg(lhs_reg, rhs_reg);
                codegen.emit_cset(dest_reg, 0); // 0 = EQ condition
            }

            IrInstruction::Div { dest, lhs, rhs } => {
                let dest_reg = codegen.get_phys_reg(*dest)?;
                let lhs_reg = codegen.get_phys_reg(*lhs)?;
                let rhs_reg = codegen.get_phys_reg(*rhs)?;

                if dest_reg != lhs_reg {
                    codegen.emit_mov_reg_reg(dest_reg, lhs_reg);
                }
                codegen.emit_sdiv_reg_reg(dest_reg, dest_reg, rhs_reg);
            }

            IrInstruction::Mod { dest, lhs, rhs } => {
                let dest_reg = codegen.get_phys_reg(*dest)?;
                let lhs_reg = codegen.get_phys_reg(*lhs)?;
                let rhs_reg = codegen.get_phys_reg(*rhs)?;

                if dest_reg != lhs_reg {
                    codegen.emit_mov_reg_reg(dest_reg, lhs_reg);
                }
                // Compute remainder: a - (a / b) * b
                codegen.emit_sdiv_reg_reg(1, dest_reg, rhs_reg); // X1 = lhs / rhs
                codegen.emit_mul_reg_reg(1, 1, rhs_reg); // X1 = (lhs / rhs) * rhs
                codegen.emit_sub_reg_reg(dest_reg, dest_reg, 1); // dest = lhs - X1
            }

            _ => {
                return Err(format!("Instrução não suportada: {:?}", instr));
            }
        }

        Ok(())
    }

    /// Compila um terminador
    fn compile_terminator(
        &self,
        term: &IrTerminator,
        codegen: &mut Arm64Codegen,
    ) -> Result<(), String> {
        match term {
            IrTerminator::Return(reg) => {
                if let Some(r) = reg {
                    let ret_reg = codegen.get_phys_reg(*r)?;
                    codegen.emit_mov_reg_reg(0, ret_reg);
                }
                codegen.emit_epilogue();
                Ok(())
            }

            IrTerminator::Jump(block_id) => {
                codegen.emit_b(*block_id);
                Ok(())
            }

            IrTerminator::Branch {
                cond,
                then_block,
                else_block,
            } => {
                let cond_reg = codegen.get_phys_reg(*cond)?;
                codegen.emit_cmp_reg_reg(cond_reg, cond_reg);
                codegen.emit_beq(*then_block);
                codegen.emit_b(*else_block);
                Ok(())
            }

            _ => Err(format!("Terminador não suportado: {:?}", term)),
        }
    }

    /// Calcula o espaço necessário para variáveis locais
    fn calculate_locals_size(&self, func: &IrFunction) -> u32 {
        func.locals.iter().map(|l| l.ty.size()).sum::<u32>()
    }
}

impl Backend for Arm64Backend {
    fn compile_module(&self, module: &IrModule) -> Result<Vec<u8>, String> {
        let mut object_code = Vec::new();

        for func in &module.functions {
            let func_code = self.compile_function(func)?;
            object_code.extend(func_code);
        }

        Ok(object_code)
    }

    fn name(&self) -> &'static str {
        "arm64"
    }

    fn target_triple(&self) -> &'static str {
        "aarch64-unknown-linux-gnu"
    }
}

/// Gerador de código ARM64
struct Arm64Codegen {
    /// Bytes de código gerados
    code: Vec<u8>,

    /// Convenção de chamada
    calling_conv: CallingConvention,

    /// Resultado de alocação de registradores
    alloc: AllocationResult,

    /// Mapeamento de RegisterId para PhysicalReg encoding
    reg_map: HashMap<RegisterId, u8>,

    /// Mapeamento de BlockId para posição de código
    label_positions: HashMap<BlockId, usize>,

    /// Saltos pendentes: (posição do offset, tamanho, BlockId destino)
    pending_jumps: Vec<(usize, usize, BlockId)>,
}

impl Arm64Codegen {
    fn new(calling_conv: CallingConvention, alloc: AllocationResult) -> Self {
        let mut reg_map = HashMap::new();

        for (vreg, phys_opt) in &alloc.alloc {
            if let Some(phys) = phys_opt {
                reg_map.insert(*vreg, phys.encoding());
            }
        }

        Self {
            code: Vec::new(),
            calling_conv,
            alloc,
            reg_map,
            label_positions: HashMap::new(),
            pending_jumps: Vec::new(),
        }
    }

    #[cfg(test)]
    fn new_for_test() -> Self {
        Self {
            code: Vec::new(),
            calling_conv: CallingConvention::SystemV,
            alloc: AllocationResult {
                alloc: HashMap::new(),
                spill_offsets: HashMap::new(),
                total_spill_size: 0,
            },
            reg_map: HashMap::new(),
            label_positions: HashMap::new(),
            pending_jumps: Vec::new(),
        }
    }

    /// Maps virtual register to physical register encoding
    fn get_phys_reg(&self, vreg: RegisterId) -> Result<u8, String> {
        self.reg_map
            .get(&vreg)
            .copied()
            .ok_or_else(|| format!("Virtual register {} not allocated", vreg))
    }

    // Instruction encoding helpers

    fn emit_u32_le(&mut self, instr: u32) {
        self.code.extend(&instr.to_le_bytes());
    }

    fn emit_mov_imm64(&mut self, rd: u8, imm: i64) {
        let rd = rd & 0x1F;

        // MOVZ X<rd>, imm16, LSL 0
        let instr0 = 0xD2800000 | ((imm as u32 & 0xFFFF) << 5) | rd as u32;
        self.emit_u32_le(instr0);

        // MOVK X<rd>, imm16, LSL 16
        let imm16_1 = ((imm as u32 >> 16) & 0xFFFF);
        let instr1 = 0xF2A00000 | (imm16_1 << 5) | rd as u32;
        self.emit_u32_le(instr1);
    }

    fn emit_mov_reg_reg(&mut self, rd: u8, rm: u8) {
        let rd = rd & 0x1F;
        let rm = rm & 0x1F;

        // ORR X<rd>, XZR, X<rm> (same as MOV)
        let instr = 0xAA000000 | ((rm as u32) << 16) | rd as u32;
        self.emit_u32_le(instr);
    }

    fn emit_add_reg_reg(&mut self, rd: u8, rn: u8, rm: u8) {
        let rd = rd & 0x1F;
        let rn = rn & 0x1F;
        let rm = rm & 0x1F;

        let instr = 0x8B000000 | ((rm as u32) << 16) | ((rn as u32) << 5) | rd as u32;
        self.emit_u32_le(instr);
    }

    fn emit_sub_reg_reg(&mut self, rd: u8, rn: u8, rm: u8) {
        let rd = rd & 0x1F;
        let rn = rn & 0x1F;
        let rm = rm & 0x1F;

        let instr = 0xCB000000 | ((rm as u32) << 16) | ((rn as u32) << 5) | rd as u32;
        self.emit_u32_le(instr);
    }

    fn emit_mul_reg_reg(&mut self, rd: u8, rn: u8, rm: u8) {
        let rd = rd & 0x1F;
        let rn = rn & 0x1F;
        let rm = rm & 0x1F;

        let instr = 0x9B007C00 | ((rm as u32) << 16) | ((rn as u32) << 5) | rd as u32;
        self.emit_u32_le(instr);
    }

    fn emit_sdiv_reg_reg(&mut self, rd: u8, rn: u8, rm: u8) {
        let rd = rd & 0x1F;
        let rn = rn & 0x1F;
        let rm = rm & 0x1F;

        let instr = 0x9AC00C00 | ((rm as u32) << 16) | ((rn as u32) << 5) | rd as u32;
        self.emit_u32_le(instr);
    }

    fn emit_cmp_reg_reg(&mut self, rn: u8, rm: u8) {
        let rn = rn & 0x1F;
        let rm = rm & 0x1F;

        let instr = 0xEB000000 | ((rm as u32) << 16) | ((rn as u32) << 5) | 31u32;
        self.emit_u32_le(instr);
    }

    fn emit_cset(&mut self, rd: u8, cond: u8) {
        let rd = rd & 0x1F;
        let cond = cond & 0x0F;

        // CSET X<rd>, <cond> is equivalent to CSINC X<rd>, XZR, XZR, <inv_cond>
        let instr = 0x9A9F07E0 | ((rd as u32) << 0) | ((cond as u32) << 12);
        self.emit_u32_le(instr);
    }

    fn emit_b(&mut self, block_id: BlockId) {
        let placeholder_offset = self.code.len();
        self.code.extend(&[0x00, 0x00, 0x00, 0x00]);
        self.record_pending_jump(block_id, placeholder_offset, 4);
    }

    fn emit_beq(&mut self, block_id: BlockId) {
        let placeholder_offset = self.code.len();
        self.code.extend(&[0x00, 0x00, 0x00, 0x01]);
        self.record_pending_jump(block_id, placeholder_offset, 4);
    }

    fn emit_nop(&mut self) {
        // NOP = 0xD503201F
        self.emit_u32_le(0xD503201F);
    }

    fn emit_label(&mut self, block_id: BlockId) {
        self.record_label(block_id);
    }

    fn emit_prologue(&mut self, stack_size: u32) {
        // STP X29, X30, [SP, -stack_size]! (save frame pointer and link register)
        let imm = ((stack_size / 8) as i32 - 1) & 0x7F;
        let instr = 0xA9800000 | ((imm as u32 & 0x7F) << 15) | (29u32 << 10) | (30u32 << 5);
        self.emit_u32_le(instr);

        // MOV X29, SP
        self.emit_mov_reg_reg(29, 31);
    }

    fn emit_epilogue(&mut self) {
        // LDP X29, X30, [SP], stack_size (restore frame pointer and link register)
        let instr = 0xA8C07FBD;
        self.emit_u32_le(instr);

        // RET
        self.emit_ret();
    }

    fn emit_ret(&mut self) {
        // RET (return to X30)
        let instr = 0xD65F03C0;
        self.emit_u32_le(instr);
    }

    fn record_label(&mut self, block_id: BlockId) {
        self.label_positions.insert(block_id, self.code.len());
    }

    fn record_pending_jump(&mut self, block_id: BlockId, offset: usize, size: usize) {
        self.pending_jumps.push((offset, size, block_id));
    }

    fn resolve_labels(&mut self) {
        for (offset, size, target_block) in self.pending_jumps.drain(..) {
            if let Some(&target_pos) = self.label_positions.get(&target_block) {
                let current_pos = offset + size;
                let delta = (target_pos as i32 - current_pos as i32) >> 2; // ARM64 offsets are 4-byte aligned

                if delta >= -(1 << 25) && delta < (1 << 25) {
                    let delta_u32 = (delta as u32) & 0x03FFFFFF;
                    let instr = 0x14000000 | delta_u32;

                    let delta_bytes = instr.to_le_bytes();
                    for (i, &byte) in delta_bytes.iter().enumerate() {
                        self.code[offset + i] = byte;
                    }
                }
            }
        }
    }

    fn finish(self) -> Vec<u8> {
        self.code
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mov_imm64() {
        let mut codegen = Arm64Codegen::new_for_test();

        codegen.emit_mov_imm64(0, 42);

        assert!(codegen.code.len() >= 8);
    }

    #[test]
    fn test_add_reg_reg() {
        let mut codegen = Arm64Codegen::new_for_test();

        codegen.emit_add_reg_reg(0, 1, 2);

        assert_eq!(codegen.code.len(), 4);
    }

    #[test]
    fn test_label_and_branch() {
        let mut codegen = Arm64Codegen::new_for_test();

        codegen.emit_b(0);
        codegen.emit_nop();
        let _ = codegen.code.len();
        codegen.emit_label(0);

        codegen.resolve_labels();

        assert!(codegen.code.len() > 0);
    }

    #[test]
    fn test_prologue_epilogue() {
        let mut codegen = Arm64Codegen::new_for_test();

        codegen.emit_prologue(16);
        let prologue_size = codegen.code.len();

        codegen.emit_epilogue();

        assert!(prologue_size > 0);
        assert!(codegen.code.len() > prologue_size);
    }
}
