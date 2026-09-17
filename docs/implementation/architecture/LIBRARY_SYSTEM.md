# Dryad Library System — Architecture Specification

**Version**: 0.1 (draft)  
**Date**: 2026-09-17  
**Status**: L1–L4 implemented  
**Related**: [STDLIB_ARCHITECTURE.md](STDLIB_ARCHITECTURE.md), triple-execution model, `.dryadc` / `.so` companions

---

## 1. Problem statement

Today Dryad can **import named symbols from `.dryad` source files**, but this is not a **library platform**:

| Capability | Today | Required for ecosystem growth |
|------------|-------|-------------------------------|
| User-authored libraries | Ad-hoc paths / CWD-relative `.dryad` | First-class packages with discovery |
| Export surface | Implicit (everything top-level) | Explicit `export` with visibility |
| Structures / classes in API | Works at runtime only | Serializable in compiled libs |
| Compiled distribution | `.dryadc` = single-script bytecode | `.dyl` = linkable library unit |
| Source convention | `.dryad` only | `.dyd` as canonical short extension |
| Dependency graph | None | `dryad.toml` / lockfile (later) |

Users must be able to:

1. Write a library in Dryad (`.dyd` / `.dryad`).
2. **Export** functions, classes, and constants with controlled visibility.
3. **Import** those symbols from other projects: `import { List } from "@acme/collections";`
4. **Compile** the library once into `.dyl` and ship it like a DLL — loadable without re-parsing source.

---

## 2. Terminology

| Term | Meaning |
|------|---------|
| **Module** | One compilation unit: one `.dyd` / `.dryad` file (or one entry in a `.dyl`) |
| **Package** | Directory tree + manifest; maps names like `@acme/foo` → files |
| **Library** | Compiled `.dyl` artifact aggregating one or more modules |
| **Specifier** | Import string: `@std/io/file`, `@acme/utils`, `./relative` (future) |
| **Export** | Symbol intentionally exposed to importers |
| **Internal** | Symbol visible only inside the module/package |

### File extensions

| Extension | Role | Analogy |
|-----------|------|---------|
| `.dryad` | Legacy / full source extension (kept forever) | `.cpp` |
| **`.dyd`** | **Preferred short source extension** for modules & libraries | `.rs` / `.py` |
| `.dryadc` | Single-entry bytecode cache (existing) | object file for one script |
| **`.dyl`** | **Dryad librarY** — compiled, importable library archive | `.dll` / `.so` / `.jar` |
| `.so` | Native AOT companion per script (existing, scalar ABI) | not a Dryad library |

**Rule**: `.dyd` and `.dryad` are **equivalent** at the language level. Tooling should prefer `.dyd` for new library code; examples may migrate gradually.

---

## 3. Current system (as-is)

### 3.1 Import syntax (implemented)

```dryad
import { readFile, writeFile } from "@std/io/file";
```

- Parser: `parse_import_declaration()` — named imports only, string literal path.
- Loader: `ModuleLoader::load_module()` — read source → lex → parse → execute in new `Environment`.
- Binding: copies requested names into importer `current_env()`.

### 3.2 Resolution (`module_loader.cpp`)

```
@std/foo/bar  →  {DRYAD_STDLIB_DIR}/@std/foo/bar.dryad
other/path    →  other/path.dryad  (or path as-is if ends with .dryad)
```

**Gaps**: no `.dyd`, no `@vendor/pkg`, no `DRYAD_LIBRARY_PATH`, no relative imports.

### 3.3 Export (not implemented)

- Lexer knows `export`; **no parser, no AST, no semantics**.
- Effective export = “define at top level and hope importers know the name”.
- `internal` on **functions** only — runtime check in `execute_import()`.

### 3.4 Compiled artifacts (wrong layer for libraries)

| Format | Magic | Scope | Importable? |
|--------|-------|-------|-------------|
| `.dryadc` | `DRYADC` v2 | `CompiledProgram` = flat function list for **one entry script** | No |
| `.so` | ELF/Mach-O | Native symbols via `dlsym`, max 4 scalar args | No (not module-aware) |

Imports always re-execute **source** today; `.dryadc` beside a module file is never loaded by `ModuleLoader`.

---

## 4. Target architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        User project                              │
│  main.dyd  ──import──►  @acme/collections  ──►  acme/collections.dyl │
└─────────────────────────────────────────────────────────────────┘
                                    │
                    ┌───────────────┴───────────────┐
                    ▼                               ▼
            Source path (.dyd)              Binary path (.dyl)
         ModuleLoader::load_source()    LibraryLoader::load_dyl()
                    │                               │
                    └───────────────┬───────────────┘
                                    ▼
                          Export table + ClassRegistry
                                    ▼
                          Importer Environment bindings
```

### 4.1 Design principles

1. **Source and binary parity** — same export table whether loaded from `.dyd` or `.dyl`.
2. **Explicit exports** — only exported symbols are importable; everything else is module-private.
3. **Isolated module scope** — module env does **not** parent to importer `global_env()` (breaking change, phased).
4. **Classes are first-class exports** — `DryadClass` metadata travels in `.dyl`.
5. **No host cheating** — library semantics implemented in Dryad/C++ runtime, not per-library C++ hacks.
6. **Triple execution preserved** — `.dyl` loads in interpreter, VM, and AOT-linked programs.

---

## 5. Source modules (`.dyd`)

### 5.1 Export syntax (proposed)

```dryad
// math.dyd
export function add(a, b) {
    return a + b;
}

export class Vec2 {
    constructor(x, y) {
        this.x = x;
        this.y = y;
    }
    length() {
        return sqrt(this.x * this.x + this.y * this.y);
    }
}

internal function _helper() { /* not importable */ }

let _cache = [];  // module-private (not exported)
```

Barrel re-export (stdlib pattern, formalized):

```dryad
// @acme/collections.dyd
export { List } from "@acme/collections/list";
export { Map } from "@acme/collections/map";
```

### 5.2 Import extensions

```dryad
// Named (current)
import { List, Map } from "@acme/collections";

// Future: namespace import
import * as Collections from "@acme/collections";

// Future: relative (within package)
import { helper } from "./utils";
```

### 5.3 Module resolution order

For specifier `S`:

1. **Built-in stdlib**: `@std/...` → `{DRYAD_STDLIB_DIR}/@std/...`.{dyd|dryad}
2. **Installed libraries**: `@vendor/pkg/...` → search `DRYAD_LIBRARY_PATH` and project `dryad_modules/`
3. **Precompiled**: if `foo.dyl` exists beside `foo.dyd` and is fresh → load `.dyl`
4. **Source fallback**: `foo.dyd` or `foo.dryad`
5. **Relative** (phase 2): resolve from importing file directory

Environment variables:

| Variable | Purpose |
|----------|---------|
| `DRYAD_STDLIB_DIR` | Stdlib root (existing) |
| `DRYAD_LIBRARY_PATH` | `:`-separated search paths for `@vendor/...` |
| `DRYAD_MODULE_CACHE` | Optional cache dir for fetched `.dyl` (future registry) |

### 5.4 Package layout (proposed)

```
mylib/
├── dryad.toml          # name, version, exports entry (phase 2)
├── src/
│   └── @acme/
│       ├── collections.dyd      # barrel
│       └── collections/
│           ├── list.dyd
│           └── map.dyd
└── lib/
    └── @acme/collections.dyl    # compiled output
```

---

## 6. Compiled libraries (`.dyl`)

### 6.1 Purpose

A **`.dyl` file** is a **single-file library distribution**:

- One or more modules (or one package entry + nested modules).
- Export table: name → kind (function | class | value) + visibility.
- Bytecode for all exported functions (VM-ready).
- Class metadata: method tables, field layout, constructor indices.
- Constant pool shared across module (strings, literals).
- Optional debug/source map back to `.dyd` paths.

Analogous to **JAR** (classes + manifest) or **.NET assembly**, not a raw `.o`.

### 6.2 File format sketch

```
┌──────────────────────────────────────┐
│ Header                                │
│   magic: "DRYADYL" (8 bytes)          │
│   version: u32                        │
│   flags: u32                          │
│   module_count: u32                   │
├──────────────────────────────────────┤
│ Package manifest (UTF-8 JSON or binary)│
│   name: "@acme/collections"           │
│   version: "1.2.0"                    │
│   dryad_language_version: "0.6.15"      │
├──────────────────────────────────────┤
│ Export table                          │
│   [{ name, kind, module_index, ... }] │
├──────────────────────────────────────┤
│ Class registry                        │
│   [{ name, methods[], fields[], ... }]│
├──────────────────────────────────────┤
│ Per-module bytecode blobs             │
│   (same structure as .dryadc functions)│
├──────────────────────────────────────┤
│ Shared constant pool                  │
├──────────────────────────────────────┤
│ Optional: embedded source (.dyd)      │
│ Optional: LLVM bitcode slice (future) │
└──────────────────────────────────────┘
```

**Version 1** scope (MVP):

- Export table + one module per `.dyl` (no multi-module aggregate yet).
- Functions + classes + `let` constants (immutable).
- Bytecode v2 compatible with current VM.
- No native code inside `.dyl` (AOT links separately).

### 6.3 Build command

```bash
# Compile library source → .dyl
dryad compile lib/@acme/collections.dyd --lib -o lib/@acme/collections.dyl

# Compile app linking against .dyl search path
dryad build app.dyd --libdir ./lib
```

Loader behavior:

```cpp
// Pseudocode
if (exists(path + ".dyl") && is_fresh(path + ".dyl", path + ".dyd"))
    return LibraryLoader::load_dyl(path + ".dyl");
else
    return ModuleLoader::load_source(path + ".dyd");
```

### 6.4 Relationship to `.dryadc` and `.so`

| Artifact | Granularity | Used by |
|----------|-------------|---------|
| `.dryadc` | Single script entry | `@compile("bytecode")` companion cache |
| `.dyl` | Library package | `import { X } from "@vendor/lib"` |
| `.so` | Single script AOT | `@compile("AOT")` native companion |

**Do not** overload `.dryadc` as library format — different lifecycle and linking model.

---

## 7. Classes / structures in libraries

Dryad uses **classes** (no separate `struct` keyword). Library support requires:

### 7.1 Export class

```dryad
export class Point {
    constructor(x, y) { this.x = x; this.y = y; }
    move(dx, dy) { this.x = this.x + dx; this.y = this.y + dy; }
}
```

Importer:

```dryad
import { Point } from "@acme/geom";
let p = new Point(1, 2);
```

### 7.2 Runtime requirements

| Layer | Change |
|-------|--------|
| Parser / AST | `export class`, `export let` |
| Semantic | Export surface validation at import sites |
| Interpreter | Already creates `DryadClass` in module env |
| `.dyl` serializer | Write class name, method names, bytecode indices |
| `.dyl` loader | Reconstruct `DryadClass` in importer without re-parsing source |
| VM | `NEW_INSTANCE`, `METHOD_CALL` against library-defined classes |
| AOT | Phase 3+ — class-aware native ABI (not v1) |

### 7.3 `internal` visibility

Extend to **classes and fields**:

```dryad
export class List {
    internal function _grow() { ... }  // method not on public API
}
```

Import rule: reject if any requested symbol is `internal` (compile-time when possible).

---

## 8. Module isolation (breaking fix)

**Current bug / limitation** (`module_loader.cpp:66`):

```cpp
auto module_env = std::make_shared<Environment>(interpreter->global_env());
```

Module code can read host globals. **Target**:

```cpp
auto module_env = std::make_shared<Environment>(nullptr);  // isolated
// Pre-bind only: other imported modules + intrinsics registry
```

Phased rollout with feature flag `DRYAD_ISOLATED_MODULES=1` for testing.

---

## 9. Implementation roadmap

### Phase L1 — Source libraries (foundation)

| Task | Description |
|------|-------------|
| L1.1 | Parse `export` on functions, classes, `let` |
| L1.2 | Export table on `Program`; reject non-exported imports |
| L1.3 | Resolve `.dyd` alongside `.dryad` in `ModuleLoader` |
| L1.4 | `DRYAD_LIBRARY_PATH` for `@vendor/pkg` → `path/@vendor/pkg/...` |
| L1.5 | Module isolation behind env flag |
| L1.6 | Tests: user lib in `tests/fixtures/libs/` |

**Gate**: `import { Vec2 } from "@test/geom"` works from `.dyd` with explicit export.

### Phase L2 — `.dyl` v1 (compiled library)

| Task | Description |
|------|-------------|
| L2.1 | Define `DRYADYL` binary schema v1 |
| L2.2 | `dryad compile --lib -o foo.dyl` |
| L2.3 | `LibraryLoader::load_dyl()` wired into `ModuleLoader` |
| L2.4 | Serialize/deserialize `DryadClass` + exported functions |
| L2.5 | Staleness: `.dyl` mtime vs `.dyd` source |
| L2.6 | VM loads imported functions from `.dyl` bytecode |

**Gate**: build `@test/geom.dyl`, import in app without source present.

### Phase L3 — Packages & tooling

| Task | Description |
|------|-------------|
| L3.1 | `dryad.toml` manifest (name, version, dependencies) |
| L3.2 | `dryad install` / local `dryad_modules/` |
| L3.3 | Barrel `export { X } from "..."` |
| L3.4 | Relative imports `./foo` |
| L3.5 | `dryad publish` (registry — future) |

### Phase L4 — AOT libraries (complete)

- Scalar + class native ABI for linked `.dyl` libraries.
- `dryad link`, `dryad compile --link-lib`, `dryad compile --lib --native`.
- See [DRYAD_TOML.md](../tooling/DRYAD_TOML.md) for package manifests.

---

## 10. Mapping to existing code

| Component | File | L1 change | L2 change |
|-----------|------|-----------|-----------|
| Parser | `parser.cpp` | `parse_export_*` | — |
| AST | `ast.h` | `ExportDeclaration` | — |
| Semantics | `semantic_analyzer.cpp` | export validation | — |
| Loader | `module_loader.cpp` | `.dyd`, lib path, isolation | dispatch `.dyl` |
| Bytecode | `bytecode_module.cpp` | — | shared serialize helpers |
| New | `library_module.h/cpp` | — | `.dyl` load/save |
| CLI | `main.cpp` | — | `--lib` flag |
| Tests | `tests/unit/library_*` | import/export | `.dyl` round-trip |

---

## 11. Open questions

1. **`.dyd` migration**: rename stdlib to `.dyd` or dual-extension indefinitely?  
   → Recommend dual-extension indefinitely; docs prefer `.dyd`.

2. **Mutable export `let`**: allow `export let counter = 0` or only `export const`?  
   → v1: immutable exported bindings only.

3. **Circular imports**: keep runtime error or add compile-time cycle detection?  
   → Keep runtime guard; add static analysis in L3.

4. **Generic / parametric classes** (`List<T>`): defer until type system matures.

5. **`.dyl` compression**: zlib optional chunk in v2.

---

## 12. Success criteria

The library system is **v1 complete** when:

1. A user can publish `mylib/src/@acme/geom.dyd` with `export class Vec2`.
2. Another project runs `import { Vec2 } from "@acme/geom"` via `DRYAD_LIBRARY_PATH`.
3. `dryad compile --lib` produces `geom.dyl` loadable without source.
4. Triple benchmark passes with an app that imports only from `.dyl`.
5. No C++ changes required per user library — only Dryad source + standard toolchain.

---

## Appendix A — Comparison with `.dryadc`

| | `.dryadc` | `.dyl` |
|---|-----------|--------|
| Magic | `DRYADC` | `DRYADYL` |
| Unit | Entry script functions | Library export surface |
| Classes | Not serialized | Class registry required |
| Import | Not used by ModuleLoader | Primary distribution |
| CLI | `compile -o x.dryadc` | `compile --lib -o x.dyl` |

## Appendix B — Example end-to-end (target state)

```bash
# Author
cat > libs/@acme/geom.dyd <<'EOF'
export class Vec2 {
    constructor(x, y) { this.x = x; this.y = y; }
}
EOF
dryad compile libs/@acme/geom.dyd --lib -o libs/@acme/geom.dyl

# Consumer
export DRYAD_LIBRARY_PATH=libs
cat > app.dyd <<'EOF'
import { Vec2 } from "@acme/geom";
let v = new Vec2(3, 4);
print(v.x);
EOF
dryad app.dyd
```

Expected output: `3`
