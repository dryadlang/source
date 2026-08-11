# Arquitetura de Runtime Mínimo Baseado em Intrinsics - Resumo Executivo

## 🎯 Mudança Arquitetural Fundamental

### Sistema Antigo (❌)
```
┌─────────────────────────────────────┐
│  Dryad Application Code             │
├─────────────────────────────────────┤
│  Native Modules em C++              │
│  - io_read_file() wrapper manual    │
│  - io_write_file() wrapper manual   │
│  - http_get() wrapper manual        │
│  - crypto_sha256() wrapper manual   │
│  - json_parse() wrapper manual      │
│  ... centenas de wrappers ...       │
└─────────────────────────────────────┘
```
**Problemas**:
- ❌ Manutenção duplicada (C++ + Dryad)
- ❌ Impossível implementar em Dryad puro
- ❌ Difícil de testar
- ❌ Runtime gigantesco

### Sistema Novo (✅)
```
┌─────────────────────────────────────┐
│  Standard Library (100% Dryad!)     │
│  - io.dryad                          │
│  - http.dryad                        │
│  - crypto.dryad                      │
│  - json.dryad                        │
└─────────────────────────────────────┘
         ↓ usa apenas
┌─────────────────────────────────────┐
│  ~50 Syscall Intrinsics (C++)       │
│  - open, read, write, close         │
│  - socket, connect, send, recv      │
│  - malloc, free                     │
│  - epoll_wait (async I/O)           │
└─────────────────────────────────────┘
```
**Benefícios**:
- ✅ Self-hosting: Dryad implementa Dryad
- ✅ Manutenção única
- ✅ Testável com mocks
- ✅ Runtime mínimo (~50 syscalls)

---

## 📐 Arquitetura Detalhada

### Camada 1: Runtime C++ (Micro-Kernel)

**Responsabilidade**: Fornecer apenas primitivas do sistema operacional.

```cpp
// runtime/intrinsics.cpp

enum class SyscallID : uint16_t {
    OPEN = 1,
    READ = 2,
    WRITE = 3,
    CLOSE = 4,
    SOCKET = 5,
    CONNECT = 6,
    // ... ~50 syscalls total
};

void VM::execute_intrinsic(SyscallID id) {
    switch (id) {
        case SyscallID::OPEN: {
            auto flags = pop_stack().as_int();
            auto path = pop_stack().as_string();
            int fd = ::open(path.c_str(), flags);
            push_stack(Value::from_int(fd));
            break;
        }
        case SyscallID::READ: {
            auto len = pop_stack().as_int();
            auto buf = pop_stack().as_buffer();
            auto fd = pop_stack().as_int();
            ssize_t n = ::read(fd, buf.data(), len);
            push_stack(Value::from_int(n));
            break;
        }
        // Apenas wrapping direto de syscalls POSIX
    }
}
```

**Tamanho**: ~500 linhas de C++ total

---

### Camada 2: Declarações Intrinsics (Dryad)

**Responsabilidade**: Declarar interface das syscalls para o compilador.

```dryad
// @std/runtime/intrinsics.dryad

@intrinsic("syscall.open")
extern function __open(path: string, flags: i32): i32;

@intrinsic("syscall.read")
extern function __read(fd: i32, buf: ptr<u8>, len: usize): isize;

@intrinsic("syscall.write")
extern function __write(fd: i32, buf: ptr<u8>, len: usize): isize;

@intrinsic("syscall.close")
extern function __close(fd: i32): void;

@intrinsic("syscall.socket")
extern function __socket(domain: i32, type: i32): i32;

@intrinsic("syscall.connect")
extern function __connect(fd: i32, addr: ptr<u8>, len: usize): i32;

// ... ~50 declarações
```

**Compilação**: O compilador reconhece `@intrinsic` e gera opcode especial:
```
INTRINSIC_SYSCALL <syscall_id>
```

---

### Camada 3: Core I/O (Dryad Puro)

**Responsabilidade**: Abstrações type-safe sobre syscalls brutas.

```dryad
// @std/io.dryad - Implementado 100% em Dryad!

import { __open, __read, __write, __close } from "@std/runtime/intrinsics";

const O_RDONLY = 0;
const O_WRONLY = 1;
const O_CREAT = 64;

export class File {
    private fd: number;
    private path: string;
    
    constructor(path: string, mode: string = "r") {
        this.path = path;
        let flags = mode == "r" ? O_RDONLY : (O_WRONLY | O_CREAT);
        this.fd = __open(path, flags);
        
        if (this.fd < 0) {
            throw new Error("Failed to open file: " + path);
        }
    }
    
    read(size: number = 4096): string {
        let buffer = Buffer.allocate(size);
        let bytesRead = __read(this.fd, buffer.ptr, size);
        
        if (bytesRead < 0) {
            throw new Error("Read failed");
        }
        
        return buffer.slice(0, bytesRead).toString();
    }
    
    write(content: string): void {
        let buffer = Buffer.fromString(content);
        let bytesWritten = __write(this.fd, buffer.ptr, buffer.size);
        
        if (bytesWritten < 0) {
            throw new Error("Write failed");
        }
    }
    
    close(): void {
        __close(this.fd);
    }
}

// Funções de conveniência
export function readFile(path: string): string {
    let file = new File(path, "r");
    let content = file.read(Infinity);
    file.close();
    return content;
}

export function writeFile(path: string, content: string): void {
    let file = new File(path, "w");
    file.write(content);
    file.close();
}
```

---

### Camada 4: Virtual File System (Dryad Puro)

**Responsabilidade**: Backends plugáveis para máxima flexibilidade.

```dryad
// @std/vfs.dryad

export interface FileSystemBackend {
    open(path: string, mode: string): FileHandle;
    read(handle: FileHandle, size: number): Buffer;
    write(handle: FileHandle, data: Buffer): number;
    close(handle: FileHandle): void;
}

// Backend nativo (usa syscalls)
class NativeBackend implements FileSystemBackend {
    open(path: string, mode: string): FileHandle {
        let flags = this._parseMode(mode);
        let fd = __open(path, flags);
        return new NativeFileHandle(fd);
    }
    
    read(handle: NativeFileHandle, size: number): Buffer {
        let buf = Buffer.allocate(size);
        let n = __read(handle.fd, buf.ptr, size);
        return buf.slice(0, n);
    }
    
    // ... usando syscalls
}

// Backend in-memory (100% Dryad, ZERO syscalls!)
class MemoryBackend implements FileSystemBackend {
    private storage: Map<string, Buffer> = new Map();
    
    open(path: string, mode: string): FileHandle {
        if (!this.storage.has(path) && mode == "w") {
            this.storage.set(path, Buffer.allocate(0));
        }
        return new MemoryFileHandle(path, this.storage);
    }
    
    read(handle: MemoryFileHandle, size: number): Buffer {
        return this.storage.get(handle.path) || Buffer.allocate(0);
    }
    
    write(handle: MemoryFileHandle, data: Buffer): number {
        this.storage.set(handle.path, data);
        return data.size;
    }
    
    // Tudo em memória, sem tocar disco!
}

// API unificada
export class FileSystem {
    private backend: FileSystemBackend;
    
    constructor(backend: FileSystemBackend = new NativeBackend()) {
        this.backend = backend;
    }
    
    readFile(path: string): string {
        let handle = this.backend.open(path, "r");
        let data = this.backend.read(handle, Infinity);
        this.backend.close(handle);
        return data.toString();
    }
}
```

**Uso**:
```dryad
// Produção: usa disco real
let fs = new FileSystem(new NativeBackend());
fs.writeFile("/data/config.json", json);

// Testes: usa memória
let mockfs = new FileSystem(new MemoryBackend());
mockfs.writeFile("/virtual/test.txt", "hello");
assert(mockfs.readFile("/virtual/test.txt") == "hello");
```

---

### Camada 5: HTTP em Dryad Puro

**Responsabilidade**: Cliente/servidor HTTP sem nenhum código C++.

```dryad
// @std/http.dryad - 100% Dryad!

import { Socket } from "@std/net";
import { Buffer } from "@std/buffer";

export class HttpClient {
    async get(url: string): Response {
        // 1. Parse URL (puro Dryad)
        let parsed = this._parseUrl(url);
        
        // 2. Conectar via socket (syscalls socket/connect)
        let socket = new Socket(parsed.host, parsed.port);
        await socket.connect();
        
        // 3. Construir request HTTP (puro Dryad)
        let request = this._buildRequest("GET", parsed.path, parsed.host);
        
        // 4. Enviar (syscall write)
        await socket.write(Buffer.fromString(request));
        
        // 5. Receber resposta (syscall read via event loop)
        let response = await socket.readAll();
        
        // 6. Parse response (puro Dryad)
        return this._parseResponse(response.toString());
    }
    
    private _parseUrl(url: string): UrlInfo {
        // Regex parsing, tudo em Dryad
        let match = url.match(/^https?:\/\/([^\/]+)(\/.*)?$/);
        return {
            host: match[1],
            path: match[2] || "/",
            port: 80
        };
    }
    
    private _buildRequest(method: string, path: string, host: string): string {
        return `${method} ${path} HTTP/1.1\r\n` +
               `Host: ${host}\r\n` +
               `Connection: close\r\n` +
               `\r\n`;
    }
    
    private _parseResponse(data: string): Response {
        let lines = data.split("\r\n");
        let statusLine = lines[0];
        let statusCode = parseInt(statusLine.split(" ")[1]);
        
        // Parse headers
        let headers = new Map<string, string>();
        let i = 1;
        while (lines[i] != "") {
            let [key, value] = lines[i].split(": ");
            headers.set(key, value);
            i++;
        }
        
        // Body
        let body = lines.slice(i + 1).join("\r\n");
        
        return new Response(statusCode, headers, body);
    }
}

class Response {
    constructor(
        public statusCode: number,
        public headers: Map<string, string>,
        public body: string
    ) {}
    
    json(): any {
        return JSON.parse(this.body); // JSON parser também em Dryad!
    }
}
```

**Uso**:
```dryad
import { HttpClient } from "@std/http";

let client = new HttpClient();
let response = await client.get("http://api.example.com/data");
let data = response.json();
```

---

### Camada 6: Event Loop para Async I/O

**Responsabilidade**: Multiplexação de I/O não-bloqueante.

```dryad
// @std/async/event_loop.dryad

@intrinsic("syscall.epoll_create")
extern function __epoll_create(): i32;

@intrinsic("syscall.epoll_wait")
extern function __epoll_wait(epfd: i32, timeout: i32): Array<i32>;

@intrinsic("syscall.epoll_ctl")
extern function __epoll_ctl(epfd: i32, op: i32, fd: i32): void;

const EPOLL_CTL_ADD = 1;
const EPOLL_CTL_DEL = 2;

export class EventLoop {
    private epoll_fd: number;
    private tasks: Map<number, Task> = new Map();
    
    constructor() {
        this.epoll_fd = __epoll_create();
    }
    
    run(): void {
        while (this.tasks.size > 0) {
            // Multiplexação: espera até algum fd estar pronto
            let ready_fds = __epoll_wait(this.epoll_fd, 100);
            
            // Acorda tasks cujos fds estão prontos
            for (let fd of ready_fds) {
                let task = this.tasks.get(fd);
                if (task) {
                    task.resume(); // Resume coroutine
                }
            }
        }
    }
    
    register(fd: number, task: Task): void {
        __epoll_ctl(this.epoll_fd, EPOLL_CTL_ADD, fd);
        this.tasks.set(fd, task);
    }
    
    unregister(fd: number): void {
        __epoll_ctl(this.epoll_fd, EPOLL_CTL_DEL, fd);
        this.tasks.delete(fd);
    }
}
```

**Como `await` funciona**:
```dryad
// Quando o usuário escreve:
let data = await socket.read();

// O compilador transforma em:
let task = new Task(() => socket.read());
eventLoop.register(socket.fd, task);
task.suspend(); // Pausa execução
// ... event loop acorda a task quando fd está pronto ...
let data = task.result; // Retoma execução
```

---

## 🔥 Buffer: Interface Type-Safe para Memória Bruta

```dryad
// @std/buffer.dryad

@intrinsic("memory.allocate")
extern function __alloc(size: usize): ptr<u8>;

@intrinsic("memory.free")
extern function __free(ptr: ptr<u8>): void;

@intrinsic("memory.copy")
extern function __memcpy(dest: ptr<u8>, src: ptr<u8>, len: usize): void;

export class Buffer {
    private data: ptr<u8>;
    private _size: number;
    
    static allocate(size: number): Buffer {
        let ptr = __alloc(size);
        return new Buffer(ptr, size);
    }
    
    static fromString(s: string): Buffer {
        let size = s.length;
        let buf = Buffer.allocate(size);
        for (let i = 0; i < size; i++) {
            buf.set(i, s.charCodeAt(i));
        }
        return buf;
    }
    
    toString(): string {
        let chars = [];
        for (let i = 0; i < this._size; i++) {
            chars.push(String.fromCharCode(this.get(i)));
        }
        return chars.join("");
    }
    
    slice(start: number, end: number): Buffer {
        let len = end - start;
        let newBuf = Buffer.allocate(len);
        __memcpy(newBuf.ptr, this.data + start, len);
        return newBuf;
    }
    
    get(index: number): number {
        if (index < 0 || index >= this._size) {
            throw new Error("Buffer index out of bounds");
        }
        return this.data[index];
    }
    
    set(index: number, value: number): void {
        if (index < 0 || index >= this._size) {
            throw new Error("Buffer index out of bounds");
        }
        this.data[index] = value & 0xFF;
    }
    
    get ptr(): ptr<u8> { return this.data; }
    get size(): number { return this._size; }
}
```

**Segurança**:
- ✅ Bounds checking automático
- ✅ GC gerencia lifecycle
- ✅ Conversões type-safe (string ↔ buffer)
- ✅ Zero-copy para syscalls

---

## 📊 Comparação: Sistema Antigo vs Novo

| Aspecto | Sistema Antigo | Sistema Novo |
|---------|----------------|--------------|
| **Stdlib** | Wrappers C++ manuais | 100% Dryad |
| **Manutenção** | Duplicada (C++ + Dryad) | Única (apenas Dryad) |
| **Linhas de C++** | ~10,000 | ~500 |
| **Testabilidade** | Difícil (syscalls reais) | Fácil (mock backends) |
| **Extensibilidade** | Requer C++ | Bibliotecas em Dryad |
| **Runtime Size** | Grande (~500KB) | Mínimo (~50KB) |
| **Performance** | Overhead de wrapper | Intrinsics diretas |
| **Portabilidade** | Adaptar cada binding | Adaptar ~50 syscalls |
| **Debuggability** | Parte opaco em C++ | Tudo visível em Dryad |
| **Self-hosting** | Impossível | Completo |

---

## 🎯 Lista das ~50 Syscalls Essenciais

### File I/O (8)
- `open`, `read`, `write`, `close`
- `lseek`, `stat`, `unlink`, `mkdir`

### Network (8)
- `socket`, `connect`, `bind`, `listen`
- `accept`, `send`, `recv`, `shutdown`

### Memory (5)
- `malloc`, `free`, `realloc`
- `memcpy`, `memset`

### Async I/O (6)
- `epoll_create`, `epoll_ctl`, `epoll_wait` (Linux)
- `kqueue`, `kevent` (macOS/BSD)
- `select` (fallback)

### Process/Thread (6)
- `fork`, `exec`, `wait`
- `pthread_create`, `pthread_join`, `pthread_detach`

### Time (4)
- `gettimeofday`, `clock_gettime`
- `sleep`, `nanosleep`

### Environment (5)
- `getenv`, `setenv`
- `getcwd`, `chdir`
- `getpid`

### Signals (3)
- `signal`, `kill`, `sigaction`

### Atomic Operations (5)
- `atomic_load`, `atomic_store`
- `atomic_compare_exchange`
- `atomic_fetch_add`
- `memory_fence`

**Total**: ~50 syscalls

---

## ⚡ Performance: Intrinsics vs Wrappers

### Wrapper Antigo
```
Dryad: readFile("data.txt")
  ↓
Call wrapper C++: native_read_file()
  ↓
Unwrap parâmetros Dryad → C++
  ↓
Call função real C++
  ↓
Wrap resultado C++ → Dryad
  ↓
Return
```
**Overhead**: ~20-50 instruções + heap allocations

### Intrinsic Novo
```
Dryad: __read(fd, buf, len)
  ↓
INTRINSIC_SYSCALL opcode
  ↓
Direct dispatch para ::read()
  ↓
Return
```
**Overhead**: ~3-5 instruções, zero allocations

**Speedup**: 5-10x mais rápido

---

## 🚀 Cronograma de Implementação

### Fase 1: Syscall Layer (1 semana)
- [ ] Definir enum SyscallID com ~50 syscalls
- [ ] Implementar dispatch table no VM
- [ ] Suporte a `@intrinsic` no parser
- [ ] Gerar opcode `INTRINSIC_SYSCALL`
- [ ] Testes básicos

### Fase 2: Core Stdlib em Dryad (2 semanas)
- [ ] `@std/buffer.dryad` (Buffer class)
- [ ] `@std/io.dryad` (File, readFile, writeFile)
- [ ] `@std/net.dryad` (Socket, TCP/UDP)
- [ ] `@std/async.dryad` (EventLoop, Task)
- [ ] Testes unitários

### Fase 3: VFS + Backends (1 semana)
- [ ] Interface FileSystemBackend
- [ ] NativeBackend (syscalls)
- [ ] MemoryBackend (in-memory)
- [ ] HttpBackend (opcional)
- [ ] Testes com mocks

### Fase 4: High-Level Libraries (2 semanas)
- [ ] `@std/http.dryad` (HttpClient, HttpServer)
- [ ] `@std/json.dryad` (parse, stringify)
- [ ] `@std/crypto.dryad` (sha256, md5, etc)
- [ ] `@std/encoding.dryad` (base64, utf-8)
- [ ] Testes end-to-end

### Fase 5: Portabilidade (1 semana)
- [ ] Adaptar syscalls para Windows (IOCP)
- [ ] Adaptar syscalls para macOS (kqueue)
- [ ] Fallbacks portáveis (select)
- [ ] CI/CD em múltiplos OS

**Total**: ~7 semanas para sistema completo self-hosted

---

## 🎓 Inspiração: Linguagens que Usam Esta Abordagem

### Go
```go
// Go stdlib usa syscall package
package main
import "syscall"

func main() {
    fd, _ := syscall.Open("file.txt", syscall.O_RDONLY, 0)
    buf := make([]byte, 1024)
    syscall.Read(fd, buf)
    syscall.Close(fd)
}
```
Todo `io.ReadFile()` é implementado em Go puro usando `syscall`.

### Zig
```zig
// Zig expõe syscalls via std.os
const std = @import("std");

pub fn main() !void {
    const fd = try std.os.open("file.txt", .{});
    defer std.os.close(fd);
    
    var buf: [1024]u8 = undefined;
    _ = try std.os.read(fd, &buf);
}
```
Stdlib de Zig é 100% Zig usando intrinsics.

### Rust
```rust
// Rust core::intrinsics + libc
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("file.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
}
```
Rust stdlib usa `core::intrinsics` e syscalls via `libc`.

---

## ✅ Conclusão

A migração para **Runtime Mínimo Baseado em Intrinsics** transforma Dryad de uma linguagem com stdlib dependente de C++ para uma linguagem **self-hosted** moderna.

**Benefícios Mensuráveis**:
- 📉 Runtime size: 500KB → 50KB (10x menor)
- 📈 Performance: 5-10x mais rápido em I/O
- 🧪 Testabilidade: 100% (mock backends)
- 🔧 Manutenção: Uma única codebase
- 🚀 Extensibilidade: Bibliotecas em Dryad

**Próximos Passos**:
1. Implementar ~50 syscalls em C++
2. Escrever stdlib em Dryad puro
3. Migrar código existente
4. Deprecar sistema antigo

Esta é a arquitetura que linguagens modernas de alta performance utilizam, e é o caminho natural para Dryad atingir maturidade.
