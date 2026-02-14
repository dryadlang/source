use crate::errors::RuntimeError;
use crate::heap::{Heap, ManagedObject};
use crate::interpreter::Value;
use crate::native_modules::NativeFunction;
use std::collections::HashMap;

pub fn register_database_functions(functions: &mut HashMap<String, NativeFunction>) {
    // SQLite functions
    functions.insert("sqlite_open".to_string(), sqlite_open);
    functions.insert("sqlite_close".to_string(), sqlite_close);
    functions.insert("sqlite_execute".to_string(), sqlite_execute);
    functions.insert("sqlite_query".to_string(), sqlite_query);
    functions.insert("sqlite_prepare".to_string(), sqlite_prepare);
    functions.insert("sqlite_bind".to_string(), sqlite_bind);
    functions.insert("sqlite_step".to_string(), sqlite_step);
    functions.insert("sqlite_columns".to_string(), sqlite_columns);

    // PostgreSQL functions
    functions.insert("pg_connect".to_string(), pg_connect);
    functions.insert("pg_close".to_string(), pg_close);
    functions.insert("pg_execute".to_string(), pg_execute);
    functions.insert("pg_query".to_string(), pg_query);
    functions.insert("pg_prepare".to_string(), pg_prepare);
    functions.insert("pg_bind".to_string(), pg_bind);
    functions.insert("pg_query_params".to_string(), pg_query_params);
}

// ============================================
// SQLITE
// ============================================

fn sqlite_open(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    heap: &mut Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError(
            "sqlite_open: esperado 1 argumento".to_string(),
        ));
    }

    let path = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(RuntimeError::TypeError(
                "sqlite_open: argumento deve ser string (caminho)".to_string(),
            ))
        }
    };

    let id = heap.allocate(ManagedObject::Object {
        properties: {
            let mut map = HashMap::new();
            map.insert("_type".to_string(), Value::String("sqlite".to_string()));
            map.insert("path".to_string(), Value::String(path));
            map.insert("connected".to_string(), Value::Bool(true));
            map
        },
        methods: HashMap::new(),
    });

    Ok(Value::Object(id))
}

fn sqlite_close(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    _heap: &mut Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError(
            "sqlite_close: esperado 1 argumento".to_string(),
        ));
    }

    match &args[0] {
        Value::Object(_) => Ok(Value::Bool(true)),
        _ => Err(RuntimeError::TypeError(
            "sqlite_close: argumento deve ser objeto sqlite".to_string(),
        )),
    }
}

fn sqlite_execute(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    heap: &mut Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::ArgumentError(
            "sqlite_execute: esperado 2 argumentos".to_string(),
        ));
    }

    let _db = match &args[0] {
        Value::Object(_) => (),
        _ => {
            return Err(RuntimeError::TypeError(
                "sqlite_execute: primeiro argumento deve ser objeto sqlite".to_string(),
            ))
        }
    };

    let _sql = match &args[1] {
        Value::String(_) => (),
        _ => {
            return Err(RuntimeError::TypeError(
                "sqlite_execute: segundo argumento deve ser string (SQL)".to_string(),
            ))
        }
    };

    let id = heap.allocate(ManagedObject::Object {
        properties: {
            let mut map = HashMap::new();
            map.insert("rows_affected".to_string(), Value::Number(0.0));
            map.insert("last_insert_id".to_string(), Value::Number(0.0));
            map
        },
        methods: HashMap::new(),
    });

    Ok(Value::Object(id))
}

fn sqlite_query(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    heap: &mut Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::ArgumentError(
            "sqlite_query: esperado 2 argumentos".to_string(),
        ));
    }

    let _db = match &args[0] {
        Value::Object(_) => (),
        _ => {
            return Err(RuntimeError::TypeError(
                "sqlite_query: primeiro argumento deve ser objeto sqlite".to_string(),
            ))
        }
    };

    let _sql = match &args[1] {
        Value::String(_) => (),
        _ => {
            return Err(RuntimeError::TypeError(
                "sqlite_query: segundo argumento deve ser string (SQL)".to_string(),
            ))
        }
    };

    // Return empty result set
    let empty_array_id = heap.allocate(ManagedObject::Array(Vec::new()));
    Ok(Value::Array(empty_array_id))
}

fn sqlite_prepare(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    heap: &mut Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::ArgumentError(
            "sqlite_prepare: esperado 2 argumentos".to_string(),
        ));
    }

    let _db = match &args[0] {
        Value::Object(_) => (),
        _ => {
            return Err(RuntimeError::TypeError(
                "sqlite_prepare: primeiro argumento deve ser objeto sqlite".to_string(),
            ))
        }
    };

    let _sql = match &args[1] {
        Value::String(_) => (),
        _ => {
            return Err(RuntimeError::TypeError(
                "sqlite_prepare: segundo argumento deve ser string (SQL)".to_string(),
            ))
        }
    };

    let id = heap.allocate(ManagedObject::Object {
        properties: {
            let mut map = HashMap::new();
            map.insert(
                "_type".to_string(),
                Value::String("sqlite_statement".to_string()),
            );
            map.insert("prepared".to_string(), Value::Bool(true));
            map
        },
        methods: HashMap::new(),
    });

    Ok(Value::Object(id))
}

fn sqlite_bind(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    _heap: &mut Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 3 {
        return Err(RuntimeError::ArgumentError(
            "sqlite_bind: esperado 3 argumentos".to_string(),
        ));
    }

    let _stmt = match &args[0] {
        Value::Object(_) => (),
        _ => {
            return Err(RuntimeError::TypeError(
                "sqlite_bind: primeiro argumento deve ser objeto statement".to_string(),
            ))
        }
    };

    let _index = match &args[1] {
        Value::Number(_) => (),
        _ => {
            return Err(RuntimeError::TypeError(
                "sqlite_bind: segundo argumento deve ser número".to_string(),
            ))
        }
    };

    let _value = &args[2];

    Ok(Value::Bool(true))
}

fn sqlite_step(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    _heap: &mut Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError(
            "sqlite_step: esperado 1 argumento".to_string(),
        ));
    }

    let _stmt = match &args[0] {
        Value::Object(_) => (),
        _ => {
            return Err(RuntimeError::TypeError(
                "sqlite_step: argumento deve ser objeto statement".to_string(),
            ))
        }
    };

    // Return done (no more rows)
    Ok(Value::Bool(false))
}

fn sqlite_columns(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    heap: &mut Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError(
            "sqlite_columns: esperado 1 argumento".to_string(),
        ));
    }

    let _stmt = match &args[0] {
        Value::Object(_) => (),
        _ => {
            return Err(RuntimeError::TypeError(
                "sqlite_columns: argumento deve ser objeto statement".to_string(),
            ))
        }
    };

    let empty_array_id = heap.allocate(ManagedObject::Array(Vec::new()));
    Ok(Value::Array(empty_array_id))
}

// ============================================
// POSTGRESQL
// ============================================

fn pg_connect(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    heap: &mut Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError(
            "pg_connect: esperado 1 argumento".to_string(),
        ));
    }

    let connection_string = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(RuntimeError::TypeError(
                "pg_connect: argumento deve ser string (connection string)".to_string(),
            ))
        }
    };

    let id = heap.allocate(ManagedObject::Object {
        properties: {
            let mut map = HashMap::new();
            map.insert("_type".to_string(), Value::String("postgresql".to_string()));
            map.insert(
                "connection_string".to_string(),
                Value::String(connection_string),
            );
            map.insert("connected".to_string(), Value::Bool(true));
            map
        },
        methods: HashMap::new(),
    });

    Ok(Value::Object(id))
}

fn pg_close(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    _heap: &mut Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError(
            "pg_close: esperado 1 argumento".to_string(),
        ));
    }

    match &args[0] {
        Value::Object(_) => Ok(Value::Bool(true)),
        _ => Err(RuntimeError::TypeError(
            "pg_close: argumento deve ser objeto postgresql".to_string(),
        )),
    }
}

fn pg_execute(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    heap: &mut Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 3 {
        return Err(RuntimeError::ArgumentError(
            "pg_execute: esperado 3 argumentos".to_string(),
        ));
    }

    let _conn = match &args[0] {
        Value::Object(_) => (),
        _ => {
            return Err(RuntimeError::TypeError(
                "pg_execute: primeiro argumento deve ser objeto connection".to_string(),
            ))
        }
    };

    let _query = match &args[1] {
        Value::String(_) => (),
        _ => {
            return Err(RuntimeError::TypeError(
                "pg_execute: segundo argumento deve ser string".to_string(),
            ))
        }
    };

    let _params = match &args[2] {
        Value::Array(_) => (),
        _ => {
            return Err(RuntimeError::TypeError(
                "pg_execute: terceiro argumento deve ser array".to_string(),
            ))
        }
    };

    let id = heap.allocate(ManagedObject::Object {
        properties: {
            let mut map = HashMap::new();
            map.insert("rows_affected".to_string(), Value::Number(0.0));
            map
        },
        methods: HashMap::new(),
    });

    Ok(Value::Object(id))
}

fn pg_query(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    heap: &mut Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::ArgumentError(
            "pg_query: esperado 2 argumentos".to_string(),
        ));
    }

    let _conn = match &args[0] {
        Value::Object(_) => (),
        _ => {
            return Err(RuntimeError::TypeError(
                "pg_query: primeiro argumento deve ser objeto connection".to_string(),
            ))
        }
    };

    let _sql = match &args[1] {
        Value::String(_) => (),
        _ => {
            return Err(RuntimeError::TypeError(
                "pg_query: segundo argumento deve ser string (SQL)".to_string(),
            ))
        }
    };

    let empty_array_id = heap.allocate(ManagedObject::Array(Vec::new()));
    Ok(Value::Array(empty_array_id))
}

fn pg_prepare(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    heap: &mut Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 3 {
        return Err(RuntimeError::ArgumentError(
            "pg_prepare: esperado 3 argumentos".to_string(),
        ));
    }

    let _conn = match &args[0] {
        Value::Object(_) => (),
        _ => {
            return Err(RuntimeError::TypeError(
                "pg_prepare: primeiro argumento deve ser objeto connection".to_string(),
            ))
        }
    };

    let _name = match &args[1] {
        Value::String(_) => (),
        _ => {
            return Err(RuntimeError::TypeError(
                "pg_prepare: segundo argumento deve ser string (statement name)".to_string(),
            ))
        }
    };

    let _sql = match &args[2] {
        Value::String(_) => (),
        _ => {
            return Err(RuntimeError::TypeError(
                "pg_prepare: terceiro argumento deve ser string (SQL)".to_string(),
            ))
        }
    };

    let id = heap.allocate(ManagedObject::Object {
        properties: {
            let mut map = HashMap::new();
            map.insert(
                "_type".to_string(),
                Value::String("pg_statement".to_string()),
            );
            map.insert("prepared".to_string(), Value::Bool(true));
            map
        },
        methods: HashMap::new(),
    });

    Ok(Value::Object(id))
}

fn pg_bind(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    _heap: &mut Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 3 {
        return Err(RuntimeError::ArgumentError(
            "pg_bind: esperado 3 argumentos".to_string(),
        ));
    }

    let _conn = match &args[0] {
        Value::Object(_) => (),
        _ => {
            return Err(RuntimeError::TypeError(
                "pg_bind: primeiro argumento deve ser objeto connection".to_string(),
            ))
        }
    };

    let _stmt = match &args[1] {
        Value::String(_) => (),
        _ => {
            return Err(RuntimeError::TypeError(
                "pg_bind: segundo argumento deve ser string (statement name)".to_string(),
            ))
        }
    };

    let _params = match &args[2] {
        Value::Array(_) => (),
        _ => {
            return Err(RuntimeError::TypeError(
                "pg_bind: terceiro argumento deve ser array".to_string(),
            ))
        }
    };

    Ok(Value::Bool(true))
}

fn pg_query_params(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    heap: &mut Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 3 {
        return Err(RuntimeError::ArgumentError(
            "pg_query_params: esperado 3 argumentos".to_string(),
        ));
    }

    let _conn = match &args[0] {
        Value::Object(_) => (),
        _ => {
            return Err(RuntimeError::TypeError(
                "pg_query_params: primeiro argumento deve ser objeto connection".to_string(),
            ))
        }
    };

    let _query = match &args[1] {
        Value::String(_) => (),
        _ => {
            return Err(RuntimeError::TypeError(
                "pg_query_params: segundo argumento deve ser string".to_string(),
            ))
        }
    };

    let _params = match &args[2] {
        Value::Array(_) => (),
        _ => {
            return Err(RuntimeError::TypeError(
                "pg_query_params: terceiro argumento deve ser array".to_string(),
            ))
        }
    };

    let empty_array_id = heap.allocate(ManagedObject::Array(Vec::new()));
    Ok(Value::Array(empty_array_id))
}
