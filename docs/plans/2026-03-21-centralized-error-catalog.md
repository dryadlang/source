# Centralized Error Catalog Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Create a single-source-of-truth error catalog file where every Dryad error is defined by code number, with English messages, so changing any error requires editing only one file.

**Architecture:** A new `error_catalog.rs` file in `crates/dryad_errors/src/` defines every error via `const fn` functions returning `ErrorDef` structs. The existing `DryadError` enum gains a `from_catalog()` constructor. All ~350 `DryadError::new(code, "msg")` call sites across lexer, parser, and runtime are migrated to use catalog functions. The `error_urls.rs` and Display impl are updated to pull from catalog data. All messages become English.

**Tech Stack:** Rust (no new dependencies)

---

## Error Code Inventory (Reference)

### Lexer (1001-1006) — 24 call sites in `crates/dryad_lexer/src/lexer.rs`
- 1001: Unexpected character (3 instances + 1 index error)
- 1002: Unclosed string literal (2 instances: string vs template)
- 1003: Unclosed block comment (1 instance)
- 1004: Number format errors (14 instances: binary/octal/hex/decimal)
- 1005: Escape sequence errors (4 instances: unicode/invalid)
- 1006: Native directive errors (3 instances)

### Parser (2005-2115 + misplaced 4002, 1002) — 155 call sites in `crates/dryad_parser/src/parser.rs`
**Duplicates to resolve:**
- 2013: Used for 3 different things (const init, assignment op, function params)
- 2018: Used for 2 things (while close paren, async function name)
- 2051: Used for 2 things (while condition close, else opening brace)
- 2090: Used for 2 things (class name after new, class closing brace)
- 2091: Used for 2 things (async method name, regular method name)
- 2102: Used for 2 things (setter parameter, index bracket)
- 2103: Used for 2 things (setter close paren, index close bracket)
**Category mismatches:**
- 4002: Used 6 times in parser for import errors (should be 2xxx)
- 1002: Used 1 time in parser for template string (should be 2xxx)

### Runtime (3001-3103 + misplaced 4001, 4003) — 175 call sites in `crates/dryad_runtime/src/interpreter.rs` + `resolver.rs`
**Duplicates to resolve:**
- 3029: 5 different property access messages
- 3100: ~15 heap reference errors (all "Heap error: X reference not found")
- 3101: ~15 heap type errors (all "Heap error: Expected X")
- 3005: Used for both arithmetic errors and native function errors
**Category mismatches:**
- 4001: Used in runtime for promise errors (should be 3xxx)
- 4003: Used in runtime (should be 3xxx)

---

## Task 1: Create ErrorCategory Enum and ErrorDef Struct

**Files:**
- Modify: `crates/dryad_errors/src/lib.rs:124-198`
- Create: `crates/dryad_errors/src/error_catalog.rs`

**Step 1: Add ErrorCategory enum and ErrorDef struct to lib.rs**

Add after line 129 (after `WarningSeverity` enum), before `DryadError` enum:

```rust
/// Error category determined by code range
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCategory {
    Lexer,    // 1000-1999
    Parser,   // 2000-2999
    Runtime,  // 3000-3999
    Type,     // 4000-4999
    Io,       // 5000-5999
    Module,   // 6000-6999
    Syntax,   // 7000-7999
    Warning,  // 8000-8999
    System,   // 9000-9999
}

/// Static error definition from the catalog
#[derive(Debug, Clone, Copy)]
pub struct ErrorDef {
    pub code: u16,
    pub category: ErrorCategory,
    pub message: &'static str,
    pub suggestion: Option<&'static str>,
}
```

**Step 2: Add `from_catalog` and `from_catalog_fmt` constructors to DryadError impl**

Add to the `impl DryadError` block (after the existing `new` method):

```rust
/// Create error from catalog definition (uses catalog message)
pub fn from_catalog(def: ErrorDef, location: SourceLocation) -> Self {
    Self::from_catalog_fmt(def, def.message, location)
}

/// Create error from catalog definition with custom message (for parameterized errors)
pub fn from_catalog_fmt(def: ErrorDef, message: &str, location: SourceLocation) -> Self {
    match def.category {
        ErrorCategory::Lexer => DryadError::Lexer {
            code: def.code,
            message: message.into(),
            location,
            debug_context: None,
        },
        ErrorCategory::Parser => DryadError::Parser {
            code: def.code,
            message: message.into(),
            location,
            expected: Vec::new(),
            found: String::new(),
            debug_context: None,
        },
        ErrorCategory::Runtime => DryadError::Runtime {
            code: def.code,
            message: message.into(),
            location,
            stack_trace: StackTrace::new(),
            debug_context: None,
        },
        ErrorCategory::Type => DryadError::Type {
            code: def.code,
            message: message.into(),
            location,
            expected_type: "unknown".into(),
            found_type: "unknown".into(),
            debug_context: None,
        },
        ErrorCategory::Io => DryadError::Io {
            code: def.code,
            message: message.into(),
            location,
            operation: "unknown".into(),
            path: None,
            debug_context: None,
        },
        ErrorCategory::Module => DryadError::Module {
            code: def.code,
            message: message.into(),
            location,
            module_name: "unknown".into(),
            debug_context: None,
        },
        ErrorCategory::Syntax => DryadError::Syntax {
            code: def.code,
            message: message.into(),
            location,
            syntax_help: None,
            debug_context: None,
        },
        ErrorCategory::Warning => DryadError::Warning {
            code: def.code,
            message: message.into(),
            location,
            severity: WarningSeverity::Medium,
            debug_context: None,
        },
        ErrorCategory::System => DryadError::System {
            code: def.code,
            message: message.into(),
            location,
            system_info: None,
            debug_context: None,
        },
    }
}
```

**Step 3: Create empty error_catalog.rs with module declaration**

Create `crates/dryad_errors/src/error_catalog.rs`:

```rust
// crates/dryad_errors/src/error_catalog.rs
//
// Centralized Error Catalog — Single Source of Truth
//
// Every Dryad error is defined here by its code number.
// To change any error message, edit ONLY this file.
//
// Naming convention: e{CODE}() returns ErrorDef for that code.
// Example: e1001() → Unexpected character error

use crate::{ErrorCategory, ErrorDef};

// =============================================================================
// LEXER ERRORS (1000-1999)
// =============================================================================

// Placeholder — errors will be added in Task 2
```

Add `pub mod error_catalog;` to lib.rs after `pub mod error_urls;` (line 7).

**Step 4: Run tests to verify compilation**

Run: `cargo test -p dryad_errors --no-run`
Expected: Compiles with no errors

**Step 5: Run existing tests**

Run: `cargo test -p dryad_errors`
Expected: All existing tests pass (no behavior changes yet)

**Step 6: Commit**

```bash
git add crates/dryad_errors/src/lib.rs crates/dryad_errors/src/error_catalog.rs
git commit -m "feat(errors): add ErrorCategory, ErrorDef structs and from_catalog constructors"
```

---

## Task 2: Populate Lexer Error Catalog (1001-1006)

**Files:**
- Modify: `crates/dryad_errors/src/error_catalog.rs`

**Step 1: Add all lexer error definitions**

Replace the placeholder in error_catalog.rs with:

```rust
// =============================================================================
// LEXER ERRORS (1000-1999)
// =============================================================================

/// E1001: Unexpected character encountered during tokenization
pub const fn e1001() -> ErrorDef {
    ErrorDef {
        code: 1001,
        category: ErrorCategory::Lexer,
        message: "Unexpected character",
        suggestion: Some("Use only letters, numbers, underscores, and valid operators"),
    }
}

/// E1002: String literal was not properly closed
pub const fn e1002() -> ErrorDef {
    ErrorDef {
        code: 1002,
        category: ErrorCategory::Lexer,
        message: "Unclosed string literal",
        suggestion: Some("Add a closing quote to terminate the string"),
    }
}

/// E1003: Block comment was not properly closed
pub const fn e1003() -> ErrorDef {
    ErrorDef {
        code: 1003,
        category: ErrorCategory::Lexer,
        message: "Unclosed block comment",
        suggestion: Some("Add */ to close the block comment"),
    }
}

/// E1004: Invalid number format (binary, octal, hex, or decimal)
pub const fn e1004() -> ErrorDef {
    ErrorDef {
        code: 1004,
        category: ErrorCategory::Lexer,
        message: "Invalid number format",
        suggestion: Some("Check number format: 0b for binary, 0o for octal, 0x for hex"),
    }
}

/// E1005: Invalid escape sequence in string
pub const fn e1005() -> ErrorDef {
    ErrorDef {
        code: 1005,
        category: ErrorCategory::Lexer,
        message: "Invalid escape sequence",
        suggestion: Some("Valid sequences: \\n, \\t, \\r, \\\\, \\\", \\', \\u{XXXX}"),
    }
}

/// E1006: Invalid native directive format
pub const fn e1006() -> ErrorDef {
    ErrorDef {
        code: 1006,
        category: ErrorCategory::Lexer,
        message: "Invalid native directive",
        suggestion: Some("Use format: #<module_name> with alphanumeric characters and underscores"),
    }
}
```

**Step 2: Verify compilation**

Run: `cargo test -p dryad_errors --no-run`
Expected: Compiles

**Step 3: Commit**

```bash
git add crates/dryad_errors/src/error_catalog.rs
git commit -m "feat(errors): populate lexer error catalog (E1001-E1006) with English messages"
```

---

## Task 3: Populate Parser Error Catalog (2005-2115) — Deduplicate Codes

**Files:**
- Modify: `crates/dryad_errors/src/error_catalog.rs`

This task resolves ALL duplicate codes by assigning new unique codes.

**Step 1: Add all parser error definitions**

Append to error_catalog.rs. Key deduplication decisions:

| Old Code | Old Usage | New Code | New Message |
|----------|-----------|----------|-------------|
| 2013 (const init) | Keep | 2013 | "Constant must have an initial value" |
| 2013 (assignment op) | Reassign | 2042 | "Invalid assignment operator" |
| 2013 (function params) | Reassign | 2043 | "Expected '(' after function name" |
| 2018 (while paren) | Keep | 2018 | "Expected ')' after function arguments" |
| 2018 (async fn name) | Reassign | 2044 | "Expected async function name" |
| 2051 (while condition) | Keep | 2051 | "Expected ')' after while condition" |
| 2051 (else brace) | Reassign | 2045 | "Expected '{' after 'else'" |
| 2090 (class name) | Keep | 2090 | "Expected class name after 'new'" |
| 2090 (class close) | Reassign | 2046 | "Expected '}' to close class body" |
| 2091 (async method) | Keep | 2091 | "Expected async method name" |
| 2091 (regular method) | Reassign | 2047 | "Expected method name" |
| 2102 (setter param) | Keep | 2102 | "Expected parameter in setter" |
| 2102 (index bracket) | Reassign | 2048 | "Expected '[' after identifier" |
| 2103 (setter paren) | Keep | 2103 | "Expected ')' in setter" |
| 2103 (index bracket) | Reassign | 2049 | "Expected ']' after index" |
| 4002 (import errors) | Reassign | 2116 | Various import-related messages |
| 1002 (template string) | Reassign | 2117 | "Unclosed template string" |

```rust
// =============================================================================
// PARSER ERRORS (2000-2999)
// =============================================================================

/// E2005: Expected closing parenthesis
pub const fn e2005() -> ErrorDef {
    ErrorDef { code: 2005, category: ErrorCategory::Parser, message: "Expected ')' after expression", suggestion: Some("Check that all parentheses are balanced") }
}

/// E2008: Invalid assignment target
pub const fn e2008() -> ErrorDef {
    ErrorDef { code: 2008, category: ErrorCategory::Parser, message: "Invalid assignment target", suggestion: Some("Left side of assignment must be a variable or property") }
}

/// E2012: Expected function name
pub const fn e2012() -> ErrorDef {
    ErrorDef { code: 2012, category: ErrorCategory::Parser, message: "Expected function name", suggestion: Some("Provide a name after 'function' keyword") }
}

/// E2013: Constant must have an initial value
pub const fn e2013() -> ErrorDef {
    ErrorDef { code: 2013, category: ErrorCategory::Parser, message: "Constant must have an initial value", suggestion: Some("Use: const name = value;") }
}

/// E2014: Expected parameter name
pub const fn e2014() -> ErrorDef {
    ErrorDef { code: 2014, category: ErrorCategory::Parser, message: "Expected parameter name", suggestion: Some("Parameter names must be valid identifiers") }
}

/// E2016: Expected ')' after parameters
pub const fn e2016() -> ErrorDef {
    ErrorDef { code: 2016, category: ErrorCategory::Parser, message: "Expected ')' after parameters", suggestion: Some("Close the parameter list with ')'") }
}

/// E2017: Expected 'function' after 'async'
pub const fn e2017() -> ErrorDef {
    ErrorDef { code: 2017, category: ErrorCategory::Parser, message: "Expected 'function' after 'async'", suggestion: Some("Use: async function name() { }") }
}

/// E2018: Expected ')' after function arguments
pub const fn e2018() -> ErrorDef {
    ErrorDef { code: 2018, category: ErrorCategory::Parser, message: "Expected ')' after function arguments", suggestion: Some("Close the argument list with ')'") }
}

/// E2023: Expected 'function' after 'thread'
pub const fn e2023() -> ErrorDef {
    ErrorDef { code: 2023, category: ErrorCategory::Parser, message: "Expected 'function' after 'thread'", suggestion: Some("Use: thread function name() { }") }
}

/// E2024: Expected thread function name
pub const fn e2024() -> ErrorDef {
    ErrorDef { code: 2024, category: ErrorCategory::Parser, message: "Expected thread function name", suggestion: Some("Provide a name after 'thread function'") }
}

/// E2028: Unexpected token in template string
pub const fn e2028() -> ErrorDef {
    ErrorDef { code: 2028, category: ErrorCategory::Parser, message: "Unexpected token in template string", suggestion: Some("Check template string interpolation syntax: ${expr}") }
}

/// E2029: Expected ')' after 'mutex('
pub const fn e2029() -> ErrorDef {
    ErrorDef { code: 2029, category: ErrorCategory::Parser, message: "Expected ')' after 'mutex('", suggestion: Some("Close the mutex expression with ')'") }
}

/// E2030: Expected '(' after 'mutex'
pub const fn e2030() -> ErrorDef {
    ErrorDef { code: 2030, category: ErrorCategory::Parser, message: "Expected '(' after 'mutex'", suggestion: Some("Use: mutex(expression)") }
}

/// E2032: Expected '(' after 'thread'
pub const fn e2032() -> ErrorDef {
    ErrorDef { code: 2032, category: ErrorCategory::Parser, message: "Expected '(' after 'thread'", suggestion: Some("Use: thread function name() { }") }
}

/// E2033: Expected statement
pub const fn e2033() -> ErrorDef {
    ErrorDef { code: 2033, category: ErrorCategory::Parser, message: "Expected statement", suggestion: Some("Provide a valid statement or expression") }
}

/// E2035: Expected '=>' after match pattern
pub const fn e2035() -> ErrorDef {
    ErrorDef { code: 2035, category: ErrorCategory::Parser, message: "Expected '=>' after match pattern", suggestion: Some("Use: pattern => expression") }
}

/// E2036: Expected '}' to close match block
pub const fn e2036() -> ErrorDef {
    ErrorDef { code: 2036, category: ErrorCategory::Parser, message: "Expected '}' to close match block", suggestion: Some("Add '}' to close the match expression") }
}

/// E2037: Expected ']' after array patterns
pub const fn e2037() -> ErrorDef {
    ErrorDef { code: 2037, category: ErrorCategory::Parser, message: "Expected ']' after array patterns", suggestion: Some("Close the array pattern with ']'") }
}

/// E2038: Expected ')' after tuple patterns
pub const fn e2038() -> ErrorDef {
    ErrorDef { code: 2038, category: ErrorCategory::Parser, message: "Expected ')' after tuple patterns", suggestion: Some("Close the tuple pattern with ')'") }
}

/// E2041: Expected '}' after object patterns
pub const fn e2041() -> ErrorDef {
    ErrorDef { code: 2041, category: ErrorCategory::Parser, message: "Expected '}' after object patterns", suggestion: Some("Close the object pattern with '}'") }
}

/// E2042: Invalid assignment operator (was duplicate 2013)
pub const fn e2042() -> ErrorDef {
    ErrorDef { code: 2042, category: ErrorCategory::Parser, message: "Invalid assignment operator", suggestion: Some("Valid operators: =, +=, -=, *=, /=, %=, **=, &&=, ||=") }
}

/// E2043: Expected '(' after function name (was duplicate 2013)
pub const fn e2043() -> ErrorDef {
    ErrorDef { code: 2043, category: ErrorCategory::Parser, message: "Expected '(' after function name", suggestion: Some("Function declaration requires parameter list: function name() { }") }
}

/// E2044: Expected async function name (was duplicate 2018)
pub const fn e2044() -> ErrorDef {
    ErrorDef { code: 2044, category: ErrorCategory::Parser, message: "Expected async function name", suggestion: Some("Provide a name: async function name() { }") }
}

/// E2045: Expected '{' after 'else' (was duplicate 2051)
pub const fn e2045() -> ErrorDef {
    ErrorDef { code: 2045, category: ErrorCategory::Parser, message: "Expected '{' after 'else'", suggestion: Some("Use: else { ... } or else if (condition) { ... }") }
}

/// E2046: Expected '}' to close class body (was duplicate 2090)
pub const fn e2046() -> ErrorDef {
    ErrorDef { code: 2046, category: ErrorCategory::Parser, message: "Expected '}' to close class body", suggestion: Some("Add '}' to close the class declaration") }
}

/// E2047: Expected method name (was duplicate 2091)
pub const fn e2047() -> ErrorDef {
    ErrorDef { code: 2047, category: ErrorCategory::Parser, message: "Expected method name", suggestion: Some("Provide a method name identifier") }
}

/// E2048: Expected '[' after identifier (was duplicate 2102)
pub const fn e2048() -> ErrorDef {
    ErrorDef { code: 2048, category: ErrorCategory::Parser, message: "Expected '[' after identifier", suggestion: Some("Use bracket notation: identifier[index]") }
}

/// E2049: Expected ']' after index (was duplicate 2103)
pub const fn e2049() -> ErrorDef {
    ErrorDef { code: 2049, category: ErrorCategory::Parser, message: "Expected ']' after index", suggestion: Some("Close the index access with ']'") }
}

/// E2051: Expected ')' after while condition
pub const fn e2051() -> ErrorDef {
    ErrorDef { code: 2051, category: ErrorCategory::Parser, message: "Expected ')' after while condition", suggestion: Some("Close the while condition: while (condition) { }") }
}

/// E2053: Expected '{' after 'do'
pub const fn e2053() -> ErrorDef {
    ErrorDef { code: 2053, category: ErrorCategory::Parser, message: "Expected '{' after 'do'", suggestion: Some("Use: do { ... } while (condition);") }
}

/// E2063: Expected '{' after for parentheses
pub const fn e2063() -> ErrorDef {
    ErrorDef { code: 2063, category: ErrorCategory::Parser, message: "Expected '{' after for parentheses", suggestion: Some("Add '{' to start the for loop body") }
}

/// E2071: Expected ']' after array index
pub const fn e2071() -> ErrorDef {
    ErrorDef { code: 2071, category: ErrorCategory::Parser, message: "Expected ']' after array index", suggestion: Some("Close the array access with ']'") }
}

/// E2073: Expected ',' or ')' in method argument list
pub const fn e2073() -> ErrorDef {
    ErrorDef { code: 2073, category: ErrorCategory::Parser, message: "Expected ',' or ')' in method argument list", suggestion: Some("Separate arguments with ',' or close with ')'") }
}

/// E2075: Expected ')' after parameters
pub const fn e2075() -> ErrorDef {
    ErrorDef { code: 2075, category: ErrorCategory::Parser, message: "Expected ')' after parameters", suggestion: Some("Close the parameter list with ')'") }
}

/// E2080: Expected '{' after 'try'
pub const fn e2080() -> ErrorDef {
    ErrorDef { code: 2080, category: ErrorCategory::Parser, message: "Expected '{' after 'try'", suggestion: Some("Use: try { ... } catch (e) { ... }") }
}

/// E2081: Expected '(' after 'catch'
pub const fn e2081() -> ErrorDef {
    ErrorDef { code: 2081, category: ErrorCategory::Parser, message: "Expected '(' after 'catch'", suggestion: Some("Use: catch (error) { ... }") }
}

/// E2082: Expected ')' after arguments
pub const fn e2082() -> ErrorDef {
    ErrorDef { code: 2082, category: ErrorCategory::Parser, message: "Expected ')' after arguments", suggestion: Some("Close the argument list with ')'") }
}

/// E2083: Expected ')' after catch variable
pub const fn e2083() -> ErrorDef {
    ErrorDef { code: 2083, category: ErrorCategory::Parser, message: "Expected ')' after catch variable", suggestion: Some("Close catch: catch (error) { ... }") }
}

/// E2085: Expected '{' after 'finally'
pub const fn e2085() -> ErrorDef {
    ErrorDef { code: 2085, category: ErrorCategory::Parser, message: "Expected '{' after 'finally'", suggestion: Some("Use: finally { ... }") }
}

/// E2090: Expected class name after 'new'
pub const fn e2090() -> ErrorDef {
    ErrorDef { code: 2090, category: ErrorCategory::Parser, message: "Expected class name after 'new'", suggestion: Some("Use: new ClassName()") }
}

/// E2091: Expected async method name
pub const fn e2091() -> ErrorDef {
    ErrorDef { code: 2091, category: ErrorCategory::Parser, message: "Expected async method name", suggestion: Some("Provide a name for the async method") }
}

/// E2092: Expected '(' after method name
pub const fn e2092() -> ErrorDef {
    ErrorDef { code: 2092, category: ErrorCategory::Parser, message: "Expected '(' after method name", suggestion: Some("Method declaration requires parameter list") }
}

/// E2094: Expected parameter name
pub const fn e2094() -> ErrorDef {
    ErrorDef { code: 2094, category: ErrorCategory::Parser, message: "Expected parameter name", suggestion: Some("Parameters must be valid identifiers") }
}

/// E2095: Expected property name
pub const fn e2095() -> ErrorDef {
    ErrorDef { code: 2095, category: ErrorCategory::Parser, message: "Expected property name", suggestion: Some("Property names must be valid identifiers") }
}

/// E2098: Expected getter name
pub const fn e2098() -> ErrorDef {
    ErrorDef { code: 2098, category: ErrorCategory::Parser, message: "Expected getter name", suggestion: Some("Use: get propertyName() { ... }") }
}

/// E2099: Expected ')' in getter
pub const fn e2099() -> ErrorDef {
    ErrorDef { code: 2099, category: ErrorCategory::Parser, message: "Expected ')' in getter", suggestion: Some("Getter takes no parameters: get name() { }") }
}

/// E2100: Expected setter name
pub const fn e2100() -> ErrorDef {
    ErrorDef { code: 2100, category: ErrorCategory::Parser, message: "Expected setter name", suggestion: Some("Use: set propertyName(value) { ... }") }
}

/// E2101: Expected '(' after setter name
pub const fn e2101() -> ErrorDef {
    ErrorDef { code: 2101, category: ErrorCategory::Parser, message: "Expected '(' after setter name", suggestion: Some("Setter requires a parameter: set name(value) { }") }
}

/// E2102: Expected parameter in setter
pub const fn e2102() -> ErrorDef {
    ErrorDef { code: 2102, category: ErrorCategory::Parser, message: "Expected parameter in setter", suggestion: Some("Setter must have exactly one parameter") }
}

/// E2103: Expected ')' in setter
pub const fn e2103() -> ErrorDef {
    ErrorDef { code: 2103, category: ErrorCategory::Parser, message: "Expected ')' in setter", suggestion: Some("Close setter parameter list with ')'") }
}

/// E2104: Expected '=' for index assignment
pub const fn e2104() -> ErrorDef {
    ErrorDef { code: 2104, category: ErrorCategory::Parser, message: "Expected '=' for index assignment", suggestion: Some("Use: array[index] = value") }
}

/// E2107: Expected '}' to close interface
pub const fn e2107() -> ErrorDef {
    ErrorDef { code: 2107, category: ErrorCategory::Parser, message: "Expected '}' to close interface", suggestion: Some("Add '}' to close the interface declaration") }
}

/// E2108: Expected 'function' in interface
pub const fn e2108() -> ErrorDef {
    ErrorDef { code: 2108, category: ErrorCategory::Parser, message: "Expected 'function' in interface", suggestion: Some("Interface methods: function name(params);") }
}

/// E2109: Expected method name in interface
pub const fn e2109() -> ErrorDef {
    ErrorDef { code: 2109, category: ErrorCategory::Parser, message: "Expected method name in interface", suggestion: Some("Provide a name for the interface method") }
}

/// E2110: Expected '(' after method name in interface
pub const fn e2110() -> ErrorDef {
    ErrorDef { code: 2110, category: ErrorCategory::Parser, message: "Expected '(' after method name in interface", suggestion: Some("Interface methods require parameter list") }
}

/// E2111: Expected parameter name in interface method
pub const fn e2111() -> ErrorDef {
    ErrorDef { code: 2111, category: ErrorCategory::Parser, message: "Expected parameter name in interface method", suggestion: Some("Parameters must be valid identifiers") }
}

/// E2112: Expected ')' after interface method parameters
pub const fn e2112() -> ErrorDef {
    ErrorDef { code: 2112, category: ErrorCategory::Parser, message: "Expected ')' after interface method parameters", suggestion: Some("Close parameter list with ')'") }
}

/// E2115: Expected '}' to close namespace
pub const fn e2115() -> ErrorDef {
    ErrorDef { code: 2115, category: ErrorCategory::Parser, message: "Expected '}' to close namespace", suggestion: Some("Add '}' to close the namespace block") }
}

/// E2116: Import statement error (was misplaced 4002)
pub const fn e2116() -> ErrorDef {
    ErrorDef { code: 2116, category: ErrorCategory::Parser, message: "Invalid import syntax", suggestion: Some("Use: import { name } from 'module' or import * as name from 'module'") }
}

/// E2117: Unclosed template string (was misplaced 1002)
pub const fn e2117() -> ErrorDef {
    ErrorDef { code: 2117, category: ErrorCategory::Parser, message: "Unclosed template string", suggestion: Some("Add closing backtick to terminate template string") }
}
```

NOTE: There are ~40 more parser error codes used in the codebase that appear only once (e.g., 2006, 2007, 2009, 2010, 2011, etc.). The implementing agent MUST scan `parser.rs` for ALL unique error codes and add each one to the catalog. The ones listed above are the known duplicates and key errors — the agent must add the rest following the same pattern.

**Step 2: Verify compilation**

Run: `cargo test -p dryad_errors --no-run`
Expected: Compiles

**Step 3: Commit**

```bash
git add crates/dryad_errors/src/error_catalog.rs
git commit -m "feat(errors): populate parser error catalog (E2005-E2117) with deduplication"
```

---

## Task 4: Populate Runtime Error Catalog (3001-3103) — Deduplicate Codes

**Files:**
- Modify: `crates/dryad_errors/src/error_catalog.rs`

**Step 1: Add all runtime error definitions**

Key deduplication decisions:

| Old Code | Issue | Resolution |
|----------|-------|------------|
| 3100 (15 uses) | All "Heap error: X ref not found" | Keep 3100 as generic heap ref error |
| 3101 (15 uses) | All "Heap error: Expected X" | Keep 3101 as generic heap type error |
| 3029 (5 uses) | Different property access msgs | Keep 3029 as generic property access error |
| 3005 (dual use) | Arithmetic + native fn error | Keep 3005=arithmetic, new 3104=native fn error |
| 4001 (runtime) | Promise error in wrong category | Move to 3105 |
| 4003 (runtime) | Runtime error in wrong category | Move to 3106 |

```rust
// =============================================================================
// RUNTIME ERRORS (3000-3999)
// =============================================================================

/// E3001: Undefined variable
pub const fn e3001() -> ErrorDef {
    ErrorDef { code: 3001, category: ErrorCategory::Runtime, message: "Undefined variable", suggestion: Some("Declare the variable with 'let' or 'const' before using it") }
}

/// E3003: Expression is not a function
pub const fn e3003() -> ErrorDef {
    ErrorDef { code: 3003, category: ErrorCategory::Runtime, message: "Expression is not a function", suggestion: Some("Ensure the value is callable before invoking it") }
}

/// E3004: Cannot determine base directory
pub const fn e3004() -> ErrorDef {
    ErrorDef { code: 3004, category: ErrorCategory::Runtime, message: "Cannot determine base directory", suggestion: Some("Check that the file path is valid") }
}

/// E3005: Invalid arithmetic operation
pub const fn e3005() -> ErrorDef {
    ErrorDef { code: 3005, category: ErrorCategory::Runtime, message: "Invalid arithmetic operation", suggestion: Some("Use compatible types: numbers with numbers") }
}

/// E3007: Division by zero
pub const fn e3007() -> ErrorDef {
    ErrorDef { code: 3007, category: ErrorCategory::Runtime, message: "Division by zero", suggestion: Some("Check that the divisor is not zero before dividing") }
}

/// E3008: Module resolution error
pub const fn e3008() -> ErrorDef {
    ErrorDef { code: 3008, category: ErrorCategory::Runtime, message: "Module resolution error", suggestion: Some("Check that the module path exists and is accessible") }
}

/// E3009: Invalid comparison
pub const fn e3009() -> ErrorDef {
    ErrorDef { code: 3009, category: ErrorCategory::Runtime, message: "Invalid comparison", suggestion: Some("Comparison is only valid for numbers") }
}

/// E3010: Break outside loop
pub const fn e3010() -> ErrorDef {
    ErrorDef { code: 3010, category: ErrorCategory::Runtime, message: "break", suggestion: None }
}

/// E3011: Continue outside loop
pub const fn e3011() -> ErrorDef {
    ErrorDef { code: 3011, category: ErrorCategory::Runtime, message: "continue", suggestion: None }
}

/// E3015: Division by zero in modulo operator
pub const fn e3015() -> ErrorDef {
    ErrorDef { code: 3015, category: ErrorCategory::Runtime, message: "Division by zero in modulo operator", suggestion: Some("Check that the divisor is not zero") }
}

/// E3020: Exception thrown
pub const fn e3020() -> ErrorDef {
    ErrorDef { code: 3020, category: ErrorCategory::Runtime, message: "Exception thrown", suggestion: Some("Use try/catch to handle exceptions") }
}

/// E3021: Return pending (internal control flow)
pub const fn e3021() -> ErrorDef {
    ErrorDef { code: 3021, category: ErrorCategory::Runtime, message: "RETURN_PENDING", suggestion: None }
}

/// E3022: Invalid 'this' context
pub const fn e3022() -> ErrorDef {
    ErrorDef { code: 3022, category: ErrorCategory::Runtime, message: "Invalid 'this' context", suggestion: Some("Use 'this' only inside class methods") }
}

/// E3023: 'super' not yet implemented
pub const fn e3023() -> ErrorDef {
    ErrorDef { code: 3023, category: ErrorCategory::Runtime, message: "'super' is not yet implemented", suggestion: Some("Use direct method calls as a workaround") }
}

/// E3025: Callback required
pub const fn e3025() -> ErrorDef {
    ErrorDef { code: 3025, category: ErrorCategory::Runtime, message: "Callback required", suggestion: Some("Provide a function as callback argument") }
}

/// E3029: Property access error
pub const fn e3029() -> ErrorDef {
    ErrorDef { code: 3029, category: ErrorCategory::Runtime, message: "Property not found", suggestion: Some("Check that the property exists on the object") }
}

/// E3030: Static property not found
pub const fn e3030() -> ErrorDef {
    ErrorDef { code: 3030, category: ErrorCategory::Runtime, message: "Static property not found", suggestion: Some("Check that the static property is defined in the class") }
}

/// E3034: Invalid property assignment
pub const fn e3034() -> ErrorDef {
    ErrorDef { code: 3034, category: ErrorCategory::Runtime, message: "Invalid property assignment", suggestion: Some("Use properties only on class instances") }
}

/// E3040: Stack overflow
pub const fn e3040() -> ErrorDef {
    ErrorDef { code: 3040, category: ErrorCategory::Runtime, message: "Stack overflow", suggestion: Some("Check for infinite recursion; consider converting to iteration") }
}

/// E3081: Array index must be a number
pub const fn e3081() -> ErrorDef {
    ErrorDef { code: 3081, category: ErrorCategory::Runtime, message: "Array index must be a number", suggestion: Some("Use numeric indices for array access") }
}

/// E3100: Heap reference not found
pub const fn e3100() -> ErrorDef {
    ErrorDef { code: 3100, category: ErrorCategory::Runtime, message: "Heap reference not found", suggestion: Some("The referenced value may have been garbage collected or is invalid") }
}

/// E3101: Heap type mismatch
pub const fn e3101() -> ErrorDef {
    ErrorDef { code: 3101, category: ErrorCategory::Runtime, message: "Heap type mismatch", suggestion: Some("The heap value is not the expected type") }
}

/// E3104: Native function error (was duplicate 3005)
pub const fn e3104() -> ErrorDef {
    ErrorDef { code: 3104, category: ErrorCategory::Runtime, message: "Native function error", suggestion: Some("Check the arguments passed to the native function") }
}

/// E3105: Promise not resolved (was misplaced 4001)
pub const fn e3105() -> ErrorDef {
    ErrorDef { code: 3105, category: ErrorCategory::Runtime, message: "Promise not resolved", suggestion: Some("Use 'await' to wait for the promise to resolve") }
}

/// E3106: Runtime type error (was misplaced 4003)
pub const fn e3106() -> ErrorDef {
    ErrorDef { code: 3106, category: ErrorCategory::Runtime, message: "Runtime type error", suggestion: Some("Check that the value is the expected type") }
}
```

NOTE: Same as Task 3 — the implementing agent MUST scan `interpreter.rs` and `resolver.rs` for ALL unique error codes and add every one to the catalog. The codes above cover the key ones. Add any remaining codes (3002, 3006, 3012-3019, 3031-3036, 3080-3085, etc.) following the same pattern.

**Step 2: Verify compilation**

Run: `cargo test -p dryad_errors --no-run`
Expected: Compiles

**Step 3: Commit**

```bash
git add crates/dryad_errors/src/error_catalog.rs
git commit -m "feat(errors): populate runtime error catalog (E3001-E3106) with deduplication"
```

---

## Task 5: Migrate Lexer Call Sites (24 instances)

**Files:**
- Modify: `crates/dryad_lexer/src/lexer.rs` (24 `DryadError::new()` calls)

**Step 1: Add use statement to lexer.rs**

Add at the top of the file, with existing imports:

```rust
use dryad_errors::error_catalog;
```

**Step 2: Replace all 24 DryadError::new() calls**

For each call site, replace the pattern:
```rust
// OLD:
DryadError::new(1001, "Caracter inesperado...")
// NEW (static message):
DryadError::from_catalog(error_catalog::e1001(), SourceLocation::unknown())
// NEW (parameterized message):
DryadError::from_catalog_fmt(error_catalog::e1004(), &format!("Invalid binary digit: '{}'", ch), SourceLocation::unknown())
```

Every hardcoded Portuguese message string is replaced. The catalog's English message is used for simple cases; `from_catalog_fmt` is used when runtime context (character, value, etc.) is needed.

**Step 3: Run lexer tests**

Run: `cargo test -p dryad_lexer`
Expected: All 233 tests pass

**Step 4: Commit**

```bash
git add crates/dryad_lexer/src/lexer.rs
git commit -m "refactor(lexer): migrate all error calls to centralized catalog with English messages"
```

---

## Task 6: Migrate Parser Call Sites (~155 instances)

**Files:**
- Modify: `crates/dryad_parser/src/parser.rs` (~155 `DryadError::new()` calls)

**Step 1: Add use statement**

```rust
use dryad_errors::error_catalog;
```

**Step 2: Replace ALL DryadError::new() calls with catalog equivalents**

This is the largest migration. Key changes:
- All Portuguese messages → English via catalog
- Duplicate codes → new unique codes per Task 3's deduplication table
- Misplaced 4002 codes → e2116()
- Misplaced 1002 code → e2117()

Example transformations:
```rust
// OLD (duplicate 2013, assignment):
DryadError::new(2013, "Operador de assignment inválido")
// NEW (deduplicated to 2042):
DryadError::from_catalog(error_catalog::e2042(), SourceLocation::unknown())

// OLD (misplaced 4002):
DryadError::new(4002, "Esperado 'from' no import")
// NEW (moved to 2116):
DryadError::from_catalog_fmt(error_catalog::e2116(), "Expected 'from' in import", SourceLocation::unknown())

// OLD (misplaced 1002):
DryadError::new(1002, "Template string não fechada")
// NEW (moved to 2117):
DryadError::from_catalog(error_catalog::e2117(), SourceLocation::unknown())
```

**Step 3: Run parser tests**

Run: `cargo test -p dryad_parser`
Expected: All 256 tests pass. Some tests may check exact error codes — update those tests to use the new deduplicated codes.

**Step 4: If tests fail, fix test expectations**

Tests that assert specific error codes for the old duplicate codes (2013, 2018, 2051, 2090, 2091, 2102, 2103, 4002, 1002) need to be updated to expect the new codes.

**Step 5: Commit**

```bash
git add crates/dryad_parser/src/parser.rs crates/dryad_parser/tests/
git commit -m "refactor(parser): migrate all error calls to centralized catalog, deduplicate codes, English messages"
```

---

## Task 7: Migrate Runtime Call Sites (~175 instances)

**Files:**
- Modify: `crates/dryad_runtime/src/interpreter.rs` (~173 `DryadError::new()` + `DryadError::runtime()` + `DryadError::Runtime { }` calls)
- Modify: `crates/dryad_runtime/src/resolver.rs` (2 `DryadError::new()` calls)

**Step 1: Add use statements**

```rust
use dryad_errors::error_catalog;
```

**Step 2: Replace ALL DryadError calls with catalog equivalents**

Key changes:
- All Portuguese messages → English via catalog
- 4001 → e3105() (promise errors)
- 4003 → e3106() (runtime type errors)
- Separate 3005 native fn errors → e3104()
- All "Heap error" instances use e3100() or e3101() with from_catalog_fmt for context

For `DryadError::Runtime { ... }` struct literals (lines 192, 229, 236, 502, 917), convert to use `from_catalog` or `from_catalog_fmt`.

**Step 3: Run runtime tests**

Run: `cargo test -p dryad_runtime`
Expected: Tests pass. Some tests may check exact error codes or messages — update those.

**Step 4: Commit**

```bash
git add crates/dryad_runtime/src/interpreter.rs crates/dryad_runtime/src/resolver.rs
git commit -m "refactor(runtime): migrate all error calls to centralized catalog, fix category mismatches, English messages"
```

---

## Task 8: Update error_urls.rs to Use Catalog

**Files:**
- Modify: `crates/dryad_errors/src/error_urls.rs`

**Step 1: Migrate all Portuguese suggestions to English**

Replace all suggestion strings with English equivalents. The error_catalog already has suggestions, but error_urls.rs provides more detailed per-code suggestions.

Translate all strings in `get_error_suggestions()` from Portuguese to English.

Update `get_error_documentation_url()` to include new codes: 2042-2049, 2116-2117, 3104-3106.

**Step 2: Run tests**

Run: `cargo test -p dryad_errors`
Expected: All pass. Tests checking Portuguese strings in `test_auto_context_generation` and `test_different_error_codes_auto_context` need to be updated to English.

**Step 3: Commit**

```bash
git add crates/dryad_errors/src/error_urls.rs
git commit -m "refactor(errors): migrate error_urls.rs suggestions to English, add new deduplicated codes"
```

---

## Task 9: Update Display Impl to English

**Files:**
- Modify: `crates/dryad_errors/src/lib.rs` (Display impl, lines 200-276, and helper functions lines 278-347)

**Step 1: Translate all Portuguese strings in Display impl and helpers**

Key translations:
- "Erro Léxico" → "Lexer Error"
- "Erro Sintático" → "Parser Error"
- "Erro de Runtime" → "Runtime Error"
- "Erro de Tipo" → "Type Error"
- "Erro de I/O" → "I/O Error"
- "Erro de Módulo" → "Module Error"
- "Erro de Sintaxe" → "Syntax Error"
- "Aviso" → "Warning"
- "Erro de Sistema" → "System Error"
- "Local:" → "Location:"
- "Esperado:" → "Expected:"
- "Encontrado:" → "Found:"
- "Tipo esperado:" → "Expected type:"
- "Tipo encontrado:" → "Found type:"
- "Operação:" → "Operation:"
- "Arquivo:" → "File:"
- "Módulo:" → "Module:"
- "Dica:" → "Hint:"
- "Sistema:" → "System:"
- "linha" → "line"
- "coluna" → "column"
- "Código:" → "Code:"
- "Variáveis locais:" → "Local variables:"
- "Sugestões:" → "Suggestions:"
- "Documentação:" → "Documentation:"
- "Código relacionado:" → "Related code:"
- Portuguese comments → English comments

**Step 2: Run tests**

Run: `cargo test -p dryad_errors`
Expected: Tests that check display output (test_error_display_formatting) need to be updated from "E1001: Erro Léxico" to "E1001: Lexer Error".

**Step 3: Commit**

```bash
git add crates/dryad_errors/src/lib.rs
git commit -m "refactor(errors): migrate Display impl and helpers to English"
```

---

## Task 10: Update Tests to English

**Files:**
- Modify: `crates/dryad_errors/src/tests.rs`

**Step 1: Update all Portuguese test strings to English**

- test_debug_context_creation: "Verifique se a variável está declarada" → English
- test_lexer_error_creation: "Caracter inválido '@'" → "Unexpected character '@'"
- test_parser_error_with_expected_tokens: "Token inesperado" → "Unexpected token"
- test_runtime_error_with_stack_trace: "Divisão por zero" → "Division by zero"
- test_error_with_debug_context: Portuguese strings → English
- test_backward_compatibility: Portuguese string → English
- test_error_display_formatting: Update assertions for English display strings
- test_auto_context_generation: Update for English suggestions
- test_different_error_codes_auto_context: Update for English strings
- test_warning_severity: "Variável não utilizada" → "Unused variable"

**Step 2: Run all tests**

Run: `cargo test -p dryad_errors`
Expected: All tests pass

**Step 3: Commit**

```bash
git add crates/dryad_errors/src/tests.rs
git commit -m "refactor(errors): migrate all error tests to English"
```

---

## Task 11: Translate Portuguese Comments to English

**Files:**
- Modify: `crates/dryad_errors/src/lib.rs` — Portuguese code comments and doc comments

**Step 1: Translate all Portuguese comments**

- Line 9: "Informações de localização no código fonte" → "Source code location information"
- Line 16: "Linha do código fonte para contexto" → "Source code line for context"
- Line 46: "Frame do stack trace" → "Stack trace frame"
- Line 51: "Contexto adicional" → "Additional context"
- Line 69: "Stack trace completo" → "Complete stack trace"
- Line 89: "Informações de contexto para debug" → "Debug context information"
- Line 92-95: Variable comments → English
- Line 143: "Tokens esperados" → "Expected tokens"
- Line 144: "Token encontrado" → "Found token"
- Line 166: "read", "write", "open", etc. → keep (already English)
- Line 289: "Mostrar ponteiro visual para o erro" → "Show visual pointer to the error"
- Line 352: "Métodos de construção simplificados" → "Simplified construction methods"
- Line 424: "Métodos específicos para criação" → "Specific methods for error creation"
- Line 519: "Adiciona contexto de debug" → "Add debug context"
- Line 537: "Adiciona automaticamente sugestões" → "Automatically add suggestions"

**Step 2: Run tests**

Run: `cargo test -p dryad_errors`
Expected: All pass (comments don't affect behavior)

**Step 3: Commit**

```bash
git add crates/dryad_errors/src/lib.rs
git commit -m "refactor(errors): translate all Portuguese comments to English in error crate"
```

---

## Task 12: Full Integration Test

**Files:** None modified — verification only

**Step 1: Run all tests across all crates**

Run: `cargo test -p dryad_errors && cargo test -p dryad_lexer && cargo test -p dryad_parser && cargo test -p dryad_runtime`
Expected: All tests pass

**Step 2: Run full workspace build**

Run: `cargo build`
Expected: Compiles (note: oak crate may have pre-existing errors — ignore those)

**Step 3: Verify no Portuguese remains in error messages**

Run: `grep -rn "Esperado\|Erro de\|não\|inválido\|Caracter\|Variável\|número\|Diretiva" crates/dryad_errors/src/ crates/dryad_lexer/src/ crates/dryad_parser/src/ crates/dryad_runtime/src/`
Expected: No matches (all Portuguese migrated to English)

**Step 4: Commit (if any fixes were needed)**

```bash
git add -A
git commit -m "fix(errors): resolve any remaining integration issues from error catalog migration"
```
