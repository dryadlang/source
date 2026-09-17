# @std/core

Fundamental interfaces and error factories used across the standard library.

**Source:** `dryad-cpp/stdlib/@std/core/`

## Modules

| Module | Path | Exports |
|--------|------|---------|
| Disposable | `@std/core/disposable` | `interface IDisposable { dispose() }` |
| Enumerable | `@std/core/enumerable` | `interface IEnumerable { forEach(callback) }` |
| Exceptions | `@std/core/exceptions` | 15 error factory functions |

## IDisposable

```dryad
import { IDisposable } from "@std/core/disposable";

// Implemented by: Stream, Socket, TcpClient, FileHandle, Buffer (via Dispose)
```

## IEnumerable

```dryad
import { IEnumerable } from "@std/core/enumerable";

// Implemented by: List, Set, Queue
```

## Exception factories

Errors are **values** (strings), not thrown exceptions:

```dryad
import { ArgumentException, IOException } from "@std/core/exceptions";

function validate(x) {
    if (x == null) {
        return ArgumentException("x cannot be null");
    }
    return null;  // no error
}
```

| Factory | Use case |
|---------|----------|
| `Exception` | Generic error |
| `ArgumentException` | Invalid argument |
| `ArgumentNullException` | Null parameter |
| `ArgumentOutOfRangeException` | Index/range error |
| `IOException` | I/O failure |
| `FileNotFoundException` | Missing file |
| `DirectoryNotFoundException` | Missing directory |
| `EndOfStreamException` | Read past EOF |
| `InvalidOperationException` | Invalid state |
| `ObjectDisposedException` | Use after dispose |
| `IndexOutOfRangeException` | Array index error |
| `KeyNotFoundException` | Map key missing |
| `NetworkException` | Network failure |
| `SocketException` | Socket error |
| `HttpException` | HTTP protocol error |

## Pattern

```dryad
let err = someOperation();
if (err != null) {
    // handle error string
    return err;
}
```

## See also

- [I/O errors](io.md) — path validation helpers
- [Net errors](net.md) — socket error helpers
