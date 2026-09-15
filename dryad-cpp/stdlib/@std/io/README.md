# @std/io — File I/O Module

High-level file operations built on a Virtual File System (VFS) with pluggable backends.

## Components

| Module | Purpose |
|--------|---------|
| `vfs.dryad` | VFS routing, `MemoryBackend`, `NativeBackend` |
| `file.dryad` | `readFile`, `writeFile`, `exists`, `removeFile`, `listDirectory`, `mkdir` |
| `filehandle.dryad` | Resource-managed read/write/append handles |
| `io_errors.dryad` | Path validation, normalization, standardized error helpers |
| `io_concurrency.dryad` | Cooperative lock wrapper (`ThreadSafeVFS`) |
| `stream.dryad` | **Deferred** — abstract Stream API awaits language support |

## Quick start

```dryad
import { readFile, writeFile, setVFSBackend } from "@std/io/file";
import { MemoryBackend } from "@std/io/vfs";

setVFSBackend(MemoryBackend());
writeFile("/test.txt", "Hello");
let content = readFile("/test.txt");
```

Native filesystem (default backend):

```dryad
import { writeFile, readFile } from "@std/io/file";

writeFile("notes.txt", "On disk");
let text = readFile("notes.txt");
```

## Error model

Errors are returned as **values** (strings from `@std/core/exceptions`), never thrown across the language boundary:

```dryad
let err = VFS_readFile(vfs, "/missing.txt");
if (err != null) {
    // handle FileNotFoundException string
}
```

Paths are normalized (`\` → `/`) before backend dispatch.

## Running examples

```bash
./bin/dryad examples/io_examples.dryad
./bin/dryad examples/file_io_usage.dryad
```

Set `DRYAD_STDLIB_DIR` if the stdlib is not at the default install path.
