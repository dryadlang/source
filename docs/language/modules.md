# Modules & Imports

Dryad uses ES6-style modules: named imports, explicit exports, and scoped package paths.

## Import syntax

```dryad
import { List, Map } from "@std/collections";
import { readFile } from "@std/io/file";
import { Vec2 } from "@test/geom";
```

- **Scoped paths** (`@std/...`, `@vendor/pkg`) resolve via stdlib root and `DRYAD_LIBRARY_PATH`
- **Relative paths** (`./utils`, `../shared`) resolve from the importing file

## Export syntax

```dryad
export function compute(x) { return x * 2; }
export class Widget { }
export let VERSION = "1.0";

export { helper } from "./internal/helpers";
```

| Modifier | Visibility |
|----------|------------|
| `export` | Available to importers |
| `internal` | Package-private (not re-exported) |
| (none) | Top-level, implicit export in scripts; controlled in `.dyd` libraries |

## File extensions

| Extension | Role |
|-----------|------|
| `.dryad` | Script / module source (legacy name, kept forever) |
| `.dyd` | Preferred short extension for libraries |
| `.dyl` | Compiled library binary (import without source) |
| `.dryadc` | Single-script bytecode cache |

## Resolution order

1. Relative path from importing file
2. `@scope/name` → `DRYAD_LIBRARY_PATH` / `dryad_modules/`
3. Source: `.dyd` preferred, `.dryad` fallback
4. Binary: `.dyl` when source is absent

## Environment

| Variable | Purpose |
|----------|---------|
| `DRYAD_STDLIB_DIR` | Path to `stdlib/` (default: next to binary) |
| `DRYAD_LIBRARY_PATH` | Colon-separated library search paths |

## Packages (dryad.toml)

For multi-module projects with dependencies:

```toml
name = "@acme/myapp"
version = "0.1.0"

[dependencies]
"@test/geom" = "../libs/@test/geom"
```

```bash
dryad install              # installs to dryad_modules/
```

See [packages guide](../libraries/packages.md) and [DRYAD_TOML](../implementation/tooling/DRYAD_TOML.md).

## Circular imports

The compiler detects import cycles statically and reports the cycle path.

## User libraries

Full architecture: [libraries/overview.md](../libraries/overview.md).

## See also

- [Standard library index](../stdlib/README.md)
- [Library system architecture](../implementation/architecture/LIBRARY_SYSTEM.md)
