# dryad.toml — Package Manifest

**Status**: implemented (L3)  
**Related**: [LIBRARY_SYSTEM.md](../architecture/LIBRARY_SYSTEM.md)

## Overview

`dryad.toml` describes a Dryad package: its name, version, optional entry module, and local dependencies. The `dryad install` command reads this file and installs dependencies into `dryad_modules/` (or a custom directory).

## Minimal example

```toml
name = "@acme/sample"
version = "0.1.0"

[dependencies]
"@test/geom" = "../../libs/@test/geom"
```

## Fields

| Field | Required | Description |
|-------|----------|-------------|
| `name` | yes | Package identifier (`@vendor/pkg`) |
| `version` | yes | Semantic version string |
| `entry` | no | Main module path (source file) |
| `[dependencies]` | no | Map of dependency name → local path |

## Dependency paths

Each dependency value may point to:

- A `.dyd` / `.dryad` file
- A directory containing `<name>.dyd`, `<name>.dryad`, or `index.dyd`

Paths are resolved relative to the manifest file location.

## Install

```bash
dryad install                          # reads ./dryad.toml
dryad install path/to/dryad.toml       # explicit manifest
dryad install --modules vendor/libs  # custom output directory
```

Install copies dependency sources and compiles each to `.dyl` under the modules directory:

```
dryad_modules/
  @test/
    geom.dyd
    geom.dyl
```

## Entry module

When `entry` is set, install also copies and compiles the package entry:

```toml
name = "@acme/sample"
version = "0.1.0"
entry = "src/app.dyd"

[dependencies]
"@test/geom" = "../../libs/@test/geom"
```

## Environment

| Variable | Effect |
|----------|--------|
| `DRYAD_STDLIB_DIR` | Stdlib root used when compiling installed modules |
| `DRYAD_LIBRARY_PATH` | Extra search paths for imports at runtime |

## Limitations (v1)

- Local path dependencies only (no remote registry)
- Minimal TOML parser (string keys/values, `[dependencies]` table)
- No lockfile yet

## See also

- `tests/fixtures/packages/sample/dryad.toml` — reference fixture
- `dryad compile --lib` — build `.dyl` libraries
- `dryad compile --lib --native` — build `.dyl` + `.so` hybrid distribution
