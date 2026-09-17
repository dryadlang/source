# Packages & dryad.toml

Package manifests for dependency management and installation.

**Full reference:** [DRYAD_TOML.md](../implementation/tooling/DRYAD_TOML.md)

## Minimal manifest

```toml
name = "@acme/sample"
version = "0.1.0"
entry = "src/main.dyd"

[dependencies]
"@test/geom" = "../libs/@test/geom"
```

## Fields

| Field | Required | Description |
|-------|----------|-------------|
| `name` | yes | Package identifier (`@vendor/pkg`) |
| `version` | yes | Semantic version |
| `entry` | no | Main module path |
| `[dependencies]` | no | Map of name → local path |

## Install

```bash
dryad install                    # reads dryad.toml in CWD
dryad install --target ./mods   # custom install directory
```

Dependencies are copied/linked into `dryad_modules/` (or `--target`).

## Dependency paths

Each dependency value may point to:

- A `.dyd` / `.dryad` file
- A directory containing `<name>.dyd`, `<name>.dryad`, or `index.dyd`

Paths resolve relative to the manifest file.

## Re-exports

```dryad
export { Vec2 } from "@test/geom";
```

## See also

- [Library overview](overview.md)
- [Module resolution](../language/modules.md)
