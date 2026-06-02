use crate::errors::RuntimeError;
use crate::heap::{Heap, HeapId, ManagedObject};
use crate::interpreter::Value;
use crate::native_modules::NativeFunction;
use serde_json::Value as JsonValue;
use std::collections::HashMap;

pub fn register_json_stream_functions(functions: &mut HashMap<String, NativeFunction>) {
    functions.insert("json_parse_incremental".to_string(), json_parse_incremental);
    functions.insert("json_parse_stream".to_string(), json_parse_stream);
    functions.insert("json_create_parser".to_string(), json_create_parser);
    functions.insert("json_parser_feed".to_string(), json_parser_feed);
    functions.insert("json_parser_done".to_string(), json_parser_done);
    functions.insert("json_encoder_create".to_string(), json_encoder_create);
    functions.insert("json_encoder_encode".to_string(), json_encoder_encode);
}

struct JsonParserState {
    buffer: String,
    position: usize,
    done: bool,
}

fn json_parse_incremental(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    _heap: &mut Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError(
            "json_parse_incremental: esperado 1 argumento".to_string(),
        ));
    }

    let json_string = match &args[0] {
        Value::String(s) => s,
        _ => {
            return Err(RuntimeError::TypeError(
                "json_parse_incremental: argumento deve ser string".to_string(),
            ))
        }
    };

    match serde_json::from_str::<JsonValue>(json_string) {
        Ok(json_value) => Ok(json_value_to_runtime_value(&json_value, _heap)),
        Err(e) => Err(RuntimeError::IoError(format!(
            "Erro ao analisar JSON: {}",
            e
        ))),
    }
}

fn json_parse_stream(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    _heap: &mut Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError(
            "json_parse_stream: esperado 1 argumento".to_string(),
        ));
    }

    let chunks = match &args[0] {
        Value::Array(id) => {
            let obj = _heap
                .get(*id)
                .ok_or_else(|| RuntimeError::HeapError("Array reference not found".to_string()))?;
            if let ManagedObject::Array(arr) = obj {
                arr.clone()
            } else {
                return Err(RuntimeError::TypeError(
                    "json_parse_stream: argumento deve ser array de strings".to_string(),
                ));
            }
        }
        _ => {
            return Err(RuntimeError::TypeError(
                "json_parse_stream: argumento deve ser array".to_string(),
            ))
        }
    };

    let mut combined = String::new();

    for chunk in chunks {
        match chunk {
            Value::String(s) => combined.push_str(&s),
            _ => {
                return Err(RuntimeError::TypeError(
                    "json_parse_stream: array deve conter apenas strings".to_string(),
                ))
            }
        }
    }

    match serde_json::from_str::<JsonValue>(&combined) {
        Ok(json_value) => Ok(json_value_to_runtime_value(&json_value, _heap)),
        Err(e) => Err(RuntimeError::IoError(format!(
            "Erro ao analisar JSON stream: {}",
            e
        ))),
    }
}

fn json_create_parser(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    heap: &mut Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 0 {
        return Err(RuntimeError::ArgumentError(
            "json_create_parser: esperado 0 argumentos".to_string(),
        ));
    }

    let parser_state = JsonParserState {
        buffer: String::new(),
        position: 0,
        done: false,
    };

    let id = heap.allocate(ManagedObject::Object {
        properties: {
            let mut map = HashMap::new();
            map.insert(
                "_type".to_string(),
                Value::String("json_parser".to_string()),
            );
            map
        },
        methods: HashMap::new(),
    });

    Ok(Value::Object(id))
}

fn json_parser_feed(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    _heap: &mut Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::ArgumentError(
            "json_parser_feed: esperado 2 argumentos".to_string(),
        ));
    }

    let parser_id = match &args[0] {
        Value::Object(id) => *id,
        _ => {
            return Err(RuntimeError::TypeError(
                "json_parser_feed: primeiro argumento deve ser objeto parser".to_string(),
            ))
        }
    };

    let chunk = match &args[1] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(RuntimeError::TypeError(
                "json_parser_feed: segundo argumento deve ser string".to_string(),
            ))
        }
    };

    let parser_obj = _heap
        .get(parser_id)
        .ok_or_else(|| RuntimeError::HeapError("Parser not found".to_string()))?;

    if let ManagedObject::Object { properties, .. } = parser_obj {
        let buffer_key = "_buffer".to_string();
        if let Some(Value::String(buffer)) = properties.get(&buffer_key) {
            let new_buffer = format!("{}{}", buffer, chunk);

            // Try to parse incrementally
            match serde_json::from_str::<JsonValue>(&new_buffer) {
                Ok(json_value) => {
                    return Ok(json_value_to_runtime_value(&json_value, _heap));
                }
                Err(_) => {
                    // Not complete yet, continue buffering
                    return Ok(Value::Null);
                }
            }
        }
    }

    Ok(Value::Null)
}

fn json_parser_done(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    _heap: &mut Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError(
            "json_parser_done: esperado 1 argumento".to_string(),
        ));
    }

    let _parser_id = match &args[0] {
        Value::Object(id) => *id,
        _ => {
            return Err(RuntimeError::TypeError(
                "json_parser_done: argumento deve ser objeto parser".to_string(),
            ))
        }
    };

    Ok(Value::Bool(true))
}

fn json_encoder_create(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    heap: &mut Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 0 {
        return Err(RuntimeError::ArgumentError(
            "json_encoder_create: esperado 0 argumentos".to_string(),
        ));
    }

    let id = heap.allocate(ManagedObject::Object {
        properties: {
            let mut map = HashMap::new();
            map.insert(
                "_type".to_string(),
                Value::String("json_encoder".to_string()),
            );
            map.insert("pretty".to_string(), Value::Bool(false));
            map
        },
        methods: HashMap::new(),
    });

    Ok(Value::Object(id))
}

fn json_encoder_encode(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    _heap: &mut Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::ArgumentError(
            "json_encoder_encode: esperado 2 argumentos".to_string(),
        ));
    }

    let encoder_id = match &args[0] {
        Value::Object(id) => *id,
        _ => {
            return Err(RuntimeError::TypeError(
                "json_encoder_encode: primeiro argumento deve ser objeto encoder".to_string(),
            ))
        }
    };

    let encoder_obj = _heap
        .get(encoder_id)
        .ok_or_else(|| RuntimeError::HeapError("Encoder not found".to_string()))?;

    let pretty = if let ManagedObject::Object { properties, .. } = encoder_obj {
        properties
            .get("pretty")
            .map(|v| matches!(v, Value::Bool(true)))
            .unwrap_or(false)
    } else {
        false
    };

    let json_value = runtime_value_to_json(&args[1], _heap)?;

    let json_string = if pretty {
        serde_json::to_string_pretty(&json_value)
    } else {
        serde_json::to_string(&json_value)
    }
    .map_err(|e| RuntimeError::IoError(format!("Erro ao codificar JSON: {}", e)))?;

    Ok(Value::String(json_string))
}

fn json_value_to_runtime_value(value: &JsonValue, heap: &mut Heap) -> Value {
    match value {
        JsonValue::Null => Value::Null,
        JsonValue::Bool(b) => Value::Bool(*b),
        JsonValue::Number(n) => Value::Number(n.as_f64().unwrap_or(0.0)),
        JsonValue::String(s) => Value::String(s.clone()),
        JsonValue::Array(arr) => {
            let runtime_array: Vec<Value> = arr
                .iter()
                .map(|v| json_value_to_runtime_value(v, heap))
                .collect();
            let id = heap.allocate(ManagedObject::Array(runtime_array));
            Value::Array(id)
        }
        JsonValue::Object(obj) => {
            let mut runtime_obj = HashMap::new();
            for (key, val) in obj {
                runtime_obj.insert(key.clone(), json_value_to_runtime_value(val, heap));
            }
            let id = heap.allocate(ManagedObject::Object {
                properties: runtime_obj,
                methods: HashMap::new(),
            });
            Value::Object(id)
        }
    }
}

fn runtime_value_to_json(value: &Value, heap: &Heap) -> Result<JsonValue, RuntimeError> {
    match value {
        Value::Null => Ok(JsonValue::Null),
        Value::Bool(b) => Ok(JsonValue::Bool(*b)),
        Value::Number(n) => Ok(JsonValue::Number(
            serde_json::Number::from_f64(*n).unwrap_or(serde_json::Number::from(0)),
        )),
        Value::String(s) => Ok(JsonValue::String(s.clone())),
        Value::Array(id) => {
            let obj = heap
                .get(*id)
                .ok_or_else(|| RuntimeError::HeapError("Array reference not found".to_string()))?;
            if let ManagedObject::Array(arr) = obj {
                let json_array: Result<Vec<JsonValue>, _> =
                    arr.iter().map(|v| runtime_value_to_json(v, heap)).collect();
                Ok(JsonValue::Array(json_array?))
            } else {
                Err(RuntimeError::TypeError(
                    "Expected array in heap".to_string(),
                ))
            }
        }
        Value::Object(id) => {
            let obj = heap
                .get(*id)
                .ok_or_else(|| RuntimeError::HeapError("Object reference not found".to_string()))?;
            if let ManagedObject::Object { properties, .. } = obj {
                let mut json_obj = serde_json::Map::new();
                for (key, val) in properties {
                    json_obj.insert(key.clone(), runtime_value_to_json(val, heap)?);
                }
                Ok(JsonValue::Object(json_obj))
            } else {
                Err(RuntimeError::TypeError(
                    "Expected object in heap".to_string(),
                ))
            }
        }
        _ => Err(RuntimeError::TypeError(
            "Tipo não suportado para JSON".to_string(),
        )),
    }
}
