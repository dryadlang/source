# @std/buffers

Managed byte buffers — the foundation for file I/O, networking, and HTTP wire encoding.

**Source:** `dryad-cpp/stdlib/@std/buffers/buffer.dryad`

## Import

```dryad
import {
    Buffer_New, Buffer_Len, Buffer_Read, Buffer_Write,
    Buffer_Slice, Buffer_FromString, Buffer_AsString, Buffer_Dispose
} from "@std/buffers/buffer";
```

## Quick start

```dryad
let buf = Buffer_New(64);
Buffer_Write(buf, 0, 72);   // 'H'
Buffer_Write(buf, 1, 105);  // 'i'

let text = Buffer_FromString("Hello");
let slice = Buffer_Slice(text, 0, 2);

Buffer_Dispose(buf);
Buffer_Dispose(text);
Buffer_Dispose(slice);
```

## API

| Function | Description |
|----------|-------------|
| `Buffer_New(size)` | Allocate buffer of `size` bytes |
| `Buffer_FromPtr(ptr, size, owns)` | Wrap native pointer |
| `Buffer_Len(buf)` | Current length |
| `Buffer_Capacity(buf)` | Allocated capacity |
| `Buffer_Read(buf, index)` | Read byte at index (bounds-checked) |
| `Buffer_Write(buf, index, value)` | Write byte at index |
| `Buffer_Fill(buf, value)` | Fill all bytes |
| `Buffer_Slice(buf, start, end)` | Zero-copy sub-range view |
| `Buffer_Resize(buf, newSize)` | Grow or shrink |
| `Buffer_CopyTo(buf, dest, destOffset, count)` | Copy bytes |
| `Buffer_FromString(str)` | UTF-8 encode string |
| `Buffer_AsString(buf)` | UTF-8 decode to string |
| `Buffer_Dispose(buf)` | Free memory |

## Used by

- `@std/io/filestream` — file read/write
- `@std/io/memory_stream` — in-memory streams
- `@std/net/socket` — send/receive buffers
- `@std/net/tls_stream` — TLS I/O
- `@std/net/http/*` — HTTP wire encoding

## Example

See `dryad-cpp/examples/02-buffers/buffer_usage.dryad`.

## Intrinsics

Buffer operations call `__alloc_bytes`, `__free_bytes`, `__buffer_get`, `__buffer_set`.
