# User Libraries & Packages

Dryad supports first-class user libraries beyond the built-in `@std/*` modules.

## Guides

| Guide | Description |
|-------|-------------|
| [Overview](overview.md) | `.dyd`, `.dyl`, exports, resolution, AOT linking |
| [Packages](packages.md) | `dryad.toml`, `dryad install`, dependencies |

## Quick start

### Write a library

`libs/@my/geom.dyd`:

```dryad
export class Vec2 {
    function Vec2(x, y) { this.x = x; this.y = y; }
    function sum() { return this.x + this.y; }
}

export function makeVec2(x, y) { return new Vec2(x, y); }
export let ORIGIN_X = 0;
```

### Use it

```dryad
import { Vec2, makeVec2 } from "@my/geom";

let v = new Vec2(3, 4);
print(v.sum());
```

```bash
export DRYAD_LIBRARY_PATH=libs
dryad app.dryad
```

### Compile to binary library

```bash
dryad compile --lib libs/@my/geom.dyd -o libs/@my/geom.dyl
```

## File types

| Extension | Role |
|-----------|------|
| `.dyd` | Library source (preferred) |
| `.dryad` | Source (legacy, supported) |
| `.dyl` | Compiled library (DRYADYL) |
| `.so` | AOT native library (hybrid distribution) |

## Example

`dryad-cpp/examples/08-libraries/library_usage.dyd` — see [08-libraries README](../../dryad-cpp/examples/08-libraries/README.md).

## Architecture

Full specification: [LIBRARY_SYSTEM.md](../implementation/architecture/LIBRARY_SYSTEM.md).

Implementation plan: [library-system-implementation.md](../implementation/plans/2026-09-17-library-system-implementation.md).
