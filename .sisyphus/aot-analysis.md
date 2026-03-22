# AOT Compiler System - Comprehensive Analysis & Gaps

**Date**: 2026-03-22  
**Status**: Discovery Phase Complete  
**Architecture**: Bytecode → IR → Backend → Binary Generator

---

## 1. SYSTEM OVERVIEW

The AOT (Ahead-of-Time) compiler converts Dryad bytecode into native executable binaries:

```
Bytecode (dryad_bytecode)
    ↓
IR (Intermediate Representation)
    ↓
Backend (x86_64 / ARM64)
    ↓
Object Code (machine instructions)
    ↓
Generator (ELF / PE)
    ↓
Executable Binary
```

### Components & Files

| Component | File | Status | Lines | Purpose |
|-----------|------|--------|-------|---------|
| **Converter** | `converter.rs` | 🟡 Partial | 268 | Bytecode → IR translation |
| **IR Types** | `ir/types.rs` | ✅ Complete | 99 | Low-level type system |
| **IR Values** | `ir/values.rs` | ✅ Complete | 131 | Constants and registers |
| **IR Instructions** | `ir/instructions.rs` | ✅ Complete | 301 | 40+ instruction types |
| **IR Module** | `ir/module.rs` | ✅ Complete | 211 | Module/function structure |
| **x86_64 Backend** | `backend/x86_64.rs` | 🟡 Partial | 378 | Machine code generation |
| **ARM64 Backend** | `backend/arm64.rs` | 🔴 Stub | ? | Not implemented |
| **ELF Generator** | `generator/elf.rs` | 🟡 Minimal | 177 | ELF binary format |
| **PE Generator** | `generator/pe.rs` | 🔴 Stub | ? | Windows PE format |

---

## 2. COMPONENT ANALYSIS

### 2.1 IR SYSTEM (✅ COMPLETE)

**Status**: Production-ready IR definition  
**Lines of code**: ~742 lines across 4 files

#### IR Types (`types.rs`)
- ✅ Primitive types: `Void`, `I8`, `I16`, `I32`, `I64`, `F32`, `F64`, `Bool`
- ✅ Complex types: `Ptr`, `Array`, `Struct`, `Union`, `Function`
- ✅ Helper methods: `size()`, `align()`, type predicates
- ✅ Type checking complete

#### IR Values (`values.rs`)
- ✅ Constants: `I8-I64`, `F32/F64`, `Bool`, `String`, `Null`, `Array`, `Struct`
- ✅ Register references with type tracking
- ✅ Global references
- ✅ Conversion utilities: `as_i64()`, `as_f64()`, type inference

#### IR Instructions (`instructions.rs`)
- ✅ **Data movement** (5): LoadConst, Move, Load, Store, LoadGlobal, LoadLocal
- ✅ **Arithmetic** (6): Add, Sub, Mul, Div, Mod, Neg
- ✅ **Comparison** (6): CmpEq, CmpNe, CmpLt, CmpLe, CmpGt, CmpGe
- ✅ **Logic/Bitwise** (6): And, Or, Xor, Not, Shl, Shr
- ✅ **Control flow** (4): Jump, Branch, Return, Call, CallIndirect
- ✅ **Memory** (3): StackAlloc, HeapAlloc, HeapFree
- ✅ **Exception** (3): Throw, TryBegin, TryEnd
- ✅ **Misc** (3): Phi, Nop, DebugLoc
- **Total**: 40+ instruction types

#### IR Module (`module.rs`)
- ✅ Module structure: functions, globals, metadata
- ✅ Register/block ID generation
- ✅ Function management API

**Verdict**: IR system is **complete and well-designed**. No gaps here.

---

### 2.2 CONVERTER: BYTECODE → IR (🟡 PARTIAL)

**Status**: ~65% implemented  
**File**: `converter.rs` (268 lines)  
**Entry point**: `BytecodeToIrConverter::convert()`

#### Implemented Bytecode Opcodes (17/~30+)
- ✅ Constants: `Constant`, `Nil`, `True`, `False`
- ✅ Arithmetic: `Add`, `Subtract`, `Multiply`, `Divide`, `Negate`
- ✅ Comparison: `Equal`, `Greater`, `Less`, `Not`
- ✅ Stack ops: `Pop`, `Return`
- ⏳ Print operations: `Print`, `PrintLn` (stubbed - TODO)

#### NOT Implemented (~13+ opcodes)
```
Missing from converter.rs line 204-207:
- Local variable access (GetLocal, SetLocal)
- Function calls (Call)
- Loop/branch control (JumpIfTrue, JumpIfFalse, Jump)
- Array/object operations
- Method calls
- Try/catch blocks
- Any other complex operations
```

#### Key Issues
1. **Print operations**: Lines 180-189 are stubs - don't actually emit IR
2. **No local variable tracking**: Stack depth tracked but locals not mapped to IR local variables
3. **No call support**: Cannot convert function calls
4. **No jump handling**: Cannot convert loops/branches
5. **Error handling in stack operations**: `push_register()` and `pop_register()` work but complex stack patterns may fail
6. **Single function only**: Handles only "main" function, no module-level functions

#### Coverage Gaps
- **Bytecode modules with multiple functions**: Not supported
- **Complex control flow**: If/while/for loops cannot be converted
- **Function parameters**: No parameter handling in IR
- **Nested scopes**: No scope tracking for closures

**Verdict**: Converter handles only **basic arithmetic and constants**. ~60% of bytecode opcodes unsupported.

---

### 2.3 x86_64 BACKEND (🟡 PARTIAL)

**Status**: ~40% implemented  
**File**: `backend/x86_64.rs` (378 lines)

#### Implemented IR Instructions (4/40+)
```rust
- ✅ LoadConst: Supports I32, I64 constants (lines 86-96)
- ✅ Add: mov/add/mov sequence (lines 99-106)
- ✅ Sub: mov/sub/mov sequence (lines 108-112)
- ✅ Mul: mov/imul/mov sequence (lines 114-118)
- ✅ CmpEq: cmp/sete sequence (lines 120-126)
```

#### NOT Implemented (36/40+ instructions)
```
Missing implementations (all hit the default case at line 128-131):
- Load/Store (memory ops)
- Div, Mod
- All other comparisons (CmpNe, CmpLt, CmpLe, CmpGt, CmpGe)
- All logic ops (And, Or, Xor, Not, Shl, Shr)
- Control flow (Jump, Branch, Call)
- Exception handling
- Any advanced features
```

#### Codegen Utility Functions (Well-Implemented)
- ✅ Stack frame setup: `emit_push_rbp()`, `emit_pop_rbp()`, `emit_mov_rbp_rsp()`
- ✅ Basic register operations: `emit_mov_reg_reg()`, `emit_mov_imm32/64()`
- ✅ Arithmetic: `emit_add_reg_reg()`, `emit_sub_reg_reg()`, `emit_imul_reg_reg()`
- ✅ Comparison: `emit_cmp_reg_reg()`, `emit_test_reg_reg()`, `emit_sete()`

#### Instruction Encoding (Correct x86_64)
All opcode bytes are correctly defined:
- REX prefix handling for 64-bit operations ✅
- ModRM byte calculation ✅
- Immediate value handling ✅

#### Critical Issues
1. **Label resolution**: Lines 352-373 - all jumps use placeholder offsets `[0x00, 0x00, 0x00, 0x00]`
   - No forward-reference tracking
   - `pending_labels` created but never used
   - Jumps will always jump to wrong addresses

2. **Register allocation**: None exists
   - Using RegisterId directly as register number
   - Virtual registers 0-127 assumed
   - Will crash on code with >128 virtual registers
   - No calling convention handling

3. **Function prologue/epilogue**: Hardcoded
   - Does not align stack to 16 bytes (required by ABI)
   - Does not preserve callee-saved registers
   - Does not handle function parameters

4. **No return value handling**: Line 146-147 assumes all returns use rax, but no tracking of where values are stored

**Verdict**: Backend has **basic opcode generation** but **critical missing features** (label resolution, register allocation, ABI compliance).

---

### 2.4 ELF GENERATOR (🟡 MINIMAL)

**Status**: Skeleton implementation  
**File**: `generator/elf.rs` (177 lines)

#### What's Implemented
- ✅ ELF header generation (64 bytes)
- ✅ Program header generation (56 bytes)
- ✅ Basic structure: Header + Program Header + Code + Padding
- ✅ 4KB alignment for executable

#### What's NOT Implemented
- ❌ Section headers (.text, .data, .rodata, .bss, .strtab, .symtab)
- ❌ Symbol table generation
- ❌ Relocation entries
- ❌ String table
- ❌ Debug information
- ❌ Proper section alignment
- ❌ Dynamic linking support
- ❌ Entry point calculation (hardcoded `0x400000 + 64 + 56`)

#### Current Limitations
1. **Cannot link with C libraries**: No symbol table = no linking
2. **Cannot make proper executables**: No relocation entries
3. **Basic executable only**: Works for simple standalone code
4. **Module structure ignored**: Doesn't use `IrModule` information

**Verdict**: ELF generator is a **bare-bones stub**. Produces minimal valid executables but cannot link with other code.

---

### 2.5 ARM64 BACKEND (🔴 STUB)

**Status**: Not implemented  
**File**: `backend/arm64.rs` - contains no code

**Verdict**: **Zero implementation**. Completely blank.

---

### 2.6 PE GENERATOR (🔴 STUB)

**Status**: Not implemented  
**File**: `generator/pe.rs` - contains no code

**Verdict**: **Zero implementation**. Completely blank.

---

## 3. IMPLEMENTATION GAPS

### Critical Gaps (Blocking Function)

| Gap | Impact | Effort |
|-----|--------|--------|
| **Label resolution in x86_64 backend** | Jumps/branches won't work | High |
| **Register allocation** | Code generation crashes on real programs | Very High |
| **Bytecode → IR missing opcodes** | Most Dryad code cannot convert | Very High |
| **ELF symbol table** | Cannot link executables | Medium |
| **Function call support** | No function invocation in AOT | Very High |
| **ABI compliance (stack alignment)** | May crash with real system calls | Medium |

### Medium Gaps (Partial Function)

| Gap | Impact | Effort |
|-----|--------|--------|
| **Print implementation** | Built-in prints don't work | Low |
| **Loop/branch conversion** | Control flow not compiled | High |
| **Local variable tracking** | Complex variable scoping fails | Medium |
| **More x86_64 instructions** | Missing Div, Mod, logic ops | Medium |
| **PE generator** | Windows target unsupported | High |

### Non-Critical Gaps (Nice-to-Have)

| Gap | Impact | Effort |
|-----|--------|--------|
| **ARM64 backend** | ARM targets unsupported | Very High |
| **Debug information** | No debugging possible | High |
| **Optimization passes** | Code not optimized | Very High |
| **Error messages** | Poor error reporting | Low |

---

## 4. CURRENT COMPILATION PIPELINE STATUS

### What Works
```
Simple Bytecode (const + basic math)
  ↓
✅ Converts to IR (small subset)
  ↓
✅ x86_64 backend generates code (5 instructions)
  ↓
✅ ELF writes minimal executable
  ↓
❌ Executable has broken jumps + no linking support
```

### What Doesn't Work
- Any code with function calls
- Any code with control flow (if/while/for)
- Any code accessing variables
- Any code with string operations
- Any code requiring runtime linking
- Any error handling

---

## 5. RECOMMENDED IMPLEMENTATION ORDER

### Phase 1: Foundation (2-3 weeks)
1. **Register allocator** - Assign virtual registers to physical x86_64 registers (rax-r15)
2. **Label resolution** - Fix jump target offsets
3. **Stack frame** - Proper prologue/epilogue + 16-byte alignment
4. **More x86_64 instructions** - Div, Mod, logic ops, additional comparisons

### Phase 2: Bytecode Coverage (1-2 weeks)
5. **Local variable conversion** - GetLocal, SetLocal, proper stack mapping
6. **Function call support** - Call/Return with parameters
7. **Loop/branch conversion** - JumpIfTrue, JumpIfFalse, Jump
8. **Print implementation** - Emit actual calls to runtime

### Phase 3: Binary Generation (2-3 weeks)
9. **ELF symbol table** - Proper linking support
10. **Relocation entries** - Fix up addresses
11. **PE generator** - Windows target support

### Phase 4: Advanced (3+ weeks)
12. **ARM64 backend** - ARM64 target support
13. **Debug info** - DWARF support
14. **Optimizations** - Peephole, constant folding, etc.

---

## 6. VERIFICATION STATUS

### Tests
- No AOT-specific tests exist
- Bytecode tests (42) pass but don't exercise AOT
- Building test: Does crate compile? ✅ Yes

### What Can Be Tested Now
```
Simple arithmetic: let x = 2 + 3; x
Simple constants: 42, "hello", true
Simple output: Generate binary, verify structure
```

### What Cannot Be Tested Yet
- Any function calls
- Any control flow
- Any variable access beyond immediate values
- Actual binary execution
- Linking

---

## 7. ARCHITECTURE QUALITY

### Strengths ✅
- Clean IR design with comprehensive instruction set
- Well-organized module structure
- Type system handles complex types properly
- x86_64 opcode generation mostly correct
- Converter framework is solid

### Weaknesses ❌
- No register allocator (assumes infinite registers)
- Jump label resolution not implemented
- No ABI compliance verification
- Symbol table + linking not thought through
- ARM64/PE treated as "future work"

### Design Risks ⚠️
1. **RegisterId directly mapped to register number** - Will fail with >8 registers in use
2. **No intermediate optimization** - Direct IR→code generation misses peephole opportunities
3. **Bytecode conversion assumes single function** - Won't handle multi-function modules
4. **ELF generator hardcodes addresses** - No relocation mechanism

---

## 8. ESTIMATED EFFORT TO "WORKING" STATE

| Milestone | Tasks | Effort | Time |
|-----------|-------|--------|------|
| **Can compile basic math** | #1-4 | 3 | 1 week |
| **Can compile if/while** | #2-7 | 4 | 1 week |
| **Can link with C stdlib** | #9-10 | 2 | 3-5 days |
| **Can compile real functions** | #5-8 | 4 | 1 week |

**Total to "Hello World" equivalent**: ~3-4 weeks of focused work

---

## 9. NEXT IMMEDIATE ACTIONS

1. **Implement register allocator** (required for everything)
2. **Fix label resolution** (required for control flow)
3. **Expand x86_64 instruction coverage** (required for common ops)
4. **Add local variable support to converter** (required for non-trivial code)

---

## 10. FILE REFERENCE GUIDE

```
/crates/dryad_aot/
├── src/
│   ├── compiler/
│   │   ├── converter.rs          ← Add missing bytecode opcodes
│   │   ├── mod.rs                ← Main orchestration (working)
│   │   └── options.rs            ← Configuration
│   ├── ir/
│   │   ├── mod.rs                ← IR exports (complete)
│   │   ├── instructions.rs       ← Instruction definitions (complete)
│   │   ├── types.rs              ← Type system (complete)
│   │   ├── values.rs             ← Constants/values (complete)
│   │   └── module.rs             ← Module structure (complete)
│   ├── backend/
│   │   ├── mod.rs                ← Backend trait
│   │   ├── x86_64.rs             ← 🔴 FIX: Add allocator, label resolution
│   │   └── arm64.rs              ← 🔴 TODO: Full implementation
│   ├── generator/
│   │   ├── mod.rs                ← Generator trait
│   │   ├── elf.rs                ← 🟡 Add symbol table, relocations
│   │   └── pe.rs                 ← 🔴 TODO: Full implementation
│   └── lib.rs
├── Cargo.toml
├── README.md
└── examples/
```

---

**Analysis Date**: 2026-03-22  
**Analyzed By**: Sisyphus  
**Status**: Ready for implementation planning
