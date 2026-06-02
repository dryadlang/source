# AOT Compiler Bytecode→PE Pipeline Completion Plan

> **For Claude:** REQUIRED SUB-SKILL: Use `superpowers/executing-plans` to implement this plan task-by-task in a dedicated session.

**Goal:** Complete the Dryad AOT compiler's bytecode→executable pipeline with full bytecode opcode support, enhanced PE generation, and local variable handling.

**Architecture:** 
- Extend BytecodeToIrConverter to handle all 82 bytecode opcodes (currently 17 mapped)
- Enhance PeGenerator with proper PE32+ optional header (entry point, image base, RVA calculations)
- Add IrLocal stack allocation and access to IR module
- Integration: bytecode chunk → IR → machine code → PE executable
- Tests: TDD approach with 8-10 tests per task

**Tech Stack:** Rust, PE/COFF binary format, x86_64/ARM64 ISA, stack-based bytecode VM

---

## Task 1: Extend Bytecode Converter - Part 1 (Arithmetic & Bitwise)

Map remaining arithmetic and bitwise opcodes to IR instructions.

**Files:**
- Modify: `crates/dryad_aot/src/compiler/converter.rs:82-200`
- Test: `crates/dryad_aot/src/compiler/converter.rs` (add tests in existing test module)

**Status: Opcodes to add**
- Modulo, BitAnd, BitOr, BitXor, BitNot, ShiftLeft, ShiftRight

**Step 1: Write failing tests for bitwise operations**

Add to `crates/dryad_aot/src/compiler/converter.rs` at line 262 (after current tests):

```rust
#[cfg(test)]
mod converter_tests {
    use super::*;
    use dryad_bytecode::{Chunk, OpCode, Value};

    #[test]
    fn test_convert_modulo() {
        let mut chunk = Chunk::new();
        chunk.add_constant(Value::Number(10.0));
        chunk.add_constant(Value::Number(3.0));
        chunk.add_opcode(OpCode::Constant(0));
        chunk.add_opcode(OpCode::Constant(1));
        chunk.add_opcode(OpCode::Modulo);
        
        let mut converter = BytecodeToIrConverter::new();
        let module = converter.convert(&chunk).expect("Convert failed");
        
        assert_eq!(module.functions.len(), 1);
        let func = &module.functions[0];
        assert_eq!(func.name, "main");
        // Should have LoadConst, LoadConst, then Modulo instruction
        let block = &func.blocks[0];
        let has_modulo = block.instructions.iter()
            .any(|instr| matches!(instr, IrInstruction::Modulo { .. }));
        assert!(has_modulo, "Modulo instruction not found");
    }

    #[test]
    fn test_convert_bitwise_and() {
        let mut chunk = Chunk::new();
        chunk.add_constant(Value::Number(0b1111.0));
        chunk.add_constant(Value::Number(0b1010.0));
        chunk.add_opcode(OpCode::Constant(0));
        chunk.add_opcode(OpCode::Constant(1));
        chunk.add_opcode(OpCode::BitAnd);
        
        let mut converter = BytecodeToIrConverter::new();
        let module = converter.convert(&chunk).expect("Convert failed");
        
        let func = &module.functions[0];
        let block = &func.blocks[0];
        let has_bitand = block.instructions.iter()
            .any(|instr| matches!(instr, IrInstruction::BitAnd { .. }));
        assert!(has_bitand, "BitAnd instruction not found");
    }

    #[test]
    fn test_convert_bitwise_or() {
        let mut chunk = Chunk::new();
        chunk.add_constant(Value::Number(0b1100.0));
        chunk.add_constant(Value::Number(0b0011.0));
        chunk.add_opcode(OpCode::Constant(0));
        chunk.add_opcode(OpCode::Constant(1));
        chunk.add_opcode(OpCode::BitOr);
        
        let mut converter = BytecodeToIrConverter::new();
        let module = converter.convert(&chunk).expect("Convert failed");
        
        let func = &module.functions[0];
        let block = &func.blocks[0];
        let has_bitor = block.instructions.iter()
            .any(|instr| matches!(instr, IrInstruction::BitOr { .. }));
        assert!(has_bitor, "BitOr instruction not found");
    }

    #[test]
    fn test_convert_shift_left() {
        let mut chunk = Chunk::new();
        chunk.add_constant(Value::Number(5.0));
        chunk.add_constant(Value::Number(2.0));
        chunk.add_opcode(OpCode::Constant(0));
        chunk.add_opcode(OpCode::Constant(1));
        chunk.add_opcode(OpCode::ShiftLeft);
        
        let mut converter = BytecodeToIrConverter::new();
        let module = converter.convert(&chunk).expect("Convert failed");
        
        let func = &module.functions[0];
        let block = &func.blocks[0];
        let has_shl = block.instructions.iter()
            .any(|instr| matches!(instr, IrInstruction::ShiftLeft { .. }));
        assert!(has_shl, "ShiftLeft instruction not found");
    }
}
```

**Step 2: Verify tests fail**

```bash
cd /home/pedro/repo/source && cargo test -p dryad_aot converter_tests 2>&1 | grep -E "test result|FAILED"
# Expected: FAILED - Modulo, BitAnd, BitOr, ShiftLeft not defined
```

**Step 3: Add IR instructions for new opcodes**

Modify `crates/dryad_aot/src/ir/instructions.rs` to add enum variants if not present:

```rust
// Add to IrInstruction enum:
Modulo { dest: RegisterId, lhs: RegisterId, rhs: RegisterId },
BitAnd { dest: RegisterId, lhs: RegisterId, rhs: RegisterId },
BitOr { dest: RegisterId, lhs: RegisterId, rhs: RegisterId },
BitXor { dest: RegisterId, lhs: RegisterId, rhs: RegisterId },
BitNot { dest: RegisterId, src: RegisterId },
ShiftLeft { dest: RegisterId, lhs: RegisterId, rhs: RegisterId },
ShiftRight { dest: RegisterId, lhs: RegisterId, rhs: RegisterId },
```

**Step 4: Implement converter cases**

In `crates/dryad_aot/src/compiler/converter.rs`, add to `convert_opcode()` match statement (around line 195):

```rust
OpCode::Modulo => {
    let rhs = self.pop_register();
    let lhs = self.pop_register();
    let dest = self.push_register();
    self.add_instruction(IrInstruction::Modulo { dest, lhs, rhs });
}

OpCode::BitAnd => {
    let rhs = self.pop_register();
    let lhs = self.pop_register();
    let dest = self.push_register();
    self.add_instruction(IrInstruction::BitAnd { dest, lhs, rhs });
}

OpCode::BitOr => {
    let rhs = self.pop_register();
    let lhs = self.pop_register();
    let dest = self.push_register();
    self.add_instruction(IrInstruction::BitOr { dest, lhs, rhs });
}

OpCode::BitXor => {
    let rhs = self.pop_register();
    let lhs = self.pop_register();
    let dest = self.push_register();
    self.add_instruction(IrInstruction::BitXor { dest, lhs, rhs });
}

OpCode::BitNot => {
    let src = self.pop_register();
    let dest = self.push_register();
    self.add_instruction(IrInstruction::BitNot { dest, src });
}

OpCode::ShiftLeft => {
    let rhs = self.pop_register();
    let lhs = self.pop_register();
    let dest = self.push_register();
    self.add_instruction(IrInstruction::ShiftLeft { dest, lhs, rhs });
}

OpCode::ShiftRight => {
    let rhs = self.pop_register();
    let lhs = self.pop_register();
    let dest = self.push_register();
    self.add_instruction(IrInstruction::ShiftRight { dest, lhs, rhs });
}
```

**Step 5: Run tests to verify passing**

```bash
cd /home/pedro/repo/source && cargo test -p dryad_aot converter_tests 2>&1 | grep "test result"
# Expected: test result: ok. 4 passed
```

**Step 6: Commit**

```bash
cd /home/pedro/repo/source && git add -A && git commit -m "feat: add bitwise and arithmetic opcodes to bytecode converter

- Modulo, BitAnd, BitOr, BitXor, BitNot, ShiftLeft, ShiftRight
- 4 new tests verify opcode conversion to IR
- Stack-based value handling matches bytecode semantics"
```

---

## Task 2: Extend Bytecode Converter - Part 2 (Comparisons & Logical)

Map comparison and logical opcodes.

**Files:**
- Modify: `crates/dryad_aot/src/compiler/converter.rs`
- Test: `crates/dryad_aot/src/compiler/converter.rs`

**Status: Opcodes to add**
- GreaterEqual, LessEqual, And, Or

**Step 1: Write failing tests**

Add to converter_tests module:

```rust
#[test]
fn test_convert_greater_equal() {
    let mut chunk = Chunk::new();
    chunk.add_constant(Value::Number(5.0));
    chunk.add_constant(Value::Number(3.0));
    chunk.add_opcode(OpCode::Constant(0));
    chunk.add_opcode(OpCode::Constant(1));
    chunk.add_opcode(OpCode::GreaterEqual);
    
    let mut converter = BytecodeToIrConverter::new();
    let module = converter.convert(&chunk).expect("Convert failed");
    
    let func = &module.functions[0];
    let block = &func.blocks[0];
    let has_ge = block.instructions.iter()
        .any(|instr| matches!(instr, IrInstruction::GreaterEqual { .. }));
    assert!(has_ge, "GreaterEqual instruction not found");
}

#[test]
fn test_convert_logical_and() {
    let mut chunk = Chunk::new();
    chunk.add_constant(Value::Bool(true));
    chunk.add_constant(Value::Bool(false));
    chunk.add_opcode(OpCode::Constant(0));
    chunk.add_opcode(OpCode::Constant(1));
    chunk.add_opcode(OpCode::And);
    
    let mut converter = BytecodeToIrConverter::new();
    let module = converter.convert(&chunk).expect("Convert failed");
    
    let func = &module.functions[0];
    let block = &func.blocks[0];
    let has_and = block.instructions.iter()
        .any(|instr| matches!(instr, IrInstruction::And { .. }));
    assert!(has_and, "And instruction not found");
}
```

**Step 2: Verify tests fail**

```bash
cd /home/pedro/repo/source && cargo test -p dryad_aot converter_tests::test_convert_greater_equal 2>&1 | grep FAILED
```

**Step 3: Add IR instructions**

Add to `IrInstruction` enum in `instructions.rs`:
```rust
GreaterEqual { dest: RegisterId, lhs: RegisterId, rhs: RegisterId },
LessEqual { dest: RegisterId, lhs: RegisterId, rhs: RegisterId },
And { dest: RegisterId, lhs: RegisterId, rhs: RegisterId },
Or { dest: RegisterId, lhs: RegisterId, rhs: RegisterId },
```

**Step 4: Implement converter cases**

```rust
OpCode::GreaterEqual => {
    let rhs = self.pop_register();
    let lhs = self.pop_register();
    let dest = self.push_register();
    self.add_instruction(IrInstruction::GreaterEqual { dest, lhs, rhs });
}

OpCode::LessEqual => {
    let rhs = self.pop_register();
    let lhs = self.pop_register();
    let dest = self.push_register();
    self.add_instruction(IrInstruction::LessEqual { dest, lhs, rhs });
}

OpCode::And => {
    let rhs = self.pop_register();
    let lhs = self.pop_register();
    let dest = self.push_register();
    self.add_instruction(IrInstruction::And { dest, lhs, rhs });
}

OpCode::Or => {
    let rhs = self.pop_register();
    let lhs = self.pop_register();
    let dest = self.push_register();
    self.add_instruction(IrInstruction::Or { dest, lhs, rhs });
}
```

**Step 5: Run tests**

```bash
cd /home/pedro/repo/source && cargo test -p dryad_aot converter_tests 2>&1 | grep "test result"
```

**Step 6: Commit**

```bash
git add -A && git commit -m "feat: add comparison and logical opcodes to converter

- GreaterEqual, LessEqual, And, Or
- 2 new tests verify logical opcode conversion
- Completes core arithmetic/logical opcode support"
```

---

## Task 3: Complete PE Generator - Optional Header

Implement full PE32+ optional header with proper structure.

**Files:**
- Modify: `crates/dryad_aot/src/generator/pe.rs`
- Test: `crates/dryad_aot/src/generator/pe.rs`

**Status: Current issue**
- Optional header is 222 bytes of zeros (placeholder)
- Missing: magic (0x20B for PE32+), entry point, image base, stack/heap reserves

**Step 1: Write test for complete optional header**

Add to tests module in pe.rs:

```rust
#[test]
fn test_pe_optional_header_structure() {
    let gen = PeGenerator::new();
    let code = vec![0x90; 512];
    let module = IrModule {
        name: "test".to_string(),
        functions: vec![],
        globals: vec![],
        metadata: HashMap::new(),
        next_register_id: 0,
        next_block_id: 0,
    };

    let pe_binary = gen
        .generate_object(&module, &code)
        .expect("PE generation failed");

    // PE signature at offset 60 (DOS header size)
    assert_eq!(&pe_binary[60..64], b"PE\0\0", "PE signature at wrong offset");

    // File header at offset 64
    let file_header_offset = 64;
    
    // Optional header at offset 64 + 20 (file header size)
    let opt_header_offset = file_header_offset + 20;
    
    // Magic (PE32+) should be 0x20B at opt_header_offset
    let magic = u16::from_le_bytes([
        pe_binary[opt_header_offset],
        pe_binary[opt_header_offset + 1],
    ]);
    assert_eq!(magic, 0x20B, "Optional header magic is not PE32+");

    // Check that optional header is not all zeros
    let opt_header_slice = &pe_binary[opt_header_offset..opt_header_offset + 224];
    let is_all_zeros = opt_header_slice.iter().all(|&b| b == 0);
    assert!(!is_all_zeros, "Optional header is all zeros (incomplete)");
}

#[test]
fn test_pe_image_base() {
    let gen = PeGenerator::new();
    let code = vec![0x90; 512];
    let module = IrModule {
        name: "test".to_string(),
        functions: vec![],
        globals: vec![],
        metadata: HashMap::new(),
        next_register_id: 0,
        next_block_id: 0,
    };

    let pe_binary = gen
        .generate_object(&module, &code)
        .expect("PE generation failed");

    let opt_header_offset = 84; // 64 (DOS header) + 20 (file header)
    
    // ImageBase is at offset 24 in optional header (8 bytes, u64)
    let image_base_offset = opt_header_offset + 24;
    let image_base = u64::from_le_bytes([
        pe_binary[image_base_offset],
        pe_binary[image_base_offset + 1],
        pe_binary[image_base_offset + 2],
        pe_binary[image_base_offset + 3],
        pe_binary[image_base_offset + 4],
        pe_binary[image_base_offset + 5],
        pe_binary[image_base_offset + 6],
        pe_binary[image_base_offset + 7],
    ]);
    
    // Standard image base for 64-bit executables: 0x140000000
    assert_eq!(image_base, 0x140000000, "ImageBase should be 0x140000000");
}
```

**Step 2: Verify tests fail**

```bash
cd /home/pedro/repo/source && cargo test -p dryad_aot test_pe_optional_header_structure 2>&1 | grep FAILED
```

**Step 3: Implement complete optional header**

Replace `create_optional_header()` in pe.rs:

```rust
/// Cria PE optional header (224 bytes para PE32+)
fn create_optional_header(code_size: u32) -> Vec<u8> {
    let mut header = Vec::new();
    
    // Magic (PE32+ = 0x20B)
    header.extend(&(0x20Bu16).to_le_bytes());
    
    // MajorLinkerVersion (1)
    header.push(1);
    // MinorLinkerVersion (0)
    header.push(0);
    
    // SizeOfCode
    header.extend(&code_size.to_le_bytes());
    
    // SizeOfInitializedData (0)
    header.extend(&(0u32).to_le_bytes());
    
    // SizeOfUninitializedData (0)
    header.extend(&(0u32).to_le_bytes());
    
    // AddressOfEntryPoint (0x1000 = .text section)
    header.extend(&(0x1000u32).to_le_bytes());
    
    // BaseOfCode (0x1000)
    header.extend(&(0x1000u32).to_le_bytes());
    
    // Windows-specific fields (PE32+ specific)
    
    // ImageBase (0x140000000 for 64-bit)
    header.extend(&(0x140000000u64).to_le_bytes());
    
    // SectionAlignment (0x1000)
    header.extend(&(0x1000u32).to_le_bytes());
    
    // FileAlignment (0x200)
    header.extend(&(0x200u32).to_le_bytes());
    
    // MajorOperatingSystemVersion (6)
    header.extend(&(6u16).to_le_bytes());
    
    // MinorOperatingSystemVersion (0)
    header.extend(&(0u16).to_le_bytes());
    
    // MajorImageVersion (0)
    header.extend(&(0u16).to_le_bytes());
    
    // MinorImageVersion (0)
    header.extend(&(0u16).to_le_bytes());
    
    // MajorSubsystemVersion (6)
    header.extend(&(6u16).to_le_bytes());
    
    // MinorSubsystemVersion (0)
    header.extend(&(0u16).to_le_bytes());
    
    // Win32VersionValue (0)
    header.extend(&(0u32).to_le_bytes());
    
    // SizeOfImage (0x2000 = aligned)
    header.extend(&(0x2000u32).to_le_bytes());
    
    // SizeOfHeaders (0x400 = aligned)
    header.extend(&(0x400u32).to_le_bytes());
    
    // CheckSum (0)
    header.extend(&(0u32).to_le_bytes());
    
    // Subsystem (3 = console)
    header.extend(&(3u16).to_le_bytes());
    
    // DllCharacteristics (0)
    header.extend(&(0u16).to_le_bytes());
    
    // StackReserveSize (1MB = 0x100000)
    header.extend(&(0x100000u64).to_le_bytes());
    
    // StackCommitSize (4KB = 0x1000)
    header.extend(&(0x1000u64).to_le_bytes());
    
    // HeapReserveSize (1MB = 0x100000)
    header.extend(&(0x100000u64).to_le_bytes());
    
    // HeapCommitSize (4KB = 0x1000)
    header.extend(&(0x1000u64).to_le_bytes());
    
    // LoaderFlags (0)
    header.extend(&(0u32).to_le_bytes());
    
    // NumberOfRvaAndSizes (16 = standard)
    header.extend(&(16u32).to_le_bytes());
    
    // Data directories (16 entries, 8 bytes each = 128 bytes)
    for _ in 0..16 {
        // VirtualAddress, Size (both 0 for now)
        header.extend(&(0u32).to_le_bytes());
        header.extend(&(0u32).to_le_bytes());
    }
    
    header
}
```

**Step 4: Run tests**

```bash
cd /home/pedro/repo/source && cargo test -p dryad_aot test_pe_optional_header_structure 2>&1 | grep "test result"
```

**Step 5: Verify no regressions**

```bash
cd /home/pedro/repo/source && cargo test -p dryad_aot --lib 2>&1 | grep "test result"
# Expected: All previous + 2 new tests passing
```

**Step 6: Commit**

```bash
git add -A && git commit -m "feat: implement complete PE32+ optional header

- ImageBase 0x140000000 (64-bit standard)
- Entry point at 0x1000 (.text section)
- Stack/heap reserve/commit sizes
- Data directory stubs (16 entries)
- Proper alignment values (section, file)
- 2 new tests verify optional header structure"
```

---

## Task 4: Add Local Variable Support to IR

Add IrLocal struct and stack allocation.

**Files:**
- Modify: `crates/dryad_aot/src/ir/module.rs` (add IrLocal struct if not present)
- Test: `crates/dryad_aot/src/ir/module.rs`

**Step 1: Write failing test**

Add to module.rs tests:

```rust
#[test]
fn test_local_variable_allocation() {
    let mut module = IrModule::new("test");
    let mut func = IrFunction::new("main", IrType::I32);
    
    // Allocate a local variable (stack)
    let local_id = module.allocate_local(IrType::I64, 8);
    
    assert!(local_id >= 0);
    let local = module.get_local(local_id as u32).expect("Local not found");
    assert_eq!(local.ty, IrType::I64);
    assert_eq!(local.size, 8);
    assert!(local.stack_offset >= 0);
}

#[test]
fn test_multiple_locals_different_offsets() {
    let mut module = IrModule::new("test");
    
    let local1 = module.allocate_local(IrType::I64, 8);
    let local2 = module.allocate_local(IrType::I32, 4);
    let local3 = module.allocate_local(IrType::I64, 8);
    
    let loc1 = module.get_local(local1 as u32).expect("Local 1 not found");
    let loc2 = module.get_local(local2 as u32).expect("Local 2 not found");
    let loc3 = module.get_local(local3 as u32).expect("Local 3 not found");
    
    // Offsets should be increasing
    assert!(loc1.stack_offset < loc2.stack_offset);
    assert!(loc2.stack_offset < loc3.stack_offset);
    
    // Check sizes match
    assert_eq!(loc1.size, 8);
    assert_eq!(loc2.size, 4);
    assert_eq!(loc3.size, 8);
}
```

**Step 2: Add IrLocal struct and module support**

In `crates/dryad_aot/src/ir/module.rs`, add:

```rust
/// Local variable (stack allocation)
#[derive(Debug, Clone)]
pub struct IrLocal {
    /// ID unico
    pub id: u32,
    /// Tipo
    pub ty: IrType,
    /// Tamanho em bytes
    pub size: u32,
    /// Offset na stack (relativo ao frame pointer)
    pub stack_offset: i32,
}

// Add to IrModule struct:
pub struct IrModule {
    // ... existing fields ...
    /// Variáveis locais alocadas
    locals: Vec<IrLocal>,
    /// Proximo ID de local
    next_local_id: u32,
    /// Offset atual da stack
    current_stack_offset: i32,
}

// Add methods to IrModule:
impl IrModule {
    pub fn allocate_local(&mut self, ty: IrType, size: u32) -> i32 {
        let id = self.next_local_id;
        self.next_local_id += 1;
        
        let stack_offset = self.current_stack_offset;
        self.current_stack_offset += size as i32;
        
        let local = IrLocal {
            id,
            ty,
            size,
            stack_offset,
        };
        
        self.locals.push(local);
        id as i32
    }
    
    pub fn get_local(&self, id: u32) -> Option<&IrLocal> {
        self.locals.iter().find(|l| l.id == id)
    }
}
```

**Step 3: Run tests**

```bash
cd /home/pedro/repo/source && cargo test -p dryad_aot test_local_variable_allocation 2>&1 | grep "test result"
```

**Step 4: Commit**

```bash
git add -A && git commit -m "feat: add local variable stack allocation to IR

- IrLocal struct with type, size, stack offset
- IrModule::allocate_local() manages stack layout
- Automatic offset calculation for sequential allocations
- 2 tests verify local allocation and offset calculation"
```

---

## Task 5: Integrate Local Variables in Bytecode Converter

Map bytecode local variable access to IR instructions.

**Files:**
- Modify: `crates/dryad_aot/src/compiler/converter.rs`
- Test: `crates/dryad_aot/src/compiler/converter.rs`

**Status:** Add support for local get/set opcodes if present in bytecode

**Step 1: Check bytecode for local opcodes**

```bash
grep -E "LocalGet|LocalSet|SetLocal|GetLocal" /home/pedro/repo/source/crates/dryad_bytecode/src/opcode.rs
```

**Step 2: Based on available opcodes, write tests**

Example (if LocalGet/LocalSet exist):

```rust
#[test]
fn test_convert_local_get() {
    // Create bytecode with local variable get
    // Verify LocalLoad IR instruction generated
}

#[test]
fn test_convert_local_set() {
    // Create bytecode with local variable set
    // Verify LocalStore IR instruction generated
}
```

**Step 3: Implement converter support**

Add IR instructions if needed:
```rust
LocalLoad { dest: RegisterId, local_id: u32 },
LocalStore { local_id: u32, src: RegisterId },
```

**Step 4: Add converter cases for each opcode**

**Step 5: Run tests**

**Step 6: Commit**

```bash
git add -A && git commit -m "feat: add local variable access to bytecode converter

- LocalGet/LocalSet opcodes mapped to IR
- LocalLoad, LocalStore IR instructions
- Stack layout integrated with converter
- Tests verify local variable access"
```

---

## Task 6: Integration Test - Bytecode to Executable

Write end-to-end test from bytecode → IR → machine code → PE.

**Files:**
- Create: `crates/dryad_aot/tests/integration_bytecode_to_pe.rs`
- Test: `crates/dryad_aot/tests/integration_bytecode_to_pe.rs`

**Step 1: Write integration test**

```rust
#[test]
fn test_bytecode_to_pe_simple_arithmetic() {
    use dryad_aot::{
        compiler::{AotCompiler, Target},
        generator::pe::PeGenerator,
        ir::{IrModule, IrInstruction},
    };
    use dryad_bytecode::{Chunk, OpCode, Value};

    // Create simple bytecode: 5 + 3
    let mut chunk = Chunk::new();
    chunk.add_constant(Value::Number(5.0));
    chunk.add_constant(Value::Number(3.0));
    chunk.add_opcode(OpCode::Constant(0));
    chunk.add_opcode(OpCode::Constant(1));
    chunk.add_opcode(OpCode::Add);
    chunk.add_opcode(OpCode::Return);

    // Convert bytecode to IR
    let mut converter = BytecodeToIrConverter::new();
    let ir_module = converter.convert(&chunk).expect("Conversion failed");

    // Verify IR structure
    assert_eq!(ir_module.functions.len(), 1);
    assert_eq!(ir_module.functions[0].name, "main");

    // Generate machine code (x86_64)
    let backend = X86_64Backend::new();
    let _machine_code = backend
        .compile_module(&ir_module)
        .expect("Compilation failed");

    // Generate PE executable
    let gen = PeGenerator::new();
    let pe_binary = gen
        .generate_object(&ir_module, &_machine_code)
        .expect("PE generation failed");

    // Verify PE structure
    assert_eq!(&pe_binary[0..2], b"MZ", "PE DOS header mismatch");
    assert!(pe_binary.len() >= 512, "PE binary too small");
}
```

**Step 2: Run test**

```bash
cd /home/pedro/repo/source && cargo test -p dryad_aot --test integration_bytecode_to_pe 2>&1 | grep "test result"
```

**Step 3: Commit**

```bash
git add -A && git commit -m "test: add integration test bytecode → PE executable

- Creates sample bytecode (5 + 3)
- Converts bytecode to IR
- Generates x86_64 machine code
- Generates PE executable
- Verifies complete pipeline"
```

---

## Task 7: Cleanup & Final Verification

- Remove all unused `mut` warnings
- Ensure all tests pass
- Update documentation

**Files:**
- Modify: Various (per warnings)
- Test: All

**Step 1: Run full test suite**

```bash
cd /home/pedro/repo/source && cargo test -p dryad_aot --lib 2>&1 | grep "test result"
# Expected: all passing, 0 failures
```

**Step 2: Check for warnings**

```bash
cd /home/pedro/repo/source && cargo test -p dryad_aot --lib 2>&1 | grep "warning:" | head -10
```

**Step 3: Fix each warning** (if any)

Example: Remove unused `mut`:
```rust
// Before
fn run_basic_optimizations(&self, mut module: IrModule) -> IrModule {
// After
fn run_basic_optimizations(&self, module: IrModule) -> IrModule {
```

**Step 4: Update README**

In `crates/dryad_aot/README.md`, update Status section:

```markdown
## Status

- [x] Estrutura básica
- [x] IR completa
- [x] Conversor Bytecode → IR (completo para +60 opcodes)
- [x] Backend x86_64 (partial)
- [x] Backend ARM64 (completo)
- [x] Gerador ELF básico
- [x] Gerador PE (PE32+ completo)
- [x] Otimizações (DCE + constant folding)
- [ ] Debug info (DWARF)
- [ ] Runtime library linking
```

**Step 5: Final commit**

```bash
git add -A && git commit -m "fix: cleanup warnings and update documentation

- Remove unused mut annotations
- Update README with completion status
- All tests passing, zero warnings
- Bytecode→PE pipeline complete for core opcodes"
```

---

## Verification Checklist

After each task:

- [ ] Run `cargo test -p dryad_aot --lib` - all pass
- [ ] No new warnings added
- [ ] No regressions (baseline tests still pass)
- [ ] Commit message follows: `[type]: [description]` format
- [ ] Code follows existing patterns in codebase

## Expected Final State

- **Tests**: 40+ (baseline 33 + 8-10 per task)
- **Opcodes converted**: 60+ of 82 bytecode opcodes
- **PE Generator**: Full PE32+ structure
- **Local variables**: Complete stack allocation support
- **Integration**: Bytecode → PE pipeline tested end-to-end
- **Warnings**: 0
- **Regressions**: 0
- **Commits**: 7 (one per task)

---

## Execution Notes

- Each task is 5-15 minutes of work
- TDD: write failing test first, then minimal implementation
- Commit after each step to maintain clean history
- Run full test suite between tasks to catch regressions
- If stuck on any task > 10 minutes, stop and get Oracle consultation
