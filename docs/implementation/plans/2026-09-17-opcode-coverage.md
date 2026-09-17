# Phase 6.11 — Opcode Coverage (arrays, for, class methods)

> **Status:** Complete (6.11) — see [BYTECODE_COVERAGE.md](BYTECODE_COVERAGE.md) for full matrix (6.12)  
> **Depends on:** Phase 6.10 (triple execution model foundation)

## Goal

Bytecode compiler + VM support the language constructs needed for real programs:

- Array literals and indexed assignment
- C-style `for` loops
- Class instantiation and method calls

## New opcodes

| Opcode | Code | Stack effect | Purpose |
|--------|------|--------------|---------|
| `NEW_ARRAY` | `0x80` | `→ arr` | Empty array |
| `ARRAY_PUSH` | `0x81` | `arr, v → arr` | Literal / push element |
| `SET_ELEMENT` | `0x82` | `arr, idx, v → v` | `arr[i] = v` |
| `GET_PROPERTY` | `0x83` | `obj → v` | Instance field / getter |
| `SET_PROPERTY` | `0x84` | `obj, v → v` | Instance field / setter |
| `GET_ELEMENT` | `0x85` | *(existing)* | Generic index read |
| `GET_ELEMENT_F64` | `0x86` | *(existing)* | Typed f64 index read |
| `METHOD_CALL` | `0x87` | `recv, args… → r` | `obj.method(args)` |
| `NEW_INSTANCE` | `0x88` | `class, args… → inst` | `new Class(args)` |

Property / method names are **u16 constant pool indices** (strings).

## Compiler emission

| AST | Bytecode |
|-----|----------|
| `[a, b, c]` | `NEW_ARRAY`; push each element + `ARRAY_PUSH` |
| `arr[i] = v` | compile `arr`, `i`, `v`; `SET_ELEMENT` |
| `for (init; cond; incr) body` | init; header: cond + `JUMP_IF_FALSE`; body; incr; `JUMP` header |
| `new Foo(args)` | load class constant; args; `NEW_INSTANCE` |
| `obj.m(args)` | load receiver; args; `METHOD_CALL` |
| `obj.field` | load receiver; `GET_PROPERTY` |

## VM runtime delegate

`METHOD_CALL` and `NEW_INSTANCE` need the interpreter for `DryadFunction::call_with_receiver` and `DryadClass::instantiate`.

`VmRuntimeDelegate` (wired by `ExecutionEngine`):

- `get_global(name)` — resolve class bindings
- `invoke_method(receiver, method, args)`
- `new_instance(class_value, args)`

Pure array opcodes run entirely inside the VM.

## Success criteria

- [x] `for` parses and runs in interpreter + bytecode
- [x] Array literals and `arr[i] = v` in bytecode match interpreter
- [x] `new` + method calls work in `@compile("bytecode")` functions
- [x] Micro-IR treats new opcodes as slow-path / intrinsic where needed
- [x] Unit tests in `bytecode_compiler_test.cpp` and `execution_engine_test.cpp`
