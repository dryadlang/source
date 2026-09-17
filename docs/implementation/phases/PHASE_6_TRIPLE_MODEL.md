# Phase 6 — Triple Execution Model

> **Status:** 6.15 in progress (example triple coverage gate)  
> **Tests:** 410 unit tests (+1 JIT slow test)  
> **Master plan:** [2026-09-16-triple-execution-model.md](../plans/2026-09-16-triple-execution-model.md)

## Overview

Dryad runs the same semantics through three tiers:

| Layer | Mode | Artifact | Speed |
|-------|------|----------|-------|
| 1 | Interpreter | — | Fast startup |
| 2 | Bytecode VM | `.dryadc` | ~10× interpreter |
| 3 | AOT / JIT | `.ll` / `.so` | Native |

Type observations from the interpreter feed specialized bytecode; bytecode metadata feeds LLVM AOT.

## Completed sub-phases

### 6.0–6.3 — Profiler, VM, compiler, `@compile()`

- `TypeProfiler` observes slots, branches, calls
- Stack VM with generic + specialized opcodes (`ADD_I32`, `GET_ELEMENT_F64`, …)
- `BytecodeCompiler` emits typed bytecode from profiles
- `@compile("interpreter"|"bytecode"|"AOT")` + `ExecutionEngine` routing

### 6.4–6.6 — CLI, LLVM IR, CALL/RET

- `dryad compile`, `dryad analyze`, `dryad --vm`
- Bytecode → Micro-IR → LLVM IR (`.ll`)
- Cross-function `CALL` / `RET` with VM call frames

### 6.7–6.8 — Runtime artifacts

- Auto-load companion `script.dryadc` on `run_file`
- `dryad compile --link -o out.so` via clang + `aot_runtime.c`
- `NativeModule` (`dlopen`) for `@compile("AOT")`

### 6.9 — Native ABI with parameters

**Problem:** Native `.so` only worked for zero-argument functions.

**Solution:**

- `FunctionMetadata.parameter_count` in `.dryadc` format **v2** (v1 still loads)
- LLVM: `define i8* @fn(i8* %arg0, i8* %arg1, …)` with `@dryad_unbox_*` into slots
- `NativeModule::invoke()` boxes `Value` args (up to 4) and calls the native symbol
- `invoke_aot()` prefers `.so` even when arguments are present

**Key files:** `llvm_backend.cpp`, `aot_runtime.c`, `native_module.cpp`, `bytecode_module.cpp`

### 6.10 — JIT hot-loop (§18)

**Problem:** Tight `while` loops stay interpreted even when types are known.

**Solution:**

1. `analyze_loops()` finds `while` back-edges (`JUMP` to earlier PC)
2. VM counts back-edge hits per loop
3. After threshold (default **1000**), `JitCompiler` compiles loop **body** fragment:
   - bytecode slice → Micro-IR → `emit_jit_loop_ir()` → clang → `dlopen`
4. On subsequent iterations, VM calls `void(i64* slots)` instead of interpreting the body

**Enable JIT in bytecode execution:**

```cpp
JitCompiler jit;
jit.configure(DRYAD_AOT_RUNTIME_SRC, DRYAD_INCLUDE_DIR);
jit.set_threshold(100);  // lower for dev/testing

engine.set_jit_delegate(&jit);
engine.enable_jit(true);
```

**Key files:** `loop_analyzer.*`, `jit_compiler.*`, `vm_jit_delegate.h`, `vm.cpp`

## `.dryadc` format v2

```
DRYADC magic
u32 version (=2)
u32 function_count
per function:
  string name
  u32 slot_count + slots[{name, index, type, confidence}]
  u8  parameter_count    ← new in v2
  u16 stack_depth
  u32 opcode_count
  bytes code + constants
```

## CLI reference

```bash
# Bytecode artifact
dryad compile script.dryad -o script.dryadc

# LLVM IR
dryad compile script.dryad --aot -o script.ll

# Native shared object
dryad compile script.dryad --aot --link -o script.so

# Run with bytecode default
dryad --vm script.dryad
```

### 6.11 — Opcode coverage

- Arrays: `NEW_ARRAY`, `ARRAY_PUSH`, `SET_ELEMENT`
- `for (init; cond; incr)` statement
- Classes: `NEW_INSTANCE`, `GET/SET_PROPERTY`, `METHOD_CALL`
- `VmRuntimeDelegate` in `ExecutionEngine` for interpreter-backed dispatch

Details: [2026-09-17-opcode-coverage.md](../plans/2026-09-17-opcode-coverage.md)

### 6.12 — Full bytecode coverage

- Opcodes: `NOT`, `DIV_I32`, `JUMP_IF_TRUE` (logical short-circuit)
- Control flow: `break` / `continue` with deferred jump patching
- Intrinsics: `__syscall_*` and `__runtime_*` → `INTRINSIC_SYSCALL`
- Unary `!` / `-`, logical `&&` / `||` short-circuit

**Bug fixes in 6.12:**
- `break` now jumps past the loop back-edge (not before it)
- `continue` in `for` loops patches to increment section (not condition)

Matrix: [BYTECODE_COVERAGE.md](../plans/BYTECODE_COVERAGE.md)

### 6.13 — Stdlib via intrinsics

- `CALL_GLOBAL` opcode dispatches imported/stdlib functions through `VmRuntimeDelegate`
- Stdlib internals remain on `INTRINSIC_SYSCALL` (`__runtime_*`, `__syscall_*`)
- Integration tests updated for reorganized `examples/` layout

Details: [2026-09-17-stdlib-via-intrinsics.md](../plans/2026-09-17-stdlib-via-intrinsics.md)

### 6.14 — Native binary output

- `AotCompileResult::binary` — artifact bytes returned after every compile target
- `compile_to_native_executable()` — ELF/PE via clang + LLVM `@main(argc, argv)`
- Top-level scripts → synthesized `__dryad_main` entry function
- Entry parameters (up to 4) mapped from `argv[1..]` via `dryad_argv_box_*`
- CLI: `--executable`, `--entry <fn>`

Details: [2026-09-17-native-binary-output.md](../plans/2026-09-17-native-binary-output.md)

### 6.15 — Example triple coverage gate (in progress)

**Goal:** Every file in `dryad-cpp/examples/manifest.json` runs in interpreter, bytecode VM, and AOT compile modes.

```bash
./examples/run_benchmark.sh --triple
```

**Done so far (43/48 triple cells):**
- CFG: `IrBasicBlock` + `Br`/`BrCond` → LLVM basic blocks
- `aot_runtime.c`: `dryad_print`, arrays (`new_array`, `push`, `get_element`)
- VM: `__dryad_main` bytecode path + interpreter fallback (stdlib, methods, closures)
- AOT: 13/15 examples compile; native `control_flow` runs with correct loop sum

**Remaining:** `io-vfs` / `http_parse` VM segfault; AOT `io-streams`/`io-paths` compile gaps; array indexing precision in native runtime.

Details: [2026-09-17-example-triple-coverage.md](../plans/2026-09-17-example-triple-coverage.md)
