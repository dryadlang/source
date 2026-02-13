# Mark-and-Sweep Garbage Collector Implementation Plan

## Current State Analysis

The Dryad runtime already has a **functional heap-based memory management system** with Mark-and-Sweep garbage collection infrastructure in place.

### ✅ Already Implemented

#### Heap Infrastructure

- [`Heap`](file:///c:/Users/Pedro%20Jesus/Downloads/source-main/source-main/crates/dryad_runtime/src/heap.rs) struct with allocation and GC methods
- [`ManagedObject`](file:///c:/Users/Pedro%20Jesus/Downloads/source-main/source-main/crates/dryad_runtime/src/heap.rs#L9-L31) enum for heap-allocated types (Array, Tuple, Lambda, Class, Instance, Object)
- [`HeapId`](file:///c:/Users/Pedro%20Jesus/Downloads/source-main/source-main/crates/dryad_runtime/src/heap.rs#L6) type alias for heap references
- [`Value`](file:///c:/Users/Pedro%20Jesus/Downloads/source-main/source-main/crates/dryad_runtime/src/value.rs#L13-L54) enum updated to use `HeapId` for reference types

#### GC Core Functions

- `Heap::collect()` - Main GC entry point with mark-and-sweep phases
- `Heap::trace_object()` - Recursive object traversal for marking
- `Heap::trace_value()` - Value-level tracing for references
- `Interpreter::collect_garbage()` - Triggers GC from interpreter
- `Interpreter::collect_roots()` - Collects all GC roots from interpreter state
- `Interpreter::collect_value_roots()` - Helper for root collection

#### Heap-Based Evaluation Functions

All major evaluation functions are implemented and using heap allocation:

- ✅ `eval_array()` - Creates arrays on heap
- ✅ `eval_tuple()` - Creates tuples on heap
- ✅ `eval_index()` - Accesses array/object elements from heap
- ✅ `eval_tuple_access()` - Accesses tuple elements from heap
- ✅ Lambda expressions - Stored on heap with closures
- ✅ `eval_class_instantiation_internal()` - Creates class instances on heap
- ✅ `eval_object_literal()` - Creates objects on heap
- ✅ `eval_property_access()` - Accesses instance/object properties from heap
- ✅ `eval_method_call_internal()` - Calls methods on heap objects
- ✅ `match_pattern()` - Pattern matching with heap values
- ✅ `execute_index_assignment()` - Modifies heap arrays/objects
- ✅ Property assignment (`Stmt::PropertyAssignment`) - Updates heap objects
- ✅ Class declarations (`Stmt::ClassDeclaration`) - Stores classes on heap

---

## 🔧 Remaining Work

### 1. GC Integration & Triggering

> [!IMPORTANT]
> The GC infrastructure exists but is **not actively triggered** during program execution.

**Tasks:**

- [ ] Add automatic GC triggers based on allocation thresholds
- [ ] Implement allocation counter in `Heap`
- [ ] Add configurable GC trigger points (e.g., every N allocations)
- [ ] Add memory pressure detection (optional)

**Proposed Changes:**

#### [MODIFY] [heap.rs](file:///c:/Users/Pedro%20Jesus/Downloads/source-main/source-main/crates/dryad_runtime/src/heap.rs)

Add allocation tracking and automatic GC triggers:

```rust
pub struct Heap {
    objects: HashMap<HeapId, (ManagedObject, bool)>,
    next_id: HeapId,
    allocation_count: usize,        // NEW: Track allocations
    gc_threshold: usize,             // NEW: Trigger GC after N allocations
    gc_stats: GcStats,               // NEW: Optional statistics
}

pub struct GcStats {
    pub total_collections: usize,
    pub total_objects_freed: usize,
    pub last_collection_freed: usize,
}

impl Heap {
    pub fn new() -&gt; Self {
        Self {
            objects: HashMap::new(),
            next_id: 1,
            allocation_count: 0,
            gc_threshold: 1000,  // Default: GC every 1000 allocations
            gc_stats: GcStats::default(),
        }
    }

    pub fn set_gc_threshold(&mut self, threshold: usize) {
        self.gc_threshold = threshold;
    }

    pub fn allocate(&mut self, obj: ManagedObject) -&gt; HeapId {
        let id = self.next_id;
        self.next_id += 1;
        self.objects.insert(id, (obj, false));

        self.allocation_count += 1;

        // Return ID and let caller decide when to GC
        id
    }

    pub fn should_collect(&self) -&gt; bool {
        self.allocation_count &gt;= self.gc_threshold
    }

    pub fn collect(&mut self, roots: &[HeapId]) {
        let before_count = self.objects.len();

        // ... existing mark-and-sweep logic ...

        let after_count = self.objects.len();
        let freed = before_count - after_count;

        self.gc_stats.total_collections += 1;
        self.gc_stats.total_objects_freed += freed;
        self.gc_stats.last_collection_freed = freed;
        self.allocation_count = 0;  // Reset counter after GC
    }
}
```

#### [MODIFY] [interpreter.rs](file:///c:/Users/Pedro%20Jesus/Downloads/source-main/source-main/crates/dryad_runtime/src/interpreter.rs)

Add automatic GC triggering in the interpreter:

```rust
impl Interpreter {
    // Call this after heap allocations in hot paths
    fn maybe_collect_garbage(&mut self) {
        if self.heap.should_collect() {
            self.collect_garbage();
        }
    }

    // Update eval_array, eval_tuple, etc. to trigger GC
    fn eval_array(&mut self, elements: &[Expr]) -&gt; Result<Value, DryadError> {
        let mut values = Vec::new();
        for element in elements {
            let value = self.evaluate(element)?;
            values.push(value);
        }

        let array_id = self.heap.allocate(ManagedObject::Array(values));
        self.maybe_collect_garbage();  // NEW: Check if GC needed
        Ok(Value::Array(array_id))
    }

    // Similar updates for eval_tuple, eval_object_literal, etc.
}
```

---

### 2. Incremental GC (Optional Enhancement)

> [!NOTE]
> This is an optional enhancement for better performance in long-running programs.

**Current:** GC runs in a single "stop-the-world" pass.

**Goal:** Spread GC work across multiple small increments to reduce pause times.

**Proposed Approach:**

- Track GC state (Idle, Marking, Sweeping)
- Process a fixed number of objects per increment
- Resume GC work on next allocation

**Implementation:**

```rust
enum GcPhase {
    Idle,
    Marking { worklist: Vec<HeapId>, processed: usize },
    Sweeping { iterator_position: usize },
}

pub struct Heap {
    // ... existing fields ...
    gc_phase: GcPhase,
    incremental_work_budget: usize,  // Objects to process per increment
}

impl Heap {
    pub fn incremental_collect(&mut self, roots: &[HeapId]) {
        match &mut self.gc_phase {
            GcPhase::Idle =&gt; {
                // Start marking phase
                self.unmark_all();
                self.gc_phase = GcPhase::Marking {
                    worklist: roots.to_vec(),
                    processed: 0,
                };
            }
            GcPhase::Marking { worklist, processed } =&gt; {
                // Process some objects from worklist
                let budget = self.incremental_work_budget;
                for _ in 0..budget {
                    if let Some(id) = worklist.pop() {
                        self.mark_object(id, worklist);
                        *processed += 1;
                    } else {
                        // Marking complete, start sweeping
                        self.gc_phase = GcPhase::Sweeping { iterator_position: 0 };
                        break;
                    }
                }
            }
            GcPhase::Sweeping { iterator_position } =&gt; {
                // Sweep some objects
                // ... sweep logic ...
                // When done, return to Idle
            }
        }
    }
}
```

---

### 3. Testing & Verification

**Create comprehensive tests to validate GC correctness:**

#### Test Categories

1. **Basic Allocation & Collection**
   - Allocate objects, verify they're freed when unreachable
   - Test with arrays, tuples, objects, lambdas, classes

2. **Reference Cycles**
   - Create circular references (A → B → A)
   - Verify GC can collect cycles when no external references exist

3. **Root Preservation**
   - Verify stack variables are preserved
   - Verify global variables are preserved
   - Verify closure captures are preserved

4. **Stress Tests**
   - Allocate thousands of objects
   - Verify memory is reclaimed
   - Test with nested structures (arrays of arrays, etc.)

5. **Edge Cases**
   - Empty arrays/tuples/objects
   - Self-referential structures
   - Deep nesting

#### [NEW] [tests/gc_tests.rs](file:///c:/Users/Pedro%20Jesus/Downloads/source-main/source-main/crates/dryad_runtime/tests/gc_tests.rs)

```rust
#[cfg(test)]
mod gc_tests {
    use dryad_runtime::Interpreter;
    use dryad_parser::Parser;

    #[test]
    fn test_basic_gc_collection() {
        let mut interpreter = Interpreter::new();

        let code = r#"
            var arr = [1, 2, 3];
            arr = null;  // Make array unreachable
        "#;

        let program = Parser::new(code).parse().unwrap();
        interpreter.execute(&program).unwrap();

        // Trigger GC manually
        interpreter.collect_garbage();

        // Verify array was collected (heap should be empty)
        assert_eq!(interpreter.heap_size(), 0);
    }

    #[test]
    fn test_reference_cycle_collection() {
        let mut interpreter = Interpreter::new();

        let code = r#"
            var obj1 = { ref: null };
            var obj2 = { ref: obj1 };
            obj1.ref = obj2;  // Create cycle

            obj1 = null;
            obj2 = null;  // Make cycle unreachable
        "#;

        let program = Parser::new(code).parse().unwrap();
        interpreter.execute(&program).unwrap();

        interpreter.collect_garbage();

        // Both objects should be collected
        assert_eq!(interpreter.heap_size(), 0);
    }

    #[test]
    fn test_closure_preservation() {
        let mut interpreter = Interpreter::new();

        let code = r#"
            fn makeCounter() {
                var count = [0];  // Array on heap
                return (x) => {
                    count[0] = count[0] + x;
                    return count[0];
                };
            }

            var counter = makeCounter();
            counter(5);  // count array should still be alive
        "#;

        let program = Parser::new(code).parse().unwrap();
        let result = interpreter.execute(&program).unwrap();

        interpreter.collect_garbage();

        // Array should NOT be collected (captured in closure)
        assert!(interpreter.heap_size() > 0);
    }

    #[test]
    fn test_gc_stress() {
        let mut interpreter = Interpreter::new();
        interpreter.heap.set_gc_threshold(100);

        let code = r#"
            for (var i = 0; i < 1000; i = i + 1) {
                var temp = [i, i*2, i*3];
                // temp becomes unreachable each iteration
            }
        "#;

        let program = Parser::new(code).parse().unwrap();
        interpreter.execute(&program).unwrap();

        // GC should have run multiple times
        assert!(interpreter.heap.gc_stats.total_collections > 0);
        assert!(interpreter.heap.gc_stats.total_objects_freed > 900);
    }
}
```

---

## Verification Plan

### Automated Tests

1. Run `cargo test --package dryad_runtime` to verify all tests pass
2. Add GC-specific tests as outlined above
3. Run with different GC thresholds to test various scenarios

### Manual Verification

1. **Memory Instrumentation**: Add logging to track allocations/deallocations

   ```rust
   println!("🗑️  GC: Collected {} objects (heap size: {} → {})",
            freed, before_count, after_count);
   ```

2. **Visual Inspection**: Run sample programs and observe GC behavior

   ```dryad
   #console

   fn createGarbage() {
       for (var i = 0; i < 100; i = i + 1) {
           var temp = [i, [i*2], { value: i*3 }];
       }
   }

   createGarbage();
   print("Done - check GC logs");
   ```

3. **Performance Testing**: Measure execution time with/without GC enabled

---

## Summary

> [!NOTE]
> The GC implementation is **90% complete**. All heap infrastructure and evaluation functions are in place.

**Remaining work:**

1. ✅ **Easy**: Add GC trigger logic (allocation counter + threshold)
2. ✅ **Medium**: Create comprehensive test suite
3. ⚠️ **Optional**: Implement incremental GC for better performance

**Estimated effort:** 2-4 hours for core GC triggering + testing, +2-3 hours for incremental GC if desired.
