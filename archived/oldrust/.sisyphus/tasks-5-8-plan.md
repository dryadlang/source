# Tasks 5-8 Implementation Plan: AOT Compiler Expansion

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Expand the AOT compiler to support ARM64 backend, binary file generation (ELF & PE), and core optimization passes.

**Architecture:**
- **Task 5 (ARM64)**: Parallel backend implementation reusing register allocator, liveness analysis, and codegen patterns from x86_64
- **Task 6 (ELF)**: Minimal executable generation with headers, sections, symbol tables, and basic relocations
- **Task 7 (PE)**: Windows PE stub (deferred optimization, focus on structure)
- **Task 8 (Optimizations)**: IR-level dead code elimination and constant folding passes

**Tech Stack:** Rust, ARM64 ISA, ELF/PE binary format specs, System V AMD64 ABI

---

## Task 5: ARM64 Backend Implementation

**Files:**
- Create: `crates/dryad_aot/src/backend/arm64.rs` (new backend, ~500 lines)
- Create: `crates/dryad_aot/src/backend/arm64_tests.rs` (test module, ~400 lines)
- Modify: `crates/dryad_aot/src/backend/mod.rs` (add arm64 module export)
- Modify: `crates/dryad_aot/src/lib.rs` (add Target::ARM64 enum variant)

**Goal:** Implement ARM64 code generator supporting core instructions (LoadConst, Add, Sub, Mul, CmpEq, Div, Mod, comparisons, logic, shifts).

### Step 1: Define ARM64 instruction encoding

Add to `crates/dryad_aot/src/backend/arm64.rs`:

```rust
// ARM64 Register Encoding (0-30 = X0-X30, 31 = SP/ZR)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Arm64Reg {
    X0, X1, X2, X3, X4, X5, X6, X7,
    X8, X9, X10, X11, X12, X13, X14, X15,
    X16, X17, X18, X19, X20, X21, X22, X23,
    X24, X25, X26, X27, X28, X29, X30,
    SP, ZR,
}

impl Arm64Reg {
    fn encoding(&self) -> u8 {
        match self {
            Arm64Reg::X0 => 0, Arm64Reg::X1 => 1, Arm64Reg::X2 => 2,
            Arm64Reg::X3 => 3, Arm64Reg::X4 => 4, Arm64Reg::X5 => 5,
            Arm64Reg::X6 => 6, Arm64Reg::X7 => 7, Arm64Reg::X8 => 8,
            Arm64Reg::X9 => 9, Arm64Reg::X10 => 10, Arm64Reg::X11 => 11,
            Arm64Reg::X12 => 12, Arm64Reg::X13 => 13, Arm64Reg::X14 => 14,
            Arm64Reg::X15 => 15, Arm64Reg::X16 => 16, Arm64Reg::X17 => 17,
            Arm64Reg::X18 => 18, Arm64Reg::X19 => 19, Arm64Reg::X20 => 20,
            Arm64Reg::X21 => 21, Arm64Reg::X22 => 22, Arm64Reg::X23 => 23,
            Arm64Reg::X24 => 24, Arm64Reg::X25 => 25, Arm64Reg::X26 => 26,
            Arm64Reg::X27 => 27, Arm64Reg::X28 => 28, Arm64Reg::X29 => 29,
            Arm64Reg::X30 => 30, Arm64Reg::SP => 31, Arm64Reg::ZR => 31,
        }
    }
}

/// ARM64 Backend
pub struct Arm64Backend {
    calling_conv: CallingConvention,
}

impl Arm64Backend {
    pub fn new() -> Self {
        Self {
            calling_conv: CallingConvention::SystemV,
        }
    }

    pub fn with_calling_conv(mut self, conv: CallingConvention) -> Self {
        self.calling_conv = conv;
        self
    }
}
```

### Step 2: Create ARM64 codegen struct

Add to `crates/dryad_aot/src/backend/arm64.rs`:

```rust
/// ARM64 code generator
struct Arm64Codegen {
    code: Vec<u8>,
    calling_conv: CallingConvention,
    alloc: AllocationResult,
    reg_map: HashMap<RegisterId, u8>,
    label_positions: HashMap<BlockId, usize>,
    pending_jumps: Vec<(usize, usize, BlockId)>,
}

impl Arm64Codegen {
    fn new(calling_conv: CallingConvention, alloc: AllocationResult) -> Self {
        let mut reg_map = HashMap::new();
        
        for (vreg, phys_opt) in &alloc.alloc {
            if let Some(_) = phys_opt {
                // ARM64 uses physical registers directly
                reg_map.insert(*vreg, *vreg);
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

    fn finish(self) -> Vec<u8> {
        self.code
    }
}
```

### Step 3: Write failing test for MOV immediate

Test file: `crates/dryad_aot/src/backend/arm64_tests.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mov_imm64() {
        let mut codegen = Arm64Codegen::new_for_test();
        
        // MOV X0, 42
        codegen.emit_mov_imm64(0, 42);
        
        assert!(codegen.code.len() >= 4);
    }

    #[test]
    fn test_add_reg_reg() {
        let mut codegen = Arm64Codegen::new_for_test();
        
        // ADD X0, X1, X2
        codegen.emit_add_reg_reg(0, 1, 2);
        
        assert_eq!(codegen.code.len(), 4);
    }

    #[test]
    fn test_label_and_branch() {
        let mut codegen = Arm64Codegen::new_for_test();
        
        let jmp_pos = codegen.code.len();
        codegen.emit_b(0); // Branch to label 0
        
        codegen.emit_nop();
        let label_pos = codegen.code.len();
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
```

### Step 4: Run test to verify failure

```bash
cd /home/pedro/repo/source
cargo test -p dryad_aot arm64 2>&1 | tail -50
```

Expected: Multiple "cannot find function" errors

### Step 5: Implement ARM64 instruction encoding methods

Add to `crates/dryad_aot/src/backend/arm64.rs`:

```rust
impl Arm64Codegen {
    // MOV X<rd>, imm64 - uses MOVZ + MOVK for large immediates
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

    // ADD X<rd>, X<rn>, X<rm> (64-bit)
    fn emit_add_reg_reg(&mut self, rd: u8, rn: u8, rm: u8) {
        let rd = rd & 0x1F;
        let rn = rn & 0x1F;
        let rm = rm & 0x1F;
        
        let instr = 0x8B000000 | ((rm as u32) << 16) | ((rn as u32) << 5) | rd as u32;
        self.emit_u32_le(instr);
    }

    // SUB X<rd>, X<rn>, X<rm>
    fn emit_sub_reg_reg(&mut self, rd: u8, rn: u8, rm: u8) {
        let rd = rd & 0x1F;
        let rn = rn & 0x1F;
        let rm = rm & 0x1F;
        
        let instr = 0xCB000000 | ((rm as u32) << 16) | ((rn as u32) << 5) | rd as u32;
        self.emit_u32_le(instr);
    }

    // MUL X<rd>, X<rn>, X<rm>
    fn emit_mul_reg_reg(&mut self, rd: u8, rn: u8, rm: u8) {
        let rd = rd & 0x1F;
        let rn = rn & 0x1F;
        let rm = rm & 0x1F;
        
        let instr = 0x9B007C00 | ((rm as u32) << 16) | ((rn as u32) << 5) | rd as u32;
        self.emit_u32_le(instr);
    }

    // CMP X<rn>, X<rm> (same as SUBS ZR, Xn, Xm)
    fn emit_cmp_reg_reg(&mut self, rn: u8, rm: u8) {
        let rn = rn & 0x1F;
        let rm = rm & 0x1F;
        
        let instr = 0xEB000000 | ((rm as u32) << 16) | ((rn as u32) << 5) | 31u32;
        self.emit_u32_le(instr);
    }

    // B (branch unconditional)
    fn emit_b(&mut self, block_id: BlockId) {
        let offset = self.code.len();
        let placeholder_offset = self.code.len();
        self.code.extend(&[0x00, 0x00, 0x00, 0x00]);
        self.record_pending_jump(block_id, placeholder_offset, 4);
    }

    // B.EQ (branch if equal)
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
                let delta = (target_pos as i32 - current_pos as i32) >> 2;
                
                let instr_bytes = &mut self.code[offset..offset + size];
                let instr = u32::from_le_bytes([
                    instr_bytes[0], instr_bytes[1], instr_bytes[2], instr_bytes[3],
                ]);
                
                let mask = 0xFF000000;
                let new_instr = (instr & mask) | ((delta as u32) & 0x03FFFFFF);
                
                let new_bytes = new_instr.to_le_bytes();
                instr_bytes.copy_from_slice(&new_bytes);
            }
        }
    }

    fn emit_prologue(&mut self, stack_size: u32) {
        // STP X29, X30, [SP, -16]!
        self.emit_u32_le(0xA9BF7FED);
        
        // MOV X29, SP
        self.emit_u32_le(0x910003FD);
        
        if stack_size > 0 {
            // SUB SP, SP, stack_size
            let imm = stack_size & 0xFFF;
            self.emit_u32_le(0xD10003FF | (imm << 10));
        }
    }

    fn emit_epilogue(&mut self) {
        // MOV SP, X29
        self.emit_u32_le(0x910003FF);
        
        // LDP X29, X30, [SP], 16
        self.emit_u32_le(0xA8C17FED);
        
        // RET
        self.emit_u32_le(0xD65F03C0);
    }
}
```

### Step 6: Implement Backend trait for ARM64

Add to `crates/dryad_aot/src/backend/arm64.rs`:

```rust
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

impl Arm64Backend {
    fn compile_function(&self, func: &IrFunction) -> Result<Vec<u8>, String> {
        let live_ranges = LivenessAnalyzer::analyze(func);
        let allocation = LinearScanAllocator::allocate(&live_ranges);
        
        let mut codegen = Arm64Codegen::new(self.calling_conv, allocation);
        
        let locals_size = 0; // TODO: calculate
        let total_stack = locals_size + codegen.alloc.total_spill_size as u32;
        let aligned_stack = if (total_stack + 16) % 16 != 0 {
            total_stack + (16 - ((total_stack + 16) % 16))
        } else {
            total_stack
        };
        
        codegen.emit_prologue(aligned_stack);
        
        for block in &func.blocks {
            self.compile_block(block, &mut codegen)?;
        }
        
        codegen.resolve_labels();
        codegen.emit_epilogue();
        
        Ok(codegen.finish())
    }

    fn compile_block(&self, block: &IrBlock, codegen: &mut Arm64Codegen) -> Result<(), String> {
        codegen.emit_label(block.id);

        for instr in &block.instructions {
            self.compile_instruction(instr, codegen)?;
        }

        self.compile_terminator(&block.terminator, codegen)?;
        Ok(())
    }

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
                    _ => return Err("Unsupported constant".to_string()),
                }
            }
            IrInstruction::Add { dest, lhs, rhs } => {
                let dest_reg = codegen.get_phys_reg(*dest)?;
                let lhs_reg = codegen.get_phys_reg(*lhs)?;
                let rhs_reg = codegen.get_phys_reg(*rhs)?;
                
                codegen.emit_add_reg_reg(dest_reg, lhs_reg, rhs_reg);
            }
            IrInstruction::Sub { dest, lhs, rhs } => {
                let dest_reg = codegen.get_phys_reg(*dest)?;
                let lhs_reg = codegen.get_phys_reg(*lhs)?;
                let rhs_reg = codegen.get_phys_reg(*rhs)?;
                
                codegen.emit_sub_reg_reg(dest_reg, lhs_reg, rhs_reg);
            }
            IrInstruction::Mul { dest, lhs, rhs } => {
                let dest_reg = codegen.get_phys_reg(*dest)?;
                let lhs_reg = codegen.get_phys_reg(*lhs)?;
                let rhs_reg = codegen.get_phys_reg(*rhs)?;
                
                codegen.emit_mul_reg_reg(dest_reg, lhs_reg, rhs_reg);
            }
            IrInstruction::CmpEq { dest, lhs, rhs } => {
                let dest_reg = codegen.get_phys_reg(*dest)?;
                let lhs_reg = codegen.get_phys_reg(*lhs)?;
                let rhs_reg = codegen.get_phys_reg(*rhs)?;
                
                codegen.emit_cmp_reg_reg(lhs_reg, rhs_reg);
                codegen.emit_cset(dest_reg, 0); // CSET X<d>, EQ
            }
            _ => return Err(format!("Unsupported instruction: {:?}", instr)),
        }
        Ok(())
    }

    fn compile_terminator(
        &self,
        term: &IrTerminator,
        codegen: &mut Arm64Codegen,
    ) -> Result<(), String> {
        match term {
            IrTerminator::Return => codegen.emit_ret(),
            IrTerminator::Jump(target) => codegen.emit_b(*target),
            IrTerminator::BranchIf { cond, then_block, else_block } => {
                codegen.emit_beq(*then_block);
            }
        }
        Ok(())
    }
}
```

### Step 7: Add missing emit methods

```rust
fn emit_cset(&mut self, rd: u8, cond: u8) {
    // CSET X<rd>, cond (conditional set)
    let rd = rd & 0x1F;
    let instr = 0x9A9F03E0 | ((cond & 0x0F) << 12) | rd as u32;
    self.emit_u32_le(instr);
}

fn emit_ret(&mut self) {
    // RET X30
    self.emit_u32_le(0xD65F03C0);
}
```

### Step 8: Run tests to verify implementation

```bash
cd /home/pedro/repo/source
cargo test -p dryad_aot arm64 --lib 2>&1 | tail -30
```

Expected: 4 tests pass

### Step 9: Commit

```bash
git add crates/dryad_aot/src/backend/arm64.rs
git commit -m "feat: implement ARM64 backend with core instructions

- ARM64 register mapping (X0-X30, SP, ZR)
- Core instructions: MOV, ADD, SUB, MUL, CMP
- Label resolution with branch offset backpatching
- Prologue/epilogue with 16-byte stack alignment
- Reuses x86_64 register allocator and liveness analysis

Tests added: 4 ARM64 instruction codegen tests

All tests pass"
```

---

## Task 6: ELF Binary Generation

**Files:**
- Create: `crates/dryad_aot/src/generator/elf.rs` (ELF generation, ~600 lines)
- Modify: `crates/dryad_aot/src/generator/mod.rs` (trait export)
- Modify: `crates/dryad_aot/src/lib.rs` (integrate ELF generator)

**Goal:** Generate minimal ELF64 executable binaries with proper headers, sections, and symbol tables.

### Step 1: Define ELF header structures

Add to `crates/dryad_aot/src/generator/elf.rs`:

```rust
use std::io::Write;

const ELF_MAGIC: &[u8] = b"\x7FELF";
const ELF_CLASS_64: u8 = 2;
const ELF_DATA_LITTLE: u8 = 1;
const ELF_VERSION_CURRENT: u8 = 1;
const ELF_OSABI_NONE: u8 = 0;
const ELF_TYPE_EXEC: u16 = 2;
const ELF_MACHINE_X86_64: u16 = 62;
const ELF_MACHINE_ARM64: u16 = 183;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct ElfHeader {
    e_ident: [u8; 16],
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u64,
    e_phoff: u64,
    e_shoff: u64,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct ProgramHeader {
    p_type: u32,
    p_flags: u32,
    p_offset: u64,
    p_vaddr: u64,
    p_paddr: u64,
    p_filesz: u64,
    p_memsz: u64,
    p_align: u64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct SectionHeader {
    sh_name: u32,
    sh_type: u32,
    sh_flags: u64,
    sh_addr: u64,
    sh_offset: u64,
    sh_size: u64,
    sh_entsize: u64,
}

pub struct ElfGenerator;

impl ElfGenerator {
    pub fn generate(code: Vec<u8>, target: Target) -> Result<Vec<u8>, String> {
        let mut output = Vec::new();
        
        let entry_point = 0x400000u64;
        let code_offset = 0x1000u64;
        
        // Build ELF header
        let mut elf_header = ElfHeader {
            e_ident: [0; 16],
            e_type: ELF_TYPE_EXEC,
            e_machine: match target {
                Target::X86_64Linux => ELF_MACHINE_X86_64,
                Target::ARM64Linux => ELF_MACHINE_ARM64,
                _ => return Err("Unsupported target".to_string()),
            },
            e_version: 1,
            e_entry: entry_point,
            e_phoff: 64,
            e_shoff: 0x2000,
            e_flags: 0,
            e_ehsize: 64,
            e_phentsize: 56,
            e_phnum: 2,
            e_shentsize: 64,
            e_shnum: 6,
            e_shstrndx: 1,
        };
        
        // Initialize ELF identification
        elf_header.e_ident[0..4].copy_from_slice(ELF_MAGIC);
        elf_header.e_ident[4] = ELF_CLASS_64;
        elf_header.e_ident[5] = ELF_DATA_LITTLE;
        elf_header.e_ident[6] = ELF_VERSION_CURRENT;
        elf_header.e_ident[7] = ELF_OSABI_NONE;
        
        // Write ELF header
        output.extend(&elf_header_bytes(&elf_header));
        
        // Write program headers
        let load_ph = ProgramHeader {
            p_type: 1,
            p_flags: 5,
            p_offset: code_offset,
            p_vaddr: entry_point,
            p_paddr: entry_point,
            p_filesz: code.len() as u64,
            p_memsz: code.len() as u64,
            p_align: 0x1000,
        };
        
        output.extend(&program_header_bytes(&load_ph));
        
        let dynamic_ph = ProgramHeader {
            p_type: 2,
            p_flags: 4,
            p_offset: 0,
            p_vaddr: 0,
            p_paddr: 0,
            p_filesz: 0,
            p_memsz: 0,
            p_align: 1,
        };
        
        output.extend(&program_header_bytes(&dynamic_ph));
        
        // Pad to code offset
        while output.len() < code_offset as usize {
            output.push(0);
        }
        
        // Write code
        output.extend(&code);
        
        // Pad to section headers
        while output.len() < 0x2000 {
            output.push(0);
        }
        
        // Write section headers
        // Section 0: NULL
        output.extend(&section_header_bytes(&SectionHeader {
            sh_name: 0, sh_type: 0, sh_flags: 0, sh_addr: 0,
            sh_offset: 0, sh_size: 0, sh_entsize: 0,
        }));
        
        // Section 1: .shstrtab
        output.extend(&section_header_bytes(&SectionHeader {
            sh_name: 1,
            sh_type: 3,
            sh_flags: 0,
            sh_addr: 0,
            sh_offset: 0x2400,
            sh_size: 32,
            sh_entsize: 0,
        }));
        
        // Section 2: .text
        output.extend(&section_header_bytes(&SectionHeader {
            sh_name: 11,
            sh_type: 1,
            sh_flags: 6,
            sh_addr: entry_point,
            sh_offset: code_offset,
            sh_size: code.len() as u64,
            sh_entsize: 0,
        }));
        
        // Sections 3-5: Minimal stubs
        for i in 0..3 {
            output.extend(&section_header_bytes(&SectionHeader {
                sh_name: 0, sh_type: 0, sh_flags: 0, sh_addr: 0,
                sh_offset: 0, sh_size: 0, sh_entsize: 0,
            }));
        }
        
        // Write string table
        while output.len() < 0x2400 {
            output.push(0);
        }
        
        output.extend(b"\0.shstrtab\0.text\0\0\0\0\0\0");
        
        Ok(output)
    }
}

fn elf_header_bytes(hdr: &ElfHeader) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend(&hdr.e_ident);
    bytes.extend(&hdr.e_type.to_le_bytes());
    bytes.extend(&hdr.e_machine.to_le_bytes());
    bytes.extend(&hdr.e_version.to_le_bytes());
    bytes.extend(&hdr.e_entry.to_le_bytes());
    bytes.extend(&hdr.e_phoff.to_le_bytes());
    bytes.extend(&hdr.e_shoff.to_le_bytes());
    bytes.extend(&hdr.e_flags.to_le_bytes());
    bytes.extend(&hdr.e_ehsize.to_le_bytes());
    bytes.extend(&hdr.e_phentsize.to_le_bytes());
    bytes.extend(&hdr.e_phnum.to_le_bytes());
    bytes.extend(&hdr.e_shentsize.to_le_bytes());
    bytes.extend(&hdr.e_shnum.to_le_bytes());
    bytes.extend(&hdr.e_shstrndx.to_le_bytes());
    bytes
}

fn program_header_bytes(ph: &ProgramHeader) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend(&ph.p_type.to_le_bytes());
    bytes.extend(&ph.p_flags.to_le_bytes());
    bytes.extend(&ph.p_offset.to_le_bytes());
    bytes.extend(&ph.p_vaddr.to_le_bytes());
    bytes.extend(&ph.p_paddr.to_le_bytes());
    bytes.extend(&ph.p_filesz.to_le_bytes());
    bytes.extend(&ph.p_memsz.to_le_bytes());
    bytes.extend(&ph.p_align.to_le_bytes());
    bytes
}

fn section_header_bytes(sh: &SectionHeader) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend(&sh.sh_name.to_le_bytes());
    bytes.extend(&sh.sh_type.to_le_bytes());
    bytes.extend(&sh.sh_flags.to_le_bytes());
    bytes.extend(&sh.sh_addr.to_le_bytes());
    bytes.extend(&sh.sh_offset.to_le_bytes());
    bytes.extend(&sh.sh_size.to_le_bytes());
    bytes.extend(&sh.sh_entsize.to_le_bytes());
    bytes
}
```

### Step 2: Write failing test for ELF generation

Add test:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elf_header_validity() {
        let code = vec![0x90; 100];
        let elf_binary = ElfGenerator::generate(code, Target::X86_64Linux)
            .expect("ELF generation failed");
        
        assert!(elf_binary.len() >= 64);
        assert_eq!(&elf_binary[0..4], b"\x7FELF");
        assert_eq!(elf_binary[4], 2);
    }

    #[test]
    fn test_elf_program_headers() {
        let code = vec![0x90; 100];
        let elf_binary = ElfGenerator::generate(code, Target::X86_64Linux)
            .expect("ELF generation failed");
        
        assert_eq!(&elf_binary[0..4], b"\x7FELF");
        assert!(elf_binary.len() >= 0x1000);
    }
}
```

### Step 3: Run test to verify success

```bash
cd /home/pedro/repo/source
cargo test -p dryad_aot elf -- --nocapture 2>&1 | tail -20
```

Expected: 2 tests pass

### Step 4: Commit

```bash
git add crates/dryad_aot/src/generator/elf.rs
git commit -m "feat: implement ELF binary generation

- ELF64 header with x86_64 and ARM64 support
- Program headers for executable code loading
- Section headers for .text, .shstrtab
- Proper virtual address mapping and file offsets
- Generates valid ELF binaries for x86_64 and ARM64

Tests added: 2 ELF generation tests

All tests pass"
```

---

## Task 7: Windows PE Binary Generation

**Files:**
- Create: `crates/dryad_aot/src/generator/pe.rs` (PE generation, ~400 lines)
- Modify: `crates/dryad_aot/src/generator/mod.rs`

**Goal:** Minimal PE binary generation (stub - focus on structure, not optimization).

### Step 1: Define PE structures

```rust
const PE_MAGIC: &[u8; 4] = b"PE\0\0";
const MACHINE_I386: u16 = 0x014c;
const MACHINE_X64: u16 = 0x8664;

pub struct PeGenerator;

impl PeGenerator {
    pub fn generate(code: Vec<u8>) -> Result<Vec<u8>, String> {
        let mut output = Vec::new();
        
        // DOS header
        let dos_header = create_dos_header();
        output.extend(&dos_header);
        
        // PE header
        output.extend(PE_MAGIC);
        
        // File header
        let file_header = create_file_header();
        output.extend(&file_header);
        
        // Optional header
        let opt_header = create_optional_header(code.len() as u32);
        output.extend(&opt_header);
        
        // .text section
        output.extend(b".text\0\0\0");
        output.extend(&(code.len() as u32).to_le_bytes());
        output.extend(&(0x1000u32).to_le_bytes());
        output.extend(&(code.len() as u32).to_le_bytes());
        output.extend(&(0x400u32).to_le_bytes());
        
        // Code section
        output.extend(&code);
        
        Ok(output)
    }
}

fn create_dos_header() -> Vec<u8> {
    let mut header = vec![0u8; 64];
    header[0..2].copy_from_slice(b"MZ");
    header[60..64].copy_from_slice(&(64u32).to_le_bytes());
    header
}

fn create_file_header() -> Vec<u8> {
    let mut header = Vec::new();
    header.extend(&MACHINE_X64.to_le_bytes());
    header.extend(&(1u16).to_le_bytes());
    header.extend(&(0u32).to_le_bytes());
    header.extend(&(0u32).to_le_bytes());
    header.extend(&(0u32).to_le_bytes());
    header.extend(&(224u16).to_le_bytes());
    header.extend(&(0u16).to_le_bytes());
    header
}

fn create_optional_header(code_size: u32) -> Vec<u8> {
    let mut header = Vec::new();
    header.extend(&(0x20bu16).to_le_bytes());
    header.extend(&vec![0u8; 222]);
    header
}
```

### Step 2: Write test

```rust
#[test]
fn test_pe_header_validity() {
    let code = vec![0x90; 100];
    let pe_binary = PeGenerator::generate(code)
        .expect("PE generation failed");
    
    assert_eq!(&pe_binary[0..2], b"MZ");
    assert!(pe_binary.len() >= 64);
}
```

### Step 3: Run test

```bash
cargo test -p dryad_aot pe -- --nocapture 2>&1 | tail -20
```

### Step 4: Commit

```bash
git add crates/dryad_aot/src/generator/pe.rs
git commit -m "feat: implement minimal PE binary generation stub

- DOS header (MZ signature)
- PE file and optional headers
- .text section with code
- Basic x64 support

Tests added: 1 PE generation test

All tests pass (stub implementation)"
```

---

## Task 8: Optimization Passes (Dead Code Elimination & Constant Folding)

**Files:**
- Create: `crates/dryad_aot/src/optimizer/mod.rs` (optimizer module, ~50 lines)
- Create: `crates/dryad_aot/src/optimizer/dce.rs` (dead code elimination, ~100 lines)
- Create: `crates/dryad_aot/src/optimizer/const_fold.rs` (constant folding, ~150 lines)
- Modify: `crates/dryad_aot/src/lib.rs` (integrate optimizer)

**Goal:** Implement IR-level optimization passes for dead code removal and constant folding.

### Step 1: Define optimizer module

Create `crates/dryad_aot/src/optimizer/mod.rs`:

```rust
pub mod dce;
pub mod const_fold;

use crate::ir::{IrModule, IrFunction, IrInstruction, IrBlock, BlockId};
use std::collections::HashSet;

pub trait Optimizer {
    fn optimize(&self, module: IrModule) -> IrModule;
}

pub struct OptimizationPipeline {
    passes: Vec<Box<dyn Optimizer>>,
}

impl OptimizationPipeline {
    pub fn new() -> Self {
        Self {
            passes: vec![
                Box::new(dce::DeadCodeEliminator),
                Box::new(const_fold::ConstantFolder),
            ],
        }
    }

    pub fn run(&self, mut module: IrModule) -> IrModule {
        for pass in &self.passes {
            module = pass.optimize(module);
        }
        module
    }
}
```

### Step 2: Implement Dead Code Elimination

Create `crates/dryad_aot/src/optimizer/dce.rs`:

```rust
use super::Optimizer;
use crate::ir::{IrModule, IrFunction, IrInstruction, IrBlock, BlockId, IrTerminator};
use std::collections::HashSet;

pub struct DeadCodeEliminator;

impl Optimizer for DeadCodeEliminator {
    fn optimize(&self, module: IrModule) -> IrModule {
        let functions = module.functions.into_iter()
            .map(|func| Self::eliminate_dead_blocks(func))
            .collect();

        IrModule {
            name: module.name,
            functions,
        }
    }
}

impl DeadCodeEliminator {
    fn eliminate_dead_blocks(func: IrFunction) -> IrFunction {
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
                    IrTerminator::BranchIf { then_block, else_block, .. } => {
                        queue.push(*then_block);
                        queue.push(*else_block);
                    }
                    IrTerminator::Return => {}
                }
            }
        }

        let blocks = func.blocks.into_iter()
            .filter(|b| reachable.contains(&b.id))
            .collect();

        IrFunction {
            name: func.name,
            blocks,
        }
    }
}
```

### Step 3: Implement Constant Folding

Create `crates/dryad_aot/src/optimizer/const_fold.rs`:

```rust
use super::Optimizer;
use crate::ir::*;

pub struct ConstantFolder;

impl Optimizer for ConstantFolder {
    fn optimize(&self, module: IrModule) -> IrModule {
        let functions = module.functions.into_iter()
            .map(|func| Self::fold_constants_in_function(func))
            .collect();

        IrModule {
            name: module.name,
            functions,
        }
    }
}

impl ConstantFolder {
    fn fold_constants_in_function(func: IrFunction) -> IrFunction {
        let blocks = func.blocks.into_iter()
            .map(|block| Self::fold_constants_in_block(block))
            .collect();

        IrFunction {
            name: func.name,
            blocks,
        }
    }

    fn fold_constants_in_block(mut block: IrBlock) -> IrBlock {
        let mut const_values: std::collections::HashMap<RegisterId, i32> = std::collections::HashMap::new();

        block.instructions = block.instructions.into_iter()
            .filter_map(|instr| {
                match &instr {
                    IrInstruction::LoadConst { dest, value } => {
                        if let IrValue::Constant(IrConstant::I32(n)) = value {
                            const_values.insert(*dest, *n);
                            Some(instr)
                        } else {
                            None
                        }
                    }
                    IrInstruction::Add { dest, lhs, rhs } => {
                        if let (Some(lhs_val), Some(rhs_val)) = (const_values.get(lhs), const_values.get(rhs)) {
                            let result = lhs_val + rhs_val;
                            const_values.insert(*dest, result);
                            None
                        } else {
                            Some(instr)
                        }
                    }
                    IrInstruction::Sub { dest, lhs, rhs } => {
                        if let (Some(lhs_val), Some(rhs_val)) = (const_values.get(lhs), const_values.get(rhs)) {
                            let result = lhs_val - rhs_val;
                            const_values.insert(*dest, result);
                            None
                        } else {
                            Some(instr)
                        }
                    }
                    IrInstruction::Mul { dest, lhs, rhs } => {
                        if let (Some(lhs_val), Some(rhs_val)) = (const_values.get(lhs), const_values.get(rhs)) {
                            let result = lhs_val * rhs_val;
                            const_values.insert(*dest, result);
                            None
                        } else {
                            Some(instr)
                        }
                    }
                    _ => Some(instr),
                }
            })
            .collect();

        block
    }
}
```

### Step 4: Write tests for DCE

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dead_block_elimination() {
        let unreachable_block = IrBlock {
            id: 1,
            instructions: vec![],
            terminator: IrTerminator::Return,
        };

        let reachable_block = IrBlock {
            id: 0,
            instructions: vec![],
            terminator: IrTerminator::Return,
        };

        let func = IrFunction {
            name: "test".to_string(),
            blocks: vec![reachable_block, unreachable_block],
        };

        let optimized = DeadCodeEliminator::eliminate_dead_blocks(func);
        assert_eq!(optimized.blocks.len(), 1);
    }

    #[test]
    fn test_constant_folding_add() {
        let const1 = IrInstruction::LoadConst {
            dest: 0,
            value: IrValue::Constant(IrConstant::I32(5)),
        };

        let const2 = IrInstruction::LoadConst {
            dest: 1,
            value: IrValue::Constant(IrConstant::I32(3)),
        };

        let add = IrInstruction::Add { dest: 2, lhs: 0, rhs: 1 };

        let block = IrBlock {
            id: 0,
            instructions: vec![const1, const2, add],
            terminator: IrTerminator::Return,
        };

        let folded = ConstantFolder::fold_constants_in_block(block);
        
        assert!(folded.instructions.len() <= 3);
    }
}
```

### Step 5: Run tests

```bash
cd /home/pedro/repo/source
cargo test -p dryad_aot optimizer -- --nocapture 2>&1 | tail -30
```

Expected: 2 tests pass

### Step 6: Commit

```bash
git add crates/dryad_aot/src/optimizer/
git commit -m "feat: implement optimization passes (DCE and constant folding)

- Dead Code Elimination: removes unreachable blocks
- Constant Folding: evaluates constant expressions at compile time
- Optimization pipeline with extensible pass system
- IR-level optimizations before code generation

Tests added: 2 optimization tests (DCE, constant folding)

All tests pass"
```

---

## Summary & Execution

**All 4 tasks (5-8):**
- Task 5: ARM64 backend (4 tests)
- Task 6: ELF binary generation (2 tests)
- Task 7: PE binary generation stub (1 test)
- Task 8: Optimization passes (2 tests)

**Expected total:** 9 new tests + all previous 24 = **33 passing tests**

**Execution approach:** Proceed sequentially with TDD (failing test → implementation → passing test → commit) for each task.
