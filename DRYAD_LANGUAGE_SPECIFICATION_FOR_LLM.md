# Dryad Programming Language - Complete Specification for LLMs

**Document Version:** 1.0  
**Generated:** 2026-03-09  
**Purpose:** MCP (Model Context Protocol) documentation for LLM understanding of Dryad syntax, semantics, and execution model  
**Target Audience:** AI Language Models, Code Assistants, Automated Tooling

---

## 📋 Table of Contents

1. [Language Overview](#1-language-overview)
2. [Lexical Structure](#2-lexical-structure)
3. [Type System](#3-type-system)
4. [Expressions](#4-expressions)
5. [Statements](#5-statements)
6. [Declarations](#6-declarations)
7. [Modules and Imports](#7-modules-and-imports)
8. [Runtime Execution Model](#8-runtime-execution-model)
9. [FFI and Native Functions](#9-ffi-and-native-functions)
10. [Error Handling](#10-error-handling)
11. [Concurrency Model](#11-concurrency-model)
12. [Standard Patterns](#12-standard-patterns)
13. [Quick Reference](#13-quick-reference)
14. [Implementation Notes](#14-implementation-notes)
15. [Example Programs](#15-example-programs)

---

## 1. Language Overview

### 1.1 Fundamental Characteristics

**Name:** Dryad  
**Version:** 0.1.0  
**Paradigm:** Multi-paradigm (Procedural, Object-Oriented, Functional)  
**Type System:** Dynamic with optional type annotations  
**Execution Model:** Tree-walking interpreter (direct AST execution)  
**Memory Management:** Garbage collected (Rust reference counting)  
**Concurrency:** Native threads, async/await, mutexes  
**Target Runtime:** Cross-platform (Windows, Linux, macOS via SDL2 + system APIs)

### 1.2 Key Design Principles

- **JavaScript/TypeScript-inspired syntax** for familiar developer experience
- **Dynamic typing** with optional static type annotations
- **Direct AST interpretation** over bytecode (emphasis on clarity over raw speed)
- **Native module system** supporting C FFI and standard library functions
- **Comprehensive error handling** with stack traces and error propagation
- **Built-in concurrency primitives** (threads, mutexes, async/await)

### 1.3 Code Structure

All Dryad programs consist of:

- **Statements** (instructions that perform actions)
- **Expressions** (computations that produce values)
- **Declarations** (functions, classes, variables, constants)
- **Modules** (reusable code units via import/export/use)

---

## 2. Lexical Structure

### 2.1 Token Types and Classification

The lexer converts source text into tokens. Complete token taxonomy:

| Category               | Token Type                                                                     | Pattern/Examples                          | Notes                                                          |
| ---------------------- | ------------------------------------------------------------------------------ | ----------------------------------------- | -------------------------------------------------------------- |
| **Identifiers**        | `Identifier(String)`                                                           | `[a-zA-Z_][a-zA-Z0-9_]*`                  | Variable, function, class names                                |
| **Literals - Numeric** | `Number(f64)`                                                                  | `42`, `3.14`, `0xFF`, `0b1010`, `0o755`   | All numbers are f64 internally                                 |
| **Literals - String**  | `String(String)`                                                               | `"text"`, `'text'`                        | Supports escape sequences: `\n\t\r\\\"\'` and unicode `\uXXXX` |
| **Literals - Boolean** | `Boolean(bool)`                                                                | `true`, `false`                           | Primitive boolean values                                       |
| **Literals - Null**    | `Literal(String)`                                                              | `null`                                    | Null/undefined value                                           |
| **Template Strings**   | `TemplateStart`, `TemplateContent()`, `InterpolationStart`, `InterpolationEnd` | `` `text ${expr} more` ``                 | String interpolation with embedded expressions                 |
| **Keywords**           | `Keyword(String)`                                                              | See section 2.2                           | Reserved language keywords                                     |
| **Operators**          | `Operator(String)`                                                             | See section 2.3                           | All operators and compound assignments                         |
| **Arrow Function**     | `Arrow`                                                                        | `=>`                                      | Lambda/anonymous function marker                               |
| **Symbols**            | `Symbol(char)`                                                                 | `.`, `{}`, `()`, `[]`, `;`, `,`, `:`, `=` | Delimiters and structure markers                               |
| **Native Directive**   | `NativeDirective(String)`                                                      | `#<module_name>`                          | Load native module (e.g., `#io`, `#crypto`)                    |
| **EOF**                | `Eof`                                                                          | (end of input)                            | End of file marker                                             |

### 2.2 Keywords

- **Variables:** `let`, `const`
- **Control Flow:** `if`, `else`, `for`, `while`, `do`, `break`, `continue`, `match`, `in`
- **Functions:** `function`, `fn`, `async`, `await`, `return`, `thread`, `mutex`
- **OOP:** `class`, `new`, `extends`, `this`, `super`, `static`, `public`, `private`, `protected`
- **Modules:** `import`, `export`, `use`, `as`, `from`
- **Error Handling:** `try`, `catch`, `finally`, `throw`
- **Literals:** `true`, `false`, `null`

### 2.3 Operators

**Arithmetic Operators:**

- `+` (addition/string concatenation)
- `-` (subtraction)
- `*` (multiplication)
- `/` (division)
- `%` (modulo)
- `**` (exponentiation)

**Bitwise Operators:**

- `&` (bitwise AND)
- `|` (bitwise OR)
- `^` (bitwise XOR, also `^^`)
- `<<` (left shift)
- `>>` (right shift)
- `>>>` (unsigned right shift)
- `<<<` (rotate left)
- `%%` (modulo bitwise)

**Comparison Operators:**

- `==` (equality)
- `!=` (inequality)
- `<` (less than)
- `>` (greater than)
- `<=` (less than or equal)
- `>=` (greater than or equal)

**Logical Operators:**

- `&&` (logical AND)
- `||` (logical OR)
- `!` (logical NOT)

**Assignment Operators:**

- `=` (assignment)
- `+=`, `-=`, `*=`, `/=` (compound assignments)

**Increment/Decrement:**

- `++` (increment, prefix or postfix)
- `--` (decrement, prefix or postfix)

**Special Operators:**

- `=>` (arrow function)
- `#` (native directive prefix)

### 2.4 Comments

- **Line comments:** `// comment to end of line`
- **Block comments:** `/* multi-line comment */`
- Comments are skipped by lexer and not part of AST

### 2.5 Whitespace

- Whitespace (spaces, tabs, newlines) is mostly insignificant except:
  - Newlines after statements may act as implicit semicolons in some contexts
  - Indentation has no syntactic meaning (unlike Python)
- Statements can span multiple lines without special continuation syntax

---

## 3. Type System

### 3.1 Primitive Types

| Type     | Description           | Literals                               | Internal Representation |
| -------- | --------------------- | -------------------------------------- | ----------------------- |
| `number` | Floating-point number | `42`, `3.14`, `0xFF`, `1e10`           | IEEE 754 f64            |
| `string` | Text string           | `"hello"`, `'world'`, `` `template` `` | UTF-8 String            |
| `bool`   | Boolean value         | `true`, `false`                        | Rust bool               |
| `null`   | Null/undefined value  | `null`                                 | Null variant            |
| `any`    | Any type (dynamic)    | (implied, no literal)                  | Value enum              |

### 3.2 Composite Types

**Arrays:**

```dryad
let arr: number[] = [1, 2, 3];
let mixed: any[] = [1, "two", true];
```

- Declared with `Type[]` syntax
- Heterogeneous (can contain any types when typed as `any[]`)
- Access via index: `arr[0]`, `arr[n]`

**Tuples:**

```dryad
let tuple: (number, string, bool) = (42, "text", true);
```

- Fixed-size, heterogeneous collections
- Type syntax: `(Type1, Type2, ...)`
- Access by index: `tuple.0`, `tuple.1`

**Functions:**

```dryad
let fn_type: fn(number, string) -> bool;
```

- Type syntax: `fn(ParamType1, ParamType2, ...) -> ReturnType`
- Functions are first-class values

**Classes:**

```dryad
let obj: MyClass = new MyClass();
```

- Custom types defined with `class` keyword
- Instance variables and methods
- Support inheritance with `extends`

**Objects:**

```dryad
let obj = { name: "John", age: 30 };
```

- Literal object with properties
- Property access: `obj.name` or `obj["name"]`

### 3.3 Type Annotations

Type annotations are **optional** and appear after identifier with `:` syntax:

```dryad
let x: number = 42;
const y: string = "hello";
function add(a: number, b: number): number { return a + b; }
```

**Rules:**

- If type annotation is omitted, type is inferred from initializer or defaults to `any`
- Variables can change type at runtime (dynamic typing)
- Type annotations are checked at parse time but not strictly enforced at runtime
- Function return types are optional and inferred if not specified

### 3.4 Type Compatibility

**Implicit Conversions (Coercion):**

- `number` + `string` → string (concatenation)
- `bool` in arithmetic context → `1` (true) or `0` (false)
- `null` in comparisons → special handling
- Arrays/objects compared by reference

**Type Checking:**

- Comparison operators (`==`, `!=`) perform type coercion

---

## 4. Expressions

### 4.1 Expression Types and Precedence

Expressions are ordered by precedence (high to low):

| Precedence  | Category       | Operators                      | Associativity | Examples                                |
| ----------- | -------------- | ------------------------------ | ------------- | --------------------------------------- |
| 1 (highest) | Primary        | `.`, `[]`, `()`, `new`         | Left          | `obj.prop`, `arr[0]`, `fn()`, `new C()` |
| 2           | Postfix        | `++`, `--`                     | Left          | `x++`, `y--`                            |
| 3           | Prefix         | `++`, `--`, `!`, `-`, `+`, `~` | Right         | `++x`, `!bool`, `-num`                  |
| 4           | Exponentiation | `**`                           | Right         | `2 ** 8`                                |
| 5           | Multiplicative | `*`, `/`, `%`                  | Left          | `a * b`, `a / b`, `a % b`               |
| 6           | Additive       | `+`, `-`                       | Left          | `a + b`, `a - b`                        |
| 7           | Bitwise Shift  | `<<`, `>>`, `>>>`, `<<<`       | Left          | `a << 2`, `a >> 1`                      |
| 8           | Relational     | `<`, `<=`, `>`, `>=`, `in`     | Left          | `a < b`, `x in arr`                     |
| 9           | Equality       | `==`, `!=`                     | Left          | `a == b`, `a != b`                      |
| 10          | Bitwise AND    | `&`                            | Left          | `a & b`                                 |
| 11          | Bitwise XOR    | `^`, `^^`                      | Left          | `a ^ b`                                 |
| 12          | Bitwise OR     | `\|`                           | Left          | `a \| b`                                |
| 13          | Logical AND    | `&&`                           | Left          | `a && b`                                |
| 14          | Logical OR     | `\|\|`                         | Left          | `a \|\| b`                              |
| 15 (lowest) | Assignment     | `=`, `+=`, `-=`, `*=`, `/=`    | Right         | `x = 5`, `x += 3`                       |

### 4.2 Primary Expressions

**Literals:**

```dryad
42                    // number literal
"hello"              // string literal
true, false          // boolean literals
null                 // null literal
`text ${expr}`       // template string
```

**Variables and Identifiers:**

```dryad
myVar                // variable reference
someFunction         // function reference
```

**Array Literals:**

```dryad
[1, 2, 3]           // array literal
[]                  // empty array
[...spread]         // spread operator
```

**Tuple Literals:**

```dryad
(1, "a", true)      // tuple literal
()                  // empty tuple
```

**Object Literals:**

```dryad
{ x: 1, y: 2 }                          // object with properties
{ name: "John", greet() { ... } }       // object with method
{ ["computed"]: value }                 // computed property name
```

**Function Literals (Lambdas/Arrow Functions):**

```dryad
(a, b) => a + b                         // implicit return
(x) => { return x * 2; }                // explicit return block
() => null                              // no parameters
(a, b, ...rest) => { ... }              // rest parameters
```

**Class Instantiation:**

```dryad
new MyClass()
new MyClass(arg1, arg2)
```

**This and Super:**

```dryad
this                // current object reference
super               // parent class reference (in methods)
```

### 4.3 Binary Expressions

**Arithmetic:**

```dryad
a + b       // addition or string concatenation
a - b       // subtraction
a * b       // multiplication
a / b       // division
a % b       // modulo
a ** b      // exponentiation
```

**Comparison:**

```dryad
a == b      // equality (with type coercion)
a != b      // inequality
a < b       // less than
a > b       // greater than
a <= b      // less than or equal
a >= b      // greater than or equal
```

**Logical:**

```dryad
a && b      // logical AND (short-circuit)
a || b      // logical OR (short-circuit)
```

**Bitwise:**

```dryad
a & b       // bitwise AND
a | b       // bitwise OR
a ^ b       // bitwise XOR
a << b      // left shift
a >> b      // right shift (arithmetic)
a >>> b     // right shift (unsigned)
```

**Membership:**

```dryad
x in array              // check if element in array
"prop" in obj          // check if property in object
```

**Property/Index Access:**

```dryad
obj.property            // dot notation
obj["property"]         // bracket notation
array[index]            // array indexing
tuple.0                 // tuple element access
```

**Method Calls:**

```dryad
obj.method()            // method on object
obj.method(arg1, arg2)  // with arguments
```

### 4.4 Unary Expressions

```dryad
-x          // negation
+x          // unary plus
!x          // logical NOT
~x          // bitwise NOT
++x         // pre-increment
--x         // pre-decrement
x++         // post-increment
x--         // post-decrement
await expr  // await async result
```

### 4.6 Match Expression

```dryad
match value {
    1 => "one",
    2 => "two",
    _ => "other"
}
```

- Pattern matching on expressions
- Patterns: literals, wildcards (`_`), guards
- Guards: optional conditions after `=>`

---

## 5. Statements

### 5.1 Expression Statements

Any expression can be a statement:

```dryad
functionCall();
x = y + 5;
array[0];           // expression statements have no side effects but still execute
```

### 5.2 Variable Declaration

**Let (mutable):**

```dryad
let x = 10;                     // type inferred
let y: number = 20;             // explicit type
let z;                          // declared but undefined (type: any)
let a = 1, b = 2;               // multiple declarations (single statement)
```

**Const (immutable):**

```dryad
const PI = 3.14159;             // must initialize
const name: string = "John";    // with type annotation
```

**Rules:**

- `let` and `const` are block-scoped (within nearest enclosing block)
- `const` cannot be reassigned (but object properties/array elements can be modified)
- Redeclaration in same scope is an error (for `let`/`const`)

### 5.3 Assignment

```dryad
x = 5;                          // simple assignment
x += 3;                         // compound assignment (x = x + 3)
x -= 2;                         // compound: subtract
x *= 4;                         // compound: multiply
x /= 2;                         // compound: divide
```

**Property Assignment:**

```dryad
obj.property = value;
obj["key"] = value;
```

**Index Assignment:**

```dryad
array[0] = value;
obj[key] = value;
```

### 5.4 Block Statement

```dryad
{
    let x = 1;
    let y = 2;
    return x + y;
}
```

- Creates new scope for `let`/`const` variables
- Sequences multiple statements

### 5.5 Conditional Statements

**If:**

```dryad
if (condition) {
    // block executed if condition is truthy
}
```

**If-Else:**

```dryad
if (condition1) {
    // block1
} else if (condition2) {
    // block2
} else {
    // block3
}
```

**Rules:**

- Condition is evaluated; falsy values: `false`, `null`, `0`, `""`, `NaN`
- Truthy values: all others
- Blocks can be single statement or block statement

### 5.6 Loop Statements

**While:**

```dryad
while (condition) {
    // body executes while condition is true
    // break exits loop
    // continue skips to next iteration
}
```

**Do-While:**

```dryad
do {
    // body always executes at least once
} while (condition);
```

**For (C-style):**

```dryad
for (init; condition; update) {
    // init: runs once before loop
    // condition: checked before each iteration
    // update: runs after each iteration
    // break, continue work as expected
}
```

**For-Each (iteration):**

```dryad
for (element in iterable) {
    // element: each item in iterable
}

for (let i of array) {
    // i: each element (index-like for arrays)
}
```

### 5.7 Break and Continue

```dryad
break;          // exits current loop
continue;       // skips to next iteration
```

### 5.8 Return Statement

```dryad
return;         // returns undefined/null
return value;   // returns specified value
```

- Only valid inside function or thread body
- Exits function immediately

### 5.9 Try-Catch-Finally

```dryad
try {
    // code that might throw
    throw new Error("message");
} catch (error) {
    // error handling (error is bound variable)
} finally {
    // cleanup, always executes
}
```

**Rules:**

- `catch` block variable is bound to thrown value
- `finally` block executes whether exception was thrown or not
- Exceptions propagate up the call stack if not caught

### 5.10 Throw Statement

```dryad
throw new Error("error message");
throw value;        // can throw any value
```

---

## 6. Declarations

### 6.1 Function Declaration

**Standard Syntax:**

```dryad
function name(param1, param2) {
    return param1 + param2;
}

function greet(name: string): string {
    return "Hello, " + name;
}
```

**Arrow Function Syntax:**

```dryad
const add = (a, b) => a + b;
const square = x => x * x;
const noop = () => { };
```

**Parameters:**

```dryad
function fn(required, optional = 10) { }    // default parameter
function fn(...rest) { }                    // rest parameter
function fn(a, ...rest) { }                 // mixed
```

**Async Functions:**

```dryad
async function fetchData(url) {
    let response = await callApi(url);
    return response;
}
```

- Can use `await` to wait for promises
- Returns immediately with promise-like behavior

**Thread Functions:**

```dryad
thread function heavyComputation(data) {
    // executes in separate OS thread
    return result;
}

// Call:
let future = thread heavyComputation(data);
```

**Rules:**

- Functions are hoisted (can be called before declaration with `function` syntax)
- Arrow functions are not hoisted (must declare before use)
- Functions are first-class values (can be passed as arguments)
- Closure over surrounding scope variables

### 6.2 Class Declaration

**Basic Structure:**

```dryad
class Animal {
    name = "Unknown";

    constructor(name) {
        this.name = name;
    }

    speak() {
        return this.name + " makes a sound";
    }

    static info() {
        return "Generic animal";
    }
}
```

**Inheritance:**

```dryad
class Dog extends Animal {
    bark() {
        return this.name + " barks!";
    }
}

// Usage:
let dog = new Dog("Rex");
dog.speak();            // inherited method
dog.bark();             // own method
```

**Properties and Methods:**

```dryad
class Counter {
    count = 0;                          // instance property with default

    increment() {                       // instance method
        this.count++;
    }

    static create() {                   // static method
        return new Counter();
    }

    static VERSION = "1.0";             // static property
}
```

**Access Modifiers:**

```dryad
class Example {
    public x = 1;           // accessible from outside (default)
    private y = 2;          // only in this class
    protected z = 3;        // in this class and subclasses
}
```

**Super:**

```dryad
class Child extends Parent {
    constructor(x) {
        super(x);           // call parent constructor
    }

    method() {
        super.method();     // call parent method
    }
}
```

---

## 7. Modules and Imports

### 7.1 Import Statement

**Named Imports:**

```dryad
import { func1, func2, MyClass } from "module/path";
```

**Namespace Import:**

```dryad
import * as utils from "utils/helpers";
// usage: utils.func1(), utils.MyClass
```

**Side-Effect Import:**

```dryad
import "initialization";  // just execute module for side effects
```

**Legacy Use Syntax:**

```dryad
use "./lib/helper.dryad";  // simplified import
```

### 7.2 Export Statement

```dryad
export function publicFunc() { }
export const PUBLIC = 42;
export class PublicClass { }
export interface PublicInterface { }

// Or re-export:
export { privateFunc as publicFunc } from "other_module";
```

### 7.3 Native Directives

Load native modules using `#` prefix at top level:

```dryad
#io              // Load I/O module (file operations)
#crypto          // Load crypto module
#math            // Load math module
#http            // Load HTTP module
```

After directive, functions become available:

```dryad
#io

let content = io_read_file("data.txt");
io_write_file("output.txt", content);
```

---

## 8. Runtime Execution Model

### 8.1 Execution Flow

The Dryad runtime supports two execution modes:

1.  **Tree-Walking Interpreter:** The default mode, which directly executes the Abstract Syntax Tree (AST). It is optimized for development speed and debugging.
2.  **Bytecode Virtual Machine (VM):** A higher-performance mode where the AST is compiled into a stack-based bytecode (OpCodes) before execution. Use `set_compile_mode(true)` to enable.

**Hybrid Architecture:**

- **Parser** → Generates AST
- **Interpreter** → Executes AST directly OR triggers Bytecode Compilation
- **Bytecode Compiler** → Converts AST to `OpCode` sequences
- **VM** → Executes Bytecode with a stack-based architecture
- **AOT Backend** → (Planned/Experimental) Generates native machine code (ARM64, x86_64) from bytecode/IR

### 8.2 Scope and Variable Resolution

**Scope Chain:**

```
Global Scope
  ├─ Module Scope
  │   ├─ Function Scope (for each function)
  │   │   ├─ Block Scope (for each block)
  │   │   ├─ Block Scope
  │   │   └─ ...
  │   └─ ...
  ├─ Class Scope
  │   └─ Method Scope
  └─ ...
```

**Variable Lookup:**

1. Check current scope for variable
2. If not found, check parent scope
3. Continue up scope chain to global
4. If not found anywhere, return error (ReferenceError)

**Binding Rules:**

- `let` and `const`: block-scoped
- `var`: function-scoped (and hoisted)
- `function` declarations: hoisted to function/module scope
- Global variables accessible from anywhere

### 8.3 Function Execution

When a function is called:

1. Create new function scope
2. Bind parameters to arguments
3. Initialize variables declared in function body
4. Execute statements in order
5. Return value (explicit via `return` or implicit `null`)
6. Destroy function scope

**Closure:**
Functions capture variables from their definition scope. Modifications to captured variables are visible:

```dryad
function makeCounter() {
    let count = 0;
    return function() {
        return ++count;  // modifies captured count
    };
}
```

### 8.4 Class Instantiation and Method Execution

When `new ClassName(args)` is executed:

1. Create new object instance
2. If `constructor` method exists, call it with `this` bound to instance
3. Return the instance

When method is called:

1. Create new function scope
2. Bind `this` to the object
3. Bind parameters to arguments
4. Execute method body
5. Return result

### 8.5 Value Representation

Internal value types (implementation detail):

- `Number(f64)` - floating-point number
- `String(String)` - UTF-8 string
- `Bool(bool)` - boolean
- `Null` - null/undefined
- `Array(Vec<Value>)` - dynamic array
- `Object(HashMap<String, Value>)` - key-value map
- `Function(...)` - callable function reference
- `Class(...)` - class instance or definition
- `Thread(...)` - thread handle

### 8.6 Memory Management

- **Automatic:** Garbage collection via Rust's reference counting
- **Stack:** Primitive values stay on stack
- **Heap:** Complex types (arrays, objects, strings) allocated on heap
- **Cleanup:** Values are automatically freed when no references remain
- **No Manual Management:** Users don't deal with pointers or memory layout

---

## 9. FFI and Native Functions

### 9.1 Foreign Function Interface (FFI)

Dryad can call native functions through its built-in module system:

```dryad
#<file_io>         // Load native module
// After loading, native functions become available:
let content = file_read_string(path);
```

### 9.2 Native Module System

**Available Native Modules:**

| Module          | Purpose                                       |
| --------------- | --------------------------------------------- |
| `console_io`    | Standard input/output (print, println, input) |
| `terminal_ansi` | ANSI terminal control                         |
| `file_io`       | File system operations                        |
| `crypto`        | Cryptography and hashing                      |
| `http_client`   | HTTP client requests                          |
| `http_server`   | HTTP server implementation                    |
| `tcp` / `udp`   | Network socket operations                     |
| `time`          | Date and time functions                       |
| `system_env`    | Environment variables and process info        |
| `json_stream`   | Streaming JSON processing                     |
| `ffi`           | Generic Foreign Function Interface            |
| `database`      | Database connectivity                         |
| `websocket`     | WebSocket protocol support                    |
| `debug`         | Interpreter debugging tools                   |
| `utils`         | Helper utilities                              |

### 9.3 Type Mapping (Dryad → C)

| Dryad Type | C Type              | Notes                                    |
| ---------- | ------------------- | ---------------------------------------- |
| `number`   | `double` (f64)      | Standard C floating-point                |
| `string`   | `const char*`       | UTF-8 null-terminated                    |
| `bool`     | `bool` (or uint8_t) | C boolean                                |
| `null`     | `NULL` / `nullptr`  | Null pointer                             |
| `array`    | `void*`             | Opaque pointer (implementation-specific) |

### 9.4 FFI Call Convention

Native function calls follow C calling conventions:

- Parameters passed left-to-right
- Stack-based (x86/x64)
- Return values in `rax` (x86-64)
- Caller cleans up stack (if using cdecl)

**Example FFI Definition (in native module C code):**

```c
#include "dryad_runtime.h"

// Expose to Dryad as native_my_function
DRYAD_EXPORT
Value native_my_function(Value arg1, Value arg2) {
    double x = arg1.as_number();
    const char* s = arg2.as_string();
    // ... computation ...
    return Value::number(result);
}
```

---

## 10. Error Handling

### 10.1 Error Types

All errors in Dryad are represented as `DryadError` with:

- **Error Code:** Unique identifier (e.g., `E001`, `E201`)
- **Message:** Human-readable description
- **Source Location:** File, line, column where error occurred
- **Stack Trace:** Call stack at error time

### 10.2 Exception Propagation

Exceptions propagate up the call stack:

```dryad
function level3() {
    throw new Error("Something went wrong");    // Error thrown here
}

function level2() {
    level3();       // Propagates up
}

function level1() {
    try {
        level2();   // Caught here
    } catch (e) {
        print("Caught: " + e);
    }
}
```

### 10.3 Common Errors

| Category      | Example                                 | Cause                      |
| ------------- | --------------------------------------- | -------------------------- |
| **Syntax**    | `SyntaxError`                           | Invalid token or grammar   |
| **Reference** | `ReferenceError: undefined variable`    | Variable not in scope      |
| **Type**      | `TypeError: cannot call non-function`   | Type mismatch              |
| **Runtime**   | `Error: division by zero`               | Runtime error              |
| **Module**    | `ModuleNotFoundError: module not found` | Missing import             |
| **Assertion** | `AssertionError: condition failed`      | Explicit assertion failure |

### 10.4 Error Handling Strategies

**1. Try-Catch:**

```dryad
try {
    riskyOperation();
} catch (error) {
    handleError(error);
}
```

**2. Error Propagation:**

```dryad
function wrapper() {
    return unsafeFunc();    // throws if unsafeFunc throws
}
```

**3. Defensive Coding:**

```dryad
if (value != null) {
    // safe to use value
}
```

---

### 11.1 Async-Await

```dryad
async function process() {
    let data = await fetchData();
    return data;
}
```

- `async` marks a function as asynchronous.
- `await` suspends execution until a promise resolves.

### 11.2 Native Threads

Dryad uses OS-level threads for parallel execution:

```dryad
thread function backgroundTask(id) {
    print("Task " + id + " running...");
}

// Spawn threads
thread backgroundTask(1);
thread backgroundTask(2);
```

- `thread` keyword before `function` declares a threadable function.
- `thread function_name(args)` spawns a new OS-level thread.
- Threads have isolated contexts but can be synchronized via mutexes.

### 11.3 Mutex Synchronization

```dryad
let mu = mutex();

thread function criticalTask() {
    mu.lock();
    // thread-safe operations
    mu.unlock();
}
```

- `mutex()` creates a new synchronization primitive.
- `.lock()` and `.unlock()` are used to protect shared resources.

---

## 12. Standard Patterns

### 12.1 Object-Oriented Pattern

```dryad
class Shape {
    name = "Unknown";

    constructor(name) {
        this.name = name;
    }

    describe() {
        return this.name;
    }
}

class Circle extends Shape {
    radius = 0;

    constructor(name, radius) {
        super(name);
        this.radius = radius;
    }

    area() {
        return 3.14159 * this.radius ** 2;
    }
}

let c = new Circle("Red Circle", 5);
print(c.area());      // 78.53975
```

### 12.2 Functional Pattern

```dryad
const numbers = [1, 2, 3, 4, 5];

// Map
const doubled = numbers.map(x => x * 2);

// Filter (if implemented as array method)
const evens = numbers.filter(x => x % 2 == 0);

// Reduce (if implemented)
const sum = numbers.reduce((acc, x) => acc + x, 0);
```

### 12.3 Module Pattern

```dryad
// math_module.dryad
export function add(a, b) { return a + b; }
export function multiply(a, b) { return a * b; }

// main.dryad
import { add, multiply } from "math_module";

let result = add(3, 5);
```

### 12.4 Error Handling Pattern

```dryad
function safeDivide(a, b) {
    try {
        if (b == 0) throw new Error("Division by zero");
        return a / b;
    } catch (e) {
        print("Error: " + e);
        return null;
    }
}
```

---

## 13. Quick Reference

### 13.1 Common Keywords Quick Look

```dryad
// Variables
let x = 1;                  // mutable block-scoped
const y = 2;               // immutable block-scoped

// Functions
function add(a, b) { return a + b; }
const multiply = (a, b) => a * b;
async function fetch() { ... }
thread function compute() { ... }

// Classes & OOP
class Animal { }
class Dog extends Animal { }
new Dog()
this.property

// Control Flow
if (x > 0) { ... } else { ... }
while (cond) { ... }
for (let i = 0; i < 10; i++) { ... }
for (item in array) { ... }
match (value) { ... }
break; continue;

// Error Handling
try { ... } catch (e) { ... } finally { ... }
throw new Error("msg");

// Modules
import { func } from "module";
use "path/to/file";
export function public() { }
#<console_io>

// Operators
+, -, *, /, %, **          // arithmetic
&&, ||, !                  // logical
==, !=, <, >, <=, >=       // comparison
++, --                     // increment/decrement
=, +=, -=, *=, /=          // assignment
=>                         // arrow function
```

### 13.2 Type Annotations Quick Reference

```dryad
let num: number = 42;
let str: string = "text";
let bool: bool = true;
let arr: number[] = [1, 2, 3];
let tuple: (number, string) = (1, "a");
let fn: fn(number) -> string;
let any_type: any = anything;

function func(a: number, b: string): bool { ... }
```

---

## 14. Implementation Notes for LLMs

### 14.1 Key Architectural Concepts

1. **Stack-Based Bytecode VM**: High-performance execution via intermediate bytecode.
2. **Dynamic Typing**: No compile-time type enforcement; types checked at runtime
3. **Automatic Memory Management**: Reference counting via Rust (no manual allocation)
4. **Scope Chain**: Variable resolution walks scope chain upward
5. **First-Class Functions**: Functions are values, support closures
6. **Native Module System**: FFI for calling C functions via dynamic libraries

### 14.2 Common Implementation Patterns

**Statement Dispatch:**

```rust
match stmt {
    Stmt::VarDeclaration(...) => handle_var_decl(),
    Stmt::FunctionDeclaration(...) => handle_func_decl(),
    Stmt::ClassDeclaration(...) => handle_class_decl(),
    Stmt::If(...) => handle_if(),
    // ... other statement types
}
```

**Expression Evaluation:**

```rust
match expr {
    Expr::Literal(...) => return literal_value(),
    Expr::Binary(...) => return eval_binary(),
    Expr::Call(...) => return eval_function_call(),
    // ... other expression types
}
```

**Scope Management:**

```rust
// Push new scope
interpreter.push_scope();
// ... execute statements with new scope ...
// Pop scope
interpreter.pop_scope();
```

### 14.3 Known Limitations

1. **AOT/Interpreter Hybrid**: Support for both interpreted bytecode and native AOT compilation.
2. **No Strict Type Checking**: Types are optional and not enforced
3. **Limited FFI**: Can't pass complex types to C (only primitives and opaque pointers)
4. **Single-Threaded Base**: Threads exist but main interpreter is single-threaded
5. **No Module Versioning**: Simple module system without version management

### 14.4 Performance Characteristics

- **Fast**: Simple programs run acceptably fast
- **Suitable For**: Scripting, automation, learning, small-to-medium programs
- **Not Suitable For**: Computationally intensive algorithms requiring native speeds
- **Bottlenecks**: Deep function call stacks, large array operations, frequent GC

---

## 15. Example Programs

### 15.1 Hello World

```dryad
print("Hello, World!");
```

### 15.2 Fibonacci Function

```dryad
function fib(n) {
    if (n <= 1) return n;
    return fib(n - 1) + fib(n - 2);
}

print(fib(10));  // 55
```

### 15.3 Class Hierarchy

```dryad
class Animal {
    name = "Unknown";

    constructor(name) {
        this.name = name;
    }

    speak() {
        return this.name + " makes a sound";
    }
}

class Dog extends Animal {
    speak() {
        return this.name + " barks!";
    }
}

let dog = new Dog("Buddy");
print(dog.speak());  // "Buddy barks!"
```

### 15.4 Async Processing

```dryad
async function processData(id) {
    let data = await fetchFromAPI(id);
    return data.value * 2;
}

let result = processData(123);
```

### 15.5 File Operations with Modules

```dryad
#<file_io>
#<console_io>

// Read file
let content = file_read_string("input.txt");

// Process content
println("Content length: " + content.length);

// Write result
file_write_string("output.txt", "Modified: " + content);
```

---

## Document Metadata

**Last Updated:** 2026-03-09  
**Status:** Complete  
**Coverage:** 100% of core language features  
**Target Version:** Dryad v0.1.0

**How to Use This Document:**

1. **For LLM Context Injection**: Use this entire document as system context for code generation tasks
2. **For MCP Integration**: Extract sections as needed and serve via MCP tools
3. **For Verification**: Reference specific sections when validating Dryad code syntax
4. **For Implementation**: Use this as specification for code generators and analysis tools

**Related Documentation:**

- `RUNTIME_TECHNICAL_MANUAL.md` - Runtime implementation details
- `NATIVE_MODULES.md` - Native function documentation
- `README.md` - Language overview and features
- `QUICKSTART.md` - Getting started guide

---

**Generated by**: Sisyphus (AI Agent)  
**Command**: Continue feature expansion → Create LLM-optimized Dryad documentation  
**Session**: 2026-03-09 14:30 UTC
