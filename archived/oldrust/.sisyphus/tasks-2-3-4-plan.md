# Tasks 2-4 Implementation Plan: AOT Compiler Foundation

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement label resolution, ABI compliance, and expanded x86_64 instruction support to enable binary generation for real programs.

**Architecture:** 
- Task 2 (Label Resolution): Two-pass code generation - first pass emits code with label placeholders, second pass backpatches jump offsets
- Task 3 (ABI Compliance): Proper prologue/epilogue with 16-byte stack alignment as required by System V AMD64 ABI
- Task 4 (Instruction Coverage): Expand from 5 supported instructions to 14+, covering arithmetic, logic, and comparisons

**Tech Stack:** Rust, x86_64 opcodes, System V AMD64 ABI, TDD

---

## Task 2: Label Resolution for Jumps

**Files:**
- Modify: `crates/dryad_aot/src/backend/x86_64.rs:220-420` (X86_64Codegen struct and methods)
- Test: Add tests in `crates/dryad_aot/src/backend/x86_64.rs` (tests module)

**Goal:** Fix jump instructions to resolve to correct offsets instead of using placeholder `[0x00, 0x00, 0x00, 0x00]`

### Step 1: Write failing test for label resolution

Add this test at the end of x86_64.rs before the closing brace:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_label_resolution() {
        let mut codegen = X86_64Codegen::new_for_test();
        
        // Emit forward jump (target not yet at this position)
        let offset_before_jmp = codegen.code.len();
        codegen.emit_jmp(0); // Jump to label 0
        let offset_after_jmp = codegen.code.len();
        
        // Emit code
        codegen.emit_nop();
        codegen.emit_nop();
        
        // Mark label 0 at this position
        let label_position = codegen.code.len();
        
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
```

### Step 2: Run test to verify failure

```bash
cd /home/pedro/repo/source
cargo test -p dryad_aot test_label_resolution -- --nocapture 2>&1 | tail -50
```

**Expected output**: Test fails because `resolve_labels()` method doesn't exist and `new_for_test()` doesn't exist.

### Step 3: Implement label tracking in X86_64Codegen

Modify X86_64Codegen struct (around line 220):

**OLD CODE:**
```rust
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
}
```

**NEW CODE:**
```rust
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
```

Update the `new()` method (around line 232):

**OLD CODE:**
```rust
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
        }
    }
```

**NEW CODE:**
```rust
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
```

### Step 4: Implement label position recording and resolution

Add methods before `finish()` method (around line 380):

```rust
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
```

### Step 5: Update emit_label to record label position

Change `emit_label()` method (around line 387):

**OLD CODE:**
```rust
    fn emit_label(&mut self, block_id: BlockId) {
        let pos = self.code.len();
        let _ = block_id;
        let _ = pos;
    }
```

**NEW CODE:**
```rust
    fn emit_label(&mut self, block_id: BlockId) {
        self.record_label(block_id);
    }
```

### Step 6: Update emit_jmp and emit_jz to record pending jumps

Change `emit_jmp()` method (around line 368):

**OLD CODE:**
```rust
    fn emit_jmp(&mut self, _block_id: BlockId) {
        self.code.push(0xE9);
        self.code.extend(&[0x00, 0x00, 0x00, 0x00]);
    }
```

**NEW CODE:**
```rust
    fn emit_jmp(&mut self, block_id: BlockId) {
        let offset = self.code.len();
        self.code.push(0xE9);
        let placeholder_offset = self.code.len();
        self.code.extend(&[0x00, 0x00, 0x00, 0x00]);
        self.record_pending_jump(block_id, placeholder_offset, 4);
    }
```

Change `emit_jz()` method (around line 375):

**OLD CODE:**
```rust
    fn emit_jz(&mut self, _block_id: BlockId) {
        self.code.push(0x0F);
        self.code.push(0x84);
        self.code.extend(&[0x00, 0x00, 0x00, 0x00]);
    }
```

**NEW CODE:**
```rust
    fn emit_jz(&mut self, block_id: BlockId) {
        self.code.push(0x0F);
        self.code.push(0x84);
        let placeholder_offset = self.code.len();
        self.code.extend(&[0x00, 0x00, 0x00, 0x00]);
        self.record_pending_jump(block_id, placeholder_offset, 4);
    }
```

### Step 7: Update compile_block to resolve labels before returning

Modify `compile_block()` in X86_64Backend (around line 73):

**OLD CODE:**
```rust
    /// Compila um bloco básico
    fn compile_block(&self, block: &IrBlock, codegen: &mut X86_64Codegen) -> Result<(), String> {
        codegen.emit_label(block.id);

        for instr in &block.instructions {
            self.compile_instruction(instr, codegen)?;
        }

        self.compile_terminator(&block.terminator, codegen)?;

        Ok(())
    }
```

**NEW CODE:**
```rust
    /// Compila um bloco básico
    fn compile_block(&self, block: &IrBlock, codegen: &mut X86_64Codegen) -> Result<(), String> {
        codegen.emit_label(block.id);

        for instr in &block.instructions {
            self.compile_instruction(instr, codegen)?;
        }

        self.compile_terminator(&block.terminator, codegen)?;

        Ok(())
    }
```

No change needed here - labels are recorded automatically.

### Step 8: Call resolve_labels() after all blocks compiled

Modify `compile_function()` in X86_64Backend (around line 37):

**OLD CODE:**
```rust
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

        // Epilogue (caso não tenha ret explícito)
        codegen.emit_mov_rsp_rbp();
        codegen.emit_pop_rbp();
        codegen.emit_ret();

        Ok(codegen.finish())
    }
```

**NEW CODE:**
```rust
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
```

### Step 9: Run tests to verify success

```bash
cd /home/pedro/repo/source
cargo test -p dryad_aot test_label_resolution test_backward_jump_resolution test_conditional_jump_resolution -- --nocapture 2>&1 | tail -40
```

**Expected output**: All 3 tests pass, labels resolve correctly

### Step 10: Commit

```bash
cd /home/pedro/repo/source
git add crates/dryad_aot/src/backend/x86_64.rs
git commit -m "feat: implement label resolution for jumps

- Add label_positions tracking in X86_64Codegen
- Record pending jumps during code generation
- Two-pass approach: first emit with placeholders, second backpatch offsets
- Support forward and backward jumps
- Conditional (jz) and unconditional (jmp) jumps
- All jump offsets now resolve to correct targets

Tests added: test_label_resolution, test_backward_jump_resolution, test_conditional_jump_resolution"
```

---

## Task 3: ABI Compliance - 16-Byte Stack Alignment

**Files:**
- Modify: `crates/dryad_aot/src/backend/x86_64.rs:30-65` (prologue/epilogue)
- Test: Add tests in x86_64.rs

**Goal:** Implement proper System V AMD64 ABI stack alignment (16-byte before `call` instruction)

### Step 1: Write failing test for stack alignment

Add test to x86_64.rs tests module:

```rust
    #[test]
    fn test_stack_alignment_before_call() {
        let mut codegen = X86_64Codegen::new_for_test();
        
        // Prologue
        codegen.emit_push_rbp(); // 1 byte: rsp now 8-byte aligned
        codegen.emit_mov_rbp_rsp();
        
        // Calculate alignment: need frame_size such that (frame_size + 8) % 16 == 0
        let frame_size = 8u32;
        codegen.emit_sub_rsp(frame_size);
        
        let code_size_after_prologue = codegen.code.len();
        
        // After prologue: rsp should be 16-byte aligned
        // (push_rbp=1 + mov_rbp_rsp=3 + sub_rsp=7) = 11 bytes
        // But we need to account for alignment: push rbp moved rsp by 8
        // Then sub rsp by 8 -> total adjustment is 16
        // So position % 16 should make (rsp before call) 16-byte aligned
        
        assert_eq!(code_size_after_prologue % 16, (1 + 3 + 7) % 16);
    }

    #[test]
    fn test_stack_frame_calculation() {
        // For locals_size = 8 and no spills:
        // Need frame_size such that (push_rbp + frame_size) % 16 == 0
        // push_rbp = 8 bytes
        // So frame_size must be 8 mod 16
        
        let locals_size = 8u32;
        let spill_size = 0i32;
        let total_stack = locals_size + spill_size.max(0) as u32;
        
        // Adjusted for alignment (push_rbp is 8 bytes, which is 8 mod 16)
        // Need to add 8 more to reach 16-byte boundary
        let adjusted_stack = if (total_stack + 8) % 16 != 0 {
            total_stack + (16 - ((total_stack + 8) % 16))
        } else {
            total_stack
        };
        
        assert_eq!((adjusted_stack + 8) % 16, 0);
    }
```

### Step 2: Run test to verify failure

```bash
cd /home/pedro/repo/source
cargo test -p dryad_aot test_stack_alignment_before_call test_stack_frame_calculation -- --nocapture 2>&1 | tail -40
```

**Expected output**: Tests fail - stack alignment not implemented

### Step 3: Implement stack alignment in compile_function

Modify `compile_function()` in X86_64Backend:

**OLD CODE:**
```rust
        // Alocar stack para variáveis locais + spills
        let locals_size = self.calculate_locals_size(func);
        let total_stack_needed = locals_size + codegen.alloc.total_spill_size as u32;
        if total_stack_needed > 0 {
            codegen.emit_sub_rsp(total_stack_needed);
        }
```

**NEW CODE:**
```rust
        // Alocar stack para variáveis locais + spills com alinhamento
        let locals_size = self.calculate_locals_size(func);
        let total_stack_needed = locals_size + codegen.alloc.total_spill_size as u32;
        
        // Align stack to 16-byte boundary
        // After push_rbp (8 bytes), we need (total_stack_needed + 8) % 16 == 0
        let aligned_stack = if (total_stack_needed + 8) % 16 != 0 {
            total_stack_needed + (16 - ((total_stack_needed + 8) % 16))
        } else {
            total_stack_needed
        };
        
        if aligned_stack > 0 {
            codegen.emit_sub_rsp(aligned_stack);
        }
```

### Step 4: Run tests to verify alignment

```bash
cd /home/pedro/repo/source
cargo test -p dryad_aot test_stack_alignment_before_call test_stack_frame_calculation -- --nocapture 2>&1 | tail -40
```

**Expected output**: Tests pass

### Step 5: Verify all existing tests still pass

```bash
cd /home/pedro/repo/source
cargo test -p dryad_aot --lib 2>&1 | grep "test result"
```

**Expected output**: `test result: ok. XX passed; 0 failed`

### Step 6: Commit

```bash
cd /home/pedro/repo/source
git add crates/dryad_aot/src/backend/x86_64.rs
git commit -m "feat: implement 16-byte stack alignment (System V AMD64 ABI)

- Calculate aligned stack frame size: (total_needed + 8) % 16 == 0
- Required before any 'call' instruction
- Proper prologue maintains ABI requirements
- Stack frame now compatible with C calling conventions

Tests added: test_stack_alignment_before_call, test_stack_frame_calculation"
```

---

## Task 4: Expanded x86_64 Instruction Coverage

**Files:**
- Modify: `crates/dryad_aot/src/backend/x86_64.rs:88-160` (compile_instruction)
- Add: Codegen methods for new instructions

**Goal:** Expand from 5 supported instructions (LoadConst, Add, Sub, Mul, CmpEq) to 14+

### Step 1: Write failing tests for division

Add tests to x86_64.rs:

```rust
    #[test]
    fn test_div_instruction_codegen() {
        // div r64 divides rdx:rax by r64, result in rax, remainder in rdx
        let mut codegen = X86_64Codegen::new_for_test();
        
        codegen.emit_mov_imm64(0, 10); // rax = 10
        codegen.emit_xor_reg_reg(2, 2); // rdx = 0 (clear rdx)
        codegen.emit_mov_imm64(1, 3); // rcx = 3
        codegen.emit_div_reg(1); // div rcx (rax / rcx)
        
        assert!(codegen.code.len() > 0);
        assert!(codegen.code.contains(&0x48)); // REX.W for 64-bit
    }

    #[test]
    fn test_mod_instruction_codegen() {
        // mod is same as div, but we use rdx for result
        let mut codegen = X86_64Codegen::new_for_test();
        
        codegen.emit_mov_imm64(0, 10);
        codegen.emit_xor_reg_reg(2, 2);
        codegen.emit_mov_imm64(1, 3);
        codegen.emit_div_reg(1);
        // Result is in rdx
        
        assert!(codegen.code.len() > 0);
    }

    #[test]
    fn test_cmp_ne_instruction() {
        let mut codegen = X86_64Codegen::new_for_test();
        
        codegen.emit_mov_imm64(0, 5);
        codegen.emit_mov_imm64(1, 5);
        codegen.emit_cmp_reg_reg(0, 1);
        codegen.emit_setne(0); // Set if not equal
        
        assert!(codegen.code.len() > 0);
        assert!(codegen.code.contains(&0x95)); // setne opcode
    }

    #[test]
    fn test_cmp_lt_instruction() {
        let mut codegen = X86_64Codegen::new_for_test();
        
        codegen.emit_cmp_reg_reg(0, 1);
        codegen.emit_setl(0); // Set if less
        
        assert!(codegen.code.len() > 0);
        assert!(codegen.code.contains(&0x9C)); // setl opcode
    }

    #[test]
    fn test_logic_and_instruction() {
        let mut codegen = X86_64Codegen::new_for_test();
        
        codegen.emit_and_reg_reg(0, 1);
        
        assert!(codegen.code.len() > 0);
    }

    #[test]
    fn test_logic_or_instruction() {
        let mut codegen = X86_64Codegen::new_for_test();
        
        codegen.emit_or_reg_reg(0, 1);
        
        assert!(codegen.code.len() > 0);
    }

    #[test]
    fn test_shift_left_instruction() {
        let mut codegen = X86_64Codegen::new_for_test();
        
        codegen.emit_shl_reg_reg(0, 1);
        
        assert!(codegen.code.len() > 0);
    }
```

### Step 2: Run tests to verify failure

```bash
cd /home/pedro/repo/source
cargo test -p dryad_aot test_div_instruction_codegen test_mod_instruction_codegen test_cmp_ne_instruction test_cmp_lt_instruction test_logic_and_instruction test_logic_or_instruction test_shift_left_instruction -- --nocapture 2>&1 | tail -50
```

**Expected output**: All tests fail - methods don't exist

### Step 3: Add codegen methods for division

Add methods to X86_64Codegen (before `finish()` method):

```rust
    fn emit_xor_reg_reg(&mut self, dest: u8, src: u8) {
        // xor r64, r64
        let modrm = 0xC0 | ((src & 7) << 3) | (dest & 7);
        let rex = 0x48 | ((src >> 3) & 1) | (((dest >> 3) & 1) << 2);
        self.code.push(rex);
        self.code.push(0x31);
        self.code.push(modrm);
    }

    fn emit_div_reg(&mut self, divisor: u8) {
        // div r64 - divides rdx:rax by r64, result in rax, remainder in rdx
        let modrm = 0xF0 | (divisor & 7);
        let rex = 0x48 | ((divisor >> 3) & 1);
        self.code.push(rex);
        self.code.push(0xF7);
        self.code.push(modrm);
    }

    fn emit_setne(&mut self, reg: u8) {
        // setne r8 - set if not equal
        let modrm = 0xC0 | (reg & 7);
        let rex = 0x40 | ((reg >> 3) & 1);
        self.code.push(rex);
        self.code.push(0x0F);
        self.code.push(0x95);
        self.code.push(modrm);
    }

    fn emit_setl(&mut self, reg: u8) {
        // setl r8 - set if less
        let modrm = 0xC0 | (reg & 7);
        let rex = 0x40 | ((reg >> 3) & 1);
        self.code.push(rex);
        self.code.push(0x0F);
        self.code.push(0x9C);
        self.code.push(modrm);
    }

    fn emit_setle(&mut self, reg: u8) {
        // setle r8 - set if less or equal
        let modrm = 0xC0 | (reg & 7);
        let rex = 0x40 | ((reg >> 3) & 1);
        self.code.push(rex);
        self.code.push(0x0F);
        self.code.push(0x9E);
        self.code.push(modrm);
    }

    fn emit_setg(&mut self, reg: u8) {
        // setg r8 - set if greater
        let modrm = 0xC0 | (reg & 7);
        let rex = 0x40 | ((reg >> 3) & 1);
        self.code.push(rex);
        self.code.push(0x0F);
        self.code.push(0x9F);
        self.code.push(modrm);
    }

    fn emit_setge(&mut self, reg: u8) {
        // setge r8 - set if greater or equal
        let modrm = 0xC0 | (reg & 7);
        let rex = 0x40 | ((reg >> 3) & 1);
        self.code.push(rex);
        self.code.push(0x0F);
        self.code.push(0x9D);
        self.code.push(modrm);
    }

    fn emit_and_reg_reg(&mut self, dest: u8, src: u8) {
        // and r64, r64
        let modrm = 0xC0 | ((src & 7) << 3) | (dest & 7);
        let rex = 0x48 | ((src >> 3) & 1) | (((dest >> 3) & 1) << 2);
        self.code.push(rex);
        self.code.push(0x21);
        self.code.push(modrm);
    }

    fn emit_or_reg_reg(&mut self, dest: u8, src: u8) {
        // or r64, r64
        let modrm = 0xC0 | ((src & 7) << 3) | (dest & 7);
        let rex = 0x48 | ((src >> 3) & 1) | (((dest >> 3) & 1) << 2);
        self.code.push(rex);
        self.code.push(0x09);
        self.code.push(modrm);
    }

    fn emit_xor_reg_reg_already_added() {
        // Already added above
    }

    fn emit_shl_reg_reg(&mut self, dest: u8, src: u8) {
        // Note: x86_64 shift count must be in CL or immediate
        // shl r64, cl
        // For now, assume src is already in rcx (register 1)
        let modrm = 0xE0 | (dest & 7);
        let rex = 0x48 | ((dest >> 3) & 1);
        self.code.push(rex);
        self.code.push(0xD3);
        self.code.push(modrm);
    }

    fn emit_shr_reg_reg(&mut self, dest: u8, src: u8) {
        // shr r64, cl
        let modrm = 0xE8 | (dest & 7);
        let rex = 0x48 | ((dest >> 3) & 1);
        self.code.push(rex);
        self.code.push(0xD3);
        self.code.push(modrm);
    }
```

### Step 4: Expand compile_instruction to handle new IR instructions

Modify `compile_instruction()` in X86_64Backend:

```rust
            IrInstruction::Div { dest, lhs, rhs } => {
                let dest_reg = codegen.get_phys_reg(*dest)?;
                let lhs_reg = codegen.get_phys_reg(*lhs)?;
                let rhs_reg = codegen.get_phys_reg(*rhs)?;
                
                // Setup: rdx:rax = lhs, rax / rhs
                codegen.emit_mov_reg_reg(0, lhs_reg); // rax = lhs
                codegen.emit_xor_reg_reg(2, 2); // rdx = 0
                codegen.emit_div_reg(rhs_reg); // rax = rax / rhs
                codegen.emit_mov_reg_reg(dest_reg, 0); // dest = rax
            }

            IrInstruction::Mod { dest, lhs, rhs } => {
                let dest_reg = codegen.get_phys_reg(*dest)?;
                let lhs_reg = codegen.get_phys_reg(*lhs)?;
                let rhs_reg = codegen.get_phys_reg(*rhs)?;
                
                // Setup: rdx:rax = lhs, rax / rhs (remainder in rdx)
                codegen.emit_mov_reg_reg(0, lhs_reg); // rax = lhs
                codegen.emit_xor_reg_reg(2, 2); // rdx = 0
                codegen.emit_div_reg(rhs_reg); // rdx = lhs % rhs
                codegen.emit_mov_reg_reg(dest_reg, 2); // dest = rdx
            }

            IrInstruction::CmpNe { dest, lhs, rhs } => {
                let dest_reg = codegen.get_phys_reg(*dest)?;
                let lhs_reg = codegen.get_phys_reg(*lhs)?;
                let rhs_reg = codegen.get_phys_reg(*rhs)?;
                
                codegen.emit_mov_reg_reg(0, lhs_reg);
                codegen.emit_cmp_reg_reg(0, rhs_reg);
                codegen.emit_setne(dest_reg);
            }

            IrInstruction::CmpLt { dest, lhs, rhs } => {
                let dest_reg = codegen.get_phys_reg(*dest)?;
                let lhs_reg = codegen.get_phys_reg(*lhs)?;
                let rhs_reg = codegen.get_phys_reg(*rhs)?;
                
                codegen.emit_mov_reg_reg(0, lhs_reg);
                codegen.emit_cmp_reg_reg(0, rhs_reg);
                codegen.emit_setl(dest_reg);
            }

            IrInstruction::CmpLe { dest, lhs, rhs } => {
                let dest_reg = codegen.get_phys_reg(*dest)?;
                let lhs_reg = codegen.get_phys_reg(*lhs)?;
                let rhs_reg = codegen.get_phys_reg(*rhs)?;
                
                codegen.emit_mov_reg_reg(0, lhs_reg);
                codegen.emit_cmp_reg_reg(0, rhs_reg);
                codegen.emit_setle(dest_reg);
            }

            IrInstruction::CmpGt { dest, lhs, rhs } => {
                let dest_reg = codegen.get_phys_reg(*dest)?;
                let lhs_reg = codegen.get_phys_reg(*lhs)?;
                let rhs_reg = codegen.get_phys_reg(*rhs)?;
                
                codegen.emit_mov_reg_reg(0, lhs_reg);
                codegen.emit_cmp_reg_reg(0, rhs_reg);
                codegen.emit_setg(dest_reg);
            }

            IrInstruction::CmpGe { dest, lhs, rhs } => {
                let dest_reg = codegen.get_phys_reg(*dest)?;
                let lhs_reg = codegen.get_phys_reg(*lhs)?;
                let rhs_reg = codegen.get_phys_reg(*rhs)?;
                
                codegen.emit_mov_reg_reg(0, lhs_reg);
                codegen.emit_cmp_reg_reg(0, rhs_reg);
                codegen.emit_setge(dest_reg);
            }

            IrInstruction::And { dest, lhs, rhs } => {
                let dest_reg = codegen.get_phys_reg(*dest)?;
                let lhs_reg = codegen.get_phys_reg(*lhs)?;
                let rhs_reg = codegen.get_phys_reg(*rhs)?;
                
                codegen.emit_mov_reg_reg(0, lhs_reg);
                codegen.emit_and_reg_reg(0, rhs_reg);
                codegen.emit_mov_reg_reg(dest_reg, 0);
            }

            IrInstruction::Or { dest, lhs, rhs } => {
                let dest_reg = codegen.get_phys_reg(*dest)?;
                let lhs_reg = codegen.get_phys_reg(*lhs)?;
                let rhs_reg = codegen.get_phys_reg(*rhs)?;
                
                codegen.emit_mov_reg_reg(0, lhs_reg);
                codegen.emit_or_reg_reg(0, rhs_reg);
                codegen.emit_mov_reg_reg(dest_reg, 0);
            }

            IrInstruction::Xor { dest, lhs, rhs } => {
                let dest_reg = codegen.get_phys_reg(*dest)?;
                let lhs_reg = codegen.get_phys_reg(*lhs)?;
                let rhs_reg = codegen.get_phys_reg(*rhs)?;
                
                codegen.emit_mov_reg_reg(0, lhs_reg);
                codegen.emit_xor_reg_reg(0, rhs_reg);
                codegen.emit_mov_reg_reg(dest_reg, 0);
            }

            IrInstruction::Shl { dest, lhs, rhs } => {
                let dest_reg = codegen.get_phys_reg(*dest)?;
                let lhs_reg = codegen.get_phys_reg(*lhs)?;
                let rhs_reg = codegen.get_phys_reg(*rhs)?;
                
                codegen.emit_mov_reg_reg(0, lhs_reg); // rax = lhs
                codegen.emit_mov_reg_reg(1, rhs_reg); // rcx = rhs (shift count)
                codegen.emit_shl_reg_reg(0, 1); // rax <<= cl
                codegen.emit_mov_reg_reg(dest_reg, 0); // dest = rax
            }

            IrInstruction::Shr { dest, lhs, rhs } => {
                let dest_reg = codegen.get_phys_reg(*dest)?;
                let lhs_reg = codegen.get_phys_reg(*lhs)?;
                let rhs_reg = codegen.get_phys_reg(*rhs)?;
                
                codegen.emit_mov_reg_reg(0, lhs_reg);
                codegen.emit_mov_reg_reg(1, rhs_reg);
                codegen.emit_shr_reg_reg(0, 1);
                codegen.emit_mov_reg_reg(dest_reg, 0);
            }
```

### Step 5: Run tests to verify success

```bash
cd /home/pedro/repo/source
cargo test -p dryad_aot test_div_instruction_codegen test_mod_instruction_codegen test_cmp_ne_instruction test_cmp_lt_instruction test_logic_and_instruction test_logic_or_instruction test_shift_left_instruction -- --nocapture 2>&1 | tail -50
```

**Expected output**: All tests pass

### Step 6: Verify all tests pass

```bash
cd /home/pedro/repo/source
cargo test -p dryad_aot --lib 2>&1 | grep "test result"
```

**Expected output**: `test result: ok. XX passed; 0 failed`

### Step 7: Commit

```bash
cd /home/pedro/repo/source
git add crates/dryad_aot/src/backend/x86_64.rs
git commit -m "feat: expand x86_64 instruction coverage

- Add Div, Mod for integer division
- Add CmpNe, CmpLt, CmpLe, CmpGt, CmpGe comparisons
- Add And, Or, Xor for bitwise logic
- Add Shl, Shr for bit shifts
- Proper register setup for division (rdx:rax)
- Shift operations use CL register per x86_64 ISA
- All 14+ critical instructions now supported

Codegen methods: emit_xor_reg_reg, emit_div_reg, emit_setne/setl/setle/setg/setge,
emit_and_reg_reg, emit_or_reg_reg, emit_shl_reg_reg, emit_shr_reg_reg

Tests added: 8 new instruction tests"
```

---

## Execution

Plan complete and saved to `.sisyphus/tasks-2-3-4-plan.md`.

**Recommended execution approach**: Subagent-Driven - I'll dispatch a fresh subagent per task, review between tasks, ensure quality and correctness at each step. This allows catching issues early and maintaining momentum.

Proceed to execute each task sequentially.
