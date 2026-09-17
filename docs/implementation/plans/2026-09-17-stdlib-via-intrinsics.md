# Phase 6.13 — Stdlib via Intrinsics

> **Status:** Complete  
> **Depends on:** [BYTECODE_COVERAGE.md](BYTECODE_COVERAGE.md) (6.12)

## Goal

Allow `@compile("bytecode")` functions to call stdlib APIs imported at program setup time, while stdlib internals continue to use `INTRINSIC_SYSCALL` for zero-overhead C++ dispatch.

## Design

| Layer | Mechanism |
|-------|-----------|
| User bytecode → stdlib function | `CALL_GLOBAL` → `VmRuntimeDelegate::invoke_global` → interpreter |
| Stdlib → C++ runtime | `__runtime_*`, `__syscall_*` → `INTRINSIC_SYSCALL` |
| User bytecode → intrinsic | Direct `INTRINSIC_SYSCALL` emission |

`CALL_GLOBAL` (opcode `0x89`) resolves function names against the interpreter global environment populated by `import` during program setup.

## Key files

- `opcode.h` — `CALL_GLOBAL = 0x89`
- `vm_runtime_delegate.h` — `invoke_global()`
- `execution_engine.cpp` — `ExecutionVmRuntime::invoke_global`
- `bytecode_compiler.cpp` — unknown local calls emit `CALL_GLOBAL`
- `vm.cpp` — dispatcher

## Tests

- `ExecutionEngineTest.BytecodeCallsStdlibBufferViaGlobal` — Buffer roundtrip from bytecode
- Integration examples: `02-buffers/buffer_usage.dryad`, `04-io/file_io_usage.dryad`

## Next

- Tier-2 JIT for `CALL_GLOBAL` hot paths (optional)
- Closures (`MAKE_CLOSURE`) for first-class functions in bytecode
