# Dryad Standard Library Architecture (Phase 4)

**Version**: 1.0  
**Date**: 2026-05-28  
**Status**: Design Specification

---

## 1. Design Philosophy

### 1.1 Core Principles

1. **Intrinsics as Energy, Stdlib as Appliances**
   - Intrinsics (`__sys_*`) are raw syscalls - users should NEVER use them directly
   - Stdlib provides high-level, safe, ergonomic abstractions over intrinsics
   - Analogy: intrinsics = electricity, stdlib = appliances

2. **.NET Framework-Level Robustness**
   - Hierarchical namespace organization for discoverability
   - Strong type safety with compile-time guarantees
   - Exception-based error handling with clear error messages
   - Resource management via IDisposable pattern
   - Comprehensive API surface covering common use cases

3. **100% Dryad Implementation**
   - Entire stdlib written in Dryad (self-hosting)
   - Minimal C++ runtime (only intrinsics layer)
   - Enables future optimizations at language level

4. **SOLID Principles Throughout**
   - Single Responsibility: focused, cohesive classes
   - Open/Closed: extensible via inheritance/composition
   - Liskov Substitution: proper inheritance hierarchies
   - Interface Segregation: small, focused interfaces
   - Dependency Inversion: depend on abstractions

---

## 2. Namespace Organization

### 2.1 Hierarchical Structure (.NET-Style)

```
@std/
├── core/                          # Fundamental types and interfaces
│   ├── object.dryad              # Base Object type
│   ├── disposable.dryad          # IDisposable interface
│   ├── enumerable.dryad          # IEnumerable<T> interface
│   ├── comparable.dryad          # IComparable<T> interface
│   └── exceptions.dryad          # Base exception hierarchy
│
├── runtime/                       # Internal runtime support
│   ├── _intrinsics.dryad         # Raw intrinsics (internal-only)
│   └── memory.dryad              # Memory management utilities
│
├── buffers/                       # Buffer management
│   └── buffer.dryad              # Managed Buffer class
│
├── io/                           # I/O fundamentals
│   ├── stream.dryad              # Abstract Stream class
│   ├── file.dryad                # FileStream, File utilities
│   ├── memory_stream.dryad       # MemoryStream
│   ├── buffered_stream.dryad     # BufferedStream wrapper
│   ├── text/                     # Text I/O
│   │   ├── reader.dryad          # TextReader, StreamReader
│   │   ├── writer.dryad          # TextWriter, StreamWriter
│   │   └── encoding.dryad        # UTF8, ASCII encodings
│   └── path.dryad                # Path manipulation utilities
│
├── collections/                   # Data structures
│   ├── list.dryad                # List<T>
│   ├── map.dryad                 # Map<K, V>
│   ├── set.dryad                 # Set<T>
│   └── queue.dryad               # Queue<T>
│
├── async/                        # Async primitives
│   ├── promise.dryad             # Promise<T> class
│   └── task.dryad                # Task abstraction
│
├── net/                          # Networking
│   ├── socket.dryad              # Socket class
│   ├── tcp_listener.dryad        # TCP server
│   ├── tcp_client.dryad          # TCP client
│   ├── udp_client.dryad          # UDP client
│   ├── network_stream.dryad      # NetworkStream
│   ├── http/                     # HTTP client/server
│   │   ├── client.dryad          # HttpClient
│   │   ├── server.dryad          # HttpServer
│   │   ├── request.dryad         # HttpRequest
│   │   ├── response.dryad        # HttpResponse
│   │   └── headers.dryad         # HttpHeaders
│   └── websocket/                # WebSocket support
│       ├── client.dryad          # WebSocket client
│       └── server.dryad          # WebSocket server
│
├── fs/                           # Filesystem operations
│   ├── file.dryad                # File utilities (read, write, delete)
│   ├── directory.dryad           # Directory utilities
│   └── file_info.dryad           # FileInfo, DirectoryInfo
│
├── text/                         # String processing
│   ├── string_builder.dryad      # Mutable string builder
│   ├── regex.dryad               # Regular expressions
│   └── json.dryad                # JSON parser/serializer
│
├── time/                         # Date/Time
│   ├── datetime.dryad            # DateTime class
│   ├── timespan.dryad            # TimeSpan class
│   └── clock.dryad               # System clock utilities
│
├── diagnostics/                  # Debugging/diagnostics
│   ├── console.dryad             # Console I/O
│   ├── debug.dryad               # Debug utilities
│   └── stopwatch.dryad           # Performance timing
│
└── process/                      # Process management
    ├── process.dryad             # Process class
    ├── environment.dryad         # Environment variables
    └── args.dryad                # Command-line args
```

### 2.2 Namespace Rules

1. **Top-level modules** (`@std/io`, `@std/net`) are broad categories
2. **Sub-modules** (`@std/io/text`, `@std/net/http`) are specialized areas
3. **Internal modules** (`@std/runtime/_intrinsics`) start with `_` and are compiler-restricted
4. **Import examples**:
   ```dryad
   import { File } from "@std/io/file";
   import { HttpClient } from "@std/net/http/client";
   import { List } from "@std/collections/list";
   ```

---

## 3. Core Abstractions

### 3.1 IDisposable Pattern

**Purpose**: Deterministic resource cleanup (files, sockets, buffers)

```dryad
// @std/core/disposable.dryad
interface IDisposable {
    dispose(): void;
}
```

**Usage Pattern** (Phase 4 - manual try/finally):
```dryad
let file = File.open("test.txt");
try {
    file.read(buffer, 1024);
} finally {
    file.dispose();  // Guaranteed cleanup
}
```

**Future Enhancement** (Phase 5+ - `using` statement):
```dryad
using (let file = File.open("test.txt")) {
    file.read(buffer, 1024);
}  // dispose() called automatically
```

---

### 3.2 Stream Hierarchy

**Purpose**: Unified abstraction for sequential I/O (files, network, memory)

```dryad
// @std/io/stream.dryad
abstract class Stream implements IDisposable {
    // Properties
    abstract readonly canRead: boolean;
    abstract readonly canWrite: boolean;
    abstract readonly canSeek: boolean;
    abstract readonly length: number;
    abstract position: number;
    
    // Core operations
    abstract read(buffer: Buffer, offset: number, count: number): number;
    abstract write(buffer: Buffer, offset: number, count: number): void;
    abstract seek(offset: number, origin: SeekOrigin): number;
    abstract flush(): void;
    abstract close(): void;
    
    // IDisposable
    dispose(): void {
        this.close();
    }
    
    // Convenience methods
    readByte(): number {
        let buf = new Buffer(1);
        let n = this.read(buf, 0, 1);
        return n > 0 ? buf.get(0) : -1;
    }
    
    writeByte(value: byte): void {
        let buf = new Buffer(1);
        buf.set(0, value);
        this.write(buf, 0, 1);
    }
}

enum SeekOrigin {
    Begin = 0,
    Current = 1,
    End = 2
}
```

**Concrete Implementations**:

1. **FileStream** - file I/O
2. **MemoryStream** - in-memory buffer
3. **NetworkStream** - TCP socket I/O
4. **BufferedStream** - wraps any stream with buffering

---

### 3.3 Exception Hierarchy

**Purpose**: Structured error handling with clear error categories

```dryad
// @std/core/exceptions.dryad

// Base exception
class Exception {
    readonly message: string;
    readonly stackTrace: string;
    
    constructor(message: string) {
        this.message = message;
        this.stackTrace = __get_stack_trace();
    }
    
    toString(): string {
        return this.message + "\n" + this.stackTrace;
    }
}

// Argument exceptions
class ArgumentException extends Exception { }
class ArgumentNullException extends ArgumentException { }
class ArgumentOutOfRangeException extends ArgumentException { }

// I/O exceptions
class IOException extends Exception { }
class FileNotFoundException extends IOException { }
class DirectoryNotFoundException extends IOException { }
class EndOfStreamException extends IOException { }

// State exceptions
class InvalidOperationException extends Exception { }
class ObjectDisposedException extends InvalidOperationException { }

// Collection exceptions
class IndexOutOfRangeException extends Exception { }
class KeyNotFoundException extends Exception { }

// Network exceptions
class NetworkException extends Exception { }
class SocketException extends NetworkException { }
class HttpException extends NetworkException { }
```

---

### 3.4 Promise<T> (Async Primitive)

**Purpose**: Asynchronous operations with composable futures

```dryad
// @std/async/promise.dryad
class Promise<T> {
    private state: PromiseState;
    private value: T | null;
    private error: Exception | null;
    private callbacks: List<PromiseCallbacks<T>>;
    
    constructor(executor: (resolve: (T) => void, reject: (Exception) => void) => void) {
        this.state = PromiseState.Pending;
        this.callbacks = new List<PromiseCallbacks<T>>();
        
        try {
            executor(
                (value) => this.resolve(value),
                (error) => this.reject(error)
            );
        } catch (e: Exception) {
            this.reject(e);
        }
    }
    
    then<U>(onSuccess: (T) => U): Promise<U> {
        // Chain success handler
    }
    
    catch(onError: (Exception) => void): Promise<T> {
        // Chain error handler
    }
    
    finally(onComplete: () => void): Promise<T> {
        // Always execute on completion
    }
    
    // Static helpers
    static resolve<T>(value: T): Promise<T> { }
    static reject<T>(error: Exception): Promise<T> { }
    static all<T>(promises: List<Promise<T>>): Promise<List<T>> { }
    static race<T>(promises: List<Promise<T>>): Promise<T> { }
}

enum PromiseState {
    Pending,
    Fulfilled,
    Rejected
}
```

---

### 3.5 IEnumerable<T> (Iteration)

**Purpose**: Unified iteration interface for collections

```dryad
// @std/core/enumerable.dryad
interface IEnumerable<T> {
    forEach(fn: (T) => void): void;
    map<U>(fn: (T) => U): IEnumerable<U>;
    filter(fn: (T) => boolean): IEnumerable<T>;
    reduce<U>(fn: (U, T) => U, initial: U): U;
}
```

---

## 4. Module Specifications

### 4.1 @std/buffers/buffer

**Purpose**: Safe, managed byte buffers with bounds checking

```dryad
// @std/buffers/buffer.dryad
class Buffer implements IDisposable {
    private data: ByteArray;  // Intrinsic type
    readonly length: number;
    
    // Construction
    constructor(size: number) {
        if (size < 0) {
            throw new ArgumentOutOfRangeException("size must be >= 0");
        }
        this.data = __alloc_bytes(size);
        this.length = size;
    }
    
    // Indexed access (bounds checked)
    get(index: number): byte {
        if (index < 0 || index >= this.length) {
            throw new IndexOutOfRangeException();
        }
        return this.data[index];
    }
    
    set(index: number, value: byte): void {
        if (index < 0 || index >= this.length) {
            throw new IndexOutOfRangeException();
        }
        this.data[index] = value;
    }
    
    // Bulk operations (optimized via intrinsics)
    copyTo(dest: Buffer, destOffset: number, count: number): void {
        this.validateCopyArgs(dest, destOffset, count);
        __memcpy(dest.data, destOffset, this.data, 0, count);
    }
    
    fill(value: byte): void {
        __memset(this.data, value, this.length);
    }
    
    // Conversions
    toString(encoding: Encoding): string {
        return encoding.decode(this);
    }
    
    static fromString(str: string, encoding: Encoding): Buffer {
        return encoding.encode(str);
    }
    
    // IDisposable
    dispose(): void {
        if (this.data != null) {
            __free_bytes(this.data);
            this.data = null;
        }
    }
}
```

**Key Features**:
- Bounds checking on every access (safety first)
- Bulk operations use intrinsics (__memcpy, __memset) for performance
- IDisposable for deterministic cleanup
- Encoding conversion helpers

---

### 4.2 @std/io/file

**Purpose**: High-level file operations wrapping file I/O intrinsics

```dryad
// @std/io/file.dryad
import { Stream, SeekOrigin } from "@std/io/stream";
import { Buffer } from "@std/buffers/buffer";

class FileStream extends Stream {
    private fd: number;
    private _position: number;
    private _canRead: boolean;
    private _canWrite: boolean;
    private closed: boolean;
    
    // Construction (internal - use File.open)
    internal constructor(fd: number, mode: FileMode) {
        this.fd = fd;
        this._position = 0;
        this._canRead = (mode == FileMode.Read || mode == FileMode.ReadWrite);
        this._canWrite = (mode == FileMode.Write || mode == FileMode.ReadWrite);
        this.closed = false;
    }
    
    // Properties
    readonly canRead: boolean { return this._canRead && !this.closed; }
    readonly canWrite: boolean { return this._canWrite && !this.closed; }
    readonly canSeek: boolean { return !this.closed; }
    
    readonly length: number {
        this.ensureNotClosed();
        return __sys_fstat(this.fd).size;
    }
    
    position: number {
        get { return this._position; }
        set { this.seek(value, SeekOrigin.Begin); }
    }
    
    // Core operations
    read(buffer: Buffer, offset: number, count: number): number {
        this.ensureCanRead();
        this.validateReadArgs(buffer, offset, count);
        
        let n = __sys_read(this.fd, buffer, offset, count);
        if (n < 0) {
            throw new IOException("Read failed");
        }
        this._position += n;
        return n;
    }
    
    write(buffer: Buffer, offset: number, count: number): void {
        this.ensureCanWrite();
        this.validateWriteArgs(buffer, offset, count);
        
        let n = __sys_write(this.fd, buffer, offset, count);
        if (n < 0) {
            throw new IOException("Write failed");
        }
        this._position += n;
    }
    
    seek(offset: number, origin: SeekOrigin): number {
        this.ensureNotClosed();
        
        let newPos = __sys_lseek(this.fd, offset, origin as number);
        if (newPos < 0) {
            throw new IOException("Seek failed");
        }
        this._position = newPos;
        return newPos;
    }
    
    flush(): void {
        this.ensureNotClosed();
        __sys_fsync(this.fd);
    }
    
    close(): void {
        if (!this.closed) {
            __sys_close(this.fd);
            this.closed = true;
        }
    }
    
    // Helpers
    private ensureNotClosed(): void {
        if (this.closed) {
            throw new ObjectDisposedException("FileStream");
        }
    }
    
    private ensureCanRead(): void {
        this.ensureNotClosed();
        if (!this._canRead) {
            throw new InvalidOperationException("Stream not readable");
        }
    }
    
    private ensureCanWrite(): void {
        this.ensureNotClosed();
        if (!this._canWrite) {
            throw new InvalidOperationException("Stream not writable");
        }
    }
}

enum FileMode {
    Read = 0,
    Write = 1,
    ReadWrite = 2,
    Append = 3
}

// Static utilities
class File {
    static open(path: string, mode: FileMode = FileMode.Read): FileStream {
        if (path == null || path.length == 0) {
            throw new ArgumentException("path cannot be empty");
        }
        
        let flags = File.modeToFlags(mode);
        let fd = __sys_open(path, flags);
        if (fd < 0) {
            throw new FileNotFoundException("Cannot open: " + path);
        }
        
        return new FileStream(fd, mode);
    }
    
    static exists(path: string): boolean {
        return __sys_access(path, 0) == 0;
    }
    
    static delete(path: string): void {
        if (__sys_unlink(path) < 0) {
            throw new IOException("Cannot delete: " + path);
        }
    }
    
    static readAllText(path: string): string {
        using (let stream = File.open(path, FileMode.Read)) {
            let buffer = new Buffer(stream.length);
            stream.read(buffer, 0, buffer.length);
            return buffer.toString(Encoding.UTF8);
        }
    }
    
    static writeAllText(path: string, content: string): void {
        using (let stream = File.open(path, FileMode.Write)) {
            let buffer = Buffer.fromString(content, Encoding.UTF8);
            stream.write(buffer, 0, buffer.length);
        }
    }
    
    private static modeToFlags(mode: FileMode): number {
        // Convert FileMode to O_* flags
    }
}
```

**Key Features**:
- FileStream extends Stream (polymorphic with other streams)
- FileMode enum (Read, Write, ReadWrite, Append)
- Static utilities (open, exists, delete, readAllText, writeAllText)
- Exception-based error handling
- IDisposable pattern

---

### 4.3 @std/collections/list

**Purpose**: Dynamic array with type safety and bounds checking

```dryad
// @std/collections/list.dryad
import { IEnumerable } from "@std/core/enumerable";

class List<T> implements IEnumerable<T> {
    private items: Array<T>;  // Internal dynamic array
    private _count: number;
    
    constructor(capacity: number = 16) {
        this.items = new Array<T>(capacity);
        this._count = 0;
    }
    
    // Properties
    readonly count: number { return this._count; }
    
    readonly capacity: number {
        return this.items.length;
    }
    
    // Indexer
    get(index: number): T {
        if (index < 0 || index >= this._count) {
            throw new IndexOutOfRangeException();
        }
        return this.items[index];
    }
    
    set(index: number, value: T): void {
        if (index < 0 || index >= this._count) {
            throw new IndexOutOfRangeException();
        }
        this.items[index] = value;
    }
    
    // Modification
    add(item: T): void {
        this.ensureCapacity(this._count + 1);
        this.items[this._count] = item;
        this._count++;
    }
    
    insert(index: number, item: T): void {
        if (index < 0 || index > this._count) {
            throw new IndexOutOfRangeException();
        }
        this.ensureCapacity(this._count + 1);
        // Shift elements right
        for (let i = this._count; i > index; i--) {
            this.items[i] = this.items[i - 1];
        }
        this.items[index] = item;
        this._count++;
    }
    
    remove(item: T): boolean {
        let index = this.indexOf(item);
        if (index >= 0) {
            this.removeAt(index);
            return true;
        }
        return false;
    }
    
    removeAt(index: number): void {
        if (index < 0 || index >= this._count) {
            throw new IndexOutOfRangeException();
        }
        // Shift elements left
        for (let i = index; i < this._count - 1; i++) {
            this.items[i] = this.items[i + 1];
        }
        this._count--;
    }
    
    clear(): void {
        this._count = 0;
    }
    
    // Search
    contains(item: T): boolean {
        return this.indexOf(item) >= 0;
    }
    
    indexOf(item: T): number {
        for (let i = 0; i < this._count; i++) {
            if (this.items[i] == item) {
                return i;
            }
        }
        return -1;
    }
    
    // IEnumerable implementation
    forEach(fn: (T) => void): void {
        for (let i = 0; i < this._count; i++) {
            fn(this.items[i]);
        }
    }
    
    map<U>(fn: (T) => U): List<U> {
        let result = new List<U>(this._count);
        for (let i = 0; i < this._count; i++) {
            result.add(fn(this.items[i]));
        }
        return result;
    }
    
    filter(fn: (T) => boolean): List<T> {
        let result = new List<T>();
        for (let i = 0; i < this._count; i++) {
            if (fn(this.items[i])) {
                result.add(this.items[i]);
            }
        }
        return result;
    }
    
    reduce<U>(fn: (U, T) => U, initial: U): U {
        let acc = initial;
        for (let i = 0; i < this._count; i++) {
            acc = fn(acc, this.items[i]);
        }
        return acc;
    }
    
    // Helpers
    private ensureCapacity(min: number): void {
        if (this.items.length < min) {
            let newCapacity = this.items.length * 2;
            if (newCapacity < min) {
                newCapacity = min;
            }
            this.resize(newCapacity);
        }
    }
    
    private resize(newCapacity: number): void {
        let newItems = new Array<T>(newCapacity);
        for (let i = 0; i < this._count; i++) {
            newItems[i] = this.items[i];
        }
        this.items = newItems;
    }
}
```

---

### 4.4 @std/net/socket

**Purpose**: Low-level socket abstraction over network intrinsics

```dryad
// @std/net/socket.dryad
import { Stream } from "@std/io/stream";

class Socket implements IDisposable {
    private fd: number;
    private _connected: boolean;
    private closed: boolean;
    
    // Construction
    constructor(domain: SocketDomain, type: SocketType) {
        this.fd = __sys_socket(domain as number, type as number);
        if (this.fd < 0) {
            throw new SocketException("Cannot create socket");
        }
        this._connected = false;
        this.closed = false;
    }
    
    // Properties
    readonly connected: boolean { return this._connected && !this.closed; }
    
    // Operations
    connect(host: string, port: number): void {
        this.ensureNotClosed();
        
        let result = __sys_connect(this.fd, host, port);
        if (result < 0) {
            throw new SocketException("Connection failed");
        }
        this._connected = true;
    }
    
    bind(host: string, port: number): void {
        this.ensureNotClosed();
        
        let result = __sys_bind(this.fd, host, port);
        if (result < 0) {
            throw new SocketException("Bind failed");
        }
    }
    
    listen(backlog: number): void {
        this.ensureNotClosed();
        
        let result = __sys_listen(this.fd, backlog);
        if (result < 0) {
            throw new SocketException("Listen failed");
        }
    }
    
    accept(): Socket {
        this.ensureNotClosed();
        
        let clientFd = __sys_accept(this.fd);
        if (clientFd < 0) {
            throw new SocketException("Accept failed");
        }
        
        let clientSocket = new Socket.__internal(clientFd);
        clientSocket._connected = true;
        return clientSocket;
    }
    
    send(buffer: Buffer, offset: number, count: number): number {
        this.ensureConnected();
        
        let n = __sys_send(this.fd, buffer, offset, count);
        if (n < 0) {
            throw new SocketException("Send failed");
        }
        return n;
    }
    
    receive(buffer: Buffer, offset: number, count: number): number {
        this.ensureConnected();
        
        let n = __sys_recv(this.fd, buffer, offset, count);
        if (n < 0) {
            throw new SocketException("Receive failed");
        }
        return n;
    }
    
    close(): void {
        if (!this.closed) {
            __sys_close(this.fd);
            this.closed = true;
            this._connected = false;
        }
    }
    
    dispose(): void {
        this.close();
    }
    
    // Helpers
    private ensureNotClosed(): void {
        if (this.closed) {
            throw new ObjectDisposedException("Socket");
        }
    }
    
    private ensureConnected(): void {
        this.ensureNotClosed();
        if (!this._connected) {
            throw new InvalidOperationException("Socket not connected");
        }
    }
    
    // Internal constructor for accepted sockets
    internal static __internal(fd: number): Socket {
        let s = new Socket(SocketDomain.IPv4, SocketType.Stream);
        s.fd = fd;
        return s;
    }
}

enum SocketDomain {
    IPv4 = 2,
    IPv6 = 10
}

enum SocketType {
    Stream = 1,  // TCP
    Datagram = 2  // UDP
}

// NetworkStream wraps Socket as Stream
class NetworkStream extends Stream {
    private socket: Socket;
    
    constructor(socket: Socket) {
        if (!socket.connected) {
            throw new ArgumentException("Socket must be connected");
        }
        this.socket = socket;
    }
    
    readonly canRead: boolean { return this.socket.connected; }
    readonly canWrite: boolean { return this.socket.connected; }
    readonly canSeek: boolean { return false; }
    readonly length: number { throw new InvalidOperationException("NetworkStream has no length"); }
    
    position: number {
        get { throw new InvalidOperationException("NetworkStream not seekable"); }
        set { throw new InvalidOperationException("NetworkStream not seekable"); }
    }
    
    read(buffer: Buffer, offset: number, count: number): number {
        return this.socket.receive(buffer, offset, count);
    }
    
    write(buffer: Buffer, offset: number, count: number): void {
        this.socket.send(buffer, offset, count);
    }
    
    seek(offset: number, origin: SeekOrigin): number {
        throw new InvalidOperationException("NetworkStream not seekable");
    }
    
    flush(): void {
        // No-op for network stream
    }
    
    close(): void {
        this.socket.close();
    }
}
```

---

## 5. Intrinsics Mapping

### 5.1 Visibility Strategy

**`internal` keyword**: Intrinsics are only accessible to `@std/*` modules, NOT user code.

```dryad
// @std/runtime/_intrinsics.dryad

// File I/O intrinsics
@intrinsic("syscall.open")
internal function __sys_open(path: string, flags: number): number;

@intrinsic("syscall.close")
internal function __sys_close(fd: number): number;

@intrinsic("syscall.read")
internal function __sys_read(fd: number, buffer: Buffer, offset: number, count: number): number;

@intrinsic("syscall.write")
internal function __sys_write(fd: number, buffer: Buffer, offset: number, count: number): number;

// ... 26 more intrinsics
```

**Compiler behavior**:
- `internal` members can only be imported by modules matching `@std/**` pattern
- User code attempting to import from `@std/runtime/_intrinsics` gets compile error
- This enforces "intrinsics as energy" - users MUST use high-level APIs

---

### 5.2 Wrapping Pattern

Every intrinsic is wrapped by at least one high-level API:

| Intrinsic | Wrapped By | Module |
|-----------|-----------|--------|
| `__sys_open` | `File.open()` | `@std/io/file` |
| `__sys_read` | `FileStream.read()` | `@std/io/file` |
| `__sys_write` | `FileStream.write()` | `@std/io/file` |
| `__sys_close` | `FileStream.close()` | `@std/io/file` |
| `__sys_socket` | `Socket()` constructor | `@std/net/socket` |
| `__sys_connect` | `Socket.connect()` | `@std/net/socket` |
| `__sys_send` | `Socket.send()` | `@std/net/socket` |
| `__sys_recv` | `Socket.receive()` | `@std/net/socket` |
| `__alloc_bytes` | `Buffer()` constructor | `@std/buffers/buffer` |
| `__memcpy` | `Buffer.copyTo()` | `@std/buffers/buffer` |
| `__memset` | `Buffer.fill()` | `@std/buffers/buffer` |

**Complete mapping** (30 intrinsics → stdlib APIs):

```
File I/O (6):
  __sys_open      → File.open(), FileStream
  __sys_close     → FileStream.close()
  __sys_read      → FileStream.read()
  __sys_write     → FileStream.write()
  __sys_lseek     → FileStream.seek()
  __sys_fsync     → FileStream.flush()

Network (8):
  __sys_socket    → Socket constructor
  __sys_connect   → Socket.connect()
  __sys_bind      → Socket.bind()
  __sys_listen    → Socket.listen()
  __sys_accept    → Socket.accept()
  __sys_send      → Socket.send(), NetworkStream.write()
  __sys_recv      → Socket.receive(), NetworkStream.read()
  __sys_shutdown  → Socket.shutdown()

Filesystem (6):
  __sys_access    → File.exists()
  __sys_unlink    → File.delete()
  __sys_rename    → File.move()
  __sys_mkdir     → Directory.create()
  __sys_rmdir     → Directory.delete()
  __sys_fstat     → FileInfo, FileStream.length

Process (5):
  __sys_getenv    → Environment.get()
  __sys_setenv    → Environment.set()
  __sys_getpid    → Process.id
  __sys_getcwd    → Directory.currentDirectory
  __sys_chdir     → Directory.setCurrentDirectory()

Time (2):
  __sys_time      → DateTime.now()
  __sys_clock     → Stopwatch, performance timing

Memory (2):
  __alloc_bytes   → Buffer constructor
  __free_bytes    → Buffer.dispose()

Utilities (1):
  __memcpy        → Buffer.copyTo()
  __memset        → Buffer.fill()
```

---

## 6. Implementation Phases

### Phase 4.1: Core Foundation (Week 1)
**Goal**: Basic infrastructure for stdlib

- [ ] Exception hierarchy (`@std/core/exceptions`)
- [ ] IDisposable interface (`@std/core/disposable`)
- [ ] Buffer class (`@std/buffers/buffer`)
- [ ] Stream abstract class (`@std/io/stream`)
- [ ] Intrinsics visibility (`internal` keyword support in compiler)

**Deliverable**: Can create buffers, throw exceptions, define streams

---

### Phase 4.2: File I/O (Week 1-2)
**Goal**: Complete file operations

- [ ] FileStream implementation
- [ ] File static utilities (open, exists, delete, readAllText, writeAllText)
- [ ] Path utilities (join, dirname, basename, extension)
- [ ] Integration tests (file read/write workflows)

**Deliverable**: Users can do all file I/O via high-level APIs

---

### Phase 4.3: Collections (Week 2)
**Goal**: Essential data structures

- [ ] IEnumerable interface
- [ ] List<T> implementation
- [ ] Map<K, V> implementation (hash map)
- [ ] Set<T> implementation (hash set)
- [ ] Unit tests for all collections

**Deliverable**: Users have List, Map, Set with full LINQ-style operations

---

### Phase 4.4: Networking (Week 2-3)
**Goal**: TCP/UDP socket operations

- [ ] Socket class (wraps socket intrinsics)
- [ ] NetworkStream (Stream adapter for sockets)
- [ ] TcpListener (server)
- [ ] TcpClient (client wrapper)
- [ ] UdpClient
- [ ] Integration tests (echo server, HTTP GET)

**Deliverable**: Users can create TCP/UDP clients and servers

---

### Phase 4.5: Async Primitives (Week 3)
**Goal**: Promise-based async

- [ ] Promise<T> implementation
- [ ] Promise.all(), Promise.race()
- [ ] Async file I/O (File.openAsync, FileStream.readAsync)
- [ ] Async network I/O (Socket.connectAsync)
- [ ] Event loop integration (if needed)

**Deliverable**: Users can do async I/O with Promises

---

### Phase 4.6: Text Processing (Week 3-4)
**Goal**: String manipulation and encoding

- [ ] Encoding classes (UTF8, ASCII)
- [ ] StringBuilder
- [ ] TextReader/TextWriter
- [ ] StreamReader/StreamWriter
- [ ] Basic Regex support

**Deliverable**: Users can process text files and build strings efficiently

---

### Phase 4.7: HTTP Client (Week 4)
**Goal**: High-level HTTP operations

- [ ] HttpClient class
- [ ] HttpRequest/HttpResponse
- [ ] HttpHeaders
- [ ] Basic HTTP methods (GET, POST, PUT, DELETE)
- [ ] Integration tests (fetch from real URLs)

**Deliverable**: Users can make HTTP requests easily

---

### Phase 4.8: JSON Support (Week 4-5)
**Goal**: JSON parsing and serialization

- [ ] JSON parser (string → object tree)
- [ ] JSON serializer (object → string)
- [ ] JsonObject, JsonArray types
- [ ] Type-safe deserialization helpers

**Deliverable**: Users can parse/serialize JSON

---

### Phase 4.9: Diagnostics & Utilities (Week 5)
**Goal**: Developer experience

- [ ] Console class (stdout, stderr, stdin)
- [ ] Debug class (assertions, conditional logging)
- [ ] Stopwatch (performance timing)
- [ ] Environment class (env vars, args)
- [ ] Process class (spawn, exec)

**Deliverable**: Complete developer toolkit

---

### Phase 4.10: Documentation & Polish (Week 5-6)
**Goal**: Production-ready stdlib

- [ ] API documentation (all public classes/methods)
- [ ] Usage examples for each module
- [ ] Performance benchmarks
- [ ] Code review and SOLID cleanup
- [ ] Integration test suite (end-to-end scenarios)

**Deliverable**: Fully documented, tested, production-grade stdlib

---

## 7. API Usage Examples

### 7.1 File I/O

```dryad
import { File, FileMode } from "@std/io/file";
import { Buffer } from "@std/buffers/buffer";

// Read entire file as text
let content = File.readAllText("config.txt");
console.log(content);

// Write text to file
File.writeAllText("output.txt", "Hello, Dryad!");

// Stream-based file copy
using (let src = File.open("input.dat", FileMode.Read)) {
    using (let dst = File.open("output.dat", FileMode.Write)) {
        let buffer = new Buffer(4096);
        let bytesRead = 0;
        
        while ((bytesRead = src.read(buffer, 0, buffer.length)) > 0) {
            dst.write(buffer, 0, bytesRead);
        }
    }
}
```

---

### 7.2 Collections

```dryad
import { List } from "@std/collections/list";
import { Map } from "@std/collections/map";

// List operations
let numbers = new List<number>();
numbers.add(1);
numbers.add(2);
numbers.add(3);

let doubled = numbers.map(x => x * 2);  // [2, 4, 6]
let evens = numbers.filter(x => x % 2 == 0);  // [2]

// Map operations
let ages = new Map<string, number>();
ages.set("Alice", 30);
ages.set("Bob", 25);

if (ages.has("Alice")) {
    console.log("Alice is " + ages.get("Alice"));
}

ages.forEach((name, age) => {
    console.log(name + ": " + age);
});
```

---

### 7.3 Networking

```dryad
import { TcpListener, TcpClient } from "@std/net/tcp";
import { Buffer } from "@std/buffers/buffer";

// TCP Server
let listener = new TcpListener("127.0.0.1", 8080);
listener.start();

console.log("Server listening on port 8080");

while (true) {
    using (let client = listener.accept()) {
        using (let stream = client.getStream()) {
            let buffer = new Buffer(1024);
            let n = stream.read(buffer, 0, buffer.length);
            
            // Echo back
            stream.write(buffer, 0, n);
        }
    }
}

// TCP Client
using (let client = new TcpClient()) {
    client.connect("example.com", 80);
    
    using (let stream = client.getStream()) {
        let request = "GET / HTTP/1.1\r\nHost: example.com\r\n\r\n";
        let buffer = Buffer.fromString(request, Encoding.ASCII);
        
        stream.write(buffer, 0, buffer.length);
        
        let response = new Buffer(4096);
        let n = stream.read(response, 0, response.length);
        console.log(response.toString(Encoding.ASCII));
    }
}
```

---

### 7.4 Async I/O with Promises

```dryad
import { File } from "@std/io/file";
import { HttpClient } from "@std/net/http/client";

// Async file read
File.openAsync("data.txt", FileMode.Read)
    .then(file => file.readAllAsync())
    .then(content => console.log(content))
    .catch(err => console.log("Error: " + err))
    .finally(() => console.log("Done"));

// Async HTTP request
let client = new HttpClient();
client.getAsync("https://api.github.com/users/octocat")
    .then(response => response.json())
    .then(data => console.log("Name: " + data.name))
    .catch(err => console.log("HTTP error: " + err));

// Promise.all for parallel operations
let files = ["file1.txt", "file2.txt", "file3.txt"];
let promises = files.map(f => File.readAllTextAsync(f));

Promise.all(promises)
    .then(contents => {
        contents.forEach(c => console.log(c));
    });
```

---

### 7.5 HTTP Client

```dryad
import { HttpClient } from "@std/net/http/client";

let client = new HttpClient();

// GET request
let response = client.get("https://jsonplaceholder.typicode.com/posts/1");
console.log(response.statusCode);  // 200
console.log(response.body);

// POST request
let body = '{"title": "foo", "body": "bar", "userId": 1}';
let postResponse = client.post(
    "https://jsonplaceholder.typicode.com/posts",
    body,
    { "Content-Type": "application/json" }
);

console.log(postResponse.body);
```

---

### 7.6 JSON Parsing

```dryad
import { Json } from "@std/text/json";

// Parse JSON string
let json = '{"name": "Alice", "age": 30, "active": true}';
let obj = Json.parse(json);

console.log(obj.name);   // "Alice"
console.log(obj.age);    // 30

// Serialize to JSON
let data = {
    users: ["Alice", "Bob", "Charlie"],
    count: 3,
    timestamp: 1234567890
};

let jsonString = Json.stringify(data);
console.log(jsonString);
// {"users":["Alice","Bob","Charlie"],"count":3,"timestamp":1234567890}
```

---

## 8. Testing Strategy

### 8.1 Unit Tests

Each module has dedicated unit tests:

```
tests/stdlib/
├── buffers/
│   └── buffer_test.dryad
├── io/
│   ├── stream_test.dryad
│   ├── file_test.dryad
│   └── memory_stream_test.dryad
├── collections/
│   ├── list_test.dryad
│   ├── map_test.dryad
│   └── set_test.dryad
├── net/
│   ├── socket_test.dryad
│   └── tcp_test.dryad
└── async/
    └── promise_test.dryad
```

### 8.2 Integration Tests

End-to-end scenarios:

```dryad
// tests/integration/file_workflow_test.dryad
test("Complete file workflow") {
    // Write → Read → Verify → Delete
    let path = "test_output.txt";
    let content = "Hello, Dryad!";
    
    File.writeAllText(path, content);
    assert(File.exists(path));
    
    let read = File.readAllText(path);
    assertEqual(read, content);
    
    File.delete(path);
    assert(!File.exists(path));
}

// tests/integration/http_workflow_test.dryad
test("HTTP GET workflow") {
    let client = new HttpClient();
    let response = client.get("https://httpbin.org/get");
    
    assertEqual(response.statusCode, 200);
    assert(response.body.contains("httpbin"));
}
```

### 8.3 Performance Benchmarks

```dryad
// tests/benchmarks/buffer_benchmark.dryad
benchmark("Buffer allocation", () => {
    let b = new Buffer(4096);
});

benchmark("Buffer.copyTo vs manual loop", () => {
    let src = new Buffer(4096);
    let dst = new Buffer(4096);
    src.copyTo(dst, 0, 4096);  // Should be faster (uses __memcpy)
});
```

---

## 9. Quality Gates

### 9.1 Code Quality

- **SOLID compliance**: Every class reviewed for SRP, OCP, LSP, ISP, DIP
- **No magic numbers**: Constants named and documented
- **Defensive programming**: Null checks, bounds checks, state validation
- **Clear naming**: No abbreviations, intention-revealing names

### 9.2 Test Coverage

- **Unit tests**: 100% coverage for core modules (Buffer, Stream, List, Map)
- **Integration tests**: All major workflows (file I/O, networking, HTTP)
- **Edge cases**: Empty inputs, boundary values, error conditions

### 9.3 Performance

- **Buffer operations**: __memcpy intrinsic for bulk copies (4-8x faster)
- **Collections growth**: Exponential capacity (avoid O(n²) on sequential adds)
- **Network I/O**: Buffering to minimize syscalls

### 9.4 Documentation

- **Every public class**: Summary, purpose, usage example
- **Every public method**: Parameters, return value, exceptions
- **Module-level docs**: Overview, common patterns, best practices

---

## 10. Success Criteria

Phase 4 is complete when:

- ✅ All 30 intrinsics are wrapped by high-level APIs
- ✅ Users can perform common tasks WITHOUT touching intrinsics
- ✅ File I/O, networking, collections, async, HTTP, JSON all functional
- ✅ 100+ unit tests passing
- ✅ 10+ integration tests passing
- ✅ Full API documentation written
- ✅ Performance benchmarks run and documented
- ✅ Code review completed (SOLID compliance verified)
- ✅ Example programs written (showcase stdlib capabilities)

---

## 11. Future Extensions (Post-Phase 4)

### Phase 5: Event Loop & Advanced Async
- async/await syntax
- Event loop implementation
- Timers, intervals
- WebSocket support

### Phase 6: Advanced Features
- Regex engine
- HTTP server framework
- Template engine
- Database drivers

### Phase 7+: Ecosystem
- Package manager
- Build system
- Testing framework
- Documentation generator

---

## Appendix A: Naming Conventions

### A.1 Casing Rules

- **Classes**: PascalCase (`FileStream`, `HttpClient`)
- **Interfaces**: PascalCase with `I` prefix (`IDisposable`, `IEnumerable`)
- **Methods**: camelCase (`open`, `readAllText`)
- **Properties**: camelCase (`length`, `canRead`)
- **Constants**: UPPER_SNAKE_CASE (`MAX_BUFFER_SIZE`)
- **Enums**: PascalCase for type, PascalCase for values (`FileMode.ReadWrite`)

### A.2 Module Naming

- **Namespaces**: lowercase with `/` separator (`@std/io/file`)
- **Files**: snake_case (`file_stream.dryad`)

---

## Appendix B: Error Message Guidelines

Good error messages are:
1. **Specific**: "File not found: /tmp/missing.txt" not "Operation failed"
2. **Actionable**: "Buffer size must be >= 0, got -1" not "Invalid argument"
3. **Contextual**: Include relevant state (path, index, size)

Examples:

```dryad
// ❌ Bad
throw new Exception("Error");

// ✅ Good
throw new ArgumentOutOfRangeException("size must be >= 0, got " + size);

// ❌ Bad
throw new IOException("Failed");

// ✅ Good
throw new FileNotFoundException("Cannot open file: " + path);
```

---

## Appendix C: SOLID Checklist

For every class, verify:

- **SRP**: Does it have ONE reason to change? (e.g., FileStream = file I/O only)
- **OCP**: Can you extend behavior WITHOUT modifying code? (e.g., BufferedStream wraps any Stream)
- **LSP**: Can subclass replace parent? (e.g., FileStream is substitutable for Stream)
- **ISP**: Are interfaces minimal? (e.g., IDisposable has only dispose())
- **DIP**: Depend on abstractions? (e.g., BufferedStream depends on Stream, not FileStream)

---

**End of Document**

**Next Steps**:
1. Review this architecture document
2. Get approval for design decisions
3. Begin Phase 4.1 implementation (Core Foundation)
4. Implement `internal` keyword in compiler
5. Create exception hierarchy and IDisposable interface
6. Build Buffer class with intrinsics wrapping
7. Proceed through phases 4.2-4.10 systematically
