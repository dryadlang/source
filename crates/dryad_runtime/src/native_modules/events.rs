use crate::interpreter::Value;
use crate::native_modules::NativeFunction;
use crate::errors::RuntimeError;
use crate::heap::{Heap, ManagedObject};
use std::collections::HashMap;

/// Native Events Module
/// Category: #<events>

pub fn register_events_functions(functions: &mut HashMap<String, NativeFunction>) {
    functions.insert("events_new".to_string(), events_new);
    functions.insert("events_on".to_string(), events_on);
    functions.insert("events_off".to_string(), events_off);
    functions.insert("events_emit".to_string(), events_emit);
}

/// Creates a new NativeEventEmitter instance
/// Returns: Object { _listeners: {} }
fn events_new(_args: &[Value], _manager: &crate::native_modules::NativeModuleManager, heap: &mut Heap) -> Result<Value, RuntimeError> {
    let mut properties = HashMap::new();
    
    // Create the _listeners dictionary (Object)
    let listeners_obj = ManagedObject::Object {
        properties: HashMap::new(),
        methods: HashMap::new(),
    };
    let listeners_id = heap.allocate(listeners_obj);
    
    properties.insert("_listeners".to_string(), Value::Object(listeners_id));
    
    let emitter_obj = ManagedObject::Object {
        properties,
        methods: HashMap::new(),
    };
    let emitter_id = heap.allocate(emitter_obj);
    
    Ok(Value::Object(emitter_id))
}

/// Subscribes a callback to an event
/// Args: emitter, event_name, callback
fn events_on(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, heap: &mut Heap) -> Result<Value, RuntimeError> {
    if args.len() != 3 {
        return Err(RuntimeError::ArgumentError("events_on expects 3 arguments (emitter, event, callback)".to_string()));
    }
    
    let emitter_val = &args[0];
    let event_name = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err(RuntimeError::ArgumentError("Event name must be a string".to_string()))
    };
    let callback = args[2].clone();
    
    // Verify callback is callable
    match &callback {
        Value::Function { .. } | Value::AsyncFunction { .. } | Value::ThreadFunction { .. } | Value::Lambda(_) => {},
        _ => return Err(RuntimeError::ArgumentError("Callback must be a function".to_string()))
    }

    // Get emitter ID
    let emitter_id = match emitter_val {
        Value::Object(id) | Value::Instance(id) => *id,
        _ => return Err(RuntimeError::ArgumentError("Emitter must be an object".to_string()))
    };

    // Get _listeners object
    let listeners_id = if let Some(obj) = heap.get(emitter_id) {
        match obj {
            ManagedObject::Object { properties, .. } | ManagedObject::Instance { properties, .. } => {
                match properties.get("_listeners") {
                    Some(Value::Object(id)) => *id,
                    _ => return Err(RuntimeError::TypeError("Emitter is missing _listeners property".to_string()))
                }
            },
            _ => return Err(RuntimeError::TypeError("Invalid emitter object".to_string()))
        }
    } else {
        return Err(RuntimeError::HeapError("Emitter object not found".to_string()));
    };

    // Get or create listener list for this event
    // Note: We need to do this in steps to avoid borrowing heap mutably twice or holding ref across allocation
    
    let mut current_list = None;
    
    if let Some(listeners_obj) = heap.get(listeners_id) {
        if let ManagedObject::Object { properties, .. } = listeners_obj {
            if let Some(list_val) = properties.get(&event_name) {
                 if let Value::Array(list_id) = list_val {
                     current_list = Some(*list_id);
                 }
            }
        }
    }
    
    let list_id = if let Some(id) = current_list {
        id
    } else {
        // Create new list
        let new_list_id = heap.allocate(ManagedObject::Array(Vec::new()));
        
        // Attach to listeners object
        if let Some(listeners_obj) = heap.get_mut(listeners_id) {
            if let ManagedObject::Object { properties, .. } = listeners_obj {
                properties.insert(event_name.clone(), Value::Array(new_list_id));
            }
        }
        new_list_id
    };
    
    // Add callback to list
    if let Some(list_obj) = heap.get_mut(list_id) {
        if let ManagedObject::Array(elements) = list_obj {
            elements.push(callback);
        }
    }
    
    Ok(Value::Null)
}

/// Unsubscribes a callback
/// Args: emitter, event_name, callback
fn events_off(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, heap: &mut Heap) -> Result<Value, RuntimeError> {
    if args.len() != 3 {
        return Err(RuntimeError::ArgumentError("events_off expects 3 arguments".to_string()));
    }
    
    let event_name = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err(RuntimeError::ArgumentError("Event name must be a string".to_string()))
    };
    // We strictly compare values for removal. For functions/lambdas this means ID comparison.
    // However, function comparison might be tricky if they are distinct values but same logic.
    // For now strict value equality.
    let callback_to_remove = &args[2];

    let emitter_id = match &args[0] {
        Value::Object(id) | Value::Instance(id) => *id,
        _ => return Err(RuntimeError::ArgumentError("Emitter must be an object".to_string()))
    };

    // Find listeners object
    let listeners_id = if let Some(obj) = heap.get(emitter_id) {
        match obj {
            ManagedObject::Object { properties, .. } | ManagedObject::Instance { properties, .. } => {
                 match properties.get("_listeners") {
                    Some(Value::Object(id)) => *id,
                    _ => return Ok(Value::Null) // No listeners, nothing to remove
                }
            },
            _ => return Ok(Value::Null)
        }
    } else {
        return Ok(Value::Null);
    };

    // Find list
    let list_id = if let Some(listeners_obj) = heap.get(listeners_id) {
        if let ManagedObject::Object { properties, .. } = listeners_obj {
             match properties.get(&event_name) {
                 Some(Value::Array(id)) => *id,
                 _ => return Ok(Value::Null)
             }
        } else {
            return Ok(Value::Null);
        }
    } else {
        return Ok(Value::Null);
    };

    // Remove from list
    if let Some(list_obj) = heap.get_mut(list_id) {
        if let ManagedObject::Array(elements) = list_obj {
             // Retain only elements that are NOT equal to callback
             // But Value doesn't implement PartialEq in a simple way for all types?
             // Actually derive(PartialEq) is usually on Value. Let's assume yes.
             // If not, we iterate and check equality manually implementation.
             // Looking at value.rs (implied), it likely derives PartialEq.
             
             // Simple removal:
             // Note: This removes ALL occurrences of this callback
             elements.retain(|e| e != callback_to_remove);
        }
    }

    Ok(Value::Null)
}

/// Emits an event
/// Args: emitter, event_name, data
/// Note: This is synchronous and "dumb". It just returns the list of callbacks 
/// to be executed by the interpreter, OR it executes them if we have access to interpreter?
/// Wait, NativeFunction signature is `fn(&[Value], &NativeModuleManager, &mut Heap) -> Result<Value, RuntimeError>`
/// It DOES NOT have access to Interpreter instance to call functions!
/// 
/// PROBLEM: We cannot call user functions (callbacks) from a NativeFunction because we don't have the Interpreter structure.
/// 
/// SOLUTION: 
/// 1. Change NativeFunction signature to include Interpreter? (Too invasive refactor)
/// 2. Return a special Value indicating "Execute these callbacks"? (Complex)
/// 3. The events_emit function creates a `ManagedObject::Array` containing all callbacks that need to run, 
///    and returns it. Dryad wrapper then iterates and calls them.
///    This delegates the "calling" back to Dryad code, which is slower but safe.
/// 
/// Let's go with solution 3 for now as it fits the architecture without major refactoring.
/// 
/// output: Array<Function> (list of callbacks to run)
fn events_emit(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, heap: &mut Heap) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::ArgumentError("events_emit expects at least 2 arguments (emitter, event)".to_string()));
    }
    
    let event_name = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err(RuntimeError::ArgumentError("Event name must be a string".to_string()))
    };

    let emitter_id = match &args[0] {
        Value::Object(id) | Value::Instance(id) => *id,
        _ => return Err(RuntimeError::ArgumentError("Emitter must be an object".to_string()))
    };

    // Find listeners object
    let listeners_id = if let Some(obj) = heap.get(emitter_id) {
        match obj {
            ManagedObject::Object { properties, .. } | ManagedObject::Instance { properties, .. } => {
                 match properties.get("_listeners") {
                    Some(Value::Object(id)) => *id,
                    _ => return Ok(Value::Array(heap.allocate(ManagedObject::Array(vec![]))))
                }
            },
            _ => return Ok(Value::Array(heap.allocate(ManagedObject::Array(vec![]))))
        }
    } else {
        return Ok(Value::Array(heap.allocate(ManagedObject::Array(vec![]))));
    };

    // Get list
    let callbacks_to_run = if let Some(listeners_obj) = heap.get(listeners_id) {
        if let ManagedObject::Object { properties, .. } = listeners_obj {
             match properties.get(&event_name) {
                 Some(Value::Array(id)) => {
                     if let Some(ManagedObject::Array(list)) = heap.get(*id) {
                         list.clone()
                     } else {
                         Vec::new()
                     }
                 },
                 _ => Vec::new()
             }
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };
    
    // Return array of callbacks. The wrapper must execute them.
    let result_array_id = heap.allocate(ManagedObject::Array(callbacks_to_run));
    Ok(Value::Array(result_array_id))
}
