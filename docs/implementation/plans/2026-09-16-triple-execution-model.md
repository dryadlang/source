# Phase 6: Triple Execution Model

> **Status:** Phase 6.10 complete — see [PHASE_6_TRIPLE_MODEL.md](../phases/PHASE_6_TRIPLE_MODEL.md)  
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

### 6.6 — CALL/RET + cross-function bytecode ✅
- `CALL func_idx, argc` / `RET` opcodes with VM call frames
- `VM::load_program()` for multi-function execution
- `BytecodeCompiler` emits `CallExpression` and `RET` on return
- `ExecutionEngine` caches `CompiledProgram` per `Program*`
- Micro-IR + AOT pipeline handle `RET` and `CALL` (call → slow path)
- `@compile` blocks compile inner body inline

### 6.7 — Runtime `.dryadc` loading ✅
- `BytecodeModuleCache` auto-loads `script.dryadc` companion on `run_file`
- `@compile("AOT")` executes precompiled bytecode when `.dryadc` exists
- Falls back to in-memory bytecode compile when no companion

### 6.8 — Native linking ✅
- `aot_runtime.c` — C ABI stubs (`dryad_variant_*`, `dryad_intrinsic_slow`, `dryad_unbox_*`)
- `dryad compile --aot --link -o out.so` — clang links LLVM IR + runtime → `.so`
- `NativeModule` — `dlopen` companion `.so`, invoke native functions with boxed parameters

### 6.9 — Native ABI with parameters ✅
- `FunctionMetadata.parameter_count` in `.dryadc` format v2
- LLVM emission: `define i8* @func(i8* %arg0, ...)` with unbox into slots
- `NativeModule::invoke()` boxes arguments and calls up to 4-parameter native functions

### 6.11 — Opcode coverage ✅
- `NEW_ARRAY`, `ARRAY_PUSH`, `SET_ELEMENT` for array literals and indexed write
- C-style `for` loop (parser + interpreter + bytecode)
- `GET_PROPERTY`, `SET_PROPERTY`, `METHOD_CALL`, `NEW_INSTANCE` for classes
- `VmRuntimeDelegate` wired through `ExecutionEngine` for object dispatch
- See [2026-09-17-opcode-coverage.md](2026-09-17-opcode-coverage.md)

### 6.10 — JIT hot-loop (§18) ✅
- `loop_analyzer` — detects while-loop back-edges in bytecode
- `JitCompiler` — on threshold, compiles loop body fragment → LLVM → `.so` via clang
- VM back-edge counting + `VmJitDelegate` dispatches hot loop body as native `void(i64* slots)`
- `ExecutionEngine::set_jit_delegate()` wires JIT into bytecode execution path

## File Layout

```
include/dryad/compiler/
  type_profiler.h          # Phase 6.0
  bytecode_compiler.h      # Phase 6.2
  execution_engine.h       # Phase 6.3 + JIT delegate wiring
include/dryad/runtime/
  opcode.h                 # Phase 6.1
  vm.h                     # Phase 6.10 JIT back-edge dispatch
  loop_analyzer.h          # Phase 6.10
  vm_jit_delegate.h        # Phase 6.10
  vm_runtime_delegate.h    # Phase 6.11 object dispatch
  bytecode_module.h        # Metadata Header + .dryadc v2
  native_module.h          # Phase 6.8–6.9
include/dryad/aot/
  micro_ir.h               # Phase 6.5
  llvm_backend.h           # Phase 6.5 + 6.9 params + 6.10 JIT IR
  jit_compiler.h           # Phase 6.10
  native_linker.h          # Phase 6.8
  analyze_pass.h
src/aot/aot_runtime.c        # C ABI: box/unbox/variant
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
- [x] All existing tests pass; new triple-model tests added per phase (403 unit tests, +1 JIT)
- [x] Cross-function bytecode calls via CALL/RET
- [x] Runtime `.dryadc` module loading for `@compile("AOT")`
- [x] Native `.so` linking from LLVM IR via clang
- [x] Opcode coverage: array literals, `for`, class methods ([6.11](2026-09-17-opcode-coverage.md))
- [x] Full native ABI with function parameters
- [x] JIT hot-loop back-edge compilation (§18)
