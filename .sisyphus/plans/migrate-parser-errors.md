# Parser Error Catalog Migration - Phase 2 Task 6

> **For Claude (Subagent):** Use AST-aware tools to safely migrate 152 multiline DryadError::new() calls to centralized error_catalog system.

**Goal:** Complete Task 6 (Parser Migration) by migrating all 152 DryadError::new() calls in crates/dryad_parser/src/parser.rs to use error_catalog functions.

**Architecture:** 
- Use ast_grep_replace to safely handle Rust multiline patterns with format! expressions
- Break into 3 chunks (~50 calls each) to avoid tool call limits
- Single-line calls first (safer), then complex multiline patterns
- Import `error_catalog` in parser.rs header and migrate each code systematically

**Status:** 
- ✅ Import already added: line 6 of parser.rs includes error_catalog
- ✅ Manual fixes: 1 of 3 `DryadError::parser()` calls done (line 329 e2003, line 1307 e2012)
- ⏳ Remaining: 150 `DryadError::new()` calls (152 - 2 manual fixes)

**Valid Codes (all exist in error_catalog):**
2005, 2008, 2012, 2013, 2014, 2016, 2017, 2018, 2023, 2024, 2028, 2029, 2030, 2032, 2033, 2035, 2036, 2037, 2038, 2041, 2051, 2053, 2063, 2071, 2073, 2075, 2080, 2081, 2082, 2083, 2085, 2090, 2091, 2092, 2094, 2095, 2098, 2099, 2101, 2102, 2103, 2104, 2107, 2108, 2109, 2110, 2111, 2112, 2116 (was 4002), 2117 (was 1002)

---

## Task 1: Verify Import and Get Code Frequency

**Files:**
- Check: `crates/dryad_parser/src/parser.rs` (lines 1-20)

**Step 1: Verify error_catalog import exists**

Check that line 6 includes `error_catalog`:
```
use dryad_errors::{error_catalog, DryadError, SourceLocation};
```

If missing, add it.

**Step 2: Get frequency distribution of all codes**

Run:
```bash
grep -o 'DryadError::new([0-9]*' crates/dryad_parser/src/parser.rs | grep -o '[0-9]*$' | sort | uniq -c | sort -rn > /tmp/parser_code_freq.txt
cat /tmp/parser_code_freq.txt
```

Expected output: List of error codes with frequency (e.g., "6 4002", "5 2033", etc.)

**Step 3: Identify top 3 codes to migrate first**

From the frequency output, note the top 3-4 codes. These will be your first targets.

---

## Task 2: Migrate Chunk 1 - Top Single-Line Patterns (~50 codes)

**Files:**
- Modify: `crates/dryad_parser/src/parser.rs`

**Strategy:** Use ast_grep_replace to match and replace single-line DryadError::new() patterns.

**AST Pattern (for single-line calls):**
```
DryadError::new($CODE, $MSG)
```

**Step 1: Migrate single-line patterns**

Run:
```bash
cd /home/pedro/repo/source && ast_grep_replace --lang rust \
  --pattern 'DryadError::new($CODE, $MSG)' \
  --rewrite 'DryadError::from_catalog(error_catalog::e$CODE(), self.current_location())' \
  crates/dryad_parser/src/parser.rs \
  --dry-run
```

Review the dry-run output. If safe (shouldn't touch format! expressions), run without --dry-run:

```bash
cd /home/pedro/repo/source && ast_grep_replace --lang rust \
  --pattern 'DryadError::new($CODE, $MSG)' \
  --rewrite 'DryadError::from_catalog(error_catalog::e$CODE(), self.current_location())' \
  crates/dryad_parser/src/parser.rs
```

**Step 2: Build to verify compilation**

```bash
cd /home/pedro/repo/source && cargo build -p dryad_parser 2>&1 | grep -E "(error|warning.*generated|Finished)"
```

Expected: "Finished" (may have warnings, but NO errors)

**Step 3: Count remaining**

```bash
grep -c 'DryadError::new(' crates/dryad_parser/src/parser.rs
```

Expected: Fewer than 152 (record the number)

**Step 4: Commit**

```bash
cd /home/pedro/repo/source && git add crates/dryad_parser/src/parser.rs && git commit -m "refactor: migrate single-line DryadError::new() calls to error_catalog"
```

---

## Task 3: Migrate Chunk 2 - Multiline with Literal Strings (~40 codes)

**Files:**
- Modify: `crates/dryad_parser/src/parser.rs`

**Strategy:** Use ast_grep to find multiline literal patterns, then use Edit tool for safety.

**Step 1: Find sample multiline patterns**

```bash
grep -B1 -A3 'DryadError::new(' /home/pedro/repo/source/crates/dryad_parser/src/parser.rs | head -40
```

Identify patterns that look like:
```rust
DryadError::new(
    2033,
    "message"
)
```

**Step 2: For each identified code, use Edit tool**

Example: If you find code 2033 used 5 times:
- Locate each occurrence by line number
- Use the Edit tool to replace the entire multiline block with one-liner
- Each edit should be: exact oldString → newString

Pattern:
```rust
// Old
return Err(DryadError::new(
    2033,
    "Esperado statement"
));

// New  
return Err(DryadError::from_catalog(error_catalog::e2033(), self.current_location()));
```

**Step 3: Build and verify**

```bash
cd /home/pedro/repo/source && cargo build -p dryad_parser 2>&1 | grep -E "(error|Finished)"
```

**Step 4: Count remaining**

```bash
grep -c 'DryadError::new(' /home/pedro/repo/source/crates/dryad_parser/src/parser.rs
```

**Step 5: Commit after every 10 codes**

```bash
git add crates/dryad_parser/src/parser.rs && git commit -m "refactor: migrate multiline literal DryadError calls (codes XXXX)"
```

---

## Task 4: Migrate Chunk 3 - Multiline with format! Expressions (~40-50 codes)

**Files:**
- Modify: `crates/dryad_parser/src/parser.rs`

**Strategy:** Use Edit tool to carefully replace patterns with format! expressions, preserving the format! call and using from_catalog_fmt.

**Step 1: Find format! patterns**

```bash
grep -B1 -A4 '&format!' /home/pedro/repo/source/crates/dryad_parser/src/parser.rs | head -50
```

Identify patterns like:
```rust
DryadError::new(
    2005,
    &format!("message {}", var)
)
```

**Step 2: For each code with format!, use Edit tool**

Pattern:
```rust
// Old
DryadError::new(
    2005,
    &format!("message {}", var)
)

// New
DryadError::from_catalog_fmt(error_catalog::e2005(), &format!("message {}", var), self.current_location())
```

Important: Preserve the entire format! expression exactly as-is, just wrap with from_catalog_fmt().

**Step 3: Build and verify**

```bash
cd /home/pedro/repo/source && cargo build -p dryad_parser 2>&1 | grep -E "(error|Finished)"
```

**Step 4: Run parser tests**

```bash
cd /home/pedro/repo/source && cargo test -p dryad_parser --lib 2>&1 | tail -5
```

Expected: "test result: ok" (no tests usually, but shouldn't error)

**Step 5: Final count**

```bash
grep -c 'DryadError::new(' /home/pedro/repo/source/crates/dryad_parser/src/parser.rs
```

Expected: **0** (all migrated)

**Step 6: Commit**

```bash
git add crates/dryad_parser/src/parser.rs && git commit -m "refactor: migrate all remaining DryadError::new() calls in parser to error_catalog"
```

---

## Task 5: Verification and Final Build

**Step 1: Full build**

```bash
cd /home/pedro/repo/source && cargo build -p dryad_parser 2>&1 | tail -3
```

Expected: "Finished" with 0 errors

**Step 2: Count from_catalog calls**

```bash
grep -c 'DryadError::from_catalog' /home/pedro/repo/source/crates/dryad_parser/src/parser.rs
```

Expected: ~155+ (all the migrated calls)

**Step 3: Verify no old-style remain**

```bash
grep -c 'DryadError::new(' /home/pedro/repo/source/crates/dryad_parser/src/parser.rs
```

Expected: **0**

**Step 4: Run comprehensive test**

```bash
cd /home/pedro/repo/source && cargo test -p dryad_parser 2>&1 | grep "test result"
```

Expected: Should show test summary (namespace tests may still fail pre-existing, that's OK)

**Step 5: Final commit message**

```bash
git add crates/dryad_parser/src/parser.rs && git commit -m "refactor: complete Task 6 - migrate all 152 parser error calls to centralized error_catalog"
```

---

## Execution Guidance

**If errors occur:**
1. Check the exact line number from the error
2. Use Read to inspect context (20 lines around)
3. Use Edit tool to make targeted fix (safer than bulk replace)
4. Rebuild immediately
5. NEVER revert — just fix forward

**If you hit the 200 tool call limit:**
- You're done with as much as possible
- Report progress: "Migrated X calls, Y remaining"
- Save uncommitted changes and return control

**Success Criteria:**
- ✅ Zero DryadError::new() calls remaining
- ✅ All 155+ calls now use from_catalog or from_catalog_fmt
- ✅ Build completes without errors
- ✅ All error codes map to valid functions in error_catalog
