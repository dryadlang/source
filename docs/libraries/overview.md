# Library System Overview

User-authored libraries with explicit exports, compiled distribution, and AOT linking.

**Status:** L1–L4 implemented  
**Full spec:** [LIBRARY_SYSTEM.md](../implementation/architecture/LIBRARY_SYSTEM.md)

## Capabilities

| Level | Feature | Status |
|-------|---------|--------|
| L1 | Source libraries (`.dyd`), `export`, `DRYAD_LIBRARY_PATH` | ✅ |
| L2 | Compiled `.dyl` libraries (`dryad compile --lib`) | ✅ |
| L3 | `dryad.toml`, `dryad install`, re-exports, relative imports | ✅ |
| L4 | AOT library linking (scalar, classes, hybrid `.dyl`+`.so`) | ✅ |

## Terminology

| Term | Meaning |
|------|---------|
| **Module** | One `.dyd` / `.dryad` file |
| **Package** | Directory + `dryad.toml` manifest |
| **Library** | Compiled `.dyl` artifact |
| **Specifier** | Import string: `@scope/name`, `./relative` |
| **Export** | Symbol exposed to importers |
| **Internal** | Package-private symbol |

## Export model

```dryad
export function publicFn() { }
export class Widget { }
export let VERSION = "1.0";
internal function helper() { }
```

Without `export`, symbols are module-private when compiling libraries.

## Resolution

1. Relative: `./utils` from importing file
2. Scoped: `@vendor/pkg` via `DRYAD_LIBRARY_PATH` / `dryad_modules/`
3. Source: `.dyd` → `.dryad`
4. Binary: `.dyl` when source absent

## Compile commands

```bash
# Source → .dyl
dryad compile --lib src/@acme/utils.dyd -o out/@acme/utils.dyl

# Source → native .so
dryad compile --lib --native src/@acme/utils.dyd -o out/utils.so

# Link libraries into AOT app
dryad compile app.dryad --aot --link-lib @acme/utils -o app.ll
```

## Circular imports

Detected statically at compile time with cycle path in the error message.

## Triple execution

Libraries work in interpreter, VM (`--vm`), and AOT modes. See `LibraryTripleTest` in unit tests.

## See also

- [Modules & imports](../language/modules.md)
- [Packages](packages.md)
- [dryad.toml reference](../implementation/tooling/DRYAD_TOML.md)
