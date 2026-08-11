use crate::errors::RuntimeError;
use crate::heap::{Heap, ManagedObject};
use crate::interpreter::Value;
use crate::native_modules::NativeFunction;
use futures::{SinkExt, StreamExt};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use tokio_tungstenite::{
    connect_async,
    accept_async,
    tungstenite::protocol::Message as WsMessage,
    WebSocketStream,
    MaybeTlsStream,
};
use tokio::net::{TcpStream, TcpListener};

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

struct WsConnection {
    stream: Arc<Mutex<Option<WsStream>>>,
    url: String,
}

lazy_static! {
    static ref RUNTIME: Runtime = Runtime::new().expect("Falha ao criar runtime Tokio para WebSockets");
    static ref WS_CONNECTIONS: Mutex<HashMap<String, Arc<WsConnection>>> = Mutex::new(HashMap::new());
    static ref WS_SERVERS: Mutex<HashMap<String, WsServerInstance>> = Mutex::new(HashMap::new());
}

struct WsServerInstance {
    host: String,
    port: u16,
    is_running: bool,
    stop_sender: Option<tokio::sync::mpsc::Sender<()>>,
}

pub fn register_websocket_functions(functions: &mut HashMap<String, NativeFunction>) {
    functions.insert("ws_connect".to_string(), ws_connect);
    functions.insert("ws_send".to_string(), ws_send);
    functions.insert("ws_receive".to_string(), ws_receive);
    functions.insert("ws_close".to_string(), ws_close);
    
    // Server functions
    functions.insert("ws_server_create".to_string(), ws_server_create);
    functions.insert("ws_server_start".to_string(), ws_server_start);
    functions.insert("ws_server_stop".to_string(), ws_server_stop);
    functions.insert("ws_server_status".to_string(), ws_server_status);
}

fn ws_connect(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    heap: &mut Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("ws_connect: esperado 1 argumento (URL)".to_string()));
    }

    let url_str = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("ws_connect: argumento deve ser string (URL)".to_string())),
    };

    let url = url_str.clone();
    let result = RUNTIME.block_on(async move {
        connect_async(&url).await
    });

    match result {
        Ok((stream, _)) => {
            let connection_id = format!("ws_{}", uuid::Uuid::new_v4());
            let conn = Arc::new(WsConnection {
                stream: Arc::new(Mutex::new(Some(stream))),
                url: url_str.clone(),
            });

            WS_CONNECTIONS.lock().unwrap().insert(connection_id.clone(), conn);

            let id = heap.allocate(ManagedObject::Object {
                properties: {
                    let mut map = HashMap::new();
                    map.insert("_type".to_string(), Value::String("websocket".to_string()));
                    map.insert("id".to_string(), Value::String(connection_id));
                    map.insert("url".to_string(), Value::String(url_str));
                    map.insert("connected".to_string(), Value::Bool(true));
                    map
                },
                methods: HashMap::new(),
            });

            Ok(Value::Object(id))
        }
        Err(e) => Err(RuntimeError::IoError(format!("Erro ao conectar WebSocket: {}", e))),
    }
}

fn ws_send(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    _heap: &mut Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::ArgumentError("ws_send: esperado 2 argumentos (id, mensagem)".to_string()));
    }

    let connection_id = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("ws_send: primeiro argumento deve ser string (id)".to_string())),
    };

    let message = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("ws_send: segundo argumento deve ser string (mensagem)".to_string())),
    };

    let conn = {
        let conns = WS_CONNECTIONS.lock().unwrap();
        conns.get(&connection_id).cloned()
    };

    if let Some(conn) = conn {
        let mut stream_lock = conn.stream.lock().unwrap();
        if let Some(mut stream) = stream_lock.take() {
            let result = RUNTIME.block_on(async move {
                stream.send(WsMessage::Text(message)).await.map(|_| stream)
            });
            match result {
                Ok(s) => {
                    *stream_lock = Some(s);
                    Ok(Value::Bool(true))
                }
                Err(e) => Err(RuntimeError::IoError(format!("Erro ao enviar mensagem WS: {}", e))),
            }
        } else {
            Err(RuntimeError::IoError("WebSocket desconectado".to_string()))
        }
    } else {
        Err(RuntimeError::ArgumentError(format!("Conexão WS não encontrada: {}", connection_id)))
    }
}

fn ws_receive(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    heap: &mut Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("ws_receive: esperado 1 argumento (id)".to_string()));
    }

    let connection_id = match &args[0] {
        Value::String(s) => s,
        _ => return Err(RuntimeError::TypeError("ws_receive: argumento deve ser string (id)".to_string())),
    };

    let conn = {
        let conns = WS_CONNECTIONS.lock().unwrap();
        conns.get(connection_id).cloned()
    };

    if let Some(conn) = conn {
        let mut stream_lock = conn.stream.lock().unwrap();
        if let Some(mut stream) = stream_lock.take() {
            let result = RUNTIME.block_on(async move {
                match stream.next().await {
                    Some(Ok(msg)) => Ok((msg, stream)),
                    Some(Err(e)) => Err((RuntimeError::IoError(format!("Erro ao receber mensagem WS: {}", e)), stream)),
                    None => Err((RuntimeError::IoError("WebSocket fechado pelo servidor".to_string()), stream)),
                }
            });

            match result {
                Ok((msg, s)) => {
                    *stream_lock = Some(s);
                    match msg {
                        WsMessage::Text(t) => {
                            let obj_id = heap.allocate(ManagedObject::Object {
                                properties: {
                                    let mut map = HashMap::new();
                                    map.insert("type".to_string(), Value::String("text".to_string()));
                                    map.insert("data".to_string(), Value::String(t));
                                    map
                                },
                                methods: HashMap::new(),
                            });
                            Ok(Value::Object(obj_id))
                        }
                        WsMessage::Binary(b) => {
                            let bytes_id = heap.allocate(ManagedObject::Array(
                                b.into_iter().map(|byte| Value::Number(byte as f64)).collect()
                            ));
                            let obj_id = heap.allocate(ManagedObject::Object {
                                properties: {
                                    let mut map = HashMap::new();
                                    map.insert("type".to_string(), Value::String("binary".to_string()));
                                    map.insert("data".to_string(), Value::Array(bytes_id));
                                    map
                                },
                                methods: HashMap::new(),
                            });
                            Ok(Value::Object(obj_id))
                        }
                        WsMessage::Close(_) => {
                            *stream_lock = None;
                            WS_CONNECTIONS.lock().unwrap().remove(connection_id);
                            Ok(Value::Null)
                        }
                        _ => Ok(Value::Null), // Outros tipos (Ping/Pong) ignorados ou tratados como null
                    }
                }
                Err((e, s)) => {
                    *stream_lock = Some(s);
                    Err(e)
                }
            }
        } else {
            Ok(Value::Null)
        }
    } else {
        Err(RuntimeError::ArgumentError(format!("Conexão WS não encontrada: {}", connection_id)))
    }
}

fn ws_close(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    _heap: &mut Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("ws_close: esperado 1 argumento (id)".to_string()));
    }

    let connection_id = match &args[0] {
        Value::String(s) => s,
        _ => return Err(RuntimeError::TypeError("ws_close: argumento deve ser string (id)".to_string())),
    };

    let conn = {
        let mut conns = WS_CONNECTIONS.lock().unwrap();
        conns.remove(connection_id)
    };

    if let Some(conn) = conn {
        let mut stream_lock = conn.stream.lock().unwrap();
        if let Some(mut stream) = stream_lock.take() {
            let _ = RUNTIME.block_on(async move {
                stream.close(None).await
            });
        }
        Ok(Value::Bool(true))
    } else {
        Ok(Value::Bool(false))
    }
}
fn ws_server_create(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    _heap: &mut Heap,
) -> Result<Value, RuntimeError> {
    if args.len() < 1 {
        return Err(RuntimeError::ArgumentError("ws_server_create: esperado pelo menos 1 argumento (id, [host], [port])".to_string()));
    }

    let id = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("ws_server_create: id deve ser string".to_string())),
    };

    let host = args.get(1).and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None }).unwrap_or_else(|| "127.0.0.1".to_string());
    let port = args.get(2).and_then(|v| if let Value::Number(n) = v { Some(*n as u16) } else { None }).unwrap_or(8080);

    let server = WsServerInstance {
        host,
        port,
        is_running: false,
        stop_sender: None,
    };

    WS_SERVERS.lock().unwrap().insert(id, server);
    Ok(Value::Null)
}

fn ws_server_start(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    _heap: &mut Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("ws_server_start: esperado 1 argumento (id)".to_string()));
    }

    let id = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("ws_server_start: id deve ser string".to_string())),
    };

    let mut servers = WS_SERVERS.lock().unwrap();
    let server = servers.get_mut(&id).ok_or_else(|| RuntimeError::ArgumentError(format!("Servidor WS não encontrado: {}", id)))?;

    if server.is_running {
        return Ok(Value::Null);
    }

    let (stop_sender, mut stop_receiver) = tokio::sync::mpsc::channel(1);
    server.stop_sender = Some(stop_sender);
    server.is_running = true;

    let addr = format!("{}:{}", server.host, server.port);
    let server_id = id.clone();

    RUNTIME.spawn(async move {
        let listener = match TcpListener::bind(&addr).await {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Erro ao iniciar servidor WS '{}' em {}: {}", server_id, addr, e);
                return;
            }
        };

        println!("🌍 Servidor WebSocket '{}' ouvindo em ws://{}", server_id, addr);

        loop {
            tokio::select! {
                _ = stop_receiver.recv() => {
                    println!("🛑 Servidor WebSocket '{}' parando...", server_id);
                    break;
                }
                accept_res = listener.accept() => {
                    if let Ok((stream, _)) = accept_res {
                        let server_id_clone = server_id.clone();
                        tokio::spawn(async move {
                            if let Ok(ws_stream) = accept_async(MaybeTlsStream::Plain(stream)).await {
                                // Por enquanto, apenas logar conexão. 
                                // Futuro: Adicionar à lista de conexões do servidor e permitir handlers.
                                println!("🔗 Servidor WS '{}': Nova conexão estabelecida", server_id_clone);
                                handle_server_ws_connection(ws_stream, server_id_clone).await;
                            }
                        });
                    }
                }
            }
        }
    });

    Ok(Value::Null)
}

async fn handle_server_ws_connection(mut ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>, server_id: String) {
    while let Some(msg) = ws_stream.next().await {
        match msg {
            Ok(WsMessage::Text(t)) => {
                println!("📨 Servidor WS '{}' recebeu: {}", server_id, t);
                // Echo para demonstração básica
                if let Err(e) = ws_stream.send(WsMessage::Text(format!("Echo: {}", t))).await {
                    eprintln!("Erro ao enviar echo WS: {}", e);
                    break;
                }
            }
            Ok(WsMessage::Close(_)) => break,
            Err(_) => break,
            _ => {}
        }
    }
    println!("👋 Servidor WS '{}': Conexão encerrada", server_id);
}

fn ws_server_stop(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    _heap: &mut Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("ws_server_stop: esperado 1 argumento (id)".to_string()));
    }

    let id = match &args[0] {
        Value::String(s) => s,
        _ => return Err(RuntimeError::TypeError("ws_server_stop: id deve ser string".to_string())),
    };

    let mut servers = WS_SERVERS.lock().unwrap();
    if let Some(server) = servers.get_mut(id) {
        if let Some(sender) = server.stop_sender.take() {
            let _ = RUNTIME.block_on(async move { sender.send(()).await });
        }
        server.is_running = false;
        Ok(Value::Bool(true))
    } else {
        Ok(Value::Bool(false))
    }
}

fn ws_server_status(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    heap: &mut Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("ws_server_status: esperado 1 argumento (id)".to_string()));
    }

    let id = match &args[0] {
        Value::String(s) => s,
        _ => return Err(RuntimeError::TypeError("ws_server_status: id deve ser string".to_string())),
    };

    let servers = WS_SERVERS.lock().unwrap();
    if let Some(server) = servers.get(id) {
        let id_obj = heap.allocate(ManagedObject::Object {
            properties: {
                let mut map = HashMap::new();
                map.insert("id".to_string(), Value::String(id.clone()));
                map.insert("host".to_string(), Value::String(server.host.clone()));
                map.insert("port".to_string(), Value::Number(server.port as f64));
                map.insert("running".to_string(), Value::Bool(server.is_running));
                map
            },
            methods: HashMap::new(),
        });
        Ok(Value::Object(id_obj))
    } else {
        Err(RuntimeError::ArgumentError(format!("Servidor WS não encontrado: {}", id)))
    }
}
