# @std/net — Architecture Plan (Phase 4.4)

**Date:** 2026-09-16  
**Status:** Approved — Phase 0 in progress  
**Scope:** TCP/UDP sockets, `NetworkStream`, high-level `TcpClient` / `TcpListener`  
**Out of scope (later phases):** TLS, async/await, `Promise`, HTTP client, WebSocket, DNS resolver

---

## 1. Goals

| Goal | Rationale |
|------|-----------|
| Blocking TCP client/server over IPv4 | Foundation for HTTP (4.7) and echo/integration tests |
| `NetworkStream extends Stream` | Reuse `Buffer` I/O, compose with existing stream utilities |
| Error-as-value (not throw) | Matches `@std/io` (`FileStream`, `Path`, VFS) — language has no working `try/catch` |
| `IDisposable` lifecycle | Sockets must always be closed; mirror `FileStream.close()` / `dispose()` |
| Testable on localhost | CI-safe, no external network dependency |
| Minimal C++ surface | Extend intrinsics only where Dryad stdlib cannot be efficient/safe |

## 2. Non-goals (Phase 4.4)

- TLS / HTTPS
- Non-blocking I/O, `select`/`poll`/`epoll`
- `async`/`await` (lexer tokens exist; no runtime) — deferred to **Phase 4.5**
- Full HTTP — deferred to **Phase 4.7**
- WebSocket, `HttpServer`
- Cross-platform production hardening (Windows path exists in C++ but untested in CI)
- Hostname resolution (`getaddrinfo`) — **4.4 accepts IPv4 literals only** (`"127.0.0.1"`)

---

## 3. Current State

### 3.1 What exists

| Layer | Status |
|-------|--------|
| C++ syscalls in `intrinsics_network.cpp` | 8 intrinsics implemented: `socket`, `connect`, `bind`, `listen`, `accept`, `send`, `recv`, `shutdown` |
| Interpreter aliases | `__socket`, `__connect`, … via `syscall.*` mapping in `interpreter.cpp` |
| `@std/io/stream.dryad` | Abstract `Stream` + `SeekOrigin` — **ready** |
| `@std/io/filestream.dryad` | Reference pattern for buffer I/O + error-as-value |
| `@std/buffers/buffer.dryad` | `Buffer` used by streams |
| `@std/core/exceptions.dryad` | `SocketException`, `NetworkException`, `HttpException` factories (return strings) |
| `@std/net/*` stdlib | **Does not exist** |
| Network tests | **None** |

### 3.2 Critical blocker

`intrinsics_registry.cpp` contains an **empty** `register_network_intrinsics()` stub (line 292) that **overrides** the real implementation in `intrinsics_network.cpp` at link time. **No socket intrinsic is registered today.**

**Prerequisite 0:** Remove the stub; verify all 8 syscalls register and add C++ unit tests.

### 3.3 Intrinsic gaps vs. planned API

| Planned (architecture doc) | Actual C++ today | Action |
|----------------------------|------------------|--------|
| `__sys_socket` | `__socket(domain, type, protocol)` | Use existing names |
| `bind(fd, host, port)` | `bind(fd, port)` — `INADDR_ANY` only | **Done in Phase 0:** `bind(fd, host, port)` with `0.0.0.0`/`*` → any, IPv4 literal otherwise |
| `send(fd, buffer, offset, count)` | `send(fd, string)` | **Add `syscall.send_buffer` / `syscall.recv_buffer`** (mirror `read_buffer`/`write_buffer`) |
| `connect(host, port)` | IPv4 `inet_pton` only | Document IPv4 literal requirement |
| `errno` / rich errors | Returns `-1` only | **Add `syscall.get_errno` (P1)** or map known failures to messages |
| Socket close | Reuse `syscall.close` / `__close` | Document sharing fd lifecycle with file I/O close |

---

## 4. Layered Architecture

```
┌─────────────────────────────────────────────────────────────┐
│  Application / Phase 4.7 HTTP (future)                      │
├─────────────────────────────────────────────────────────────┤
│  TcpClient          TcpListener          UdpClient (P2)     │
│  getStream()        accept() → TcpClient sendTo/recvFrom    │
├─────────────────────────────────────────────────────────────┤
│  NetworkStream extends Stream                               │
│  read(buffer,off,cnt)  write(buffer,off,cnt)  no seek      │
├─────────────────────────────────────────────────────────────┤
│  Socket implements IDisposable                              │
│  connect / bind / listen / accept / send / receive / close  │
├─────────────────────────────────────────────────────────────┤
│  @std/net/net_errors.dryad  — validation, error helpers     │
├─────────────────────────────────────────────────────────────┤
│  C++ intrinsics (syscall.*)  — POSIX/Winsock, blocking only │
└─────────────────────────────────────────────────────────────┘
```

**Dependency rule:** Upper layers never call intrinsics directly except through `Socket` (and `internal` module helpers). User code imports `@std/net/tcp_client`, not `__socket`.

---

## 5. Module Layout

```
stdlib/@std/net/
├── README.md
├── net.dryad                 # barrel: TcpClient, TcpListener, Socket, NetworkStream, constants
├── constants.dryad           # SocketDomain, SocketType, ShutdownHow (runtime objects, not enum)
├── net_errors.dryad          # Net_validatePort, SocketError(...), helpers
├── socket.dryad              # Socket class + internal Socket_fromFd(fd)
├── network_stream.dryad      # NetworkStream extends Stream
├── tcp_listener.dryad        # TcpListener
├── tcp_client.dryad          # TcpClient
└── udp_client.dryad          # UdpClient (P2)
```

### 5.1 Why separate files

- Matches `@std/io` (`filestream`, `memory_stream`, `path` split)
- `Socket` stays low-level; `TcpClient`/`TcpListener` are ergonomic facades
- Easier testing and incremental delivery per task

### 5.2 Constants (no `enum` in language)

```dryad
// constants.dryad — same pattern as FileMode
let SocketDomain_IPv4 = 2;
let SocketDomain_IPv6 = 10;   // intrinsic support deferred; constant reserved

let SocketType_Stream = 1;    // TCP
let SocketType_Datagram = 2;  // UDP

let Shutdown_Read = 0;
let Shutdown_Write = 1;
let Shutdown_Both = 2;
```

---

## 6. Public API (adapted to Dryad today)

### 6.1 `Socket` (`socket.dryad`)

Low-level wrapper around fd. **All methods return `null` on success or an error string on failure** (never throw).

```dryad
class Socket implements IDisposable {
    constructor(domain, type, protocol = 0) { ... }

    connected: boolean { get; }   // true after connect() or for Socket_fromFd accepted clients
    closed: boolean { get; }

    connect(host, port)           // → null | SocketException(...)
    bind(host, port)              // host: IPv4 literal, "0.0.0.0", "*", or ""; port: 0 = ephemeral
    listen(backlog = 128)
    accept()                      // → Socket | error string

    send(buffer, offset, count)   // → nbytes | error
    receive(buffer, offset, count)// → nbytes | error (0 = peer closed)

    shutdown(how)                 // Shutdown_Read | Write | Both
    close()
    dispose() { this.close(); }
}

// Accepted sockets — no static methods in language
internal function Socket_fromFd(fd) { ... }
```

**State machine:**

```
[created] → connect → [connected] → shutdown/close → [closed]
[created] → bind → listen → [listening] → accept → Socket_fromFd → [connected]
```

Listening sockets: `connected == false`; I/O methods return `SocketException("Not connected")`.

### 6.2 `NetworkStream` (`network_stream.dryad`)

```dryad
class NetworkStream extends Stream {
    constructor(socket) { ... }  // socket must be connected

    canRead / canWrite → socket.connected && !socket.closed
    canSeek → false
    length / position → return InvalidOperationException(...) on access

    read(buffer, offset, count)   // → nbytes | error (matches FileStream)
    write(buffer, offset, count) // → nbytes | error
    seek(...)                    // → InvalidOperationException
    flush()                      // → null (no-op)
    close()                      // → closes underlying socket
}
```

Factory helper:

```dryad
function NetworkStream_fromSocket(socket) { return new NetworkStream(socket); }
```

### 6.3 `TcpListener` (`tcp_listener.dryad`)

Facade over listening `Socket`.

```dryad
class TcpListener implements IDisposable {
    constructor(port, backlog = 128) { ... }

    readonly port: number { get; }
    readonly isListening: boolean { get; }

    start()                       // bind + listen → null | error
    accept()                      // → TcpClient | error
    stop()                        // close listener socket
    dispose() { this.stop(); }
}
```

### 6.4 `TcpClient` (`tcp_client.dryad`)

```dryad
class TcpClient implements IDisposable {
    constructor(host, port) { ... }  // creates socket + connect

    readonly connected: boolean { get; }

    connect(host, port)           // if constructed empty (future)
    getStream()                   // → NetworkStream | error
    send(buffer, offset, count)   // delegate to socket
    receive(buffer, offset, count)
    close() / dispose()
}
```

### 6.5 `UdpClient` (P2 — `udp_client.dryad`)

```dryad
class UdpClient implements IDisposable {
    constructor() { ... }         // Socket(Datagram)
    sendTo(host, port, buffer, offset, count)
    receiveFrom(buffer, offset, count)  // → { nbytes, host, port } or error
    close()
}
```

---

## 7. Error Model

Follow `@std/io` strictly:

```dryad
let err = client.connect("127.0.0.1", 8080);
if (err != null) {
    // err is SocketException("...") string
    return err;
}
```

| Situation | Return |
|-----------|--------|
| Syscall failure (`< 0`) | `SocketException("Connect failed")` — enrich with `get_errno` when available |
| Use after close | `ObjectDisposedException("Socket")` |
| Wrong state (send on listener) | `InvalidOperationException("...")` |
| Bad args (port out of range) | `ArgumentException("...")` via `net_errors.dryad` |
| `recv` returns 0 | Propagate `0` from `receive()` — caller handles graceful close |

**No `throw`** until language gains exception semantics.

---

## 8. C++ Intrinsic Changes (Prerequisites)

### P0 — before any Dryad stdlib

1. **Remove empty `register_network_intrinsics()` stub** in `intrinsics_registry.cpp`
2. **Add buffer I/O syscalls:**

```cpp
syscall.send_buffer(fd, buffer_handle, offset, count) → nbytes | -1
syscall.recv_buffer(fd, buffer_handle, offset, count) → nbytes | -1 | 0
```

Implementation mirrors `syscall.read_buffer` / `write_buffer` in `intrinsics_registry.cpp` (same `MemoryManager` buffer handles).

3. **C++ unit tests** (`tests/unit/network_intrinsics_test.cpp`):
   - Create socket pair or loopback connect
   - send_buffer / recv_buffer roundtrip
   - bind/listen/accept on `127.0.0.1:0` (ephemeral port)

### P1 — quality

4. `syscall.get_errno()` → last errno after failed syscall (thread-local in single-threaded VM = sufficient)
5. `syscall.bind_addr(fd, host, port)` — if we want `bind("0.0.0.0", port)` at API level
6. `syscall.set_reuseaddr(fd, bool)` — avoid `EADDRINUSE` in tests

### Deferred

- `getaddrinfo` / DNS
- IPv6 connect/bind
- Non-blocking + selector

---

## 9. Security & Sensitivity

| Risk | Mitigation in 4.4 |
|------|-------------------|
| Arbitrary network access from user scripts | Document; no sandbox yet. Future: capability flag / `--allow-net` CLI |
| TLS absent | Document "cleartext only"; HTTP phase must warn |
| Blocking forever on `accept`/`recv` | Document blocking semantics; async in 4.5 |
| Resource leaks (fd exhaustion) | `IDisposable` + `close()`; tests verify fd closed after `dispose` |
| Binding all interfaces | Default `INADDR_ANY`; tests use `127.0.0.1` only |
| Injection via host string | Validate IPv4 literal format in `net_errors.dryad` before `connect` |
| Integer port overflow | Validate `1 <= port <= 65535` in Dryad before syscall |

**CI policy:** Integration tests bind to `127.0.0.1` with ephemeral port (port `0` intrinsic support) or fixed high port `49152+`.

---

## 10. Testing Strategy

Single-threaded interpreter **cannot** run blocking `accept` and `connect` concurrently in one Dryad script.

### Layer 1 — C++ unit tests (intrinsics)

Native tests, no interpreter. Loopback TCP:

- Server: `bind(0)` → `getsockname` → `listen` → `accept`
- Client: `connect` → `send_buffer` → verify on server side

### Layer 2 — Dryad stdlib unit tests

| Test | Approach |
|------|----------|
| Socket create/close | No network |
| Property guards | Call `send` on closed socket → error string |
| Invalid port | `connect("127.0.0.1", -1)` → `ArgumentException` |

### Layer 3 — Integration (echo)

**Recommended:** C++ `integration_tests/net_echo_test.cpp`:

1. Native thread: bind `127.0.0.1:0`, echo loop (read → write)
2. Main thread: run Dryad script via `Interpreter` — `TcpClient` connect, send, assert response
3. Join thread, assert clean shutdown

Alternative: two-process test (`dryad server.dryad` background + `dryad client.dryad`) — harder in CI.

### Layer 4 — Raw HTTP smoke (optional 4.4 stretch)

`TcpClient` → `getStream()` → write `"GET / HTTP/1.0\r\n\r\n"` → read into `Buffer` — validates stream bridge toward Phase 4.7.

---

## 11. Implementation Phases

### Phase 0 — Intrinsics fix (1 day)

- [ ] Remove registry stub
- [ ] `send_buffer` / `recv_buffer`
- [ ] C++ network intrinsic tests
- [ ] Optional: `get_errno`, `SO_REUSEADDR`

### Phase 4.4.1 — Core Socket (1 day)

- [ ] `constants.dryad`, `net_errors.dryad`
- [ ] `socket.dryad` — construct, close, properties
- [ ] Dryad unit tests (no network)

### Phase 4.4.2 — TCP client path (1 day)

- [ ] `connect(host, port)`
- [ ] `send` / `receive` via buffer intrinsics
- [ ] `tcp_client.dryad`
- [ ] Integration: client-only against C++ echo thread

### Phase 4.4.3 — TCP server path (1 day)

- [ ] `bind`, `listen`, `accept`, `Socket_fromFd`
- [ ] `shutdown`
- [ ] `tcp_listener.dryad`
- [ ] Integration: full echo (C++ server + Dryad client, then Dryad server + Dryad client via thread)

### Phase 4.4.4 — NetworkStream (0.5 day)

- [ ] `network_stream.dryad`
- [ ] Stream compliance tests (mirror `filestream_test` patterns)

### Phase 4.4.5 — Raw HTTP smoke (0.5 day)

- [ ] Integration test: `GET / HTTP/1.0` against local test server or httpbin on localhost
- [ ] Validates `NetworkStream` + buffer read path end-to-end

### Phase 4.4.6 — Docs & examples (0.5 day)

- [ ] `README.md`, `examples/net_echo_client.dryad`
- [ ] Update `PHASE_4_PROGRESS_REPORT.md`

### Phase 4.4.7 — UDP (P2, after TCP stable)

- [ ] `udp_client.dryad`
- [ ] Datagram echo test

**Estimated total:** ~5 days (matches plan ~28h)

---

## 12. Relationship to Future Phases

```
Phase 4.4  Socket, NetworkStream, Tcp*, Udp*     ← this document
    ↓
Phase 4.5  Promise<T>, connectAsync (thread pool or simulated)
    ↓
Phase 4.7  HttpClient uses TcpClient + NetworkStream
    ↓
Phase 5    epoll/kqueue, true async, TLS consideration
```

`HttpClient` (4.7) should depend only on `Stream`/`NetworkStream`, not raw `Socket`, so HTTP parsing stays protocol-layer clean.

---

## 13. Review Decisions (2026-09-16)

| # | Question | Decision |
|---|----------|----------|
| 1 | `bind(host, port)` | **Yes** — full host + port from day one (`127.0.0.1`, `0.0.0.0`, `*`) |
| 2 | Ephemeral port | **Yes** — `port = 0` + `syscall.getsockname(fd)` returns assigned port |
| 3 | `--allow-net` | **Yes, Phase 0** — CLI flag + `DRYAD_ALLOW_NET=1` env; network syscalls denied by default |
| 4 | Windows CI | **POSIX-only for now** — Winsock code kept, CI runs Linux only until Phase 5 |
| 5 | `UdpClient` | **P2** — only after TCP base (Socket, NetworkStream, TcpClient/TcpListener) is solid |
| 6 | Raw HTTP smoke | **Yes in 4.4** — `GET / HTTP/1.0` over `TcpClient`/`NetworkStream` before Phase 4.7 |

### Phase 0 deliverables (approved)

- [x] Remove duplicate `register_network_intrinsics()` stub
- [x] `bind(fd, host, port)`, `getsockname`, `send_buffer`, `recv_buffer`, `set_reuseaddr`, `get_errno`
- [x] `--allow-net` / `DRYAD_ALLOW_NET` gating
- [x] C++ loopback intrinsic tests (`network_intrinsics_test.cpp`)

---

## 14. Divergences from `STDLIB_ARCHITECTURE.md`

The architecture doc shows idealized C#-style API (`throw`, `enum`, static `__internal`). **This plan supersedes for implementation:**

| Architecture doc | This plan |
|------------------|-----------|
| `throw new SocketException` | Return error string |
| `enum SocketDomain` | `let SocketDomain_IPv4 = 2` constants |
| `Socket.__internal(fd)` static | `internal function Socket_fromFd(fd)` |
| `__sys_*` intrinsics | `__socket`, `__connect`, … (existing) |
| `bind(host, port)` | Implemented in Phase 0 intrinsics |
| NetworkStream in `socket.dryad` | Separate `network_stream.dryad` |

---

## 15. Acceptance Criteria (Phase 4.4 complete)

- [ ] All socket intrinsics registered and covered by C++ tests
- [ ] `TcpClient` / `TcpListener` / `NetworkStream` implemented with error-as-value
- [ ] Loopback echo integration test passes in CI
- [ ] Raw HTTP smoke test passes in CI
- [ ] Network denied without `--allow-net`
- [ ] No fd leaks in integration test (verified via test harness or repeat bind)
- [ ] README documents IPv4-only, blocking, cleartext, localhost testing
- [ ] ~31 new tests (per original plan estimate)
