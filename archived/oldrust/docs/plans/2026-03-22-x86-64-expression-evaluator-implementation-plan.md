# x86_64 Expression Evaluator Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement full x86_64 code generation for the Dryad AOT compiler, enabling compiled binaries to execute arithmetic/logical expressions and call built-in functions like `print()`.

**Architecture:** The x86_64 backend receives an IR module and emits raw machine code bytes. Each IR instruction maps to one or more x86_64 instructions. The PE/ELF generators embed these bytes into proper executable formats with correct headers and section attributes.

**Tech Stack:** Rust, x86_64 machine code (Intel syntax), PE32+/ELF binary formats

---

## Phase 1: Missing Comparison Instructions

### Task 1.1: Implement CmpNe Instruction

**Files:**
- Modify: `crates/dryad_aot/src/backend/x86_64.rs:100-166`

**Step 1: Write the failing test**

Add to the test module in `x86_64.rs`:

```rust
#[test]
fn test_cmp_ne_instruction() {
    let mut codegen = X86_64Codegen::new_for_test();
    
    // Setup: load 5 into rax, 3 into rcx
    codegen.emit_mov_imm64(0, 5);
    codegen.emit_mov_imm64(1, 3);
    
    // cmp rax, rcx
    codegen.emit_cmp_reg_reg(0, 1);
    // setne al
    codegen.emit_setne(0);
    
    // Verify setne byte (0x95) is present
    assert!(codegen.code.contains(&0x95), "setne opcode not found");
    assert!(codegen.code.len() > 5);
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p dryad_aot x86_64::tests::test_cmp_ne_instruction -- --nocapture`
Expected: PASS (instruction already exists via `emit_setne`)

**Step 3: Add IrInstruction::CmpNe handler**

In `compile_instruction` method, add:

```rust
IrInstruction::CmpNe { dest, lhs, rhs } => {
    let dest_reg = codegen.get_phys_reg(*dest)?;
    let lhs_reg = codegen.get_phys_reg(*lhs)?;
    let rhs_reg = codegen.get_phys_reg(*rhs)?;

    codegen.emit_mov_reg_reg(0, lhs_reg);
    codegen.emit_cmp_reg_reg(0, rhs_reg);
    codegen.emit_setne(dest_reg);
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test -p dryad_aot x86_64::tests -- --nocapture`
Expected: PASS

**Step 5: Commit**

```bash
git add crates/dryad_aot/src/backend/x86_64.rs
git commit -m "feat: implement CmpNe instruction in x86_64 backend"
```

---

### Task 1.2: Implement CmpLt, CmpLe, CmpGt, CmpGe

**Files:**
- Modify: `crates/dryad_aot/src/backend/x86_64.rs:100-166`

**Step 1: Add test for all comparison instructions**

```rust
#[test]
fn test_cmp_lt_instruction() {
    let mut codegen = X86_64Codegen::new_for_test();
    codegen.emit_mov_imm64(0, 3);
    codegen.emit_mov_imm64(1, 5);
    codegen.emit_cmp_reg_reg(0, 1);
    codegen.emit_setl(0);
    assert!(codegen.code.contains(&0x9C), "setl opcode not found");
}

#[test]
fn test_cmp_le_instruction() {
    let mut codegen = X86_64Codegen::new_for_test();
    codegen.emit_mov_imm64(0, 3);
    codegen.emit_mov_imm64(1, 5);
    codegen.emit_cmp_reg_reg(0, 1);
    codegen.emit_setle(0);
    assert!(codegen.code.contains(&0x9E), "setle opcode not found");
}

#[test]
fn test_cmp_gt_instruction() {
    let mut codegen = X86_64Codegen::new_for_test();
    codegen.emit_mov_imm64(0, 5);
    codegen.emit_mov_imm64(1, 3);
    codegen.emit_cmp_reg_reg(0, 1);
    codegen.emit_setg(0);
    assert!(codegen.code.contains(&0x9F), "setg opcode not found");
}

#[test]
fn test_cmp_ge_instruction() {
    let mut codegen = X86_64Codegen::new_for_test();
    codegen.emit_mov_imm64(0, 5);
    codegen.emit_mov_imm64(1, 3);
    codegen.emit_cmp_reg_reg(0, 1);
    codegen.emit_setge(0);
    assert!(codegen.code.contains(&0x9D), "setge opcode not found");
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test -p dryad_aot x86_64::tests::test_cmp_lt_instruction -- --nocapture`
Expected: FAIL (emit_setl not defined in IrInstruction handler)

**Step 3: Add all comparison handlers**

In `compile_instruction`:

```rust
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
```

**Step 4: Run tests to verify they pass**

Run: `cargo test -p dryad_aot x86_64::tests -- --nocapture`
Expected: PASS

**Step 5: Commit**

```bash
git add crates/dryad_aot/src/backend/x86_64.rs
git commit -m "feat: implement CmpLt, CmpLe, CmpGt, CmpGe in x86_64 backend"
```

---

## Phase 2: Bitwise Operations

### Task 2.1: Implement And, Or, Xor, Not Instructions

**Files:**
- Modify: `crates/dryad_aot/src/backend/x86_64.rs:100-166`

**Step 1: Add tests**

```rust
#[test]
fn test_bitwise_and_instruction() {
    let mut codegen = X86_64Codegen::new_for_test();
    codegen.emit_mov_imm64(0, 0b1100);
    codegen.emit_mov_imm64(1, 0b1010);
    codegen.emit_and_reg_reg(0, 1);
    // Result: 0b1000
    assert!(codegen.code.contains(&0x21)); // and r/m64, r64
}

#[test]
fn test_bitwise_or_instruction() {
    let mut codegen = X86_64Codegen::new_for_test();
    codegen.emit_mov_imm64(0, 0b1100);
    codegen.emit_mov_imm64(1, 0b1010);
    codegen.emit_or_reg_reg(0, 1);
    assert!(codegen.code.contains(&0x09)); // or r/m64, r64
}

#[test]
fn test_bitwise_xor_instruction() {
    let mut codegen = X86_64Codegen::new_for_test();
    codegen.emit_mov_imm64(0, 0b1100);
    codegen.emit_mov_imm64(1, 0b1010);
    codegen.emit_xor_reg_reg(0, 1);
    assert!(codegen.code.contains(&0x31)); // xor r/m64, r64
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test -p dryad_aot x86_64::tests::test_bitwise_and -- --nocapture`
Expected: FAIL

**Step 3: Add IrInstruction handlers**

```rust
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
```

**Step 4: Add emit_not method**

In `X86_64Codegen`:

```rust
fn emit_not(&mut self, reg: u8) {
    // not r64
    let modrm = 0xC0 | ((reg & 7) << 3) | (reg & 7);
    let rex = 0x48 | ((reg >> 3) & 1);
    self.code.push(rex);
    self.code.push(0xF7);
    self.code.push(modrm | 0x10); // 0x10 = NOT opcode extension
}
```

**Step 5: Add Not handler**

```rust
IrInstruction::Not { dest, src } => {
    let dest_reg = codegen.get_phys_reg(*dest)?;
    let src_reg = codegen.get_phys_reg(*src)?;
    codegen.emit_mov_reg_reg(dest_reg, src_reg);
    codegen.emit_not(dest_reg);
}
```

**Step 6: Run tests to verify they pass**

Run: `cargo test -p dryad_aot x86_64::tests -- --nocapture`
Expected: PASS

**Step 7: Commit**

```bash
git add crates/dryad_aot/src/backend/x86_64.rs
git commit -m "feat: implement And, Or, Xor, Not bitwise instructions"
```

---

### Task 2.2: Implement Shift Operations (Shl, Shr)

**Files:**
- Modify: `crates/dryad_aot/src/backend/x86_64.rs:100-166`

**Step 1: Add tests**

```rust
#[test]
fn test_shift_left_instruction() {
    let mut codegen = X86_64Codegen::new_for_test();
    codegen.emit_mov_imm64(0, 1);    // value = 1
    codegen.emit_mov_imm64(1, 3);    // shift = 3 (cl)
    codegen.emit_shl_reg_reg(0, 1);
    assert!(codegen.code.contains(&0xD3)); // shl r/m64, cl
}

#[test]
fn test_shift_right_instruction() {
    let mut codegen = X86_64Codegen::new_for_test();
    codegen.emit_mov_imm64(0, 8);    // value = 8
    codegen.emit_mov_imm64(1, 2);    // shift = 2 (cl)
    codegen.emit_shr_reg_reg(0, 1);
    assert!(codegen.code.contains(&0xD3)); // shr r/m64, cl
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test -p dryad_aot x86_64::tests::test_shift_left -- --nocapture`
Expected: FAIL

**Step 3: Add emit_sar method**

```rust
fn emit_sar_reg_reg(&mut self, dest: u8, src: u8) {
    // sar r64, cl (arithmetic shift right, preserves sign)
    let modrm = 0xE0 | (dest & 7);
    let rex = 0x48 | ((dest >> 3) & 1);
    self.code.push(rex);
    self.code.push(0xD3);
    self.code.push(modrm);
}
```

**Step 4: Add IrInstruction handlers**

```rust
IrInstruction::Shl { dest, lhs, rhs } => {
    let dest_reg = codegen.get_phys_reg(*dest)?;
    let lhs_reg = codegen.get_phys_reg(*lhs)?;
    let rhs_reg = codegen.get_phys_reg(*rhs)?;
    codegen.emit_mov_reg_reg(dest_reg, lhs_reg);
    // Shift count must be in rcx (register 1)
    codegen.emit_mov_reg_reg(1, rhs_reg);
    codegen.emit_shl_reg_reg(dest_reg, 1);
}

IrInstruction::Shr { dest, lhs, rhs } => {
    let dest_reg = codegen.get_phys_reg(*dest)?;
    let lhs_reg = codegen.get_phys_reg(*lhs)?;
    let rhs_reg = codegen.get_phys_reg(*rhs)?;
    codegen.emit_mov_reg_reg(dest_reg, lhs_reg);
    codegen.emit_mov_reg_reg(1, rhs_reg);
    codegen.emit_shr_reg_reg(dest_reg, 1);
}
```

**Step 5: Run tests to verify they pass**

Run: `cargo test -p dryad_aot x86_64::tests -- --nocapture`
Expected: PASS

**Step 6: Commit**

```bash
git add crates/dryad_aot/src/backend/x86_64.rs
git commit -m "feat: implement Shl, Shr shift instructions"
```

---

## Phase 3: Arithmetic Operations

### Task 3.1: Implement Div Instruction

**Files:**
- Modify: `crates/dryad_aot/src/backend/x86_64.rs:100-166`

**Step 1: Add test**

```rust
#[test]
fn test_div_instruction_codegen() {
    let mut codegen = X86_64Codegen::new_for_test();
    
    // 10 / 2
    codegen.emit_mov_imm64(0, 10);     // rax = 10
    codegen.emit_xor_reg_reg(2, 2);   // rdx = 0 (high dividend)
    codegen.emit_mov_imm64(1, 2);     // rcx = 2
    codegen.emit_div_reg(1);           // rax = 10 / 2 = 5
    
    assert!(codegen.code.contains(&0xF7)); // div opcode
    assert!(codegen.code.contains(&0xF1)); // modrm for div rcx
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p dryad_aot x86_64::tests::test_div_instruction -- --nocapture`
Expected: FAIL

**Step 3: Add IrInstruction::Div handler**

```rust
IrInstruction::Div { dest, lhs, rhs } => {
    let dest_reg = codegen.get_phys_reg(*dest)?;
    let lhs_reg = codegen.get_phys_reg(*lhs)?;
    let rhs_reg = codegen.get_phys_reg(*rhs)?;
    
    // Dividend: rdx:rax (for 64-bit division)
    codegen.emit_mov_reg_reg(0, lhs_reg);  // rax = lhs
    codegen.emit_xor_reg_reg(2, 2);        // rdx = 0 (unsigned)
    codegen.emit_div_reg(rhs_reg);         // rax = rdx:rax / rhs
    codegen.emit_mov_reg_reg(dest_reg, 0); // dest = rax
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test -p dryad_aot x86_64::tests::test_div -- --nocapture`
Expected: PASS

**Step 5: Commit**

```bash
git add crates/dryad_aot/src/backend/x86_64.rs
git commit -m "feat: implement Div instruction in x86_64 backend"
```

---

### Task 3.2: Implement Mod Instruction

**Files:**
- Modify: `crates/dryad_aot/src/backend/x86_64.rs:100-166`

**Step 1: Add test**

```rust
#[test]
fn test_mod_instruction_codegen() {
    let mut codegen = X86_64Codegen::new_for_test();
    
    // 10 % 3 = 1
    codegen.emit_mov_imm64(0, 10);     // rax = 10
    codegen.emit_xor_reg_reg(2, 2);   // rdx = 0
    codegen.emit_mov_imm64(1, 3);     // rcx = 3
    codegen.emit_div_reg(1);           // rdx = 10 % 3 = 1
    
    // Result is in rdx, need to move to dest
    codegen.emit_mov_reg_reg(0, 2);    // rax = rdx
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p dryad_aot x86_64::tests::test_mod_instruction -- --nocapture`
Expected: FAIL

**Step 3: Add IrInstruction::Mod handler**

```rust
IrInstruction::Mod { dest, lhs, rhs } => {
    let lhs_reg = codegen.get_phys_reg(*lhs)?;
    let rhs_reg = codegen.get_phys_reg(*rhs)?;
    let dest_reg = codegen.get_phys_reg(*dest)?;
    
    // Dividend: rdx:rax
    codegen.emit_mov_reg_reg(0, lhs_reg);  // rax = lhs
    codegen.emit_xor_reg_reg(2, 2);        // rdx = 0
    codegen.emit_div_reg(rhs_reg);         // rdx = lhs % rhs
    codegen.emit_mov_reg_reg(dest_reg, 2); // dest = rdx
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test -p dryad_aot x86_64::tests -- --nocapture`
Expected: PASS

**Step 5: Commit**

```bash
git add crates/dryad_aot/src/backend/x86_64.rs
git commit -m "feat: implement Mod instruction in x86_64 backend"
```

---

### Task 3.3: Implement Neg Instruction

**Files:**
- Modify: `crates/dryad_aot/src/backend/x86_64.rs:100-166`

**Step 1: Add test**

```rust
#[test]
fn test_neg_instruction() {
    let mut codegen = X86_64Codegen::new_for_test();
    codegen.emit_mov_imm64(0, 5);
    codegen.emit_neg(0);  // rax = -5
    assert!(codegen.code.contains(&0xF7)); // neg opcode
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p dryad_aot x86_64::tests::test_neg -- --nocapture`
Expected: FAIL

**Step 3: Add emit_neg method**

```rust
fn emit_neg(&mut self, reg: u8) {
    // neg r64
    let modrm = 0xD8 | (reg & 7);
    let rex = 0x48 | ((reg >> 3) & 1);
    self.code.push(rex);
    self.code.push(0xF7);
    self.code.push(modrm);
}
```

**Step 4: Add IrInstruction::Neg handler**

```rust
IrInstruction::Neg { dest, src } => {
    let dest_reg = codegen.get_phys_reg(*dest)?;
    let src_reg = codegen.get_phys_reg(*src)?;
    codegen.emit_mov_reg_reg(dest_reg, src_reg);
    codegen.emit_neg(dest_reg);
}
```

**Step 5: Run tests to verify they pass**

Run: `cargo test -p dryad_aot x86_64::tests -- --nocapture`
Expected: PASS

**Step 6: Commit**

```bash
git add crates/dryad_aot/src/backend/x86_64.rs
git commit -m "feat: implement Neg instruction in x86_64 backend"
```

---

## Phase 4: Control Flow & Runtime

### Task 4.1: Update PE Generator to Use Real Code

**Files:**
- Modify: `crates/dryad_aot/src/generator/pe.rs`

**Step 1: Read current implementation**

Check `generate_code_section` or similar method that currently writes NOPs

**Step 2: Modify to accept and write real machine code**

```rust
/// Generate code section with actual machine code
fn generate_code_section(&self, code: &[u8]) -> Vec<u8> {
    // Pad to alignment
    let alignment = 0x1000u32; // 4KB page
    let padded_size = (code.len() as u32 + alignment - 1) & !(alignment - 1);
    let mut section = Vec::with_capacity(padded_size as usize);
    section.extend(code);
    
    // Pad with zeros
    section.resize(padded_size as usize, 0);
    section
}
```

**Step 3: Update generate_object to accept machine code**

```rust
pub fn generate_object(
    &self,
    module: &IrModule,
    machine_code: &[u8],
) -> Result<Vec<u8>, String> {
    // ... existing header generation ...
    
    // Generate code section with REAL bytes
    let code_section = self.generate_code_section(machine_code);
    
    // ... rest of PE structure ...
}
```

**Step 4: Commit**

```bash
git add crates/dryad_aot/src/generator/pe.rs
git commit -m "feat: update PE generator to use real machine code"
```

---

### Task 4.2: Wire x86_64 Backend into Compilation Pipeline

**Files:**
- Modify: `crates/dryad_aot/src/compiler/mod.rs` or similar

**Step 1: Read current compiler structure**

Find where bytecode is converted to IR and where we can inject backend compilation

**Step 2: Add compile_to_binary method**

```rust
use crate::backend::x86_64::X86_64Backend;
use crate::generator::pe::PeGenerator;

/// Compile Dryad source to PE32+ binary
pub fn compile_to_binary(source: &str, output_path: &str) -> Result<(), String> {
    // 1. Lex + Parse
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;
    let parser = Parser::new(tokens);
    let ast = parser.parse()?;
    
    // 2. Compile to Bytecode
    let chunk = Compiler::new().compile(&ast);
    
    // 3. Convert to IR
    let ir_module = BytecodeToIrConverter::new().convert(&chunk)?;
    
    // 4. Generate x86_64 code [NEW]
    let backend = X86_64Backend::new();
    let machine_code = backend.compile_module(&ir_module)?;
    
    // 5. Generate PE binary with real code
    let pe_gen = PeGenerator::new();
    let pe_binary = pe_gen.generate_object(&ir_module, &machine_code)?;
    
    // 6. Write to file
    std::fs::write(output_path, pe_binary)?;
    
    Ok(())
}
```

**Step 3: Commit**

```bash
git add crates/dryad_aot/src/compiler/mod.rs
git commit -m "feat: wire x86_64 backend into compilation pipeline"
```

---

### Task 4.3: Create End-to-End Integration Test

**Files:**
- Create: `crates/dryad_aot/tests/integration_x86_64_execution.rs`

**Step 1: Write integration test**

```rust
#[test]
fn test_simple_arithmetic_compilation() {
    use dryad_aot::compiler::compile_to_binary;
    use std::process::Command;
    use std::fs;
    use tempfile::NamedTempFile;
    
    // Create temp file for binary
    let temp_bin = NamedTempFile::with_suffix(".exe").unwrap();
    let bin_path = temp_bin.path();
    
    // Compile "print(5 + 3)"
    let source = r#"
        let a = 5;
        let b = 3;
        print(a + b);
    "#;
    
    compile_to_binary(source, bin_path.to_str().unwrap())
        .expect("Compilation failed");
    
    // Execute binary
    let output = Command::new(bin_path)
        .output()
        .expect("Failed to execute binary");
    
    // Verify
    assert!(output.status.success(), "Binary exited with error");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("8"), "Expected '8' but got: {}", stdout);
}
```

**Step 2: Run test to verify it fails (missing implementation)**

Run: `cargo test -p dryad_aot integration_x86_64_execution -- --nocapture`
Expected: FAIL (compile_to_binary not exported)

**Step 3: Export compile_to_binary from lib.rs**

In `crates/dryad_aot/src/lib.rs`:
```rust
pub use compiler::compile_to_binary;
```

**Step 4: Run test again**

Expected: FAIL (actual compilation or execution)

**Step 5: Debug and fix issues**

This is where the real work happens - debug any compilation or execution issues

**Step 6: Commit**

```bash
git add crates/dryad_aot/tests/integration_x86_64_execution.rs
git add crates/dryad_aot/src/lib.rs
git add crates/dryad_aot/src/compiler/mod.rs
git commit -m "test: add end-to-end x86_64 compilation test"
```

---

## Phase 5: Verification & Polish

### Task 5.1: Run Full Test Suite

**Step 1: Run all AOT tests**

```bash
cargo test -p dryad_aot --workspace
```

**Step 2: Verify no regressions**

All tests should pass, including:
- 43 existing lib tests
- 1 existing integration test
- New tests from Phase 1-4

**Step 3: Run clippy**

```bash
cargo clippy -p dryad_aot -- -D warnings
```

**Step 4: Fix any warnings or errors**

---

### Task 5.2: Verify Binary Execution

**Step 1: Create manual test binary**

```bash
cd target/debug
# Create test source
echo 'print(5 + 3);' > test.dryad
# Compile
cargo run --bin dryad_aot -- test.dryad -o test_binary
# Execute
./test_binary
```

Expected output: `8`

**Step 2: Test multiple expressions**

```dryad
print(10 / 3);  // 3
print(10 % 3);  // 1
print(5 > 3);   // 1 (true)
print(5 < 3);   // 0 (false)
```

**Step 3: Commit verification**

```bash
git add -A
git commit -m "test: verify binary execution works correctly"
```

---

## Summary

| Phase | Tasks | Description |
|-------|-------|-------------|
| 1 | 1.1-1.2 | Comparison instructions (CmpNe, CmpLt, etc.) |
| 2 | 2.1-2.2 | Bitwise operations (And, Or, Xor, Not, Shl, Shr) |
| 3 | 3.1-3.3 | Arithmetic (Div, Mod, Neg) |
| 4 | 4.1-4.3 | Generator integration & E2E test |
| 5 | 5.1-5.2 | Verification & polish |

**Total Estimated Tasks**: 11 major tasks
**Estimated Commits**: 11-15

---

## References

- Design: `docs/plans/2026-03-22-x86-64-expression-evaluator-design.md`
- Backend: `crates/dryad_aot/src/backend/x86_64.rs`
- IR: `crates/dryad_aot/src/ir/instructions.rs`
- Generator: `crates/dryad_aot/src/generator/pe.rs`

---

**Plan Status**: Ready for Execution
**Last Updated**: 2026-03-22
