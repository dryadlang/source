# Propostas para Sistema Auto-Suficiente de I/O e Funcionalidades Nativas

## Problema Atual

O sistema atual força:
- ❌ Escrever cada binding manualmente em C/Rust
- ❌ Dependência de código nativo para operações básicas
- ❌ Manutenção duplicada (Dryad + C/Rust)
- ❌ Impossibilidade de implementar em Dryad puro

## Objetivo

✅ Sistema auto-suficiente onde Dryad pode implementar suas próprias funcionalidades  
✅ Geração automática de bindings quando necessário  
✅ Self-hosting: Dryad implementa Dryad  
✅ Minimal runtime: apenas syscalls essenciais em C++

---

## Proposta 1: Syscall Interface + Standard Library em Dryad Puro

### Conceito

Fornecer apenas **syscalls primitivas** em C++, e implementar toda a standard library em Dryad.

### Arquitetura

```
┌─────────────────────────────────────────┐
│  Dryad Standard Library (@std)          │
│  ↓ Implementado 100% em Dryad           │
│  - io.dryad (filesystem abstraction)    │
│  - http.dryad (HTTP client/server)      │
│  - crypto.dryad (algorithms)            │
│  - json.dryad (parser/serializer)       │
└─────────────────────────────────────────┘
            ↓ usa apenas
┌─────────────────────────────────────────┐
│  Minimal Syscall Interface (C++)        │
│  ↓ Apenas 20-30 syscalls primitivas     │
│  - syscall.open(path, flags)            │
│  - syscall.read(fd, buffer, size)       │
│  - syscall.write(fd, buffer, size)      │
│  - syscall.close(fd)                    │
│  - syscall.socket(domain, type)         │
│  - syscall.malloc(size)                 │
│  - syscall.free(ptr)                    │
└─────────────────────────────────────────┘
            ↓ chama
┌─────────────────────────────────────────┐
│  OS Kernel (Linux/Windows/macOS)        │
└─────────────────────────────────────────┘
```

### Exemplo: Implementação de I/O em Dryad Puro

```dryad
// @std/io.dryad - Implementado 100% em Dryad!

// Syscalls primitivas (providas pelo runtime)
extern syscall {
    function open(path: string, flags: number): number;
    function read(fd: number, buffer: Buffer, size: number): number;
    function write(fd: number, buffer: Buffer, size: number): number;
    function close(fd: number): void;
}

// Constantes
const O_RDONLY = 0;
const O_WRONLY = 1;
const O_RDWR = 2;
const O_CREAT = 64;

// Classe File implementada em Dryad
export class File {
    private fd: number;
    private path: string;
    
    constructor(path: string, mode: string = "r") {
        this.path = path;
        let flags = this._parseMode(mode);
        this.fd = syscall.open(path, flags);
        
        if (this.fd < 0) {
            throw new Error("Failed to open file: " + path);
        }
    }
    
    read(): string {
        let buffer = Buffer.allocate(4096);
        let bytesRead = syscall.read(this.fd, buffer, 4096);
        return buffer.toString();
    }
    
    write(content: string): void {
        let buffer = Buffer.fromString(content);
        syscall.write(this.fd, buffer, buffer.size);
    }
    
    close(): void {
        syscall.close(this.fd);
    }
    
    private _parseMode(mode: string): number {
        match mode {
            "r" => O_RDONLY,
            "w" => O_WRONLY | O_CREAT,
            "rw" => O_RDWR,
            _ => throw new Error("Invalid mode: " + mode)
        }
    }
}

// Funções de conveniência (também em Dryad)
export function readFile(path: string): string {
    let file = new File(path, "r");
    let content = file.read();
    file.close();
    return content;
}

export function writeFile(path: string, content: string): void {
    let file = new File(path, "w");
    file.write(content);
    file.close();
}

export function exists(path: string): bool {
    try {
        let fd = syscall.open(path, O_RDONLY);
        if (fd >= 0) {
            syscall.close(fd);
            return true;
        }
        return false;
    } catch (e) {
        return false;
    }
}
```

### Uso pelo Programador

```dryad
import { readFile, writeFile, File } from "@std/io";

// Simples
let content = readFile("data.txt");
writeFile("output.txt", content);

// Ou com controle fino
let file = new File("large.txt", "r");
while (!file.eof()) {
    let chunk = file.read(1024);
    process(chunk);
}
file.close();
```

### Vantagens

✅ **Self-Hosting**: Standard library escrita em Dryad  
✅ **Manutenção Única**: Não duplica código em C++  
✅ **Extensível**: Usuários podem implementar suas próprias bibliotecas  
✅ **Minimal Runtime**: Apenas ~30 syscalls em C++  
✅ **Debuggable**: Todo código visível em Dryad  
✅ **Portável**: Apenas syscalls precisam ser adaptadas por OS  

---

## Proposta 2: Compile-Time Intrinsics + Geração Automática

### Conceito

Usar **intrinsics** (funções especiais reconhecidas pelo compilador) e gerar código automaticamente.

### Sintaxe

```dryad
// Declarar operação nativa via intrinsic
@intrinsic("syscall.read")
extern function __syscall_read(fd: i32, buf: ptr, len: usize): isize;

// Wrapper type-safe em Dryad
function readBytes(fd: number, count: number): Buffer {
    let buffer = Buffer.allocate(count);
    let bytesRead = __syscall_read(fd, buffer.ptr, count);
    
    if (bytesRead < 0) {
        throw new Error("Read failed");
    }
    
    buffer.resize(bytesRead);
    return buffer;
}
```

### Como Funciona

1. **Compiler reconhece `@intrinsic`** e gera código assembly direto
2. **Sem overhead de FFI**: chamada direta
3. **Wrappers em Dryad**: lógica de erro, conversão de tipos

### Exemplo: HTTP Client Implementado em Dryad

```dryad
// @std/http.dryad

import { Socket } from "@std/net";
import { Buffer } from "@std/buffer";

export class HttpClient {
    async get(url: string): Response {
        // Parse URL
        let parsed = this._parseUrl(url);
        
        // Conectar via socket (syscall)
        let socket = new Socket(parsed.host, parsed.port);
        
        // Construir request HTTP (puro Dryad)
        let request = this._buildRequest("GET", parsed.path, parsed.host);
        
        // Enviar (syscall write)
        socket.write(request);
        
        // Receber resposta (syscall read)
        let response = await socket.readAll();
        
        // Parse response (puro Dryad)
        return this._parseResponse(response);
    }
    
    // Tudo implementado em Dryad!
    private _parseUrl(url: string): UrlInfo { ... }
    private _buildRequest(method: string, path: string, host: string): string { ... }
    private _parseResponse(data: string): Response { ... }
}

class Response {
    statusCode: number;
    headers: Map<string, string>;
    body: string;
    
    json(): any {
        return JSON.parse(this.body); // JSON parser também em Dryad!
    }
}
```

### Vantagens

✅ **Zero Overhead**: Intrinsics compilam direto para assembly  
✅ **Type Safe**: Wrappers Dryad adicionam segurança  
✅ **Composable**: Bibliotecas em Dryad usam intrinsics  
✅ **Optimizable**: Compiler pode inline intrinsics  

---

## Proposta 3: Foreign Function Reflection + Auto-Generation

### Conceito

**Sistema reflete bibliotecas C automaticamente** e gera bindings Dryad sem escrever manualmente.

### Como Funciona

```bash
# Gerar bindings automaticamente a partir de header C
$ dryad-bindgen /usr/include/sqlite3.h --output @std/sqlite.dryad

# Resultado: arquivo Dryad gerado automaticamente
```

### Arquivo Gerado Automaticamente

```dryad
// @std/sqlite.dryad - GERADO AUTOMATICAMENTE

@ffi("libsqlite3.so")
extern "C" {
    type sqlite3 = opaque;
    type sqlite3_stmt = opaque;
    
    function sqlite3_open(
        filename: string,
        ppDb: ptr<sqlite3>
    ): i32;
    
    function sqlite3_prepare_v2(
        db: ptr<sqlite3>,
        sql: string,
        nByte: i32,
        ppStmt: ptr<sqlite3_stmt>,
        pzTail: ptr<string>
    ): i32;
    
    function sqlite3_step(stmt: ptr<sqlite3_stmt>): i32;
    function sqlite3_finalize(stmt: ptr<sqlite3_stmt>): i32;
    function sqlite3_close(db: ptr<sqlite3>): i32;
}

// Wrapper idiomático gerado automaticamente
export class Database {
    private db: ptr<sqlite3>;
    
    constructor(filename: string) {
        let result = sqlite3_open(filename, &this.db);
        if (result != 0) {
            throw new Error("Failed to open database");
        }
    }
    
    query(sql: string): ResultSet {
        let stmt: ptr<sqlite3_stmt>;
        sqlite3_prepare_v2(this.db, sql, -1, &stmt, null);
        return new ResultSet(stmt);
    }
    
    close(): void {
        sqlite3_close(this.db);
    }
}
```

### Ferramentas

```bash
# Gerar bindings de qualquer biblioteca C
dryad-bindgen curl.h -o @std/curl.dryad
dryad-bindgen openssl.h -o @std/openssl.dryad
dryad-bindgen postgres.h -o @std/postgres.dryad

# Ou de bibliotecas C++
dryad-bindgen --lang=cpp opencv.hpp -o @std/opencv.dryad
```

### Vantagens

✅ **Zero Esforço Manual**: Bindings gerados automaticamente  
✅ **Sempre Atualizado**: Re-gerar quando biblioteca atualizar  
✅ **Type Safe**: Parser entende tipos C  
✅ **Ecossistema Imenso**: Acesso a todo código C/C++ existente  

---

## Proposta 4: Bootstrap Compiler Self-Hosted

### Conceito

**Dryad compila a si próprio**, incluindo runtime e stdlib.

### Fases

#### Fase 1: Bootstrap Compiler (C++)
Compiler mínimo em C++ que entende Dryad básico

#### Fase 2: Self-Hosted Compiler (Dryad)
Reescrever compiler em Dryad usando o bootstrap compiler

#### Fase 3: Runtime em Dryad
Runtime, GC, e stdlib implementados em Dryad

### Exemplo: GC Implementado em Dryad

```dryad
// @std/runtime/gc.dryad

@intrinsic("memory.allocate")
extern function __alloc(size: usize): ptr<void>;

@intrinsic("memory.free")
extern function __free(ptr: ptr<void>): void;

// GC implementado em Dryad!
export class GarbageCollector {
    private heap: Array<HeapBlock>;
    private roots: Set<ptr<void>>;
    
    allocate(size: number): ptr<void> {
        // Mark phase
        this.mark();
        
        // Sweep phase
        this.sweep();
        
        // Compact phase (opcional)
        if (this.fragmentation() > 0.3) {
            this.compact();
        }
        
        // Alocar
        return this._allocateBlock(size);
    }
    
    private mark(): void {
        // Marcar objetos alcançáveis a partir das roots
        let visited = new Set<ptr<void>>();
        for (let root of this.roots) {
            this._markRecursive(root, visited);
        }
    }
    
    private sweep(): void {
        // Liberar objetos não marcados
        for (let block of this.heap) {
            if (!block.marked) {
                __free(block.ptr);
                this.heap.remove(block);
            }
        }
    }
    
    // ... implementação completa em Dryad
}
```

### Vantagens

✅ **Total Controle**: GC, Runtime customizável em Dryad  
✅ **Experimentação**: Fácil testar algoritmos diferentes  
✅ **Debuggable**: Todo runtime visível  
✅ **Self-Hosting**: Linguagem compila a si mesma  

---

## Proposta 5: Virtual File System + Pure Dryad Implementation

### Conceito

**Abstração de filesystem** implementada em Dryad com backends plugáveis.

### Arquitetura

```dryad
// @std/vfs.dryad - Virtual File System

// Interface que qualquer backend deve implementar
export interface FileSystemBackend {
    open(path: string, mode: string): FileHandle;
    read(handle: FileHandle, size: number): Buffer;
    write(handle: FileHandle, data: Buffer): number;
    close(handle: FileHandle): void;
    list(path: string): Array<FileInfo>;
}

// Backend nativo usando syscalls
class NativeBackend implements FileSystemBackend {
    open(path: string, mode: string): FileHandle {
        let fd = syscall.open(path, this._modeToFlags(mode));
        return new FileHandle(fd);
    }
    
    // ... implementação usando syscalls
}

// Backend in-memory (100% Dryad, sem syscalls!)
class MemoryBackend implements FileSystemBackend {
    private files: Map<string, Buffer> = new Map();
    
    open(path: string, mode: string): FileHandle {
        if (!this.files.has(path) && mode.includes("w")) {
            this.files.set(path, Buffer.allocate(0));
        }
        return new MemoryFileHandle(path, this.files);
    }
    
    read(handle: MemoryFileHandle, size: number): Buffer {
        return this.files.get(handle.path).slice(0, size);
    }
    
    write(handle: MemoryFileHandle, data: Buffer): number {
        this.files.set(handle.path, data);
        return data.size;
    }
    
    // Tudo em memória, nenhum syscall!
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
    
    writeFile(path: string, content: string): void {
        let handle = this.backend.open(path, "w");
        let buffer = Buffer.fromString(content);
        this.backend.write(handle, buffer);
        this.backend.close(handle);
    }
}
```

### Uso

```dryad
import { FileSystem, MemoryBackend } from "@std/vfs";

// Backend nativo (syscalls)
let fs = new FileSystem();
fs.writeFile("/tmp/data.txt", "hello");

// Backend in-memory (100% Dryad, zero syscalls!)
let memfs = new FileSystem(new MemoryBackend());
memfs.writeFile("/virtual/file.txt", "data");

// Backend HTTP (Dryad puro!)
class HttpBackend implements FileSystemBackend {
    // Ler/escrever via HTTP requests
}
let httpfs = new FileSystem(new HttpBackend("https://api.example.com"));

// Backend S3 (Dryad puro!)
let s3fs = new FileSystem(new S3Backend(credentials));
```

### Vantagens

✅ **Testável**: Mock filesystem para testes  
✅ **Flexível**: Múltiplos backends (disk, memory, HTTP, S3)  
✅ **Composable**: Backends podem ser combinados  
✅ **Pure Dryad**: Backends podem ser 100% Dryad  

---

## Proposta 6: Compiler Plugins para Code Generation

### Conceito

**Plugins do compilador** geram código Dryad automaticamente durante compilação.

### Exemplo: SQL Macro

```dryad
// Usuário escreve
@sql("SELECT * FROM users WHERE age > ?")
function getUsers(age: number): Array<User>;

// Compiler expande automaticamente para:
function getUsers(age: number): Array<User> {
    let conn = Database.getConnection();
    let stmt = conn.prepare("SELECT * FROM users WHERE age > ?");
    stmt.bind(1, age);
    let results = stmt.execute();
    return results.map(row => new User(row));
}
```

### Outro Exemplo: HTTP Route Macro

```dryad
// Usuário escreve
@http.get("/users/:id")
function getUserById(id: string): Response {
    let user = database.findUser(id);
    return Response.json(user);
}

// Compiler gera código de routing, parsing, serialização automaticamente
```

### Vantagens

✅ **Menos Boilerplate**: Compiler gera código repetitivo  
✅ **Type Safe**: Macros podem validar em compile-time  
✅ **Extensível**: Usuários podem criar suas próprias macros  
✅ **Zero Runtime Cost**: Tudo expandido em compile-time  

---

## Comparação e Recomendação

| Proposta | Self-Hosting | Simplicidade | Flexibilidade | Manutenção |
|----------|--------------|--------------|---------------|------------|
| 1. Syscall Interface + Stdlib Dryad | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| 2. Intrinsics + Auto-Gen | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| 3. FFI Reflection | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ |
| 4. Bootstrap Self-Hosted | ⭐⭐⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ |
| 5. Virtual FS | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| 6. Compiler Plugins | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ |

## 🎯 Recomendação: Abordagem Combinada

### Arquitetura Ideal

```
┌──────────────────────────────────────────────────────┐
│  Standard Library (@std) - 100% Dryad                │
│  ├─ io.dryad (VFS + backends)                        │
│  ├─ http.dryad (HTTP client/server)                  │
│  ├─ crypto.dryad (algorithms)                        │
│  ├─ json.dryad (parser)                              │
│  └─ sql.dryad (query builder)                        │
└──────────────────────────────────────────────────────┘
                    ↓ usa
┌──────────────────────────────────────────────────────┐
│  Intrinsics Layer (Compiler-recognized)              │
│  ├─ @intrinsic("syscall.*")                          │
│  ├─ @intrinsic("memory.*")                           │
│  └─ @intrinsic("atomic.*")                           │
└──────────────────────────────────────────────────────┘
                    ↓ gera
┌──────────────────────────────────────────────────────┐
│  Minimal Runtime (C++ - apenas ~50 syscalls)         │
│  ├─ syscall wrappers (open, read, write, socket)    │
│  ├─ memory primitives (malloc, free)                │
│  └─ atomic operations (cas, fence)                   │
└──────────────────────────────────────────────────────┘
                    ↓ chama
┌──────────────────────────────────────────────────────┐
│  Operating System (Linux/Windows/macOS)              │
└──────────────────────────────────────────────────────┘
```

### Código Exemplo Completo

```dryad
// ============================================
// @std/io.dryad - Standard Library em Dryad
// ============================================

// Syscalls primitivas (intrinsics)
@intrinsic("syscall.open")
extern function __open(path: string, flags: i32): i32;

@intrinsic("syscall.read")
extern function __read(fd: i32, buf: ptr<u8>, len: usize): isize;

@intrinsic("syscall.write")
extern function __write(fd: i32, buf: ptr<u8>, len: usize): isize;

@intrinsic("syscall.close")
extern function __close(fd: i32): void;

// Constantes
const O_RDONLY = 0;
const O_WRONLY = 1;
const O_CREAT = 64;

// Virtual File System (abstração)
interface FileSystemBackend {
    open(path: string, mode: string): FileHandle;
    read(handle: FileHandle, size: number): Buffer;
    write(handle: FileHandle, data: Buffer): number;
    close(handle: FileHandle): void;
}

// Backend nativo (usa syscalls)
class NativeBackend implements FileSystemBackend {
    open(path: string, mode: string): FileHandle {
        let flags = mode == "r" ? O_RDONLY : (O_WRONLY | O_CREAT);
        let fd = __open(path, flags);
        if (fd < 0) throw new Error("Failed to open: " + path);
        return new NativeFileHandle(fd);
    }
    
    read(handle: NativeFileHandle, size: number): Buffer {
        let buf = Buffer.allocate(size);
        let n = __read(handle.fd, buf.ptr, size);
        if (n < 0) throw new Error("Read failed");
        return buf.slice(0, n);
    }
    
    write(handle: NativeFileHandle, data: Buffer): number {
        let n = __write(handle.fd, data.ptr, data.size);
        if (n < 0) throw new Error("Write failed");
        return n;
    }
    
    close(handle: NativeFileHandle): void {
        __close(handle.fd);
    }
}

// Backend in-memory (100% Dryad, zero syscalls!)
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
    
    close(handle: MemoryFileHandle): void {
        // No-op para memory backend
    }
}

// API pública
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
    
    writeFile(path: string, content: string): void {
        let handle = this.backend.open(path, "w");
        this.backend.write(handle, Buffer.fromString(content));
        this.backend.close(handle);
    }
}

// Funções de conveniência
export function readFile(path: string): string {
    return new FileSystem().readFile(path);
}

export function writeFile(path: string, content: string): void {
    return new FileSystem().writeFile(path, content);
}
```

### Uso pelo Programador

```dryad
// Importar biblioteca standard (implementada em Dryad!)
import { readFile, writeFile, FileSystem, MemoryBackend } from "@std/io";

// Uso simples
let content = readFile("data.txt");
writeFile("output.txt", content.toUpperCase());

// Testes com backend in-memory (sem tocar disco!)
let memfs = new FileSystem(new MemoryBackend());
memfs.writeFile("/virtual/test.txt", "hello");
assert(memfs.readFile("/virtual/test.txt") == "hello");

// HTTP backend (100% Dryad!)
import { HttpBackend } from "@std/io/backends/http";
let httpfs = new FileSystem(new HttpBackend("https://cdn.example.com"));
let asset = httpfs.readFile("/assets/logo.png");
```

## Benefícios da Solução Combinada

1. ✅ **Self-Hosting Total**: stdlib em Dryad, runtime mínimo
2. ✅ **Zero Manutenção Duplicada**: Não escrever bindings manualmente
3. ✅ **Testável**: Backends in-memory para testes
4. ✅ **Extensível**: Usuários podem criar backends customizados
5. ✅ **Performance**: Intrinsics compilam direto para assembly
6. ✅ **Portável**: Apenas syscalls básicas precisam ser adaptadas
7. ✅ **Debuggable**: Todo código em Dryad visível
8. ✅ **Composable**: Backends plugáveis e combináveis

## Plano de Implementação

### Fase 1: Syscall Layer (1 semana)
- Definir ~50 syscalls primitivas em C++
- Implementar como intrinsics no compiler
- Testes básicos

### Fase 2: Core Stdlib em Dryad (2 semanas)
- io.dryad (filesystem)
- buffer.dryad (manipulação de bytes)
- string.dryad (operações de string)

### Fase 3: VFS + Backends (1 semana)
- Interface FileSystemBackend
- NativeBackend (syscalls)
- MemoryBackend (in-memory)

### Fase 4: High-level Libraries (2 semanas)
- http.dryad (client/server)
- json.dryad (parser/serializer)
- crypto.dryad (algorithms)

### Fase 5: Auto-generation Tools (1 semana)
- dryad-bindgen (C headers → Dryad)
- Documentação e exemplos

**Total: ~7 semanas para sistema completo self-hosted**

## Conclusão

A melhor abordagem combina:
- **Intrinsics mínimas** (~50 syscalls em C++)
- **Standard library 100% em Dryad**
- **Virtual File System** com backends plugáveis
- **Auto-generation** de bindings quando necessário

Isso elimina totalmente a necessidade de escrever bindings manualmente e permite que Dryad seja self-hosted, mantendo performance e flexibilidade.
