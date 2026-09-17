# Phase 6.15 — Example Triple Coverage Gate

> **Status:** In progress  
> **Gate:** Every example in `dryad-cpp/examples/manifest.json` must run in all three execution modes.

## Definition of done

| Mode | Command | Success criteria |
|------|---------|------------------|
| **Interpreter** | `dryad script.dryad` | Runs without error |
| **Bytecode VM** | `dryad --vm script.dryad` | Runs without error |
| **AOT** | `dryad compile script.dryad --aot -o out.ll` | Compiles to LLVM IR (link/executable is next tier) |

Run the matrix:

```bash
cd dryad-cpp
./examples/run_benchmark.sh --triple
```

## Current gaps (AOT / micro_ir)

Bytecode VM coverage is largely complete (phase 6.12). **AOT lags behind** because `bytecode → micro_ir` must lower every opcode the examples emit.

### Milestone A — Language basics (`01-language/`)

| Example | Interpreter | VM | AOT compile |
|---------|-------------|-----|-------------|
| `basics.dryad` | ✅ | ✅ | ✅ |
| `control_flow.dryad` | ✅ | ✅ | ✅ (native exec: loops/print; array sum partial) |
| `functions.dryad` | ✅ | ✅ | ✅ (closures via interpreter fallback) |
| `classes.dryad` | ✅ | ✅ | ✅ |
| `compile_decorator.dryad` | ✅ | ✅ | ✅ |

**Recent (6.15):**
- CFG: `IrBasicBlock` + `Br`/`BrCond` → LLVM `br`/`br i1`
- `aot_runtime.c`: `dryad_print`, `dryad_new_array`, `dryad_array_push`, `dryad_variant_get_element`
- VM: `__dryad_main` bytecode path, interpreter fallback for stdlib/methods/closures
- `ADD` → `AddI32` when both operands are `I32`

### Milestone B — Stdlib (`02-` … `07-`)

Buffers, collections, I/O, async, net examples require `CALL_GLOBAL` + intrinsic slow-path implementations in `aot_runtime.c`, or interpreter fallback via a richer native runtime delegate.

## Work order

1. **micro_ir opcode parity** — map every bytecode opcode (comparisons ✅, generic arithmetic, arrays, calls)
2. **LLVM CFG** — emit basic blocks for `JUMP` / `JUMP_IF_FALSE` / `JUMP_IF_TRUE`
3. **aot_runtime.c** — `dryad_intrinsic_slow`, `dryad_variant_get_element`, `print`, stdlib hooks
4. **Gate script** — `run_benchmark.sh --triple` stays green; add CI job when coverage > 80%

## Key files

| Area | Path |
|------|------|
| Examples manifest | `dryad-cpp/examples/manifest.json` |
| Triple benchmark | `dryad-cpp/examples/run_benchmark.sh --triple` |
| micro_ir lowering | `dryad-cpp/src/aot/micro_ir.cpp` |
| LLVM emission | `dryad-cpp/src/aot/llvm_backend.cpp` |
| Native runtime | `dryad-cpp/src/aot/aot_runtime.c` |
