# @std/net — Networking

Blocking TCP/UDP sockets, DNS resolution, optional TLS, and a minimal HTTP client. Requires `--allow-net` (or `DRYAD_ALLOW_NET=1`).

## Modules

| Module | Purpose |
|--------|---------|
| `constants.dryad` | `SocketDomain`, `SocketType`, `Shutdown` |
| `socket.dryad` | Low-level TCP `Socket` class |
| `network_stream.dryad` | `NetworkStream extends Stream` |
| `tcp_client.dryad` | `TcpClient_connect(host, port)` |
| `tcp_listener.dryad` | `TcpListener` server facade |
| `udp_client.dryad` | UDP send/receive |
| `dns.dryad` | Hostname → IPv4 resolution |
| `tls_stream.dryad` | TLS client wrapper (requires OpenSSL build) |
| `socket_async.dryad` | Promise wrappers over blocking socket I/O |
| `net_errors.dryad` | Error helpers |
| `http/http_client.dryad` | Minimal HTTP GET/POST client |
| `http/http_url.dryad` | URL parsing |
| `http/http_headers.dryad` | Header map |
| `http/http_request.dryad` | Request builder |
| `http/http_response.dryad` | Response parser |
| `net.dryad` | Barrel imports |

## Quick start — TCP

```dryad
import { TcpClient_connect } from "@std/net/tcp_client";
import { Buffer_FromString, Buffer_AsString, Buffer_Dispose } from "@std/buffers/buffer";

let client = TcpClient_connect("127.0.0.1", 8080);
if (client._lastError == null && client.connected) {
    let buf = Buffer_FromString("hello");
    client.send(buf, 0, 5);
    client.receive(buf, 0, 5);
    print(Buffer_AsString(buf));
    Buffer_Dispose(buf);
    client.close();
}
```

## Quick start — HTTP

```dryad
import { HttpClient_get } from "@std/net/http/http_client";

let response = HttpClient_get("http://127.0.0.1:8080/");
if (response.ok) {
    print(response.statusCode);
    print(response.body);
}
```

## Quick start — UDP

```dryad
import { UdpClient_create, UdpClient_sendTo, UdpClient_receiveFrom } from "@std/net/udp_client";

let udp = UdpClient_create();
UdpClient_sendTo(udp, "127.0.0.1", 9000, "ping");
let reply = UdpClient_receiveFrom(udp, 1024);
```

## CLI

```bash
dryad --allow-net examples/06-net/tcp_client.dryad
```

Build with TLS (optional):

```bash
cmake -B build -DDRYAD_ENABLE_TLS=ON
cmake --build build
```

## C++ Intrinsics

| Intrinsic | Description |
|-----------|-------------|
| `__socket` | Create socket |
| `__connect` / `__bind` / `__listen` / `__accept` | TCP lifecycle |
| `__send` / `__recv` | String I/O |
| `__send_buffer` / `__recv_buffer` | Buffer I/O |
| `__sendto_buffer` / `__recvfrom_buffer` | UDP I/O |
| `__resolve_host` | DNS (IPv4 via getaddrinfo) |
| `__shutdown` / `__get_errno` | Socket control |
| `__tls_client_handshake` | TLS client (OpenSSL) |
| `__tls_send_buffer` / `__tls_recv_buffer` | TLS I/O |

## Known limitations

- Blocking I/O only (no epoll/kqueue event loop)
- DNS returns IPv4 addresses only
- TLS is client-only with no rigorous certificate verification
- HTTP client is minimal (no redirects, cookies, or chunked encoding)
- Errors returned as strings in most APIs, not thrown
- Single-threaded interpreter cannot run blocking `accept` and `connect` concurrently in one script (use C++ test threads or `@std/async/threading`)

## Architecture

See [net architecture plan](../implementation/plans/2026-09-16-net-architecture.md) for syscall mapping, test strategy, and phase roadmap.
