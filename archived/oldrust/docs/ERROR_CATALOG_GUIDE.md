# Error Catalog Guide — Dryad Language

> **Purpose**: This guide explains how to use, maintain, and extend the centralized error catalog system in Dryad. It's the definitive reference for developers working with errors.  
> **Audience**: All contributors  
> **Last Updated**: 2026-03-21  
> **Status**: Living document

---

## Table of Contents

1. [Overview](#overview)
2. [Error Code Ranges](#error-code-ranges)
3. [The Error Catalog System](#the-error-catalog-system)
4. [Using from_catalog](#using-from_catalog)
5. [Using from_catalog_fmt](#using-from_catalog_fmt)
6. [Adding New Error Codes](#adding-new-error-codes)
7. [Migration Patterns](#migration-patterns)
8. [Error Messages](#error-messages)
9. [Best Practices](#best-practices)
10. [Troubleshooting](#troubleshooting)

---

## Overview

The **error catalog** is a centralized system in `crates/dryad_errors/src/error_catalog.rs` that defines all possible error codes, messages, and suggestions for the Dryad compiler.

### Why Centralized?

✅ **Single source of truth** — All error messages defined once  
✅ **Consistency** — Same message across all components  
✅ **Maintenance** — Update error in one place, affects everywhere  
✅ **Localization** — Easy to translate all messages  
✅ **Type safety** — Compile-time validation of error codes  

### Current Status

- **164 error definitions** across all compiler phases
- **100% English** (Portuguese messages removed)
- **Const functions** for compile-time access
- **Complete** — All error codes in use have catalog entries

---

## Error Code Ranges

Error codes are partitioned by compiler phase:

| Range | Phase | File | Count |
|-------|-------|------|-------|
| **1000-1999** | Lexer (tokenization) | `crates/dryad_lexer/src/lexer.rs` | ~10 codes |
| **2000-2999** | Parser (syntax analysis) | `crates/dryad_parser/src/parser.rs` | ~60 codes |
| **3000-3999** | Runtime (execution) | `crates/dryad_runtime/src/interpreter.rs` | ~90 codes |
| **4000-4999** | Type system | (future) | ~5 codes |
| **5000-5999** | I/O operations | `crates/dryad_runtime/src/native_modules/file_io.rs` | ~20 codes |
| **6000-6999** | Module system | `crates/dryad_runtime/src/native_modules/` | ~5 codes |

**Naming convention:**
- Error code functions are named `eXXXX()` where XXXX is the code
- Examples: `e1001()`, `e2050()`, `e3015()`, `e4001()`

---

## The Error Catalog System

### Structure

Each error in the catalog is defined as a `const fn` returning an `ErrorDef`:

```rust
pub const fn e2050() -> ErrorDef {
    ErrorDef {
        code: 2050,
        category: ErrorCategory::Parser,
        message: "Unexpected end of input while parsing expression",
        suggestion: Some("Check that all opening brackets/parentheses are closed"),
    }
}
```

### ErrorDef Structure

```rust
pub struct ErrorDef {
    pub code: u16,                      // Error code (1000-6999)
    pub category: ErrorCategory,        // Lexer, Parser, Runtime, etc.
    pub message: &'static str,          // Generic error message
    pub suggestion: Option<&'static str>, // Suggested fix or reference
}
```

### ErrorCategory Enum

```rust
pub enum ErrorCategory {
    Lexer,      // Tokenization errors
    Parser,     // Syntax/parsing errors
    Runtime,    // Execution errors
    Type,       // Type checking errors
    Io,         // File I/O errors
    Module,     // Module system errors
    Syntax,     // Syntactic/structural errors
}
```

---

## Using from_catalog

**Use this when the catalog's default message is appropriate.**

### Basic Pattern

```rust
use dryad_errors::{error_catalog, DryadError, SourceLocation};

// Simple case: no custom message needed
return Err(DryadError::from_catalog(
    error_catalog::e2003(),
    SourceLocation::unknown()
));
```

### When Location Information Is Available

```rust
// With file/line/column information
return Err(DryadError::from_catalog(
    error_catalog::e3015(),
    self.current_location()  // or any SourceLocation
));
```

### Real-World Examples

**Parser — Missing semicolon:**
```rust
// OLD (before catalog):
return Err(DryadError::new(2003, "Expected ';' after statement"));

// NEW (with catalog):
return Err(DryadError::from_catalog(
    error_catalog::e2003(),
    self.current_location()
));
```

**Lexer — Invalid escape sequence:**
```rust
return Err(DryadError::from_catalog(
    error_catalog::e1005(),
    SourceLocation::new(
        Some(filename),
        self.line,
        self.column,
        self.position
    )
));
```

---

## Using from_catalog_fmt

**Use this when you need a custom message while keeping the error code structured.**

### Basic Pattern

```rust
// Custom message (preserves exact string from original code)
return Err(DryadError::from_catalog_fmt(
    error_catalog::e3025(),  // Error code definition
    &format!("Property '{}' not found on object", prop_name),  // Custom message
    self.current_location()  // Location info
));
```

### When to Use from_catalog_fmt

1. **Variable interpolation**: Error message needs runtime values
   ```rust
   DryadError::from_catalog_fmt(
       error_catalog::e3005(),
       &format!("Undefined variable: '{}'", var_name),
       location
   )
   ```

2. **Context-specific messages**: Error varies based on situation
   ```rust
   DryadError::from_catalog_fmt(
       error_catalog::e3010(),
       &format!("Function '{}' called with {} args, expected {}", 
           func_name, got, expected),
       location
   )
   ```

3. **Preserving exact original messages**: During migration
   ```rust
   // Preserves the exact message from before catalog migration
   DryadError::from_catalog_fmt(
       error_catalog::e3029(),
       "Setter 'foo' não é acessível (visibilidade: pub)",  // Preserved exact string
       SourceLocation::unknown()
   )
   ```

### Real-World Examples from Interpreter

**Runtime — Undefined variable:**
```rust
// With variable name interpolation
return Err(DryadError::from_catalog_fmt(
    error_catalog::e3005(),
    &format!("Undefined variable: '{}'", identifier),
    self.current_location()
));
```

**Runtime — Function not callable:**
```rust
return Err(DryadError::from_catalog_fmt(
    error_catalog::e3003(),
    &format!("'{}' is not a function", expr_name),
    location
));
```

**Runtime — Array index out of bounds:**
```rust
return Err(DryadError::from_catalog_fmt(
    error_catalog::e3015(),
    &format!("Index {} out of bounds for array of length {}", index, len),
    location
));
```

---

## Adding New Error Codes

### Step 1: Choose the Right Range

Determine the error code based on compiler phase:

| Phase | Range | Choose |
|-------|-------|--------|
| Lexer error | 1000-1999 | Next available (e.g., 1007) |
| Parser error | 2000-2999 | Next available (e.g., 2100) |
| Runtime error | 3000-3999 | Next available (e.g., 3040) |

### Step 2: Add to error_catalog.rs

```rust
// In crates/dryad_errors/src/error_catalog.rs

// =============================================================================
// RUNTIME ERRORS (3000-3999)
// =============================================================================

pub const fn e3040() -> ErrorDef {
    ErrorDef {
        code: 3040,
        category: ErrorCategory::Runtime,
        message: "Recursive function call exceeded maximum depth",
        suggestion: Some("Reduce recursion depth or check for infinite loops"),
    }
}
```

### Step 3: Document in SYNTAX_MANIFEST.md

Add to Appendix B (Error Codes):

```markdown
| 3040 | Runtime | Recursive function call exceeded maximum depth |
```

### Step 4: Use in Code

```rust
use dryad_errors::{error_catalog, DryadError};

// In interpreter.rs or any runtime code
if self.call_depth > MAX_RECURSION_DEPTH {
    return Err(DryadError::from_catalog(
        error_catalog::e3040(),
        self.current_location()
    ));
}
```

### Step 5: Add Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recursion_depth_error() {
        let error = error_catalog::e3040();
        assert_eq!(error.code, 3040);
        assert_eq!(error.category, ErrorCategory::Runtime);
        assert!(error.message.contains("Recursive"));
    }
}
```

### Checklist for New Error Codes

- [ ] Code chosen in correct range (1000-1999, 2000-2999, etc.)
- [ ] ErrorDef created in `error_catalog.rs`
- [ ] Message is clear and actionable
- [ ] Suggestion provided (helps user fix the problem)
- [ ] Code is `pub const fn`
- [ ] Documentation added to SYNTAX_MANIFEST.md
- [ ] Used in at least one place with `from_catalog` or `from_catalog_fmt`
- [ ] Tests verify the error definition exists and has correct values

---

## Migration Patterns

### Pattern 1: Simple String Literal

**Before (old system):**
```rust
DryadError::new(2003, "Expected ';' after statement")
```

**After (with catalog):**
```rust
DryadError::from_catalog(error_catalog::e2003(), self.current_location())
```

### Pattern 2: Format String

**Before:**
```rust
DryadError::new(3005, &format!("Undefined variable: '{}'", var_name))
```

**After:**
```rust
DryadError::from_catalog_fmt(
    error_catalog::e3005(),
    &format!("Undefined variable: '{}'", var_name),
    self.current_location()
)
```

### Pattern 3: Complex Multiline

**Before:**
```rust
return Err(DryadError::new(
    3029,
    &format!("Property '{}' não é acessível (visibilidade: {:?})", 
        property_name, visibility),
));
```

**After:**
```rust
return Err(DryadError::from_catalog_fmt(
    error_catalog::e3029(),
    &format!("Property '{}' não é acessível (visibilidade: {:?})", 
        property_name, visibility),
    SourceLocation::unknown()
));
```

### Pattern 4: When Location Is Available

**Before:**
```rust
return Err(DryadError::new(2050, "Unexpected token"));
```

**After (use actual location):**
```rust
return Err(DryadError::from_catalog(
    error_catalog::e2050(),
    SourceLocation::new(
        Some(self.filename.clone()),
        self.line,
        self.column,
        self.position
    )
));
```

---

## Error Messages

### Writing Good Error Messages

**The formula:**
```
<WHAT> <WHERE/CONTEXT> <WHY/SUGGESTION>
```

### Examples

| Bad | Good |
|-----|------|
| "Error" | "Unexpected end of input while parsing expression" |
| "invalid input" | "Invalid number format: binary numbers must start with 0b" |
| "fail" | "Function 'foo' not defined in this scope" |

### Message Guidelines

1. **Be specific** — Not "error" but "what kind of error"
2. **Use active voice** — "Expected ';'" not "Semicolon expected"
3. **Include context** — Mention variable names, function names, operators
4. **One error per message** — Don't combine multiple issues
5. **English only** — User-facing messages are in Portuguese via stdout formatting, but internal catalog is English

### Suggestions

The `suggestion` field should help users fix the problem:

```rust
// GOOD: Actionable suggestion
suggestion: Some("Add a closing quote to terminate the string")

// BAD: Restates the problem
suggestion: Some("String is not closed")

// GOOD: Points to documentation or rules
suggestion: Some("Valid escape sequences: \\n, \\t, \\r, \\\\, \\\", \\u{XXXX}")

// GOOD: Lists valid values
suggestion: Some("Use 0b for binary, 0o for octal, 0x for hexadecimal")
```

---

## Best Practices

### ✅ DO

1. **Use `from_catalog` when possible** — Keeps error definitions centralized
2. **Provide accurate locations** — Pass `self.current_location()` not `SourceLocation::unknown()`
3. **Use format strings for dynamic content** — Never concatenate strings for user-facing error messages
4. **Add suggestion when you know the fix** — It helps users immediately
5. **Keep error codes consistent** — Once assigned, never change them (external tools depend on codes)
6. **Test error paths** — Unit tests for error conditions matter as much as success paths

### ❌ DON'T

1. **Don't use `DryadError::new()`** — Deprecated, use `from_catalog` instead
2. **Don't modify catalog messages** — They're part of the public API
3. **Don't create new error codes willy-nilly** — Coordinate ranges, avoid conflicts
4. **Don't mix English and Portuguese** — Catalog is English; user output is Portuguese
5. **Don't use generic codes for multiple situations** — Create specific codes when needed
6. **Don't omit `SourceLocation`** — Always provide location, at minimum `SourceLocation::unknown()`

---

## Troubleshooting

### Problem: "Unresolved reference `error_catalog::eXXXX`"

**Cause:** Error code doesn't exist in catalog.

**Solution:** 
1. Check the code number is correct
2. Verify it's defined in `error_catalog.rs`
3. If not, add it (see "Adding New Error Codes")

```rust
// Check what codes are available for runtime (3xxx)
grep "pub const fn e3" crates/dryad_errors/src/error_catalog.rs
```

### Problem: "Mismatched types: expected `SourceLocation`, found `String`"

**Cause:** Using `from_catalog` with wrong argument types.

**Solution:** Pass `SourceLocation`, not a string:

```rust
// WRONG:
DryadError::from_catalog(error_catalog::e3005(), "Location info")

// RIGHT:
DryadError::from_catalog(error_catalog::e3005(), self.current_location())
```

### Problem: Error message shows generic catalog message but I need specific info

**Solution:** Use `from_catalog_fmt` instead:

```rust
// Wrong — shows generic message:
DryadError::from_catalog(error_catalog::e3005(), location)

// Right — includes variable name:
DryadError::from_catalog_fmt(
    error_catalog::e3005(),
    &format!("Undefined variable: '{}'", var_name),
    location
)
```

### Problem: Multiple error codes seem to overlap in meaning

**Cause:** Legacy system had too many codes; consolidation needed.

**Solution:** Contact the maintainers to merge similar codes. The catalog should grow slowly, not rapidly.

---

## Quick Reference Card

### Imports
```rust
use dryad_errors::{error_catalog, DryadError, SourceLocation};
```

### Basic Error (no custom message)
```rust
Err(DryadError::from_catalog(error_catalog::eXXXX(), location))
```

### Custom Message
```rust
Err(DryadError::from_catalog_fmt(
    error_catalog::eXXXX(),
    &format!("Custom: {}", variable),
    location
))
```

### Unknown Location
```rust
Err(DryadError::from_catalog(error_catalog::eXXXX(), SourceLocation::unknown()))
```

### Add New Code
1. Choose range (1xxx, 2xxx, 3xxx, etc.)
2. Add `pub const fn eXXXX() -> ErrorDef { ... }` in error_catalog.rs
3. Use in code with `from_catalog` or `from_catalog_fmt`
4. Document in SYNTAX_MANIFEST.md Appendix B
5. Add test in dryad_errors/src/tests.rs

---

## References

- **Error Definitions**: `crates/dryad_errors/src/error_catalog.rs`
- **Error Types**: `crates/dryad_errors/src/lib.rs` (DryadError enum)
- **Error Documentation**: `SYNTAX_MANIFEST.md` Appendix B
- **Standardization Rules**: `STANDARDIZATION_MANIFEST.md` Section 2

---

**Last Updated**: 2026-03-21  
**Phase 2 Status**: Complete (344 calls migrated to error_catalog)  
**Next**: Phase 3 — Full test suite validation
