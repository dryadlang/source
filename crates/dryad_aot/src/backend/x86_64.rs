// crates/dryad_aot/src/backend/x86_64.rs
//! Backend x86_64
//!
//! Gera código de máquina x86_64 a partir da IR.

use super::{
    liveness::LivenessAnalyzer,
    register_allocator::{AllocationResult, LinearScanAllocator, PhysicalReg},
    Backend,
};
use crate::ir::*;
use std::collections::HashMap;

/// Backend para x86_64
pub struct X86_64Backend {
    /// Convenção de chamada
    calling_conv: CallingConvention,
}

#[derive(Debug, Clone, Copy)]
pub enum CallingConvention {
    /// System V AMD64 ABI (Linux, macOS)
    SystemV,
    /// Windows x64 calling convention
    Windows,
}

impl X86_64Backend {
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

        let mut codegen = X86_64Codegen::new(self.calling_conv, allocation);

        // Prologue
        codegen.emit_push_rbp();
        codegen.emit_mov_rbp_rsp();

        // Alocar stack para variáveis locais + spills
        let locals_size = self.calculate_locals_size(func);
        let total_stack_needed = locals_size + codegen.alloc.total_spill_size as u32;
        if total_stack_needed > 0 {
            codegen.emit_sub_rsp(total_stack_needed);
        }

        // Compilar cada bloco
        for block in &func.blocks {
            self.compile_block(block, &mut codegen)?;
        }

        // Resolver saltos após compilar todos os blocos
        codegen.resolve_labels();

        // Epilogue (caso não tenha ret explícito)
        codegen.emit_mov_rsp_rbp();
        codegen.emit_pop_rbp();
        codegen.emit_ret();

        Ok(codegen.finish())
    }

    /// Compila um bloco básico
    fn compile_block(&self, block: &IrBlock, codegen: &mut X86_64Codegen) -> Result<(), String> {
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
        codegen: &mut X86_64Codegen,
    ) -> Result<(), String> {
        match instr {
            IrInstruction::LoadConst { dest, value } => {
                let dest_reg = codegen.get_phys_reg(*dest)?;
                match value {
                    IrValue::Constant(IrConstant::I32(n)) => {
                        codegen.emit_mov_imm32(dest_reg, *n);
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

                codegen.emit_mov_reg_reg(0, lhs_reg); // rax = lhs
                codegen.emit_add_reg_reg(0, rhs_reg); // rax += rhs
                codegen.emit_mov_reg_reg(dest_reg, 0); // dest = rax
            }

            IrInstruction::Sub { dest, lhs, rhs } => {
                let dest_reg = codegen.get_phys_reg(*dest)?;
                let lhs_reg = codegen.get_phys_reg(*lhs)?;
                let rhs_reg = codegen.get_phys_reg(*rhs)?;

                codegen.emit_mov_reg_reg(0, lhs_reg);
                codegen.emit_sub_reg_reg(0, rhs_reg);
                codegen.emit_mov_reg_reg(dest_reg, 0);
            }

            IrInstruction::Mul { dest, lhs, rhs } => {
                let dest_reg = codegen.get_phys_reg(*dest)?;
                let lhs_reg = codegen.get_phys_reg(*lhs)?;
                let rhs_reg = codegen.get_phys_reg(*rhs)?;

                codegen.emit_mov_reg_reg(0, lhs_reg);
                codegen.emit_imul_reg_reg(0, rhs_reg);
                codegen.emit_mov_reg_reg(dest_reg, 0);
            }

            IrInstruction::CmpEq { dest, lhs, rhs } => {
                let dest_reg = codegen.get_phys_reg(*dest)?;
                let lhs_reg = codegen.get_phys_reg(*lhs)?;
                let rhs_reg = codegen.get_phys_reg(*rhs)?;

                codegen.emit_mov_reg_reg(0, lhs_reg);
                codegen.emit_cmp_reg_reg(0, rhs_reg);
                codegen.emit_sete(dest_reg);
            }

            _ => {
                return Err(format!("Instrução não suportada: {:?}", instr));
            }
        }

        Ok(())
    }

    /// Compila um terminador de bloco
    fn compile_terminator(
        &self,
        term: &IrTerminator,
        codegen: &mut X86_64Codegen,
    ) -> Result<(), String> {
        match term {
            IrTerminator::Return(reg) => {
                if let Some(r) = reg {
                    let ret_reg = codegen.get_phys_reg(*r)?;
                    codegen.emit_mov_reg_reg(0, ret_reg); // rax = return value
                }
                // Epilogue
                codegen.emit_mov_rsp_rbp();
                codegen.emit_pop_rbp();
                codegen.emit_ret();
            }

            IrTerminator::Jump(block_id) => {
                codegen.emit_jmp(*block_id);
            }

            IrTerminator::Branch {
                cond,
                then_block,
                else_block,
            } => {
                let cond_reg = codegen.get_phys_reg(*cond)?;
                codegen.emit_test_reg_reg(cond_reg, cond_reg);
                codegen.emit_jz(*else_block);
                codegen.emit_jmp(*then_block);
            }

            _ => {
                return Err(format!("Terminator não suportado: {:?}", term));
            }
        }

        Ok(())
    }

    /// Calcula o espaço necessário para variáveis locais
    fn calculate_locals_size(&self, func: &IrFunction) -> u32 {
        func.locals.iter().map(|l| l.ty.size()).sum::<u32>()
    }
}

impl Backend for X86_64Backend {
    fn compile_module(&self, module: &IrModule) -> Result<Vec<u8>, String> {
        let mut object_code = Vec::new();

        for func in &module.functions {
            let func_code = self.compile_function(func)?;
            object_code.extend(func_code);
        }

        Ok(object_code)
    }

    fn name(&self) -> &'static str {
        "x86_64"
    }

    fn target_triple(&self) -> &'static str {
        match self.calling_conv {
            CallingConvention::SystemV => "x86_64-unknown-linux-gnu",
            CallingConvention::Windows => "x86_64-pc-windows-gnu",
        }
    }
}

/// Gerador de código x86_64
struct X86_64Codegen {
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

impl X86_64Codegen {
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

    // Instruções básicas

    fn emit_push_rbp(&mut self) {
        // push rbp (0x55)
        self.code.push(0x55);
    }

    fn emit_pop_rbp(&mut self) {
        // pop rbp (0x5D)
        self.code.push(0x5D);
    }

    fn emit_mov_rbp_rsp(&mut self) {
        // mov rbp, rsp (0x48 0x89 0xE5)
        self.code.extend(&[0x48, 0x89, 0xE5]);
    }

    fn emit_mov_rsp_rbp(&mut self) {
        // mov rsp, rbp (0x48 0x89 0xEC)
        self.code.extend(&[0x48, 0x89, 0xEC]);
    }

    fn emit_ret(&mut self) {
        // ret (0xC3)
        self.code.push(0xC3);
    }

    fn emit_sub_rsp(&mut self, amount: u32) {
        // sub rsp, imm32
        // 0x48 0x81 0xEC imm32
        self.code.extend(&[0x48, 0x81, 0xEC]);
        self.code.extend(&amount.to_le_bytes());
    }

    fn emit_mov_reg_reg(&mut self, dest: u8, src: u8) {
        // mov r64, r64
        // REX.W (0x48) + 0x89 + ModRM
        let modrm = 0xC0 | ((src & 7) << 3) | (dest & 7);
        let rex = 0x48 | ((src >> 3) & 1) | (((dest >> 3) & 1) << 2);
        self.code.push(rex);
        self.code.push(0x89);
        self.code.push(modrm);
    }

    fn emit_mov_imm32(&mut self, reg: u8, value: i32) {
        // mov r32, imm32
        // 0xB8+rd imm32
        let opcode = 0xB8 + (reg & 7);
        let rex = 0x40 | ((reg >> 3) & 1);
        self.code.push(rex);
        self.code.push(opcode);
        self.code.extend(&value.to_le_bytes());
    }

    fn emit_mov_imm64(&mut self, reg: u8, value: i64) {
        // mov r64, imm64
        // REX.W (0x48) + 0xB8+rd imm64
        let opcode = 0xB8 + (reg & 7);
        let rex = 0x48 | ((reg >> 3) & 1);
        self.code.push(rex);
        self.code.push(opcode);
        self.code.extend(&value.to_le_bytes());
    }

    fn emit_add_reg_reg(&mut self, dest: u8, src: u8) {
        // add r64, r64
        let modrm = 0xC0 | ((src & 7) << 3) | (dest & 7);
        let rex = 0x48 | ((src >> 3) & 1) | (((dest >> 3) & 1) << 2);
        self.code.push(rex);
        self.code.push(0x01);
        self.code.push(modrm);
    }

    fn emit_sub_reg_reg(&mut self, dest: u8, src: u8) {
        // sub r64, r64
        let modrm = 0xC0 | ((src & 7) << 3) | (dest & 7);
        let rex = 0x48 | ((src >> 3) & 1) | (((dest >> 3) & 1) << 2);
        self.code.push(rex);
        self.code.push(0x29);
        self.code.push(modrm);
    }

    fn emit_imul_reg_reg(&mut self, dest: u8, src: u8) {
        // imul r64, r64
        let modrm = 0xC0 | ((dest & 7) << 3) | (src & 7);
        let rex = 0x48 | ((dest >> 3) & 1) | (((src >> 3) & 1) << 2);
        self.code.push(rex);
        self.code.push(0x0F);
        self.code.push(0xAF);
        self.code.push(modrm);
    }

    fn emit_cmp_reg_reg(&mut self, lhs: u8, rhs: u8) {
        // cmp r64, r64
        let modrm = 0xC0 | ((rhs & 7) << 3) | (lhs & 7);
        let rex = 0x48 | ((rhs >> 3) & 1) | (((lhs >> 3) & 1) << 2);
        self.code.push(rex);
        self.code.push(0x39);
        self.code.push(modrm);
    }

    fn emit_test_reg_reg(&mut self, reg1: u8, reg2: u8) {
        // test r64, r64
        let modrm = 0xC0 | ((reg2 & 7) << 3) | (reg1 & 7);
        let rex = 0x48 | ((reg2 >> 3) & 1) | (((reg1 >> 3) & 1) << 2);
        self.code.push(rex);
        self.code.push(0x85);
        self.code.push(modrm);
    }

    fn emit_sete(&mut self, reg: u8) {
        // sete r8
        // 0x0F 0x94 /r
        let modrm = 0xC0 | (reg & 7);
        let rex = 0x40 | ((reg >> 3) & 1);
        self.code.push(rex);
        self.code.push(0x0F);
        self.code.push(0x94);
        self.code.push(modrm);
    }

    fn emit_jmp(&mut self, block_id: BlockId) {
        let offset = self.code.len();
        self.code.push(0xE9);
        let placeholder_offset = self.code.len();
        self.code.extend(&[0x00, 0x00, 0x00, 0x00]);
        self.record_pending_jump(block_id, placeholder_offset, 4);
    }

    fn emit_jz(&mut self, block_id: BlockId) {
        self.code.push(0x0F);
        self.code.push(0x84);
        let placeholder_offset = self.code.len();
        self.code.extend(&[0x00, 0x00, 0x00, 0x00]);
        self.record_pending_jump(block_id, placeholder_offset, 4);
    }

    fn emit_label(&mut self, block_id: BlockId) {
        self.record_label(block_id);
    }

    #[cfg(test)]
    fn emit_nop(&mut self) {
        self.code.push(0x90);
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
                let delta = target_pos as i32 - current_pos as i32;

                let delta_bytes = delta.to_le_bytes();
                for (i, &byte) in delta_bytes.iter().enumerate() {
                    self.code[offset + i] = byte;
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
    fn test_label_resolution() {
        let mut codegen = X86_64Codegen::new_for_test();

        // Emit forward jump (target not yet at this position)
        let offset_before_jmp = codegen.code.len();
        codegen.emit_jmp(0); // Jump to label 0

        // Emit code
        codegen.emit_nop();
        codegen.emit_nop();

        // Mark label 0 at this position
        let label_position = codegen.code.len();
        codegen.emit_label(0);

        // Now resolve labels
        codegen.resolve_labels();

        // Extract the 4-byte offset from the jump instruction
        let offset_bytes = &codegen.code[offset_before_jmp + 1..offset_before_jmp + 5];
        let offset_value = i32::from_le_bytes([
            offset_bytes[0],
            offset_bytes[1],
            offset_bytes[2],
            offset_bytes[3],
        ]);

        // Calculate expected offset: target - (jmp_position + jmp_instruction_size)
        let expected_offset = label_position as i32 - (offset_before_jmp as i32 + 5);

        assert_eq!(offset_value, expected_offset);
    }

    #[test]
    fn test_backward_jump_resolution() {
        let mut codegen = X86_64Codegen::new_for_test();

        // Emit some code
        codegen.emit_nop();
        codegen.emit_nop();

        let label_0_pos = codegen.code.len();
        codegen.emit_label(0);

        // Emit more code
        codegen.emit_nop();

        // Emit backward jump to label 0
        let jmp_pos = codegen.code.len();
        codegen.emit_jmp(0);

        codegen.resolve_labels();

        let offset_bytes = &codegen.code[jmp_pos + 1..jmp_pos + 5];
        let offset_value = i32::from_le_bytes([
            offset_bytes[0],
            offset_bytes[1],
            offset_bytes[2],
            offset_bytes[3],
        ]);

        let expected_offset = label_0_pos as i32 - (jmp_pos as i32 + 5);
        assert_eq!(offset_value, expected_offset);
    }

    #[test]
    fn test_conditional_jump_resolution() {
        let mut codegen = X86_64Codegen::new_for_test();

        let jmp_pos = codegen.code.len();
        codegen.emit_jz(0); // jz to label 0

        // Emit nops
        for _ in 0..5 {
            codegen.emit_nop();
        }

        let label_0_pos = codegen.code.len();
        codegen.emit_label(0);

        codegen.resolve_labels();

        let offset_bytes = &codegen.code[jmp_pos + 2..jmp_pos + 6];
        let offset_value = i32::from_le_bytes([
            offset_bytes[0],
            offset_bytes[1],
            offset_bytes[2],
            offset_bytes[3],
        ]);

        let expected_offset = label_0_pos as i32 - (jmp_pos as i32 + 6);
        assert_eq!(offset_value, expected_offset);
    }
}
