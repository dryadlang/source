# AOT Compiler Implementation Plan

**Status**: Ready for execution  
**Priority**: High - blocks all binary generation functionality  
**Scope**: Fix critical gaps to achieve "basic compilation" milestone

---

## PHASE 1: FOUNDATION (Register Allocator + Label Resolution)

### Goal
Enable x86_64 backend to generate correct code for real programs by:
1. Assigning virtual registers to physical x86_64 registers
2. Fixing jump target offsets
3. Implementing proper calling convention

### Why This Phase First
- Everything after depends on this
- Without it, generated code is non-functional
- Relatively contained scope (~500 lines)

---

## TASK 1: Implement x86_64 Register Allocator

**File**: `crates/dryad_aot/src/backend/x86_64.rs`  
**Scope**: Add register allocation pass before code generation

### Requirements

#### 1.1 Register Management
- Map virtual RegisterIds (0-∞) to physical x86_64 registers
- Preserve x86_64 ABI callee-saved registers (rbx, r12-r15, rbp)
- Use caller-saved registers for temporaries (rax, rcx, rdx, rsi, rdi, r8-r11)
- Track register lifecycle (liveness analysis)

#### 1.2 Calling Convention Implementation
- System V AMD64 ABI (Linux): rdi, rsi, rdx, rcx, r8, r9 for parameters
- Return values in rax/rdx
- Stack alignment: 16-byte boundary before call instruction
- Local variables in stack frame

#### 1.3 Stack Frame Management
- Calculate local variable stack offset at function start
- Adjust rsp for locals + spill slots
- Preserve stack alignment invariant

### Implementation Strategy

**Step 1**: Create `RegisterAllocator` struct
```rust
struct RegisterAllocator {
    virtual_to_physical: HashMap<RegisterId, u8>,
    physical_free: Vec<u8>,
    spill_stack: HashMap<RegisterId, i32>,
    next_spill_offset: i32,
}
```

**Step 2**: Implement liveness analysis
- Track which registers are live at each instruction
- Mark reads and writes

**Step 3**: Linear scan allocation
- Iterate through IR instructions
- Assign physical registers to virtual registers
- Spill to stack when all physical registers in use

**Step 4**: Codegen modifications
- Replace all register operations to use physical register mapping
- Insert spill/reload code where needed
- Update stack offset calculations

### Success Criteria
- [ ] Virtual registers map 1:1 to physical registers
- [ ] No "infinite register" assumptions
- [ ] Stack alignment verified
- [ ] Spill code generated for high-register-pressure functions

---

## TASK 2: Fix Label Resolution for Jumps

**File**: `crates/dryad_aot/src/backend/x86_64.rs`  
**Scope**: Implement two-pass code generation for jump offset fixup

### Requirements

#### 2.1 Two-Pass Code Generation
- **Pass 1**: Generate code, record label positions and jump placeholders
- **Pass 2**: Fixup jump offsets to correct addresses

#### 2.2 Label Tracking
- Record position (offset in code) of each block label
- Track pending jumps that need fixup
- Handle forward references (jumps to not-yet-generated blocks)

#### 2.3 Jump Offset Calculation
- Calculate relative offset: `target_offset - (current_offset + instruction_size)`
- Encode as 32-bit signed immediate
- Validate within ±2GB range

### Implementation Strategy

**Step 1**: Modify `X86_64Codegen` to track labels
```rust
struct X86_64Codegen {
    code: Vec<u8>,
    label_positions: HashMap<BlockId, usize>,
    pending_jumps: Vec<(usize, BlockId)>,
    ...
}
```

**Step 2**: Pass 1 - Generate with placeholders
- Emit jumps with `[0x00, 0x00, 0x00, 0x00]` placeholder
- Record offset of placeholder in `pending_jumps`
- Record label position when emitted

**Step 3**: Pass 2 - Fixup offsets
```rust
for (offset, target_block) in pending_jumps {
    let target = label_positions[&target_block];
    let delta = target as i32 - (offset as i32 + 4);
    code[offset..offset+4].copy_from_slice(&delta.to_le_bytes());
}
```

**Step 4**: Handle forward references
- First pass: emit all labels
- Second pass: backpatch jump offsets

### Success Criteria
- [ ] All jumps resolve to correct addresses
- [ ] No placeholder values remain in generated code
- [ ] Forward jumps (to not-yet-generated blocks) work
- [ ] Backward jumps (loops) work

---

## TASK 3: Implement Proper x86_64 Function Prologue/Epilogue

**File**: `crates/dryad_aot/src/backend/x86_64.rs`  
**Scope**: ABI-compliant stack frame setup

### Requirements

#### 3.1 Stack Frame Layout
```
[rsp+N]   <- parameter 7+ (caller's stack)
[rsp+0]   <- return address (caller setup)
[rbp-0]   <- rbp (saved from caller)
[rbp-8]   <- local variable 0
[rbp-16]  <- local variable 1
...
[rsp]     <- current top of stack (after sub rsp, N)
```

#### 3.2 Prologue Requirements
- Push rbp (caller's base pointer)
- mov rbp, rsp (establish new base)
- sub rsp, N (allocate locals + spill space)
- **Maintain 16-byte alignment**: (rsp before `call`) % 16 == 0

#### 3.3 Epilogue Requirements
- mov rsp, rbp (restore stack pointer)
- pop rbp (restore caller's rbp)
- ret (return to caller)

### Implementation Strategy

**Step 1**: Calculate frame size
- Sum of: local variables + spill slots + padding for alignment

**Step 2**: Update prologue
```rust
// Current (incomplete):
codegen.emit_push_rbp();           // 1 byte: 0x55
codegen.emit_mov_rbp_rsp();        // 3 bytes: 0x48 0x89 0xE5
// Missing: alignment check + sub rsp

// New (complete):
codegen.emit_push_rbp();           // 1 byte: 0x55
codegen.emit_mov_rbp_rsp();        // 3 bytes: 0x48 0x89 0xE5
let frame_size = allocate_frame(locals, spills);
if frame_size > 0 {
    codegen.emit_sub_rsp(frame_size);
    // Verify: (frame_size + 8) % 16 == 0
}
```

**Step 3**: Validate alignment
- After push rbp: rsp is 8-byte aligned
- After sub rsp, N: rsp must be 16-byte aligned
- Therefore: N must be 8 mod 16

**Step 4**: Update function entry/exit
- Apply to all function boundaries
- Verify with test

### Success Criteria
- [ ] Stack frame properly allocated
- [ ] 16-byte alignment maintained
- [ ] Local variables accessible via [rbp-offset]
- [ ] No stack corruption

---

## TASK 4: Expand x86_64 Instruction Coverage

**File**: `crates/dryad_aot/src/backend/x86_64.rs`  
**Scope**: Implement missing IR instruction codegen

### Requirements

#### 4.1 Arithmetic Instructions (Currently: Add/Sub/Mul, Need: Div/Mod)
- Div: dst = lhs / rhs (64-bit division)
- Mod: dst = lhs % rhs (remainder)

#### 4.2 Comparison Instructions (Currently: CmpEq, Need: 5 more)
- CmpNe: dest = (lhs != rhs)
- CmpLt: dest = (lhs < rhs)
- CmpLe: dest = (lhs <= rhs)
- CmpGt: dest = (lhs > rhs)
- CmpGe: dest = (lhs >= rhs)

#### 4.3 Logic/Bitwise Instructions
- And, Or, Xor: Logical operations
- Not: Bitwise complement
- Shl, Shr: Shifts

### Implementation Strategy

**Step 1**: Add codegen methods for div/mod
- 64-bit division: `div r64` (uses rax:rdx for dividend, result in rax)
- Modulo: same operation, take remainder from rdx
- Clear rdx before division (signed division quirk)

**Step 2**: Add codegen methods for comparisons
- Use cmp/sete/setne/setl/setle/setg/setge
- Store result in register (0 or 1)

**Step 3**: Add codegen methods for logic/bitwise
- and/or/xor: straightforward reg-reg operations
- not: Bitwise complement
- shl/shr: shift operations with CL register

**Step 4**: Expand match arms in `compile_instruction()`
- Add case for each new instruction
- Generate correct opcode sequence
- Test each one

### Success Criteria
- [ ] All arithmetic operations implemented
- [ ] All comparison operations implemented
- [ ] All logic/bitwise operations implemented
- [ ] No panics on edge cases (division by zero handled by runtime)

---

## TASK 5: Add Memory Load/Store Instructions

**File**: `crates/dryad_aot/src/backend/x86_64.rs`  
**Scope**: Implement Load/Store IR instructions

### Requirements

#### 5.1 Memory Access
- Load: dest = [ptr]
- Store: [ptr] = value
- Support immediate offsets for local variables

#### 5.2 Memory Addressing Modes
- Register indirect: `[rax]`
- Register + displacement: `[rbp - 8]`
- Register + register: `[rbp + rcx]`

### Implementation Strategy

**Step 1**: Add memory addressing codegen
```rust
fn emit_mov_reg_mem(&mut self, dest: u8, base: u8, offset: i32)
fn emit_mov_mem_reg(&mut self, base: u8, offset: i32, src: u8)
```

**Step 2**: Handle Load instruction
- Move from memory to register

**Step 3**: Handle Store instruction
- Move from register to memory

### Success Criteria
- [ ] Local variables can be read/written
- [ ] Memory operations use correct addressing modes

---

## TASK 6: Enhance Bytecode Converter

**File**: `crates/dryad_aot/src/compiler/converter.rs`  
**Scope**: Support missing bytecode opcodes

### Immediate Priorities

#### 6.1 GetLocal / SetLocal
- Map bytecode local variable indices to IR registers
- Maintain scope stack for variable offset tracking

#### 6.2 Loop/Branch Control
- Jump: Unconditional branch
- JumpIfTrue/JumpIfFalse: Conditional branch
- Create multiple IR blocks instead of linear

#### 6.3 Function Calls
- Call: Invoke another function
- Return: Exit function with value

#### 6.4 Print Implementation (Stub Fix)
- Emit call to runtime print function
- Pass value as parameter

### Success Criteria
- [ ] Bytecode with variables converts to IR
- [ ] Bytecode with if/while converts to multiple blocks
- [ ] Bytecode with functions converts correctly
- [ ] Print operations emit proper IR calls

---

## TASK 7: Improve ELF Generator

**File**: `crates/dryad_aot/src/generator/elf.rs`  
**Scope**: Add symbol table and relocation entries

### Requirements

#### 7.1 Section Headers
- .text: Code section
- .data: Initialized data
- .bss: Uninitialized data
- .symtab: Symbol table
- .strtab: String table

#### 7.2 Symbol Table
- Encode function symbols
- Mark external symbols (for linking)

#### 7.3 Relocation Entries
- For function calls to external functions
- For data references

### Success Criteria
- [ ] ELF file includes proper section headers
- [ ] Symbol table populated
- [ ] Can be linked with gcc/ld

---

## TASK 8: Implement Basic PE Generator

**File**: `crates/dryad_aot/src/generator/pe.rs`  
**Scope**: Windows executable generation

### Similar to ELF but for Windows PE format
- DOS header
- PE header
- Section table
- Code section
- Data sections

---

## IMPLEMENTATION CHECKLIST

### Phase 1a: Register Allocator
- [ ] Create `RegisterAllocator` struct
- [ ] Implement liveness analysis
- [ ] Linear scan allocation
- [ ] Spill slot management
- [ ] Test with simple functions

### Phase 1b: Label Resolution
- [ ] Track label positions
- [ ] Record pending jumps
- [ ] Fixup offsets in second pass
- [ ] Test forward/backward jumps

### Phase 1c: ABI Compliance
- [ ] Correct prologue/epilogue
- [ ] 16-byte stack alignment
- [ ] Local variable frame setup
- [ ] Test with simple programs

### Phase 1d: Instruction Coverage
- [ ] Division/modulo
- [ ] All comparisons
- [ ] Logic/bitwise ops
- [ ] Memory load/store
- [ ] Test each operation

### Phase 2: Bytecode Converter
- [ ] GetLocal/SetLocal support
- [ ] Jump/branch conversion
- [ ] Function call support
- [ ] Print implementation fix
- [ ] Test with real Dryad code

### Phase 3: Binary Generators
- [ ] ELF symbol table
- [ ] ELF relocation entries
- [ ] Basic PE support
- [ ] Test linking

---

## TESTING STRATEGY

### Unit Tests
```rust
#[test]
fn test_register_allocation() { }
#[test]
fn test_label_resolution() { }
#[test]
fn test_div_instruction() { }
#[test]
fn test_jump_forward_backward() { }
```

### Integration Tests
```
Bytecode: let x = 2; let y = 3; x + y
Expected: Executable that returns 5
```

### Binary Verification
```bash
objdump -d output.elf  # Verify opcode correctness
readelf -a output.elf  # Verify ELF structure
./output              # Verify execution
```

---

## ESTIMATED EFFORT

| Task | Hours | Days |
|------|-------|------|
| Task 1: Register Allocator | 12-16 | 2-3 |
| Task 2: Label Resolution | 6-8 | 1-2 |
| Task 3: Prologue/Epilogue | 4-6 | 1 |
| Task 4: Instruction Coverage | 8-12 | 1-2 |
| Task 5: Memory Operations | 4-6 | 1 |
| Task 6: Bytecode Converter | 16-20 | 2-3 |
| Task 7: ELF Generator | 12-16 | 2 |
| Task 8: PE Generator | 12-16 | 2-3 |
| **Total** | **74-100** | **12-16 days** |

**Minimum viable (Tasks 1-5)**: 34-48 hours ≈ 5-6 days

---

## ACCEPTANCE CRITERIA

### Minimum Viable
- [ ] Can compile simple arithmetic: `2 + 3 * 4`
- [ ] Generated x86_64 code is correct
- [ ] No stack corruption
- [ ] Register allocation works
- [ ] Labels/jumps resolve correctly

### Fully Working (Phase 1+2)
- [ ] Can compile functions with locals
- [ ] Can compile if/while loops
- [ ] Can compile function calls
- [ ] All bytecode operations supported
- [ ] ELF executables work

### Bonus (Phase 3)
- [ ] Windows PE executables work
- [ ] Proper linking with C stdlib
- [ ] Debug information present

---

**Plan Created**: 2026-03-22  
**Ready to Execute**: Yes  
**Approval Gate**: None - proceed to task selection
