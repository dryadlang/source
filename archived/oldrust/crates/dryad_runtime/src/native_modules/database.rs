use crate::errors::RuntimeError;
use crate::heap::{Heap, ManagedObject};
use crate::interpreter::Value;
use crate::native_modules::NativeFunction;
use rusqlite::{Connection, params};
use std::collections::HashMap;
use hex;
use std::sync::Mutex;
use lazy_static::lazy_static;

use tokio_postgres::{Client, NoTls};
use tokio::runtime::Runtime;
use std::sync::Arc;

lazy_static! {
    static ref SQLITE_CONNECTIONS: Mutex<HashMap<String, Connection>> = Mutex::new(HashMap::new());
    static ref PG_CLIENTS: Mutex<HashMap<String, Arc<Client>>> = Mutex::new(HashMap::new());
    static ref RUNTIME: Runtime = Runtime::new().expect("Falha ao criar runtime Tokio para Banco de Dados");
}

pub fn register_database_functions(functions: &mut HashMap<String, NativeFunction>) {
    functions.insert("sqlite_open".to_string(), sqlite_open);
    functions.insert("sqlite_close".to_string(), sqlite_close);
    functions.insert("sqlite_execute".to_string(), sqlite_execute);
    functions.insert("sqlite_query".to_string(), sqlite_query);
    
    // PostgreSQL
    functions.insert("pg_connect".to_string(), pg_connect);
    functions.insert("pg_execute".to_string(), pg_execute);
    functions.insert("pg_query".to_string(), pg_query);
    functions.insert("pg_close".to_string(), pg_close);
}

// ============================================
// SQLITE IMPLEMENTATION
// ============================================

fn sqlite_open(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    heap: &mut Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("sqlite_open: esperado 1 argumento (caminho)".to_string()));
    }

    let path = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("sqlite_open: argumento deve ser string".to_string())),
    };

    let conn = if path == ":memory:" {
        Connection::open_in_memory()
    } else {
        Connection::open(&path)
    }.map_err(|e| RuntimeError::IoError(format!("Erro ao abrir banco SQLite: {}", e)))?;

    let connection_id = format!("db_{}", uuid::Uuid::new_v4());
    SQLITE_CONNECTIONS.lock().unwrap().insert(connection_id.clone(), conn);

    let id = heap.allocate(ManagedObject::Object {
        properties: {
            let mut map = HashMap::new();
            map.insert("_type".to_string(), Value::String("sqlite".to_string()));
            map.insert("id".to_string(), Value::String(connection_id));
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
        return Err(RuntimeError::ArgumentError("sqlite_close: esperado 1 argumento (id)".to_string()));
    }

    let connection_id = match &args[0] {
        Value::String(s) => s,
        _ => return Err(RuntimeError::TypeError("sqlite_close: argumento deve ser string (id)".to_string())),
    };

    let mut conns = SQLITE_CONNECTIONS.lock().unwrap();
    if conns.remove(connection_id).is_some() {
        Ok(Value::Bool(true))
    } else {
        Ok(Value::Bool(false))
    }
}

fn sqlite_execute(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    heap: &mut Heap,
) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::ArgumentError("sqlite_execute: esperado pelo menos 2 argumentos (id, sql, [params])".to_string()));
    }

    let connection_id = match &args[0] {
        Value::String(s) => s,
        _ => return Err(RuntimeError::TypeError("sqlite_execute: primeiro argumento deve ser string (id)".to_string())),
    };

    let sql = match &args[1] {
        Value::String(s) => s,
        _ => return Err(RuntimeError::TypeError("sqlite_execute: segundo argumento deve ser string (SQL)".to_string())),
    };

    let mut conns = SQLITE_CONNECTIONS.lock().unwrap();
    let conn = conns.get_mut(connection_id).ok_or_else(|| RuntimeError::ArgumentError(format!("Conexão SQLite não encontrada: {}", connection_id)))?;

    // Implementação básica de execute sem parâmetros por enquanto para simplificar o mapeamento
    match conn.execute(sql, []) {
        Ok(rows_affected) => {
            let last_id = conn.last_insert_rowid();
            let id = heap.allocate(ManagedObject::Object {
                properties: {
                    let mut map = HashMap::new();
                    map.insert("rows_affected".to_string(), Value::Number(rows_affected as f64));
                    map.insert("last_insert_id".to_string(), Value::Number(last_id as f64));
                    map
                },
                methods: HashMap::new(),
            });
            Ok(Value::Object(id))
        }
        Err(e) => Err(RuntimeError::IoError(format!("Erro ao executar SQL: {}", e))),
    }
}

fn sqlite_query(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    heap: &mut Heap,
) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::ArgumentError("sqlite_query: esperado pelo menos 2 argumentos (id, sql, [params])".to_string()));
    }

    let connection_id = match &args[0] {
        Value::String(s) => s,
        _ => return Err(RuntimeError::TypeError("sqlite_query: primeiro argumento deve ser string (id)".to_string())),
    };

    let sql = match &args[1] {
        Value::String(s) => s,
        _ => return Err(RuntimeError::TypeError("sqlite_query: segundo argumento deve ser string (SQL)".to_string())),
    };

    let conns = SQLITE_CONNECTIONS.lock().unwrap();
    let conn = conns.get(connection_id).ok_or_else(|| RuntimeError::ArgumentError(format!("Conexão SQLite não encontrada: {}", connection_id)))?;

    let mut stmt = conn.prepare(sql).map_err(|e| RuntimeError::IoError(format!("Erro ao preparar query: {}", e)))?;
    let column_names: Vec<String> = stmt.column_names().iter().map(|s| s.to_string()).collect();

    let mut rows_data = Vec::new();
    let mut rows_iter = stmt.query_map([], |row| {
        let mut row_values = Vec::new();
        for (i, _) in column_names.iter().enumerate() {
            row_values.push(row.get::<usize, rusqlite::types::Value>(i)?);
        }
        Ok(row_values)
    }).map_err(|e| RuntimeError::IoError(format!("Erro ao executar query: {}", e)))?;

    for row_result in rows_iter {
        rows_data.push(row_result.map_err(|e| RuntimeError::IoError(format!("Erro ao ler linha: {}", e)))?);
    }

    let mut rows_vec = Vec::new();
    for row_values in rows_data {
        let mut props = HashMap::new();
        for (i, name) in column_names.iter().enumerate() {
            let val = &row_values[i];
            props.insert(name.clone(), sqlite_value_to_dryad_from_value(val, heap));
        }
        
        let obj_id = heap.allocate(ManagedObject::Object {
            properties: props,
            methods: HashMap::new(),
        });
        rows_vec.push(Value::Object(obj_id));
    }

    let array_id = heap.allocate(ManagedObject::Array(rows_vec));
    Ok(Value::Array(array_id))
}

fn sqlite_value_to_dryad_from_value(val: &rusqlite::types::Value, heap: &mut Heap) -> Value {
    match val {
        rusqlite::types::Value::Null => Value::Null,
        rusqlite::types::Value::Integer(i) => Value::Number(*i as f64),
        rusqlite::types::Value::Real(f) => Value::Number(*f),
        rusqlite::types::Value::Text(s) => Value::String(s.clone()),
        rusqlite::types::Value::Blob(b) => {
            // Converter Blob para string hex ou similar se necessário
            Value::String(hex::encode(b))
        }
    }
}

fn sqlite_value_to_dryad(val: rusqlite::types::ValueRef, heap: &mut Heap) -> Value {
    match val {
        rusqlite::types::ValueRef::Null => Value::Null,
        rusqlite::types::ValueRef::Integer(i) => Value::Number(i as f64),
        rusqlite::types::ValueRef::Real(f) => Value::Number(f),
        rusqlite::types::ValueRef::Text(s) => {
            let s_str = std::str::from_utf8(s).unwrap_or("");
            Value::String(s_str.to_string())
        }
        rusqlite::types::ValueRef::Blob(b) => {
            Value::String(hex::encode(b))
        }
    }
}

// ============================================
// POSTGRESQL MOCKS
// ============================================

fn pg_connect(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    heap: &mut Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("pg_connect: esperado 1 argumento (connection_string)".to_string()));
    }

    let conn_str = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("pg_connect: argumento deve ser string".to_string())),
    };

    let result = RUNTIME.block_on(async {
        let (client, connection) = tokio_postgres::connect(&conn_str, NoTls).await
            .map_err(|e| RuntimeError::IoError(format!("Erro ao conectar PostgreSQL: {}", e)))?;

        // A conexão precisa ser spawnada em segundo plano
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("PostgreSQL connection error: {}", e);
            }
        });

        Ok(client)
    });

    match result {
        Ok(client) => {
            let connection_id = format!("pg_{}", uuid::Uuid::new_v4());
            PG_CLIENTS.lock().unwrap().insert(connection_id.clone(), Arc::new(client));

            let id = heap.allocate(ManagedObject::Object {
                properties: {
                    let mut map = HashMap::new();
                    map.insert("_type".to_string(), Value::String("postgres".to_string()));
                    map.insert("id".to_string(), Value::String(connection_id));
                    map.insert("conn_str".to_string(), Value::String(conn_str));
                    map.insert("connected".to_string(), Value::Bool(true));
                    map
                },
                methods: HashMap::new(),
            });

            Ok(Value::Object(id))
        }
        Err(e) => Err(e),
    }
}

fn pg_execute(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    heap: &mut Heap,
) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::ArgumentError("pg_execute: esperado pelo menos 2 argumentos (id, sql, [params])".to_string()));
    }

    let connection_id = match &args[0] {
        Value::String(s) => s,
        _ => return Err(RuntimeError::TypeError("pg_execute: primeiro argumento deve ser string (id)".to_string())),
    };

    let sql = match &args[1] {
        Value::String(s) => s,
        _ => return Err(RuntimeError::TypeError("pg_execute: segundo argumento deve ser string (SQL)".to_string())),
    };

    let clients = PG_CLIENTS.lock().unwrap();
    let client = clients.get(connection_id).ok_or_else(|| RuntimeError::ArgumentError(format!("Conexão PostgreSQL não encontrada: {}", connection_id)))?.clone();

    // Implementação básica de execute sem parâmetros complexos por enquanto
    let rows_affected = RUNTIME.block_on(async {
        client.execute(sql, &[]).await
            .map_err(|e| RuntimeError::IoError(format!("Erro ao executar PostgreSQL SQL: {}", e)))
    })?;

    let id = heap.allocate(ManagedObject::Object {
        properties: {
            let mut map = HashMap::new();
            map.insert("rows_affected".to_string(), Value::Number(rows_affected as f64));
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
    if args.len() < 2 {
        return Err(RuntimeError::ArgumentError("pg_query: esperado pelo menos 2 argumentos (id, sql, [params])".to_string()));
    }

    let connection_id = match &args[0] {
        Value::String(s) => s,
        _ => return Err(RuntimeError::TypeError("pg_query: primeiro argumento deve ser string (id)".to_string())),
    };

    let sql = match &args[1] {
        Value::String(s) => s,
        _ => return Err(RuntimeError::TypeError("pg_query: segundo argumento deve ser string (SQL)".to_string())),
    };

    let clients = PG_CLIENTS.lock().unwrap();
    let client = clients.get(connection_id).ok_or_else(|| RuntimeError::ArgumentError(format!("Conexão PostgreSQL não encontrada: {}", connection_id)))?.clone();

    let rows = RUNTIME.block_on(async {
        client.query(sql, &[]).await
            .map_err(|e| RuntimeError::IoError(format!("Erro ao executar PostgreSQL query: {}", e)))
    })?;

    let mut rows_vec = Vec::new();
    for row in rows {
        let mut props = HashMap::new();
        for (i, column) in row.columns().iter().enumerate() {
            let name = column.name().to_string();
            // TODO: Mapear mais tipos PostgreSQL
            let val = if let Ok(s) = row.try_get::<usize, String>(i) {
                Value::String(s)
            } else if let Ok(n) = row.try_get::<usize, i32>(i) {
                Value::Number(n as f64)
            } else if let Ok(n) = row.try_get::<usize, f64>(i) {
                Value::Number(n)
            } else if let Ok(b) = row.try_get::<usize, bool>(i) {
                Value::Bool(b)
            } else {
                Value::Null
            };
            props.insert(name, val);
        }
        
        let obj_id = heap.allocate(ManagedObject::Object {
            properties: props,
            methods: HashMap::new(),
        });
        rows_vec.push(Value::Object(obj_id));
    }

    let array_id = heap.allocate(ManagedObject::Array(rows_vec));
    Ok(Value::Array(array_id))
}

fn pg_close(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    _heap: &mut Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("pg_close: esperado 1 argumento (id)".to_string()));
    }

    let connection_id = match &args[0] {
        Value::String(s) => s,
        _ => return Err(RuntimeError::TypeError("pg_close: argumento deve ser string (id)".to_string())),
    };

    let mut clients = PG_CLIENTS.lock().unwrap();
    if clients.remove(connection_id).is_some() {
        Ok(Value::Bool(true))
    } else {
        Ok(Value::Bool(false))
    }
}

