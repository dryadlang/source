use crate::errors::RuntimeError;
use crate::heap::{Heap, ManagedObject};
use crate::interpreter::Value;
use crate::native_modules::NativeFunction;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub fn register_websocket_functions(functions: &mut HashMap<String, NativeFunction>) {
    functions.insert("ws_connect".to_string(), ws_connect);
    functions.insert("ws_send".to_string(), ws_send);
    functions.insert("ws_receive".to_string(), ws_receive);
    functions.insert("ws_close".to_string(), ws_close);
    functions.insert("ws_create_server".to_string(), ws_create_server);
    functions.insert("ws_server_accept".to_string(), ws_server_accept);
    functions.insert("ws_server_send".to_string(), ws_server_send);
    functions.insert("ws_server_receive".to_string(), ws_server_receive);
}

struct WsConnection {
    connected: bool,
    url: String,
}

lazy_static::lazy_static! {
    static ref WS_CONNECTIONS: std::sync::Mutex<HashMap<String, Arc<Mutex<WsConnection>>>> =
        std::sync::Mutex::new(HashMap::new());
}

fn ws_connect(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    heap: &mut Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError(
            "ws_connect: esperado 1 argumento".to_string(),
        ));
    }

    let url = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(RuntimeError::TypeError(
                "ws_connect: argumento deve ser string (URL)".to_string(),
            ))
        }
    };

    let connection_id = format!("ws_{}", uuid::Uuid::new_v4());

    let mut connections = WS_CONNECTIONS
        .lock()
        .map_err(|_| RuntimeError::SystemError("Erro ao acessar conexões WebSocket".to_string()))?;

    connections.insert(
        connection_id.clone(),
        Arc::new(Mutex::new(WsConnection {
            connected: true,
            url: url.clone(),
        })),
    );

    let id = heap.allocate(ManagedObject::Object {
        properties: {
            let mut map = HashMap::new();
            map.insert("_type".to_string(), Value::String("websocket".to_string()));
            map.insert("id".to_string(), Value::String(connection_id.clone()));
            map.insert("url".to_string(), Value::String(url));
            map.insert("connected".to_string(), Value::Bool(true));
            map
        },
        methods: HashMap::new(),
    });

    Ok(Value::Object(id))
}

fn ws_send(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    _heap: &mut Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::ArgumentError(
            "ws_send: esperado 2 argumentos".to_string(),
        ));
    }

    let connection_id = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(RuntimeError::TypeError(
                "ws_send: primeiro argumento deve ser string (connection id)".to_string(),
            ))
        }
    };

    let message = match &args[1] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(RuntimeError::TypeError(
                "ws_send: segundo argumento deve ser string (mensagem)".to_string(),
            ))
        }
    };

    let connections = WS_CONNECTIONS
        .lock()
        .map_err(|_| RuntimeError::SystemError("Erro ao acessar conexões WebSocket".to_string()))?;

    if let Some(conn) = connections.get(&connection_id) {
        let conn_guard = conn
            .lock()
            .map_err(|_| RuntimeError::SystemError("Erro ao lockear conexão".to_string()))?;
        if conn_guard.connected {
            return Ok(Value::Bool(true));
        }
    }

    Ok(Value::Bool(false))
}

fn ws_receive(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    heap: &mut Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError(
            "ws_receive: esperado 1 argumento".to_string(),
        ));
    }

    let connection_id = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(RuntimeError::TypeError(
                "ws_receive: argumento deve ser string (connection id)".to_string(),
            ))
        }
    };

    let connections = WS_CONNECTIONS
        .lock()
        .map_err(|_| RuntimeError::SystemError("Erro ao acessar conexões WebSocket".to_string()))?;

    if let Some(conn) = connections.get(&connection_id) {
        let conn_guard = conn
            .lock()
            .map_err(|_| RuntimeError::SystemError("Erro ao lockear conexão".to_string()))?;
        if conn_guard.connected {
            let id = heap.allocate(ManagedObject::Object {
                properties: {
                    let mut map = HashMap::new();
                    map.insert("type".to_string(), Value::String("text".to_string()));
                    map.insert("data".to_string(), Value::String("".to_string()));
                    map
                },
                methods: HashMap::new(),
            });
            return Ok(Value::Object(id));
        }
    }

    Ok(Value::Null)
}

fn ws_close(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    _heap: &mut Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError(
            "ws_close: esperado 1 argumento".to_string(),
        ));
    }

    let connection_id = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(RuntimeError::TypeError(
                "ws_close: argumento deve ser string (connection id)".to_string(),
            ))
        }
    };

    let mut connections = WS_CONNECTIONS
        .lock()
        .map_err(|_| RuntimeError::SystemError("Erro ao acessar conexões WebSocket".to_string()))?;

    if let Some(conn) = connections.get(&connection_id) {
        let mut conn_guard = conn
            .lock()
            .map_err(|_| RuntimeError::SystemError("Erro ao lockear conexão".to_string()))?;
        conn_guard.connected = false;
    }

    connections.remove(&connection_id);

    Ok(Value::Bool(true))
}

fn ws_create_server(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    heap: &mut Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError(
            "ws_create_server: esperado 1 argumento".to_string(),
        ));
    }

    let port = match &args[0] {
        Value::Number(n) => *n as u16,
        _ => {
            return Err(RuntimeError::TypeError(
                "ws_create_server: argumento deve ser número (porta)".to_string(),
            ))
        }
    };

    let id = heap.allocate(ManagedObject::Object {
        properties: {
            let mut map = HashMap::new();
            map.insert(
                "_type".to_string(),
                Value::String("websocket_server".to_string()),
            );
            map.insert("port".to_string(), Value::Number(port as f64));
            map.insert("running".to_string(), Value::Bool(false));
            map
        },
        methods: HashMap::new(),
    });

    Ok(Value::Object(id))
}

fn ws_server_accept(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    heap: &mut Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError(
            "ws_server_accept: esperado 1 argumento".to_string(),
        ));
    }

    let _server_id = match &args[0] {
        Value::Object(_) => (),
        _ => {
            return Err(RuntimeError::TypeError(
                "ws_server_accept: argumento deve ser objeto server".to_string(),
            ))
        }
    };

    let id = heap.allocate(ManagedObject::Object {
        properties: {
            let mut map = HashMap::new();
            map.insert(
                "_type".to_string(),
                Value::String("websocket_connection".to_string()),
            );
            map.insert("connected".to_string(), Value::Bool(true));
            map
        },
        methods: HashMap::new(),
    });

    Ok(Value::Object(id))
}

fn ws_server_send(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    _heap: &mut Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::ArgumentError(
            "ws_server_send: esperado 2 argumentos".to_string(),
        ));
    }

    let _conn_id = match &args[0] {
        Value::Object(_) => (),
        _ => {
            return Err(RuntimeError::TypeError(
                "ws_server_send: primeiro argumento deve ser objeto conexão".to_string(),
            ))
        }
    };

    let _message = match &args[1] {
        Value::String(_) => (),
        _ => {
            return Err(RuntimeError::TypeError(
                "ws_server_send: segundo argumento deve ser string".to_string(),
            ))
        }
    };

    Ok(Value::Bool(true))
}

fn ws_server_receive(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    heap: &mut Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError(
            "ws_server_receive: esperado 1 argumento".to_string(),
        ));
    }

    let _conn_id = match &args[0] {
        Value::Object(_) => (),
        _ => {
            return Err(RuntimeError::TypeError(
                "ws_server_receive: argumento deve ser objeto conexão".to_string(),
            ))
        }
    };

    let id = heap.allocate(ManagedObject::Object {
        properties: {
            let mut map = HashMap::new();
            map.insert("type".to_string(), Value::String("text".to_string()));
            map.insert("data".to_string(), Value::String("".to_string()));
            map
        },
        methods: HashMap::new(),
    });

    Ok(Value::Object(id))
}
