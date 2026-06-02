# x86_64 Expression Evaluator - Design Document

**Date**: 2026-03-22  
**Author**: Tech Lead  
**Status**: Approved  
**Version**: 1.0

---

## 1. Overview

### 1.1 Goal

Implement full x86_64 code generation for the Dryad AOT compiler, enabling compiled binaries to:
- Execute arithmetic and logical expressions
- Call Dryad built-in functions (like `print()`)
- Return computed values
- Be recognized and run by the operating system

### 1.2 Scope

**In Scope**:
- Complete IR instruction → x86_64 machine code implementation
- Entry point with minimal runtime bridge
- Integration with PE/ELF generators
- End-to-end integration tests

**Out of Scope** (for v1.0LTS):
- DWARF debug symbols
- Full standard library
- Dynamic linking
- ARM64 code generation (exists as scaffolding)

---

## 2. Architecture

### 2.1 Compilation Pipeline

```
Dryad Source ("print(5 + 3)")
        ↓
Bytecode Compiler (dryad_bytecode)
        ↓
Bytecode Chunk [Constant(5), Constant(3), Add, Call(print), Return]
        ↓
BytecodeToIrConverter (existing)
        ↓
IR Module [LoadConst, Add, Call, Return]
        ↓
X86_64Backend (enhanced) [NEW: full codegen]
        ↓
Raw Machine Code [bytes]
        ↓
PeGenerator / ElfGenerator (updated)
        ↓
Executable File (PE/ELF)
```

### 2.2 Entry Point Structure

```
_binary_start:

; Standard prologue
push rbp
mov rbp, rsp

; Reserve stack space for locals
sub rsp, <frame_size>

; Call compiled main function
call _main

; Exit with result
mov rdi, rax        ; exit code = return value
mov rax, 60         ; sys_exit syscall number
syscall

; === COMPILED MAIN FUNCTION ===
_main:
    push rbp
    mov rbp, rsp
    
    ; Generated code here:
    ; Load constants, perform operations, call functions
    
    pop rbp
    ret

; === RUNTIME FUNCTIONS ===
_dryad_print:
    ; Bridge to Dryad's print builtin
    ; ... (see Section 3.3)
    ret
```

---

## 3. IR Instructions → x86_64 Implementation

### 3.1 Already Implemented (Scaffolding)

| Instruction | Status | Handler |
|-------------|--------|---------|
| `LoadConst` | ✅ Working | `emit_mov_imm32/64` |
| `Add` | ✅ Working | `emit_mov_reg_reg`, `emit_add_reg_reg` |
| `Sub` | ✅ Working | `emit_sub_reg_reg` |
| `Mul` | ✅ Working | `emit_imul_reg_reg` |
| `CmpEq` | ✅ Working | `emit_cmp_reg_reg`, `emit_sete` |
| `Return` | ✅ Working | `emit_ret`, register restore |

### 3.2 Missing Instructions (Need Implementation)

#### Comparison Operations

| Instruction | x86_64 Encoding | SetCC |
|------------|-----------------|-------|
| `CmpNe` | `cmp lhs, rhs` + `setne` | 0x95 |
| `CmpLt` | `cmp lhs, rhs` + `setl` | 0x9C |
| `CmpLe` | `cmp lhs, rhs` + `setle` | 0x9E |
| `CmpGt` | `cmp lhs, rhs` + `setg` | 0x9F |
| `CmpGe` | `cmp lhs, rhs` + `setge` | 0x9D |

**Implementation Pattern**:
```rust
IrInstruction::CmpNe { dest, lhs, rhs } => {
    let dest_reg = codegen.get_phys_reg(*dest)?;
    let lhs_reg = codegen.get_phys_reg(*lhs)?;
    let rhs_reg = codegen.get_phys_reg(*rhs)?;
    
    codegen.emit_mov_reg_reg(0, lhs_reg);      // rax = lhs
    codegen.emit_cmp_reg_reg(0, rhs_reg);      // compare rax, rhs
    codegen.emit_setne(dest_reg);               // dest = (rax != rhs)
}
```

#### Logical Operations

| Instruction | x86_64 Encoding |
|------------|-----------------|
| `LogicalAnd` | `test lhs, rhs` + `setnz` + `and` |
| `LogicalOr` | `or lhs, rhs` + `setnz` |

**Short-circuit evaluation** (future): requires jump-based branching

#### Bitwise Operations

| Instruction | x86_64 Encoding |
|------------|-----------------|
| `And` | `and r64, r64` |
| `Or` | `or r64, r64` |
| `Xor` | `xor r64, r64` |
| `Not` | `not r64` |
| `Shl` | `shl r64, cl` |
| `Shr` | `shr r64, cl` |
| `Mod` | `div` + extract rdx |

**Note**: `Mod` requires special handling:
```rust
// x86_64 div/idiv: dividend in rdx:rax, divisor in reg
// Result: quotient in rax, remainder in rdx
codegen.emit_mov_reg_reg(0, lhs_reg);     // rax = lhs
codegen.emit_xor_reg_reg(2, 2);          // rdx = 0 (for positive)
// For negative: cdq to sign-extend rax into rdx
codegen.emit_div_reg(rhs_reg);            // rax = lhs / rhs
codegen.emit_mov_reg_reg(dest_reg, 2);    // dest = rdx (remainder)
```

#### Arithmetic

| Instruction | x86_64 Encoding |
|------------|-----------------|
| `Div` | `div/idiv r64` |
| `Mod` | `div/idiv` + use rdx |
| `Neg` | `neg r64` |

#### Control Flow

| Instruction | x86_64 Encoding |
|------------|-----------------|
| `Jump` | `jmp rel32` (0xE9 + offset) |
| `Branch` | `test/jz` + `jmp` |
| `Call` | `call rel32` (0xE8 + offset) |
| `CallIndirect` | `call r64` (0xFF /2) |

**Jump Distance Handling**:
- Short jumps (0xEB): -128 to +127 bytes
- Near jumps (0xE9): 32-bit relative offset
- Use near jumps for all (simpler, no distance calculation)

### 3.3 Runtime Bridge

#### Print Function Bridge

Dryad's VM has a `print()` native function. For AOT compilation, we need a bridge:

```c
// _dryad_print.c (part of runtime library)
void _dryad_print(double value) {
    // Convert Dryad value to string
    // Call system printf
    printf("%g\n", value);
}

void _dryad_print_string(char* str, int len) {
    printf("%.*s\n", len, str);
}
```

**Linking Strategy**:
1. **Static**: Compile `_dryad_print.c` → object file → link with final binary
2. **Intrinsic**: Emit call to external `_dryad_print` (linker resolves)

#### Exit Handling

```asm
_exit_with_value:
    ; rdi = exit code (already set by caller)
    mov rax, 60        ; sys_exit
    syscall
    ; No return
```

---

## 4. Generator Integration

### 4.1 PE32+ Generator Changes

**Current**: Writes NOPs (0x90) as code section content

**New**: Uses actual machine code from backend

```rust
// generator/pe.rs

struct SectionHeader {
    virtual_size: u32,
    virtual_address: u32,
    raw_size: u32,
    raw_offset: u32,
    // ...
}

// In generate_object:
fn generate_object(&self, module: &IrModule, code: &[u8]) -> Result<Vec<u8>, String> {
    // ... header generation ...
    
    // Generate code section with REAL bytes
    let code_section = self.generate_code_section(code)?;
    
    // ... rest of PE structure ...
}

// Mark section as executable and readable
fn generate_code_section(&self, code: &[u8]) -> Vec<u8> {
    let mut section = Vec::with_capacity(code.len());
    
    // Section characteristics:
    // IMAGE_SCN_CNT_CODE (0x20)
    // IMAGE_SCN_MEM_EXECUTE (0x20000000)
    // IMAGE_SCN_MEM_READ (0x40000000)
    // = 0x60000020
    
    section.extend(code);
    section
}
```

### 4.2 ELF Generator Changes

Similar updates for ELF format:
- `.text` section with SHF_EXECINSTR flag
- Proper section headers
- Symbol table for linking

---

## 5. Testing Strategy

### 5.1 Unit Tests (backend)

Each new `emit_*` instruction gets a unit test:

```rust
#[test]
fn test_cmp_ne_instruction() {
    let mut codegen = X86_64Codegen::new_for_test();
    
    codegen.emit_mov_imm64(0, 5);      // rax = 5
    codegen.emit_mov_imm64(1, 3);     // rcx = 3
    codegen.emit_cmp_reg_reg(0, 1);   // compare rax, rcx
    codegen.emit_setne(0);            // rax = (rax != rcx)
    
    // Verify bytes generated
    assert!(codegen.code.contains(&0x95)); // setne opcode
}
```

### 5.2 Integration Tests

**Test 1: Simple Arithmetic**
```rust
#[test]
fn test_arithmetic_expression() {
    // 5 + 3 = 8
    // Expected output: "8\n"
    // Expected exit code: 0
}
```

**Test 2: Comparison**
```rust
#[test]
fn test_comparison() {
    // 5 > 3 returns true (1)
    // print(5 > 3) should output "true\n" or "1\n"
}
```

**Test 3: Multiple Operations**
```rust
#[test]
fn test_complex_expression() {
    // (2 + 3) * 4 = 20
    // print((2 + 3) * 4) should output "20\n"
}
```

**Test 4: Division and Modulo**
```rust
#[test]
fn test_division() {
    // 10 / 3 = 3
    // 10 % 3 = 1
}
```

### 5.3 End-to-End Test

```rust
#[test]
fn test_full_compilation_pipeline() {
    // 1. Compile Dryad source
    let source = "print(5 + 3)";
    
    // 2. Run through full pipeline
    let binary = compile_dryad_to_binary(source);
    
    // 3. Execute binary
    let output = Command::new(&binary)
        .output()
        .expect("Failed to execute binary");
    
    // 4. Verify
    assert_eq!(String::from_utf8_lossy(&output.stdout), "8\n");
    assert_eq!(output.status.code(), Some(0));
}
```

---

## 6. File Changes

| File | Changes |
|------|---------|
| `backend/x86_64.rs` | Add missing `emit_*` methods + instruction handlers |
| `generator/pe.rs` | Update to use real code bytes, not NOPs |
| `generator/elf.rs` | Same updates as PE |
| `compiler/mod.rs` | Wire backend into compile pipeline |
| `tests/integration_x86_64.rs` | New integration tests |
| `runtime/_dryad_runtime.c` | New: minimal runtime library |

---

## 7. Success Criteria

### 7.1 Functional Requirements

- [ ] All IR instructions generate correct x86_64 machine code
- [ ] Binary executes and produces correct output
- [ ] `print(5 + 3)` outputs "8\n"
- [ ] `print(10 / 3)` outputs "3\n"
- [ ] `print(10 % 3)` outputs "1\n"
- [ ] `print(5 > 3)` outputs "1\n"
- [ ] Exit code reflects computation result

### 7.2 Quality Requirements

- [ ] All existing tests pass (zero regressions)
- [ ] New tests for each instruction type
- [ ] Integration tests verify end-to-end execution
- [ ] No `unwrap()` in production code
- [ ] Code follows STANDARDIZATION_MANIFEST

### 7.3 Performance Requirements

- [ ] Compilation time < 1 second for simple expressions
- [ ] Generated binary size < 10KB for simple programs
- [ ] Execution time < 10ms for simple programs

---

## 8. Timeline

**Estimated Effort**: 12-16 hours

**Session Breakdown**:
1. **Session 2a**: Missing comparisons (CmpNe, CmpLt, CmpLe, CmpGt, CmpGe)
2. **Session 2b**: Bitwise operations (And, Or, Xor, Not, Shl, Shr)
3. **Session 2c**: Arithmetic (Div, Mod, Neg)
4. **Session 3a**: Control flow (Jump, Branch, Call)
5. **Session 3b**: Runtime bridge + print integration
6. **Session 4**: Integration tests + bug fixes

---

## 9. Risks and Mitigations

### Risk 1: Division by Zero

**Issue**: x86_64 div by zero causes CPU exception

**Mitigation**: Add runtime check before division:
```asm
cmp rdx, 0
je _handle_divide_by_zero
div rdx
```

### Risk 2: Register Allocation Conflicts

**Issue**: LinearScan may assign same register to conflicting live ranges

**Mitigation**: Verify existing liveness analysis tests pass; add edge case tests

### Risk 3: Stack Alignment

**Issue**: Incorrect alignment causes segfault on some CPUs

**Mitigation**: Existing 16-byte alignment in prologue is tested; verify continues to work

---

## 10. References

- [x86_64 Instruction Set Reference](https://www.felixcloutier.com/x86/)
- [System V AMD64 ABI](https://gitlab.com/x86-psABIs/x86-64-ABI)
- [PE/COFF Specification](https://learn.microsoft.com/en-us/windows/win32/debug/pe-format)
- [ELF Specification](https://refspecs.linuxfoundation.org/elf/elf.pdf)

---

**Document Status**: Approved  
**Next Step**: Create implementation plan
