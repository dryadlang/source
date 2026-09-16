# Phase 6: Triple Execution Model

> **Status:** Phase 6.4–6.5 complete (CLI compile/analyze + AOT LLVM IR)  
> **Spec reference:** Dryad Conventions §8, §13, §14  
> **Date:** 2026-09-16

## Goal

Implement Dryad's three-layer execution pipeline where each layer feeds the next with progressively richer type information. Semantics are identical across all layers; only performance differs (Property 8.4).

## Architecture

```
.dryad source
    │
    ▼
┌─────────────────────────────────────┐
│  Layer 1: AST Interpreter           │
│  • Default execution mode           │
│  • TypeProfiler collects observations│
│  • Branch + call-site profiles      │
└──────────────┬──────────────────────┘
               │ AST + TypeProfile
               ▼
┌─────────────────────────────────────┐
│  Layer 2: Bytecode VM               │
│  • AST + profile → specialized ops  │
│  • ADD_F64 when OBSERVED f64        │
│  • Metadata Header per function     │
│  • Output: .dryadc                  │
└──────────────┬──────────────────────┘
               │ Type-annotated bytecode
               ▼
┌─────────────────────────────────────┐
│  Layer 3: AOT Compiler (LLVM)       │
│  • Bytecode → Micro-IR (SSA)        │
│  • Micro-IR → LLVM IR               │
│  • Fast/Slow intrinsic dual-port    │
│  • ELF/PE native binary             │
└─────────────────────────────────────┘
```

## Execution Modes

| Mode | Input | Type Info | Profile |
|------|-------|-----------|---------|
| AST Interpreter | `.dryad` source | Runtime observations | Fastest startup, slowest execution |
| Bytecode VM | AST + observations | Metadata Header | ~10× interpreter |
| AOT (LLVM) | `.dryadc` bytecode | Bind Once (STATIC/OBSERVED) | Native speed |

## Type Intelligence Contract

| Layer | Responsibility |
|-------|----------------|
| **Interpreter** | Observes types at every slot write, call site, and branch |
| **Bytecode VM** | Emits specialized opcodes from observations; writes Metadata Header |
| **AOT** | Consumes header; zero type checks on STATIC/OBSERVED paths |

### Type Confidence Levels

| Level | Source | Bytecode behavior |
|-------|--------|-------------------|
| `STATIC` | Type annotation or CFG proof | Native opcode, no check |
| `OBSERVED` | Consistent runtime observations | Native opcode |
| `UNKNOWN` | Conflicting types observed | Generic Variant dispatch |

## Implementation Phases

### 6.0 — Type Profiler ✅ (this commit)
- `TypeProfiler` class: slot, branch, call-site profiles
- Hooks in interpreter, evaluator, statement executor, function call
- Unit tests

### 6.1 — VM Foundation ✅ (this commit)
- `opcode.h`, `vm.h`, `bytecode_builder.h` — stack VM with generic + specialized opcodes
- `INTRINSIC_SYSCALL` dispatcher via `IntrinsicsRegistry::call_by_id`
- `LOAD_FAST` / `STORE_FAST`, `GET_ELEMENT_F64`, control flow (`JUMP`, `HALT`)

### 6.2 — Bytecode Compiler ✅ (this commit)
- `BytecodeCompiler`: AST → bytecode using `TypeProfile`
- Type-specialized emission (`ADD_I32`, `ADD_F64`, `GET_ELEMENT_F64`)
- `FunctionMetadata` header per function (slots, stack depth, opcode count)
- `.dryadc` serialize/load via `save_compiled_program` / `load_compiled_program`

### 6.3 — `@compile()` + ExecutionEngine ✅ (this commit)
- Parser: `@compile("interpreter"|"bytecode"|"AOT")` on functions and blocks
- `ExecutionEngine` routes calls to interpreter or bytecode VM
- Transitive mode via scope stack (§8.10)
- CLI: `dryad --vm` sets default bytecode mode

### 6.4 — CLI compile + analyze ✅
- `dryad compile input.dryad -o out.dryadc` — bytecode artifact
- `dryad compile --aot -o out.ll` — LLVM IR output
- `dryad analyze` / `dryad --analyze` — Fast/Slow path report (§14.7)
- `--no-profile` skips interpreter dry-run

### 6.5 — AOT LLVM IR backend ✅ (foundation)
- Bytecode → Micro-IR (SSA-style) → LLVM IR text (`.ll`)
- `dryad_intrinsic_slow` / `dryad_variant_*` slow-path declarations
- Native object linking deferred (use `clang out.ll` manually)

### 6.6 — JIT (§18 refinement, future)
- Loop back-edge counting; hot fragment compilation
- Not part of core triple model

## File Layout

```
include/dryad/compiler/
  type_profiler.h          # Phase 6.0
  bytecode_compiler.h      # Phase 6.2
include/dryad/runtime/
  opcode.h                 # Phase 6.1
  vm.h
  bytecode_module.h        # Metadata Header + .dryadc
include/dryad/aot/
  micro_ir.h               # Phase 6.5
  llvm_backend.h
  analyze_pass.h
```

## Metadata Header (per function)

```
FuncID:       <name or hash>
Parameters:   [{slot, type, confidence}, ...]
Locals:       [{slot, type, confidence}, ...]
StackDepth:   <max stack depth>
OpcodeCount:  <total opcodes>
```

## Example: Type-Specialized Emission

```dryad
function compute(arr) {
  let sum = 0;
  for (let i = 0; i < arr.len(); i++) {
    sum = sum + arr[i];
  }
  return sum;
}
```

After interpreter profiling (OBSERVED):
- `sum` → f64
- `i` → i32
- `arr` → array (element f64 if homogeneous)

Bytecode emitted:
```
LOAD_CONST 0
STORE_FAST sum          ; f64 OBSERVED
LOAD_CONST 0
STORE_FAST i            ; i32 OBSERVED
.loop:
  LOAD_FAST i
  LOAD_FAST arr
  GET_ELEMENT_F64       ; not GET_ELEMENT_GENERIC
  ADD_F64
  STORE_FAST sum
  ...
```

## `@compile()` Decorator (Phase 6.3)

```dryad
@compile("AOT")
function hotPath(data: Buffer) { ... }

function mixedMode() {
  let config = io_read_file("config.json");  // interpreter

  @compile("bytecode") {
    for (let item in parsed.items) {
      heavyProcessing(item);
    }
  }
}
```

Replaces legacy `set_compile_mode()` API.

## Success Criteria

- [x] TypeProfiler collects slot/branch/call observations during interpretation
- [x] VM foundation: stack dispatcher, specialized opcodes, INTRINSIC_SYSCALL
- [x] Bytecode compiler emits specialized opcodes from TypeProfile
- [x] `@compile()` selects execution backend per scope
- [x] AOT emits LLVM IR from bytecode (`.ll`; native link via clang)
- [x] `dryad analyze` reports Fast/Slow path sites
- [ ] All existing tests pass; new triple-model tests added per phase
