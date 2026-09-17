# Library examples

Demonstrates the Dryad library system (`.dyd` source + `.dyl` binary).

## Prerequisites

```bash
export DRYAD_LIBRARY_PATH=../../tests/fixtures/libs
```

## Run

```bash
cd dryad-cpp
./build/bin/dryad examples/08-libraries/library_usage.dyd
```

## Compile library

```bash
./build/bin/dryad compile tests/fixtures/libs/@test/geom.dyd --lib -o /tmp/geom.dyl
./build/bin/dryad link /tmp/geom.dyl -o /tmp/geom.ll
./build/bin/dryad compile tests/fixtures/libs/@test/geom.dyd --lib --native -o /tmp/geom.dyl
```

## Package install

```bash
./build/bin/dryad install tests/fixtures/packages/sample/dryad.toml --modules /tmp/dryad_modules
```

See [docs/implementation/tooling/DRYAD_TOML.md](../../../docs/implementation/tooling/DRYAD_TOML.md).
