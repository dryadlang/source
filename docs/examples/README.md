# Examples

Runnable Dryad examples demonstrating language features and standard library APIs.

**Canonical catalog:** [dryad-cpp/examples/README.md](../../dryad-cpp/examples/README.md)

## Run all examples

```bash
cd dryad-cpp
cmake -B build && cmake --build build -j$(nproc)
./examples/run_benchmark.sh
```

## Triple coverage

Every example should pass interpreter, VM, and AOT:

```bash
./examples/run_benchmark.sh --triple
```

## By category

| Category | Path | Docs |
|----------|------|------|
| Language basics | `01-language/` | [syntax](../language/syntax.md) |
| Buffers | `02-buffers/` | [buffers](../stdlib/buffers.md) |
| Collections | `03-collections/` | [collections](../stdlib/collections.md) |
| I/O | `04-io/` | [io](../stdlib/io.md) |
| Async | `05-async/` | [async](../stdlib/async.md) |
| Networking | `06-net/` | [net](../stdlib/net.md) |
| Tooling | `07-tooling/` | [cli](../tooling/cli.md) |
| Libraries | `08-libraries/` | [libraries](../libraries/README.md) |
| DryadSquare | `dryadsquare/` | Turing benchmark |

## Special examples

| Example | Notes |
|---------|-------|
| `tcp_client.dryad` | Requires `--allow-net` + echo server on `:9090` |
| `library_usage.dyd` | Requires `DRYAD_LIBRARY_PATH=tests/fixtures/libs` |
| `dryadsquare/` | Full Turing machine benchmark |
