// crates/dryad_aot/src/backend/x86_64.rs
//! Backend x86_64
//!
//! Gera código de máquina x86_64 a partir da IR.

use super::Backend;
use crate::ir::*;

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
        let mut codegen = X86_64Codegen::new(self.calling_conv);
        
        // Prologue
        codegen.emit_push_rbp();
        codegen.emit_mov_rbp_rsp();
        
        // Alocar stack para variáveis locais
        let locals_size = self.calculate_locals_size(func);
        if locals_size > 0 {
            codegen.emit_sub_rsp(locals_size);
        }
        
        // Compilar cada bloco
        for block in &func.blocks {
            self.compile_block(block, &mut codegen)?;
        }
        
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
    fn compile_instruction(&self, instr: &IrInstruction, codegen: &mut X86_64Codegen) -> Result<(), String> {
        match instr {
            IrInstruction::LoadConst { dest, value } => {
                // TODO: Implementar carregamento de constantes
                match value {
                    IrValue::Constant(IrConstant::I32(n)) => {
                        codegen.emit_mov_imm32(*dest as u8, *n);
                    }
                    IrValue::Constant(IrConstant::I64(n)) => {
                        codegen.emit_mov_imm64(*dest as u8, *n);
                    }
                    _ => return Err(format!("Constante não suportada: {:?}", value)),
                }
            }
            
            IrInstruction::Add { dest, lhs, rhs } => {
                // mov rax, lhs
                // add rax, rhs
                // mov dest, rax
                codegen.emit_mov_reg_reg(0, *lhs as u8);  // rax = lhs
                codegen.emit_add_reg_reg(0, *rhs as u8);   // rax += rhs
                codegen.emit_mov_reg_reg(*dest as u8, 0);  // dest = rax
            }
            
            IrInstruction::Sub { dest, lhs, rhs } => {
                codegen.emit_mov_reg_reg(0, *lhs as u8);
                codegen.emit_sub_reg_reg(0, *rhs as u8);
                codegen.emit_mov_reg_reg(*dest as u8, 0);
            }
            
            IrInstruction::Mul { dest, lhs, rhs } => {
                codegen.emit_mov_reg_reg(0, *lhs as u8);
                codegen.emit_imul_reg_reg(0, *rhs as u8);
                codegen.emit_mov_reg_reg(*dest as u8, 0);
            }
            
            IrInstruction::CmpEq { dest, lhs, rhs } => {
                // cmp lhs, rhs
                // sete dest
                codegen.emit_mov_reg_reg(0, *lhs as u8);
                codegen.emit_cmp_reg_reg(0, *rhs as u8);
                codegen.emit_sete(*dest as u8);
            }
            
            _ => {
                // Outras instruções ainda não implementadas
                return Err(format!("Instrução não suportada: {:?}", instr));
            }
        }
        
        Ok(())
    }
    
    /// Compila um terminador de bloco
    fn compile_terminator(&self, term: &IrTerminator, codegen: &mut X86_64Codegen) -> Result<(), String> {
        match term {
            IrTerminator::Return(reg) => {
                if let Some(r) = reg {
                    // mov rax, reg
                    codegen.emit_mov_reg_reg(0, *r as u8);
                }
                // Epilogue
                codegen.emit_mov_rsp_rbp();
                codegen.emit_pop_rbp();
                codegen.emit_ret();
            }
            
            IrTerminator::Jump(block_id) => {
                codegen.emit_jmp(*block_id);
            }
            
            IrTerminator::Branch { cond, then_block, else_block } => {
                // test cond, cond
                // jz else_block
                // jmp then_block
                codegen.emit_test_reg_reg(*cond as u8, *cond as u8);
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
    
    /// Labels pendentes (para resolver saltos)
    pending_labels: Vec<(BlockId, usize)>,
}

impl X86_64Codegen {
    fn new(calling_conv: CallingConvention) -> Self {
        Self {
            code: Vec::new(),
            calling_conv,
            pending_labels: Vec::new(),
        }
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
    
    fn emit_jmp(&mut self, _block_id: BlockId) {
        // jmp rel32
        // TODO: Implementar resolução de labels
        self.code.push(0xE9);
        self.code.extend(&[0x00, 0x00, 0x00, 0x00]); // Placeholder
    }
    
    fn emit_jz(&mut self, _block_id: BlockId) {
        // jz rel32
        // TODO: Implementar resolução de labels
        self.code.push(0x0F);
        self.code.push(0x84);
        self.code.extend(&[0x00, 0x00, 0x00, 0x00]); // Placeholder
    }
    
    fn emit_label(&mut self, block_id: BlockId) {
        // Marca a posição atual como um label
        let pos = self.code.len();
        // TODO: Resolver saltos pendentes para este label
        let _ = block_id; // Evitar warning por enquanto
        let _ = pos;
    }
    
    fn finish(self) -> Vec<u8> {
        self.code
    }
}
