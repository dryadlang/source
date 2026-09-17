# Library System — Implementation Plan (L1–L4)

**Date**: 2026-09-17  
**Status**: L4 complete (scalar + class AOT, hybrid .dyl+.so)  
**Architecture**: [LIBRARY_SYSTEM.md](../architecture/LIBRARY_SYSTEM.md)

---

## Overview

| Phase | Goal | Gate |
|-------|------|------|
| **L1** | Source libraries with explicit `export` | `import { Vec2 } from "@test/geom"` via `.dyd` + `DRYAD_LIBRARY_PATH` |
| **L2** | Compiled `.dyl` distribution | App imports from `.dyl` without source |
| **L3** | Package tooling | `dryad.toml`, install, re-exports, relative imports |
| **L4** | AOT library linking | Native ABI for classes in `.dyl` |

---

## L1 — Source libraries (current sprint)

### L1.1 Export syntax

- [x] `is_exported` on `FunctionDeclaration`, `ClassDeclaration`, `VariableDeclaration`
- [x] Parser: `export function`, `export class`, `export let` / `export const`
- [x] Reject `export internal` combinations

### L1.2 Export validation

- [x] `ModuleLoader` builds export table from AST after parse
- [x] If module has **any** explicit export → only exported names importable
- [x] If module has **no** exports → legacy (all top-level non-internal bindings)
- [x] `internal` functions/classes blocked at import (existing + extended)

### L1.3 Module resolution

- [x] Try `.dyd` then `.dryad` for every resolved path
- [x] `DRYAD_LIBRARY_PATH` (`:`-separated) for `@vendor/pkg/...`
- [x] `@std/...` unchanged (stdlib root)

### L1.4 Module isolation

- [x] `Interpreter::runtime_env()` — intrinsics + natives
- [x] `global_env()` parents to `runtime_env()`
- [x] Module env parents to `runtime_env()` only (not user globals)

### L1.5 API surface

- [x] `ModuleLoader::set_library_paths()`
- [x] `Interpreter::set_library_paths()`

### L1.6 Tests

- [x] `tests/fixtures/libs/@test/geom.dyd`
- [x] `tests/unit/library_test.cpp` — export, reject private, `.dyd`, library path

---

## L2 — `.dyl` compiled libraries (complete)

### L2.1 Binary format `DRYADYL` v1

```
Header: magic "DRYADYL", version u32, flags u32
Manifest: package name, version, language version (strings)
Export table: [{ name, kind: function|class|value, index }]
Class registry: [{ name, methods[], ctor_index }]
Bytecode blobs: per-function (reuse .dryadc encoding)
Constant pool: shared literals
Optional: embedded source for debug
```

### L2.2 Build

```bash
dryad compile path/to/lib.dyd --lib -o path/to/lib.dyl
```

Files: `library_module.h/cpp`, `main.cpp` `--lib` flag

### L2.3 Loader

- `LibraryLoader::load_dyl()` → `ModuleInfo` (env + exports)
- `ModuleLoader::resolve_module_path()` prefers fresh `.dyl` over `.dyd`
- Staleness: `.dyl` mtime ≥ source mtime

### L2.4 Class serialization

- Serialize `DryadClass` name, super, method table, bytecode indices
- Deserialize into importer `runtime_env` scope module env

### L2.5 Gate

Build `@test/geom.dyl`, delete `geom.dyd`, app still imports `Vec2`.

---

## L3 — Packages & tooling (complete)

### L3.1 Manifest

- [x] `dryad.toml` — `name`, `version`, `entry`, `[dependencies]`
- [x] `package_manifest.cpp` — minimal TOML parser

### L3.2 Install

- [x] `dryad install path/to/dryad.toml` → `dryad_modules/` (or `--modules`)
- [x] Copies dependency sources and compiles to `.dyl`

### L3.3 Re-exports

- [x] `export { X } from "@pkg/mod"` barrel syntax
- [x] `execute_export_from` at runtime
- [x] Re-exported symbols included in `--lib` compile output

### L3.4 Relative imports

- [x] `import { f } from "./utils"` and `../` paths
- [x] Import context stack from `run_file` and module load

### L3.5 Compile options

- [x] `CompileOptions::library_paths` for dependency resolution during `--lib`
- [x] `prefer_source_modules` when compiling libraries (re-export from source, not stub `.dyl`)

### L3.6 Tests

- [x] `tests/fixtures/packages/sample/`
- [x] `tests/unit/package_test.cpp` (4 tests)

### L3.7 Static circular-import detection

- [x] `import_cycle_detector.cpp` — compile-time cycle detection
- [x] `tests/fixtures/libs/@cycle/` + `import_cycle_test.cpp`

### L3.8 Documentation

- [x] [DRYAD_TOML.md](../tooling/DRYAD_TOML.md)

---

## L4 — AOT libraries (complete)

### L4.1 Scalar library linking

- [x] `library_linker.cpp` — `.dyl` → LLVM IR (`library_to_ir_module`)
- [x] `compile_library_to_llvm_ir()` / `dryad link lib.dyl [-o out.ll] [--shared]`
- [x] `CompileOptions::link_libraries` + `--link-lib` on `dryad compile`
- [x] Direct `CALL_GLOBAL` → linked library symbol in LLVM backend
- [x] `tests/fixtures/libs/@test/math.dyd` + `library_aot_test.cpp`

### L4.2 Class-aware native ABI

- [x] Class ctor/method IR emission from `.dyl`
- [x] `dryad_instance_*` runtime helpers in `aot_runtime.c`
- [x] `NEW_INSTANCE` / `METHOD_CALL` / `GET_PROPERTY` / `SET_PROPERTY` native dispatch
- [x] `LibraryAotTest.LinksExecutableAgainstDylClassLibrary` (Vec2.sum gate)

### L4.3 Hybrid `.dyl` + `.so` distribution

- [x] `compile_library_to_native()` + `library_native_companion_path()`
- [x] `dryad compile --lib --native`
- [x] `dryad install` emits `.so` companion when clang is available
- [x] `LibraryAotTest.EmitsNativeCompanionForDylLibrary`

### Examples

- [x] `examples/08-libraries/library_usage.dyd`

---

## File map

| File | L1 | L2 |
|------|----|----|
| `ast.h` | `is_exported`, `is_internal` on class | — |
| `parser.cpp` | `export` keyword | — |
| `module_loader.cpp` | resolve, exports, isolation | `.dyl` dispatch |
| `interpreter.cpp` | `runtime_env_` split | — |
| `library_module.cpp` | — | new |
| `main.cpp` | — | `--lib` |

---

## Backward compatibility

- Stdlib modules without `export` keep implicit export semantics
- `.dryad` remains valid everywhere `.dyd` is accepted
- `DRYAD_LIBRARY_PATH` optional; unset = no vendor path search
