# Dryad Standard Library - Implementation Plan (Phase 4)

**Version**: 1.0  
**Date**: 2026-05-28  
**Status**: Ready for Execution

---

## Overview

This document breaks down Phase 4 (Standard Library) into **atomic, testable tasks** following TDD discipline. Each task has clear deliverables, acceptance criteria, and dependencies.

**Estimated Duration**: 5-6 weeks  
**Total Tasks**: 89 tasks across 10 sub-phases

---

## Phase 4.1: Core Foundation (Week 1 - Days 1-3)

**Goal**: Basic infrastructure for stdlib (exceptions, interfaces, buffer)

### Task 4.1.1: Implement `internal` keyword support
**Priority**: P0 (blocking)  
**Estimated Time**: 4 hours  
**Dependencies**: None

**Deliverables**:
- [ ] Add `internal` keyword to lexer (TOKEN_INTERNAL)
- [ ] Extend parser to recognize `internal` modifier on functions/classes
- [ ] Add visibility field to AST nodes (FunctionDecl, ClassDecl)
- [ ] Update interpreter to track module paths during imports
- [ ] Implement visibility check: reject imports of `internal` members from non-@std modules
- [ ] Unit tests: internal function/class declaration
- [ ] Unit tests: import validation (should fail for user code, succeed for @std)

**Acceptance Criteria**:
```dryad
// @std/runtime/_intrinsics.dryad
@intrinsic("test")
internal function __test(): void { }

// @std/io/file.dryad (should work)
import { __test } from "@std/runtime/_intrinsics";

// user_code.dryad (should FAIL at compile time)
import { __test } from "@std/runtime/_intrinsics";  // ERROR: Cannot import internal member
```

**Files to modify**:
- `src/lexer/lexer.cpp` (add TOKEN_INTERNAL)
- `src/parser/parser.cpp` (parse internal modifier)
- `include/dryad/ast/ast.h` (add visibility field)
- `src/interpreter/interpreter.cpp` (visibility checking on imports)
- `tests/unit/internal_keyword_test.cpp`

---

### Task 4.1.2: Create exception hierarchy base
**Priority**: P0  
**Estimated Time**: 3 hours  
**Dependencies**: None

**Deliverables**:
- [ ] Create `stdlib/@std/core/exceptions.dryad`
- [ ] Implement `Exception` base class (message, stackTrace fields)
- [ ] Add `__get_stack_trace()` intrinsic (C++ implementation)
- [ ] Implement `toString()` method
- [ ] Unit tests: create exception, verify message/stackTrace
- [ ] Unit tests: throw and catch exception

**Acceptance Criteria**:
```dryad
import { Exception } from "@std/core/exceptions";

let ex = new Exception("Test error");
console.log(ex.message);      // "Test error"
console.log(ex.stackTrace);   // Stack trace string
console.log(ex.toString());   // "Test error\n<stack>"

throw ex;  // Should propagate
```

**Files to create**:
- `stdlib/@std/core/exceptions.dryad`
- `tests/stdlib/core/exceptions_test.dryad`

**Intrinsic to add**:
- `__get_stack_trace(): string` in `src/runtime/intrinsics_misc.cpp`

---

### Task 4.1.3: Implement exception subclasses
**Priority**: P0  
**Estimated Time**: 2 hours  
**Dependencies**: 4.1.2

**Deliverables**:
- [ ] ArgumentException (+ subclasses: ArgumentNullException, ArgumentOutOfRangeException)
- [ ] IOException (+ subclasses: FileNotFoundException, DirectoryNotFoundException, EndOfStreamException)
- [ ] InvalidOperationException (+ ObjectDisposedException)
- [ ] IndexOutOfRangeException
- [ ] KeyNotFoundException
- [ ] NetworkException (+ subclasses: SocketException, HttpException)
- [ ] Unit tests for each exception type

**Acceptance Criteria**:
```dryad
throw new ArgumentNullException("param cannot be null");
throw new FileNotFoundException("File not found: test.txt");
throw new IndexOutOfRangeException("Index 5 out of range [0, 3]");
```

**Files to modify**:
- `stdlib/@std/core/exceptions.dryad` (add 11 exception classes)
- `tests/stdlib/core/exceptions_test.dryad` (test each type)

---

### Task 4.1.4: Create IDisposable interface
**Priority**: P0  
**Estimated Time**: 1 hour  
**Dependencies**: None

**Deliverables**:
- [ ] Create `stdlib/@std/core/disposable.dryad`
- [ ] Define `IDisposable` interface with `dispose()` method
- [ ] Unit test: class implementing IDisposable

**Acceptance Criteria**:
```dryad
import { IDisposable } from "@std/core/disposable";

class Resource implements IDisposable {
    dispose(): void {
        console.log("Disposed");
    }
}

let r = new Resource();
r.dispose();  // "Disposed"
```

**Files to create**:
- `stdlib/@std/core/disposable.dryad`
- `tests/stdlib/core/disposable_test.dryad`

---

### Task 4.1.5: Implement Buffer class (construction)
**Priority**: P0  
**Estimated Time**: 4 hours  
**Dependencies**: 4.1.2 (exceptions), 4.1.4 (IDisposable)

**Deliverables**:
- [ ] Create `stdlib/@std/buffers/buffer.dryad`
- [ ] Implement Buffer constructor (size validation)
- [ ] Add `__alloc_bytes(size: number): ByteArray` intrinsic
- [ ] Add `__free_bytes(data: ByteArray): void` intrinsic
- [ ] Implement `length` property (readonly)
- [ ] Implement `dispose()` method
- [ ] Unit tests: allocation, length, dispose
- [ ] Unit tests: negative size throws ArgumentOutOfRangeException

**Acceptance Criteria**:
```dryad
import { Buffer } from "@std/buffers/buffer";

let buf = new Buffer(1024);
assertEqual(buf.length, 1024);

buf.dispose();  // Should free memory

// Should throw
try {
    let invalid = new Buffer(-1);
    fail("Should have thrown");
} catch (e: ArgumentOutOfRangeException) {
    // Expected
}
```

**Files to create**:
- `stdlib/@std/buffers/buffer.dryad`
- `tests/stdlib/buffers/buffer_test.dryad`

**Intrinsics to add**:
- `src/runtime/intrinsics_memory.cpp` (add __alloc_bytes, __free_bytes)

---

### Task 4.1.6: Implement Buffer indexed access
**Priority**: P0  
**Estimated Time**: 3 hours  
**Dependencies**: 4.1.5

**Deliverables**:
- [ ] Implement `get(index: number): byte` with bounds checking
- [ ] Implement `set(index: number, value: byte): void` with bounds checking
- [ ] Unit tests: get/set in bounds
- [ ] Unit tests: get/set out of bounds throws IndexOutOfRangeException
- [ ] Unit tests: negative index throws

**Acceptance Criteria**:
```dryad
let buf = new Buffer(10);
buf.set(0, 65);  // 'A'
assertEqual(buf.get(0), 65);

buf.set(10, 66);  // Should throw IndexOutOfRangeException
buf.get(-1);      // Should throw IndexOutOfRangeException
```

**Files to modify**:
- `stdlib/@std/buffers/buffer.dryad`
- `tests/stdlib/buffers/buffer_test.dryad`

---

### Task 4.1.7: Implement Buffer bulk operations
**Priority**: P1  
**Estimated Time**: 4 hours  
**Dependencies**: 4.1.6

**Deliverables**:
- [ ] Add `__memcpy(dest: ByteArray, destOffset: number, src: ByteArray, srcOffset: number, count: number): void` intrinsic
- [ ] Add `__memset(data: ByteArray, value: byte, count: number): void` intrinsic
- [ ] Implement `copyTo(dest: Buffer, destOffset: number, count: number): void`
- [ ] Implement `fill(value: byte): void`
- [ ] Unit tests: copyTo with valid args
- [ ] Unit tests: copyTo with invalid args (bounds checking)
- [ ] Unit tests: fill entire buffer

**Acceptance Criteria**:
```dryad
let src = new Buffer(10);
src.fill(65);  // Fill with 'A'

let dst = new Buffer(10);
src.copyTo(dst, 0, 10);

assertEqual(dst.get(0), 65);
assertEqual(dst.get(9), 65);
```

**Files to modify**:
- `stdlib/@std/buffers/buffer.dryad`
- `tests/stdlib/buffers/buffer_test.dryad`
- `src/runtime/intrinsics_memory.cpp`

---

### Task 4.1.8: Create Stream abstract class
**Priority**: P0  
**Estimated Time**: 3 hours  
**Dependencies**: 4.1.4 (IDisposable), 4.1.5 (Buffer)

**Deliverables**:
- [ ] Create `stdlib/@std/io/stream.dryad`
- [ ] Define abstract Stream class implementing IDisposable
- [ ] Add abstract properties: canRead, canWrite, canSeek, length, position
- [ ] Add abstract methods: read, write, seek, flush, close
- [ ] Define SeekOrigin enum (Begin, Current, End)
- [ ] Implement concrete methods: readByte, writeByte
- [ ] Implement dispose() (calls close())
- [ ] Unit tests: cannot instantiate abstract class (verify in interpreter)

**Acceptance Criteria**:
```dryad
import { Stream, SeekOrigin } from "@std/io/stream";

// Cannot instantiate abstract class
let s = new Stream();  // Should fail

// Subclass can implement
class TestStream extends Stream {
    // ... implement abstract members
}
```

**Files to create**:
- `stdlib/@std/io/stream.dryad`
- `tests/stdlib/io/stream_test.dryad`

---

### Task 4.1.9: Integration test - Core foundation
**Priority**: P1  
**Estimated Time**: 2 hours  
**Dependencies**: 4.1.1 through 4.1.8

**Deliverables**:
- [ ] Create end-to-end test using exceptions + buffer + stream
- [ ] Test: allocate buffer, fill, dispose
- [ ] Test: throw exception, catch, verify stackTrace
- [ ] Test: implement custom Stream subclass
- [ ] Verify all 4.1.x tests passing

**Acceptance Criteria**:
```dryad
// Complete workflow
import { Buffer } from "@std/buffers/buffer";
import { Exception } from "@std/core/exceptions";

try {
    let buf = new Buffer(100);
    buf.fill(65);
    
    assertEqual(buf.get(0), 65);
    
    buf.dispose();
    
    // Access after dispose should fail
    buf.get(0);
} catch (e: Exception) {
    console.log("Caught: " + e.message);
    assert(e.stackTrace.length > 0);
}
```

**Files to create**:
- `tests/integration/core_foundation_test.dryad`

---

**Phase 4.1 Checkpoint**: 
- ✅ 9 tasks completed
- ✅ ~26 hours estimated
- ✅ Deliverable: Exception hierarchy, IDisposable, Buffer, Stream abstractions working
- ✅ Test count: ~30 unit tests + 1 integration test

---

## Phase 4.2: File I/O (Week 1-2, Days 4-8)

**Goal**: Complete file operations (FileStream, File utilities, Path)

### Task 4.2.1: Implement FileMode enum
**Priority**: P0  
**Estimated Time**: 30 minutes  
**Dependencies**: None

**Deliverables**:
- [ ] Create `stdlib/@std/io/file.dryad`
- [ ] Define FileMode enum (Read, Write, ReadWrite, Append)
- [ ] Unit test: enum values

**Acceptance Criteria**:
```dryad
import { FileMode } from "@std/io/file";
assertEqual(FileMode.Read, 0);
assertEqual(FileMode.Write, 1);
```

**Files to create**:
- `stdlib/@std/io/file.dryad`

---

### Task 4.2.2: Implement FileStream constructor and properties
**Priority**: P0  
**Estimated Time**: 3 hours  
**Dependencies**: 4.1.8 (Stream), 4.2.1

**Deliverables**:
- [ ] Implement FileStream class extending Stream
- [ ] Add internal constructor (fd, mode)
- [ ] Implement canRead, canWrite, canSeek properties
- [ ] Implement position property (getter/setter)
- [ ] Add closed flag
- [ ] Add ensureNotClosed(), ensureCanRead(), ensureCanWrite() helpers
- [ ] Unit tests: properties return correct values

**Acceptance Criteria**:
```dryad
// Internal construction only (user uses File.open)
let fs = new FileStream(3, FileMode.Read);  // fd=3
assert(fs.canRead);
assert(!fs.canWrite);
assert(fs.canSeek);
assertEqual(fs.position, 0);
```

**Files to modify**:
- `stdlib/@std/io/file.dryad`
- `tests/stdlib/io/file_test.dryad`

---

### Task 4.2.3: Implement FileStream.read()
**Priority**: P0  
**Estimated Time**: 3 hours  
**Dependencies**: 4.2.2

**Deliverables**:
- [ ] Implement read(buffer, offset, count) method
- [ ] Validate arguments (null buffer, bounds)
- [ ] Call __sys_read intrinsic
- [ ] Update position on success
- [ ] Throw IOException on syscall failure
- [ ] Unit tests: read from file, verify bytes read
- [ ] Unit tests: read with invalid args throws

**Acceptance Criteria**:
```dryad
// Assuming fd 3 is open for reading
let fs = new FileStream(3, FileMode.Read);
let buf = new Buffer(100);
let n = fs.read(buf, 0, 100);

assert(n >= 0);
assertEqual(fs.position, n);
```

**Files to modify**:
- `stdlib/@std/io/file.dryad`
- `tests/stdlib/io/file_test.dryad`

---

### Task 4.2.4: Implement FileStream.write()
**Priority**: P0  
**Estimated Time**: 3 hours  
**Dependencies**: 4.2.2

**Deliverables**:
- [ ] Implement write(buffer, offset, count) method
- [ ] Validate arguments
- [ ] Call __sys_write intrinsic
- [ ] Update position on success
- [ ] Throw IOException on failure
- [ ] Unit tests: write to file, verify bytes written
- [ ] Unit tests: write to read-only stream throws InvalidOperationException

**Acceptance Criteria**:
```dryad
let fs = new FileStream(4, FileMode.Write);
let buf = Buffer.fromString("Hello", Encoding.UTF8);
fs.write(buf, 0, buf.length);

assertEqual(fs.position, 5);
```

**Files to modify**:
- `stdlib/@std/io/file.dryad`
- `tests/stdlib/io/file_test.dryad`

---

### Task 4.2.5: Implement FileStream.seek() and flush()
**Priority**: P0  
**Estimated Time**: 2 hours  
**Dependencies**: 4.2.2

**Deliverables**:
- [ ] Implement seek(offset, origin) method
- [ ] Call __sys_lseek intrinsic
- [ ] Update position on success
- [ ] Implement flush() method (calls __sys_fsync)
- [ ] Unit tests: seek to different positions
- [ ] Unit tests: flush succeeds

**Acceptance Criteria**:
```dryad
let fs = new FileStream(3, FileMode.Read);
fs.seek(10, SeekOrigin.Begin);
assertEqual(fs.position, 10);

fs.seek(5, SeekOrigin.Current);
assertEqual(fs.position, 15);

fs.flush();  // Should not throw
```

**Files to modify**:
- `stdlib/@std/io/file.dryad`
- `tests/stdlib/io/file_test.dryad`

---

### Task 4.2.6: Implement FileStream.close() and length
**Priority**: P0  
**Estimated Time**: 2 hours  
**Dependencies**: 4.2.2

**Deliverables**:
- [ ] Implement close() method (calls __sys_close)
- [ ] Mark stream as closed
- [ ] Implement length property (uses __sys_fstat)
- [ ] Unit tests: close stream, verify subsequent ops throw ObjectDisposedException
- [ ] Unit tests: length returns correct file size

**Acceptance Criteria**:
```dryad
let fs = new FileStream(3, FileMode.Read);
fs.close();

// Operations after close should throw
try {
    fs.read(buf, 0, 10);
    fail();
} catch (e: ObjectDisposedException) {
    // Expected
}
```

**Files to modify**:
- `stdlib/@std/io/file.dryad`
- `tests/stdlib/io/file_test.dryad`

---

### Task 4.2.7: Implement File.open() static method
**Priority**: P0  
**Estimated Time**: 3 hours  
**Dependencies**: 4.2.6

**Deliverables**:
- [ ] Implement File class with static open(path, mode) method
- [ ] Validate path (not null/empty)
- [ ] Convert FileMode to O_* flags
- [ ] Call __sys_open intrinsic
- [ ] Return FileStream on success
- [ ] Throw FileNotFoundException on failure
- [ ] Unit tests: open existing file
- [ ] Unit tests: open non-existent file throws
- [ ] Unit tests: open with different modes

**Acceptance Criteria**:
```dryad
import { File, FileMode } from "@std/io/file";

let fs = File.open("test.txt", FileMode.Read);
assert(fs.canRead);
fs.close();

// Non-existent file
try {
    File.open("missing.txt", FileMode.Read);
    fail();
} catch (e: FileNotFoundException) {
    // Expected
}
```

**Files to modify**:
- `stdlib/@std/io/file.dryad`
- `tests/stdlib/io/file_test.dryad`

---

### Task 4.2.8: Implement File.exists(), File.delete()
**Priority**: P1  
**Estimated Time**: 2 hours  
**Dependencies**: 4.2.7

**Deliverables**:
- [ ] Implement File.exists(path) using __sys_access
- [ ] Implement File.delete(path) using __sys_unlink
- [ ] Unit tests: exists returns true for existing file
- [ ] Unit tests: exists returns false for missing file
- [ ] Unit tests: delete removes file
- [ ] Unit tests: delete non-existent throws IOException

**Acceptance Criteria**:
```dryad
// Create file first (via external means)
assert(File.exists("test.txt"));

File.delete("test.txt");
assert(!File.exists("test.txt"));

// Delete missing file
try {
    File.delete("missing.txt");
    fail();
} catch (e: IOException) {
    // Expected
}
```

**Files to modify**:
- `stdlib/@std/io/file.dryad`
- `tests/stdlib/io/file_test.dryad`

---

### Task 4.2.9: Implement File.readAllText(), File.writeAllText()
**Priority**: P1  
**Estimated Time**: 3 hours  
**Dependencies**: 4.2.7, Task 4.6.1 (Encoding - can stub for now)

**Deliverables**:
- [ ] Stub Encoding.UTF8 for now (implement fully in 4.6)
- [ ] Implement File.readAllText(path) - open, read all, close
- [ ] Implement File.writeAllText(path, content) - open, write all, close
- [ ] Use try/finally to ensure close (no `using` yet)
- [ ] Unit tests: writeAllText then readAllText, verify content matches
- [ ] Unit tests: readAllText on missing file throws

**Acceptance Criteria**:
```dryad
import { File } from "@std/io/file";

let content = "Hello, Dryad!";
File.writeAllText("test.txt", content);

let read = File.readAllText("test.txt");
assertEqual(read, content);

File.delete("test.txt");
```

**Files to modify**:
- `stdlib/@std/io/file.dryad`
- `stdlib/@std/io/text/encoding.dryad` (stub UTF8)
- `tests/stdlib/io/file_test.dryad`

---

### Task 4.2.10: Implement Path utilities
**Priority**: P1  
**Estimated Time**: 4 hours  
**Dependencies**: None

**Deliverables**:
- [ ] Create `stdlib/@std/io/path.dryad`
- [ ] Implement Path.join(parts...) - join path segments with `/`
- [ ] Implement Path.dirname(path) - return directory portion
- [ ] Implement Path.basename(path) - return filename
- [ ] Implement Path.extension(path) - return extension (e.g., ".txt")
- [ ] Unit tests for all path operations

**Acceptance Criteria**:
```dryad
import { Path } from "@std/io/path";

assertEqual(Path.join("foo", "bar", "baz.txt"), "foo/bar/baz.txt");
assertEqual(Path.dirname("/tmp/test.txt"), "/tmp");
assertEqual(Path.basename("/tmp/test.txt"), "test.txt");
assertEqual(Path.extension("test.txt"), ".txt");
assertEqual(Path.extension("test"), "");
```

**Files to create**:
- `stdlib/@std/io/path.dryad`
- `tests/stdlib/io/path_test.dryad`

---

### Task 4.2.11: Integration test - File I/O workflow
**Priority**: P1  
**Estimated Time**: 2 hours  
**Dependencies**: 4.2.1 through 4.2.10

**Deliverables**:
- [ ] End-to-end test: create file, write, read, verify, delete
- [ ] Test: file copy using streams
- [ ] Test: Path utilities with File operations
- [ ] Verify all 4.2.x tests passing

**Acceptance Criteria**:
```dryad
// Complete file workflow
import { File, FileMode } from "@std/io/file";
import { Path } from "@std/io/path";

let path = Path.join("tmp", "test.txt");
File.writeAllText(path, "Test content");
assert(File.exists(path));

let content = File.readAllText(path);
assertEqual(content, "Test content");

File.delete(path);
assert(!File.exists(path));

// Stream-based copy
File.writeAllText("src.txt", "Source");
let src = File.open("src.txt", FileMode.Read);
let dst = File.open("dst.txt", FileMode.Write);

let buf = new Buffer(1024);
let n = src.read(buf, 0, buf.length);
dst.write(buf, 0, n);

src.close();
dst.close();

assertEqual(File.readAllText("dst.txt"), "Source");
```

**Files to create**:
- `tests/integration/file_io_workflow_test.dryad`

---

**Phase 4.2 Checkpoint**:
- ✅ 11 tasks completed
- ✅ ~28 hours estimated
- ✅ Deliverable: Complete file I/O (FileStream, File utilities, Path)
- ✅ Test count: ~40 unit tests + 1 integration test

---

## Phase 4.3: Collections (Week 2, Days 9-11)

**Goal**: Essential data structures (List, Map, Set)

### Task 4.3.1: Create IEnumerable interface
**Priority**: P0  
**Estimated Time**: 1 hour  
**Dependencies**: None

**Deliverables**:
- [ ] Create `stdlib/@std/core/enumerable.dryad`
- [ ] Define IEnumerable<T> interface
- [ ] Add forEach, map, filter, reduce methods
- [ ] Unit test: class implementing IEnumerable

**Files to create**:
- `stdlib/@std/core/enumerable.dryad`
- `tests/stdlib/core/enumerable_test.dryad`

---

### Task 4.3.2: Implement List<T> - construction and properties
**Priority**: P0  
**Estimated Time**: 2 hours  
**Dependencies**: 4.3.1

**Deliverables**:
- [ ] Create `stdlib/@std/collections/list.dryad`
- [ ] Implement List<T> class implementing IEnumerable<T>
- [ ] Add private items array, count field
- [ ] Implement constructor with capacity parameter
- [ ] Implement count, capacity properties
- [ ] Unit tests: create list, verify count/capacity

**Files to create**:
- `stdlib/@std/collections/list.dryad`
- `tests/stdlib/collections/list_test.dryad`

---

### Task 4.3.3: Implement List<T> - indexer and modification
**Priority**: P0  
**Estimated Time**: 3 hours  
**Dependencies**: 4.3.2

**Deliverables**:
- [ ] Implement get(index), set(index, value) with bounds checking
- [ ] Implement add(item) with auto-resize
- [ ] Implement insert(index, item)
- [ ] Implement remove(item), removeAt(index)
- [ ] Implement clear()
- [ ] Unit tests for all operations
- [ ] Unit tests: out of bounds throws IndexOutOfRangeException

**Files to modify**:
- `stdlib/@std/collections/list.dryad`
- `tests/stdlib/collections/list_test.dryad`

---

### Task 4.3.4: Implement List<T> - search and enumeration
**Priority**: P0  
**Estimated Time**: 3 hours  
**Dependencies**: 4.3.3

**Deliverables**:
- [ ] Implement contains(item), indexOf(item)
- [ ] Implement forEach(fn)
- [ ] Implement map<U>(fn)
- [ ] Implement filter(fn)
- [ ] Implement reduce<U>(fn, initial)
- [ ] Unit tests for all enumeration methods

**Files to modify**:
- `stdlib/@std/collections/list.dryad`
- `tests/stdlib/collections/list_test.dryad`

---

### Task 4.3.5: Implement Map<K, V> - hash map structure
**Priority**: P0  
**Estimated Time**: 5 hours  
**Dependencies**: 4.3.1

**Deliverables**:
- [ ] Create `stdlib/@std/collections/map.dryad`
- [ ] Implement Map<K, V> with hash table (buckets array)
- [ ] Add hash function for keys
- [ ] Implement set(key, value) - insert/update with collision handling
- [ ] Implement get(key) - return value or null
- [ ] Implement has(key) - check existence
- [ ] Implement delete(key) - remove entry
- [ ] Implement size property
- [ ] Unit tests for all operations

**Files to create**:
- `stdlib/@std/collections/map.dryad`
- `tests/stdlib/collections/map_test.dryad`

---

### Task 4.3.6: Implement Map<K, V> - enumeration
**Priority**: P1  
**Estimated Time**: 2 hours  
**Dependencies**: 4.3.5

**Deliverables**:
- [ ] Implement forEach(fn: (key, value) => void)
- [ ] Implement keys(): List<K>
- [ ] Implement values(): List<V>
- [ ] Unit tests for enumeration

**Files to modify**:
- `stdlib/@std/collections/map.dryad`
- `tests/stdlib/collections/map_test.dryad`

---

### Task 4.3.7: Implement Set<T>
**Priority**: P1  
**Estimated Time**: 4 hours  
**Dependencies**: 4.3.5 (can reuse hash map internals)

**Deliverables**:
- [ ] Create `stdlib/@std/collections/set.dryad`
- [ ] Implement Set<T> using Map<T, boolean> internally
- [ ] Implement add(item), remove(item)
- [ ] Implement has(item)
- [ ] Implement size property
- [ ] Implement forEach(fn)
- [ ] Unit tests for all operations

**Files to create**:
- `stdlib/@std/collections/set.dryad`
- `tests/stdlib/collections/set_test.dryad`

---

### Task 4.3.8: Implement Queue<T>
**Priority**: P2 (nice to have)  
**Estimated Time**: 3 hours  
**Dependencies**: None

**Deliverables**:
- [ ] Create `stdlib/@std/collections/queue.dryad`
- [ ] Implement Queue<T> with circular buffer
- [ ] Implement enqueue(item), dequeue()
- [ ] Implement peek(), isEmpty()
- [ ] Unit tests

**Files to create**:
- `stdlib/@std/collections/queue.dryad`
- `tests/stdlib/collections/queue_test.dryad`

---

### Task 4.3.9: Integration test - Collections workflow
**Priority**: P1  
**Estimated Time**: 2 hours  
**Dependencies**: 4.3.1 through 4.3.7

**Deliverables**:
- [ ] End-to-end test using List, Map, Set together
- [ ] Test: build collection, enumerate, transform
- [ ] Verify all 4.3.x tests passing

**Acceptance Criteria**:
```dryad
import { List, Map, Set } from "@std/collections";

let list = new List<number>();
list.add(1);
list.add(2);
list.add(3);

let doubled = list.map(x => x * 2);
assertEqual(doubled.count, 3);
assertEqual(doubled.get(0), 2);

let map = new Map<string, number>();
map.set("age", 30);
assertEqual(map.get("age"), 30);

let set = new Set<string>();
set.add("foo");
set.add("foo");  // Duplicate
assertEqual(set.size, 1);
```

**Files to create**:
- `tests/integration/collections_workflow_test.dryad`

---

**Phase 4.3 Checkpoint**:
- ✅ 9 tasks completed
- ✅ ~25 hours estimated
- ✅ Deliverable: List, Map, Set, Queue collections
- ✅ Test count: ~35 unit tests + 1 integration test

---

## Phase 4.4: Networking (Week 2-3, Days 12-16)

**Goal**: TCP/UDP socket operations

### Task 4.4.1: Implement Socket class - construction
**Priority**: P0  
**Estimated Time**: 3 hours  
**Dependencies**: 4.1.2 (exceptions), 4.1.4 (IDisposable)

**Deliverables**:
- [ ] Create `stdlib/@std/net/socket.dryad`
- [ ] Define SocketDomain enum (IPv4, IPv6)
- [ ] Define SocketType enum (Stream, Datagram)
- [ ] Implement Socket constructor (calls __sys_socket)
- [ ] Add connected, closed properties
- [ ] Implement close() and dispose()
- [ ] Unit tests: create socket, verify properties

**Files to create**:
- `stdlib/@std/net/socket.dryad`
- `tests/stdlib/net/socket_test.dryad`

---

### Task 4.4.2: Implement Socket.connect()
**Priority**: P0  
**Estimated Time**: 2 hours  
**Dependencies**: 4.4.1

**Deliverables**:
- [ ] Implement connect(host, port) method
- [ ] Call __sys_connect intrinsic
- [ ] Set connected flag on success
- [ ] Throw SocketException on failure
- [ ] Unit tests: connect to localhost (requires server setup)

**Files to modify**:
- `stdlib/@std/net/socket.dryad`
- `tests/stdlib/net/socket_test.dryad`

---

### Task 4.4.3: Implement Socket server methods (bind, listen, accept)
**Priority**: P0  
**Estimated Time**: 4 hours  
**Dependencies**: 4.4.1

**Deliverables**:
- [ ] Implement bind(host, port)
- [ ] Implement listen(backlog)
- [ ] Implement accept() - returns new Socket for client
- [ ] Add internal __internal(fd) constructor for accepted sockets
- [ ] Unit tests: bind/listen/accept workflow

**Files to modify**:
- `stdlib/@std/net/socket.dryad`
- `tests/stdlib/net/socket_test.dryad`

---

### Task 4.4.4: Implement Socket I/O (send, receive)
**Priority**: P0  
**Estimated Time**: 3 hours  
**Dependencies**: 4.4.2

**Deliverables**:
- [ ] Implement send(buffer, offset, count)
- [ ] Implement receive(buffer, offset, count)
- [ ] Validate socket is connected
- [ ] Throw SocketException on failure
- [ ] Unit tests: send/receive data

**Files to modify**:
- `stdlib/@std/net/socket.dryad`
- `tests/stdlib/net/socket_test.dryad`

---

### Task 4.4.5: Implement NetworkStream
**Priority**: P0  
**Estimated Time**: 3 hours  
**Dependencies**: 4.4.4, 4.1.8 (Stream)

**Deliverables**:
- [ ] Create NetworkStream class extending Stream
- [ ] Wrap Socket for Stream interface
- [ ] Implement read(), write() delegating to socket
- [ ] Implement canRead, canWrite, canSeek properties
- [ ] Throw InvalidOperationException on seek/length access
- [ ] Unit tests: create NetworkStream from socket, read/write

**Files to modify**:
- `stdlib/@std/net/socket.dryad` (add NetworkStream class)
- `tests/stdlib/net/socket_test.dryad`

---

### Task 4.4.6: Implement TcpListener
**Priority**: P1  
**Estimated Time**: 3 hours  
**Dependencies**: 4.4.3

**Deliverables**:
- [ ] Create `stdlib/@std/net/tcp_listener.dryad`
- [ ] Implement TcpListener class wrapping Socket
- [ ] Implement start() - bind + listen
- [ ] Implement accept() - returns TcpClient
- [ ] Implement stop() - close socket
- [ ] Unit tests: start listener, accept connection

**Files to create**:
- `stdlib/@std/net/tcp_listener.dryad`
- `tests/stdlib/net/tcp_test.dryad`

---

### Task 4.4.7: Implement TcpClient
**Priority**: P1  
**Estimated Time**: 3 hours  
**Dependencies**: 4.4.2, 4.4.5

**Deliverables**:
- [ ] Create `stdlib/@std/net/tcp_client.dryad`
- [ ] Implement TcpClient class wrapping Socket
- [ ] Implement connect(host, port)
- [ ] Implement getStream() - returns NetworkStream
- [ ] Implement close()
- [ ] Unit tests: connect to server, get stream

**Files to create**:
- `stdlib/@std/net/tcp_client.dryad`
- `tests/stdlib/net/tcp_test.dryad`

---

### Task 4.4.8: Implement UdpClient
**Priority**: P2  
**Estimated Time**: 4 hours  
**Dependencies**: 4.4.1

**Deliverables**:
- [ ] Create `stdlib/@std/net/udp_client.dryad`
- [ ] Implement UdpClient class (Datagram socket)
- [ ] Implement send(buffer, host, port)
- [ ] Implement receive() - returns buffer + remote endpoint
- [ ] Unit tests: send/receive UDP datagrams

**Files to create**:
- `stdlib/@std/net/udp_client.dryad`
- `tests/stdlib/net/udp_test.dryad`

---

### Task 4.4.9: Integration test - Echo server/client
**Priority**: P1  
**Estimated Time**: 3 hours  
**Dependencies**: 4.4.1 through 4.4.7

**Deliverables**:
- [ ] Create echo server using TcpListener
- [ ] Create echo client using TcpClient
- [ ] Test: client sends message, server echoes back, client verifies
- [ ] Verify all 4.4.x tests passing

**Acceptance Criteria**:
```dryad
// Echo server (in background thread or separate process)
import { TcpListener } from "@std/net/tcp_listener";

let listener = new TcpListener("127.0.0.1", 8080);
listener.start();

let client = listener.accept();
let stream = client.getStream();
let buf = new Buffer(1024);
let n = stream.read(buf, 0, buf.length);
stream.write(buf, 0, n);  // Echo
client.close();

// Echo client
import { TcpClient } from "@std/net/tcp_client";

let client = new TcpClient();
client.connect("127.0.0.1", 8080);
let stream = client.getStream();

let msg = Buffer.fromString("Hello", Encoding.UTF8);
stream.write(msg, 0, msg.length);

let response = new Buffer(1024);
let n = stream.read(response, 0, response.length);
assertEqual(response.toString(Encoding.UTF8), "Hello");

client.close();
```

**Files to create**:
- `tests/integration/echo_server_test.dryad`

---

**Phase 4.4 Checkpoint**:
- ✅ 9 tasks completed
- ✅ ~28 hours estimated
- ✅ Deliverable: Socket, TcpListener, TcpClient, UdpClient, NetworkStream
- ✅ Test count: ~30 unit tests + 1 integration test

---

## Phase 4.5: Async Primitives (Week 3, Days 17-19)

**Goal**: Promise-based async operations

### Task 4.5.1: Implement Promise<T> - construction
**Priority**: P0  
**Estimated Time**: 4 hours  
**Dependencies**: 4.1.2 (exceptions)

**Deliverables**:
- [ ] Create `stdlib/@std/async/promise.dryad`
- [ ] Define PromiseState enum (Pending, Fulfilled, Rejected)
- [ ] Implement Promise<T> constructor with executor function
- [ ] Add state, value, error, callbacks fields
- [ ] Implement resolve(value) and reject(error) internal methods
- [ ] Unit tests: create promise, resolve, verify state

**Files to create**:
- `stdlib/@std/async/promise.dryad`
- `tests/stdlib/async/promise_test.dryad`

---

### Task 4.5.2: Implement Promise.then()
**Priority**: P0  
**Estimated Time**: 4 hours  
**Dependencies**: 4.5.1

**Deliverables**:
- [ ] Implement then<U>(onSuccess) - chain promises
- [ ] Handle already-resolved promise (call callback immediately)
- [ ] Handle pending promise (queue callback)
- [ ] Return new Promise<U>
- [ ] Unit tests: then() chaining, transformation

**Files to modify**:
- `stdlib/@std/async/promise.dryad`
- `tests/stdlib/async/promise_test.dryad`

---

### Task 4.5.3: Implement Promise.catch() and finally()
**Priority**: P0  
**Estimated Time**: 3 hours  
**Dependencies**: 4.5.2

**Deliverables**:
- [ ] Implement catch(onError) - error handling
- [ ] Implement finally(onComplete) - cleanup
- [ ] Unit tests: error propagation, finally execution

**Files to modify**:
- `stdlib/@std/async/promise.dryad`
- `tests/stdlib/async/promise_test.dryad`

---

### Task 4.5.4: Implement Promise static helpers
**Priority**: P1  
**Estimated Time**: 3 hours  
**Dependencies**: 4.5.1

**Deliverables**:
- [ ] Implement Promise.resolve<T>(value)
- [ ] Implement Promise.reject<T>(error)
- [ ] Implement Promise.all<T>(promises) - wait for all
- [ ] Implement Promise.race<T>(promises) - first to complete
- [ ] Unit tests for all static methods

**Files to modify**:
- `stdlib/@std/async/promise.dryad`
- `tests/stdlib/async/promise_test.dryad`

---

### Task 4.5.5: Add async file I/O (File.readAllTextAsync)
**Priority**: P1  
**Estimated Time**: 3 hours  
**Dependencies**: 4.5.3, 4.2.9 (File.readAllText)

**Deliverables**:
- [ ] Implement File.readAllTextAsync(path) - returns Promise<string>
- [ ] Use __sys_read_async intrinsic (or simulate with setTimeout + sync read)
- [ ] Unit tests: async file read, verify promise resolution

**Files to modify**:
- `stdlib/@std/io/file.dryad`
- `tests/stdlib/io/file_async_test.dryad`

---

### Task 4.5.6: Add async socket operations
**Priority**: P1  
**Estimated Time**: 3 hours  
**Dependencies**: 4.5.3, 4.4.2 (Socket.connect)

**Deliverables**:
- [ ] Implement Socket.connectAsync(host, port) - returns Promise<void>
- [ ] Implement Socket.sendAsync(buffer) - returns Promise<number>
- [ ] Implement Socket.receiveAsync(buffer) - returns Promise<number>
- [ ] Unit tests: async socket operations

**Files to modify**:
- `stdlib/@std/net/socket.dryad`
- `tests/stdlib/net/socket_async_test.dryad`

---

### Task 4.5.7: Integration test - Async workflows
**Priority**: P1  
**Estimated Time**: 2 hours  
**Dependencies**: 4.5.1 through 4.5.6

**Deliverables**:
- [ ] Test: Promise.all with multiple file reads
- [ ] Test: Promise.race with timeout
- [ ] Test: Async HTTP-like request (socket + promise)
- [ ] Verify all 4.5.x tests passing

**Acceptance Criteria**:
```dryad
import { File } from "@std/io/file";
import { Promise } from "@std/async/promise";

// Parallel file reads
let promises = [
    File.readAllTextAsync("file1.txt"),
    File.readAllTextAsync("file2.txt"),
    File.readAllTextAsync("file3.txt")
];

Promise.all(promises)
    .then(contents => {
        assertEqual(contents.length, 3);
        console.log("All files read");
    });
```

**Files to create**:
- `tests/integration/async_workflow_test.dryad`

---

**Phase 4.5 Checkpoint**:
- ✅ 7 tasks completed
- ✅ ~22 hours estimated
- ✅ Deliverable: Promise<T>, async file I/O, async socket operations
- ✅ Test count: ~25 unit tests + 1 integration test

---

## Remaining Phases Summary (4.6 - 4.10)

Due to length, I'll provide condensed task breakdowns for remaining phases:

---

## Phase 4.6: Text Processing (Week 3-4, Days 20-23)

**Tasks** (8 total, ~24 hours):
- 4.6.1: Implement Encoding interface and UTF8 encoder/decoder
- 4.6.2: Implement ASCII encoding
- 4.6.3: Implement StringBuilder class
- 4.6.4: Implement TextReader/TextWriter abstract classes
- 4.6.5: Implement StreamReader (reads text from Stream)
- 4.6.6: Implement StreamWriter (writes text to Stream)
- 4.6.7: Implement basic Regex class (simple pattern matching)
- 4.6.8: Integration test - text processing workflow

---

## Phase 4.7: HTTP Client (Week 4, Days 24-26)

**Tasks** (9 total, ~28 hours):
- 4.7.1: Implement HttpHeaders class (key-value pairs)
- 4.7.2: Implement HttpRequest class (method, url, headers, body)
- 4.7.3: Implement HttpResponse class (statusCode, headers, body)
- 4.7.4: Implement HttpClient.send(request) - low-level
- 4.7.5: Implement HttpClient.get(url) - convenience
- 4.7.6: Implement HttpClient.post(url, body) - convenience
- 4.7.7: Add PUT, DELETE, PATCH methods
- 4.7.8: Add async versions (getAsync, postAsync)
- 4.7.9: Integration test - HTTP requests to real URLs

---

## Phase 4.8: JSON Support (Week 4-5, Days 27-29)

**Tasks** (7 total, ~26 hours):
- 4.8.1: Implement JsonValue base class (Object, Array, String, Number, Boolean, Null)
- 4.8.2: Implement JsonObject (key-value pairs)
- 4.8.3: Implement JsonArray (indexed values)
- 4.8.4: Implement JSON tokenizer/lexer
- 4.8.5: Implement JSON parser (string → JsonValue tree)
- 4.8.6: Implement JSON serializer (JsonValue → string)
- 4.8.7: Integration test - parse/serialize JSON

---

## Phase 4.9: Diagnostics & Utilities (Week 5, Days 30-32)

**Tasks** (10 total, ~24 hours):
- 4.9.1: Implement Console class (stdout, stderr, stdin)
- 4.9.2: Implement Console.log, Console.error methods
- 4.9.3: Implement Console.readLine (stdin)
- 4.9.4: Implement Debug class (assert, conditional logging)
- 4.9.5: Implement Stopwatch class (performance timing)
- 4.9.6: Implement Environment class (getEnv, setEnv)
- 4.9.7: Implement Environment.args (command-line arguments)
- 4.9.8: Implement Process class (current process info)
- 4.9.9: Implement Process.spawn (future - complex)
- 4.9.10: Integration test - diagnostics workflow

---

## Phase 4.10: Documentation & Polish (Week 5-6, Days 33-40)

**Tasks** (12 total, ~40 hours):
- 4.10.1: Write API documentation for @std/core modules
- 4.10.2: Write API documentation for @std/io modules
- 4.10.3: Write API documentation for @std/collections
- 4.10.4: Write API documentation for @std/net
- 4.10.5: Write API documentation for @std/async
- 4.10.6: Write API documentation for @std/text
- 4.10.7: Create usage examples for each module
- 4.10.8: Run performance benchmarks (Buffer, Collections, I/O)
- 4.10.9: SOLID code review (all modules)
- 4.10.10: Create comprehensive integration test suite
- 4.10.11: Write STDLIB_FINAL_REPORT.md
- 4.10.12: Tag Phase 4 completion commit

---

## Task Summary

| Phase | Tasks | Estimated Hours | Test Count |
|-------|-------|----------------|------------|
| 4.1 Core Foundation | 9 | 26 | ~31 |
| 4.2 File I/O | 11 | 28 | ~41 |
| 4.3 Collections | 9 | 25 | ~36 |
| 4.4 Networking | 9 | 28 | ~31 |
| 4.5 Async Primitives | 7 | 22 | ~26 |
| 4.6 Text Processing | 8 | 24 | ~28 |
| 4.7 HTTP Client | 9 | 28 | ~30 |
| 4.8 JSON Support | 7 | 26 | ~25 |
| 4.9 Diagnostics | 10 | 24 | ~32 |
| 4.10 Documentation | 12 | 40 | ~10 |
| **TOTAL** | **89** | **271** | **~290** |

**Timeline**: 271 hours ÷ 8 hours/day = **34 working days (6.8 weeks)**

---

## Execution Strategy

### 1. TDD Discipline (MANDATORY)
- Write test FIRST for every feature
- Run test (should fail)
- Implement minimum code to pass
- Refactor, verify test still passes
- NO feature without test

### 2. Atomic Commits
- One commit per task completion
- Commit message format: `[Phase 4.X.Y] Task description`
- Example: `[Phase 4.1.5] Implement Buffer class construction`

### 3. Verification Gates
After each phase:
- [ ] All unit tests passing (100%)
- [ ] Integration test passing
- [ ] `lsp_diagnostics` clean
- [ ] SOLID review completed
- [ ] Commit tagged with phase number

### 4. Parallel Work (Where Possible)
- Phase 4.3 (Collections) can start after 4.1 (no file I/O dependency)
- Phase 4.5 (Async) can overlap with 4.6 (Text) for non-dependent tasks

### 5. Daily Progress Tracking
- Create TODO list from this plan at start of each phase
- Mark tasks `in_progress` before starting
- Mark tasks `completed` immediately after verification
- Update IMPLEMENTATION_LOG.md daily with progress

---

## Success Criteria (Phase 4 Complete)

- ✅ All 89 tasks completed
- ✅ ~290 tests passing (100% pass rate)
- ✅ All 30 intrinsics wrapped by high-level APIs
- ✅ Zero `internal` intrinsics exposed to user code
- ✅ Full API documentation written
- ✅ 10+ end-to-end integration tests
- ✅ Performance benchmarks documented
- ✅ SOLID compliance verified
- ✅ STDLIB_FINAL_REPORT.md written
- ✅ Commit tagged: `phase-4-complete`

---

## Risk Mitigation

| Risk | Mitigation |
|------|-----------|
| `internal` keyword complex to implement | Start with simple visibility check, enhance later |
| Promise implementation complex | Study existing JS Promise implementations, simplify initial version |
| Hash map collisions not handled well | Use simple chaining, optimize in later phase |
| Async I/O requires event loop | Simulate async with promises + sync syscalls initially, add true async in Phase 5 |
| Regex too complex for Phase 4 | Implement basic pattern matching only (literal strings, wildcards), full regex in Phase 6 |

---

**End of Implementation Plan**

**Next Step**: Begin Phase 4.1, Task 4.1.1 (Implement `internal` keyword)
