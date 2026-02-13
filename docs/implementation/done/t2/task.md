# Task: Implement Mark-and-Sweep Garbage Collector

## Research & Planning

- [x] Analyze current Value and memory management in dryad_runtime
- [x] Create implementation plan for GC
- [x] Define `Heap` and `ManagedObject` structures

## Heap Implementation Refactor

- [x] Implement `Heap` to manage allocations
- [x] Update Value to use HeapId for reference types
- [ ] Update evaluation of complex types:
  - [ ] Arrays (`eval_array`)
  - [ ] Tuples (`eval_tuple`)
  - [ ] Lambda expressions
  - [ ] Class declarations
  - [ ] Object literals

## Update Interpreter Logic

- [ ] Update interpreter logic for heap-based values:
  - [ ] Array/Tuple/Object access (`eval_index`, `eval_tuple_access`)
  - [ ] Method calls (`eval_method_call_internal`)
  - [ ] Property access (`eval_property_access`)
  - [ ] Pattern matching (`match_pattern`)
  - [ ] Class instantiation (`eval_class_instantiation_internal`)
  - [ ] Property assignment (`Stmt::PropertyAssignment`)
  - [ ] Index assignment (`Stmt::IndexAssignment`)

## GC Implementation

- [x] Implement Root Tracing (Stack, Globals)
- [x] Implement Mark phase (recursive traversal)
- [x] Implement Sweep phase (reclaiming memory)
- [ ] Implement simple incremental logic (optional/basic)
- [ ] Integrate GC triggers in interpreter

## Testing & Verification

- [ ] Create stress tests with reference cycles
- [ ] Verify memory release via logs/instrumentation
