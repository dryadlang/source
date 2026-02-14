use crate::value::{ClassGetter, ClassMethod, ClassProperty, ClassSetter, ObjectMethod, Value};
use dryad_parser::ast::Expr;
use std::collections::HashMap;

pub type HeapId = usize;

#[derive(Debug, Clone, Default)]
pub struct GcStats {
    pub total_collections: usize,
    pub total_objects_freed: usize,
    pub last_collection_freed: usize,
}

#[derive(Debug, Clone)]
pub enum ManagedObject {
    Array(Vec<Value>),
    Tuple(Vec<Value>),
    Lambda {
        params: Vec<String>,
        body: Expr,
        closure: HashMap<String, Value>,
    },
    Class {
        name: String,
        parent: Option<String>,
        methods: HashMap<String, ClassMethod>,
        properties: HashMap<String, ClassProperty>,
        getters: HashMap<String, ClassGetter>,
        setters: HashMap<String, ClassSetter>,
    },
    Instance {
        class_name: String,
        properties: HashMap<String, Value>,
    },
    Object {
        properties: HashMap<String, Value>,
        methods: HashMap<String, ObjectMethod>,
    },
}

pub struct Heap {
    objects: HashMap<HeapId, (ManagedObject, bool)>, // (objeto, mark_bit)
    next_id: HeapId,
    allocation_count: usize,
    gc_threshold: usize,
    pub gc_stats: GcStats,
}

impl Heap {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
            next_id: 1,
            allocation_count: 0,
            gc_threshold: 1000, // Default: trigger GC every 1000 allocations
            gc_stats: GcStats::default(),
        }
    }

    pub fn set_gc_threshold(&mut self, threshold: usize) {
        self.gc_threshold = threshold;
    }

    pub fn should_collect(&self) -> bool {
        self.allocation_count >= self.gc_threshold
    }

    pub fn heap_size(&self) -> usize {
        self.objects.len()
    }

    pub fn allocate(&mut self, obj: ManagedObject) -> HeapId {
        let id = self.next_id;
        self.next_id += 1;
        self.objects.insert(id, (obj, false));
        self.allocation_count += 1;
        id
    }

    pub fn get(&self, id: HeapId) -> Option<&ManagedObject> {
        self.objects.get(&id).map(|(obj, _)| obj)
    }

    pub fn get_mut(&mut self, id: HeapId) -> Option<&mut ManagedObject> {
        self.objects.get_mut(&id).map(|(obj, _)| obj)
    }

    pub fn collect(&mut self, roots: &[HeapId]) {
        let before_count = self.objects.len();

        // 1. Unmark all
        for (_, mark) in self.objects.values_mut() {
            *mark = false;
        }

        // 2. Mark from roots
        let mut worklist = roots.to_vec();
        while let Some(id) = worklist.pop() {
            if let Some((obj, mark)) = self.objects.get_mut(&id) {
                if !*mark {
                    *mark = true;
                    // Clone the object to avoid borrow checker issues
                    let obj_clone = obj.clone();
                    // Add references from this object to worklist
                    self.trace_object(&obj_clone, &mut worklist);
                }
            }
        }

        // 3. Sweep
        self.objects.retain(|_, (_, mark)| *mark);

        // 4. Update statistics
        let after_count = self.objects.len();
        let freed = before_count.saturating_sub(after_count);

        self.gc_stats.total_collections += 1;
        self.gc_stats.total_objects_freed += freed;
        self.gc_stats.last_collection_freed = freed;
        self.allocation_count = 0; // Reset counter after GC

        #[cfg(debug_assertions)]
        if freed > 0 {
            eprintln!(
                "🗑️  GC: Collected {} objects (heap size: {} → {})",
                freed, before_count, after_count
            );
        }
    }

    fn trace_object(&self, obj: &ManagedObject, worklist: &mut Vec<HeapId>) {
        match obj {
            ManagedObject::Array(elements) | ManagedObject::Tuple(elements) => {
                for val in elements {
                    self.trace_value(val, worklist);
                }
            }
            ManagedObject::Lambda { closure, .. } => {
                for val in closure.values() {
                    self.trace_value(val, worklist);
                }
            }
            ManagedObject::Class { properties, .. } => {
                for prop in properties.values() {
                    if let Some(val) = &prop.default_value {
                        self.trace_value(val, worklist);
                    }
                }
            }
            ManagedObject::Instance { properties, .. } => {
                for val in properties.values() {
                    self.trace_value(val, worklist);
                }
            }
            ManagedObject::Object { properties, .. } => {
                for val in properties.values() {
                    self.trace_value(val, worklist);
                }
            }
        }
    }

    fn trace_value(&self, val: &Value, worklist: &mut Vec<HeapId>) {
        match val {
            Value::Array(id)
            | Value::Tuple(id)
            | Value::Lambda(id)
            | Value::Class(id)
            | Value::Instance(id)
            | Value::Object(id) => {
                worklist.push(*id);
            }
            Value::Promise {
                value: Some(val), ..
            } => {
                self.trace_value(val, worklist);
            }
            _ => {}
        }
    }
}
