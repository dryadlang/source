# Runtime Error Catalog Migration - Phase 2 Task 7 (Completion)

> **For Claude (Subagent):** Complete runtime error catalog migration by migrating remaining 121 DryadError::new() calls.

**Goal:** Finish Task 7 by migrating all remaining `DryadError::new()` calls in crates/dryad_runtime/src/interpreter.rs (121 remaining after initial 44) and crates/dryad_runtime/src/resolver.rs.

**Current Status:**
- ✅ 44 calls migrated (e3100, e3101, e3081)
- ⏳ **121 remaining** in interpreter.rs
- ⏳ **2 remaining** in resolver.rs
- Total: **123 remaining**

**Architecture:**
- Use same bulk Perl + Edit tool approach as parser migration
- Valid codes: e3000, e3002, e3005, e3007-3011, e3015, e3020-3025, e3030, e3035, etc. (all runtime codes 3xxx exist)
- Use `self.current_location()` for location argument (same as parser)
- Handle both single-line and multiline patterns

---

## Task 1: Get Current State and Code Distribution

**Step 1: Verify import exists**

Check that interpreter.rs line 11 includes `error_catalog`:
```bash
head -15 crates/dryad_runtime/src/interpreter.rs | grep error_catalog
```

If missing, add: `use dryad_errors::{error_catalog, DryadError, SourceLocation, ...};`

**Step 2: Get code frequency**

```bash
grep -o 'DryadError::new([0-9]*' crates/dryad_runtime/src/interpreter.rs | grep -o '[0-9]*$' | sort | uniq -c | sort -rn | head -20
```

Record top codes to prioritize.

**Step 3: Count current state**

```bash
echo "interpreter.rs:"
grep -c 'DryadError::new(' crates/dryad_runtime/src/interpreter.rs
echo "resolver.rs:"
grep -c 'DryadError::new(' crates/dryad_runtime/src/resolver.rs
```

Expected: ~121 in interpreter.rs, ~2 in resolver.rs

---

## Task 2: Migrate Interpreter Runtime Errors

**Step 1: Write bulk Perl migration script**

Use Perl to migrate all single-line and multiline patterns simultaneously:

```bash
cat > /tmp/migrate_runtime.pl << 'PERLEOF'
#!/usr/bin/perl
use strict;
use warnings;

my $file = "crates/dryad_runtime/src/interpreter.rs";
open my $fh, '<', $file or die "Cannot open $file: $!";
my $content = do { local $/; <$fh> };
close $fh;

# Pattern 1: Single-line DryadError::new(CODE, "msg")
$content =~ s/DryadError::new\((\d+),\s*"([^"]*)"\)/
    "DryadError::from_catalog(error_catalog::e$1(), self.current_location())"
/ge;

# Pattern 2: Single-line with format!
$content =~ s/DryadError::new\((\d+),\s*(\&format!\([^)]*(?:\([^)]*\)[^)]*)*\))\)/
    "DryadError::from_catalog_fmt(error_catalog::e$1(), $2, self.current_location())"
/ge;

# Pattern 3: Multiline with string (handle trailing comma)
$content =~ s/DryadError::new\(\s*(\d+),\s*"([^"]*)"\s*,?\s*\)/
    "DryadError::from_catalog(error_catalog::e$1(), self.current_location())"
/ge;

# Pattern 4: Multiline with format! (handle trailing comma)
$content =~ s/DryadError::new\(\s*(\d+),\s*(\&format!\([^)]*(?:\([^)]*\)[^)]*)*\))\s*,?\s*\)/
    "DryadError::from_catalog_fmt(error_catalog::e$1(), $2, self.current_location())"
/ge;

open $fh, '>', $file or die "Cannot write $file: $!";
print $fh $content;
close $fh;

print "Migration completed\n";
PERLEOF

cd /home/pedro/repo/source && perl /tmp/migrate_runtime.pl
```

**Step 2: Build to verify**

```bash
cd /home/pedro/repo/source && cargo build -p dryad_runtime 2>&1 | grep -E "(error|Finished)"
```

Expected: "Finished" with 0 errors

**Step 3: Count remaining**

```bash
grep -c 'DryadError::new(' crates/dryad_runtime/src/interpreter.rs
```

Expected: Much fewer (if any)

**Step 4: Commit**

```bash
cd /home/pedro/repo/source && git add crates/dryad_runtime/src/interpreter.rs && git commit -m "refactor: migrate interpreter runtime errors to error_catalog"
```

---

## Task 3: Migrate Resolver Runtime Errors

**Step 1: Check resolver.rs**

```bash
grep 'DryadError::new(' crates/dryad_runtime/src/resolver.rs
```

**Step 2: If any remain, use Edit tool**

For each remaining call, use the Edit tool to replace with from_catalog equivalent. Same pattern as interpreter.

**Step 3: Build and verify**

```bash
cargo build -p dryad_runtime 2>&1 | grep -E "(error|Finished)"
```

**Step 4: Commit**

```bash
git add crates/dryad_runtime/src/resolver.rs && git commit -m "refactor: migrate resolver errors to error_catalog"
```

---

## Task 4: Final Verification and Integration Test

**Step 1: Full build across all crates**

```bash
cd /home/pedro/repo/source && cargo build 2>&1 | grep -E "(error|Finished)" | tail -3
```

Expected: "Finished" with 0 errors

**Step 2: Zero remaining calls**

```bash
echo "Parser:" && grep -c 'DryadError::new(' crates/dryad_parser/src/parser.rs
echo "Runtime (interpreter):" && grep -c 'DryadError::new(' crates/dryad_runtime/src/interpreter.rs
echo "Runtime (resolver):" && grep -c 'DryadError::new(' crates/dryad_runtime/src/resolver.rs
echo "Lexer:" && grep -c 'DryadError::new(' crates/dryad_lexer/src/lexer.rs
```

Expected: All **0** (all migrated across entire codebase)

**Step 3: Count from_catalog usage**

```bash
echo "from_catalog calls:" && grep -r 'DryadError::from_catalog' crates/ | wc -l
```

Expected: 300+ total across all crates

**Step 4: Run comprehensive tests**

```bash
cd /home/pedro/repo/source && cargo test -p dryad_errors -p dryad_lexer 2>&1 | grep "test result"
```

Expected: All tests passing (or pre-existing failures only)

**Step 5: Final commit**

```bash
git add -A && git commit -m "refactor: complete Phase 2 Task 7 - full runtime error catalog migration"
```

---

## Success Criteria

- ✅ Zero `DryadError::new()` calls in interpreter.rs
- ✅ Zero `DryadError::new()` calls in resolver.rs  
- ✅ All 121+ runtime calls now use `from_catalog` or `from_catalog_fmt`
- ✅ Build succeeds with 0 errors
- ✅ Entire codebase (lexer + parser + runtime) migrated
