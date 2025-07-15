use crate::interpreter::RuntimeValue;
use crate::errors::RuntimeError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use tungstenite::{accept, connect, Message};
use tungstenite::stream::MaybeTlsStream;
use std::net::{TcpListener, TcpStream};

// Estruturas para gerenciar sockets e clientes
lazy_static! {
    static ref WS_SERVERS: Arc<Mutex<HashMap<u64, TcpListener>>> = Arc::new(Mutex::new(HashMap::new()));
    static ref WS_CLIENTS: Arc<Mutex<HashMap<u64, WsSocket>>> = Arc::new(Mutex::new(HashMap::new()));
    static ref WS_COUNTER: Arc<Mutex<u64>> = Arc::new(Mutex::new(1));
}

enum WsSocket {
    Plain(tungstenite::WebSocket<TcpStream>),
    Tls(tungstenite::WebSocket<MaybeTlsStream<TcpStream>>),
}

/// Registra todas as funções nativas do módulo websocket
pub fn register_websocket_functions(functions: &mut HashMap<String, fn(&[RuntimeValue]) -> Result<RuntimeValue, RuntimeError>>) {
    functions.insert("native_ws_listen".to_string(), native_ws_listen);
    // functions.insert("native_ws_accept".to_string(), native_ws_accept);
    functions.insert("native_ws_send_all".to_string(), native_ws_send_all);
    functions.insert("native_ws_broadcast".to_string(), native_ws_broadcast);
    functions.insert("native_ws_broadcast_except".to_string(), native_ws_broadcast_except);
    functions.insert("native_ws_close_all".to_string(), native_ws_close_all);
    functions.insert("native_ws_close_client".to_string(), native_ws_close_client);
    functions.insert("native_ws_is_connected_client".to_string(), native_ws_is_connected_client);
    functions.insert("native_ws_set_timeout".to_string(), native_ws_set_timeout);
    functions.insert("native_ws_get_clients".to_string(), native_ws_get_clients);

    functions.insert("native_ws_connect".to_string(), native_ws_connect);
    functions.insert("native_ws_send".to_string(), native_ws_send);
    functions.insert("native_ws_recv".to_string(), native_ws_recv);
    functions.insert("native_ws_close".to_string(), native_ws_close);
    functions.insert("native_ws_is_connected".to_string(), native_ws_is_connected);
    functions.insert("native_ws_set_timeout".to_string(), native_ws_set_timeout);
    functions.insert("native_ws_set_nodelay".to_string(), native_ws_set_nodelay);
    functions.insert("native_ws_set_keepalive".to_string(), native_ws_set_keepalive);
    functions.insert("native_ws_set_reuseaddr".to_string(), native_ws_set_reuseaddr);
}

// ========================
// Funções do Servidor
// ========================

fn native_ws_listen(args: &[RuntimeValue]) -> Result<RuntimeValue, RuntimeError> {
    let port = match &args[0] {
        RuntimeValue::Number(n) => *n as u16,
        _ => return Err(RuntimeError::TypeError("native_ws_listen: argumento deve ser número".to_string())),
    };
    let addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&addr).map_err(|e| RuntimeError::NetworkError(e.to_string()))?;

    let mut counter = WS_COUNTER.lock().unwrap();
    let socket_id = *counter;
    *counter += 1;
    drop(counter);

    WS_SERVERS.lock().unwrap().insert(socket_id, listener);

    Ok(RuntimeValue::Number(socket_id as f64))
}

// Precisa de correção
// fn native_ws_accept(args: &[RuntimeValue]) -> Result<RuntimeValue, RuntimeError> {
//     let socket_id = match &args[0] {
//         RuntimeValue::Number(n) => *n as u64,
//         _ => return Err(RuntimeError::TypeError("native_ws_accept: argumento deve ser número".to_string())),
//     };

//     // Recupera o listener do hashmap
//     let listener = {
//         let servers = WS_SERVERS.lock().unwrap();
//         match servers.get(&socket_id) {
//             Some(l) => l.try_clone().map_err(|e| RuntimeError::NetworkError(e.to_string()))?,
//             None => return Err(RuntimeError::ArgumentError("Socket não encontrado".to_string())),
//         }
//     };

//     // Aceita a conexão
//     match listener.accept() {
//         Ok((stream, _)) => {
//             // Cria o WebSocket diretamente com o stream (sem aceitar duas vezes)
//             let stream = MaybeTlsStream::Plain(stream);
//             let ws = tungstenite::WebSocket::from_raw_socket(
//                 stream,
//                 tungstenite::protocol::Role::Server,
//                 None,
//             );

//             let mut counter = WS_COUNTER.lock().unwrap();
//             let client_id = *counter;
//             *counter += 1;

//             WS_CLIENTS.lock().unwrap().insert(client_id, WsSocket::Plain(ws));

//             Ok(RuntimeValue::Number(client_id as f64))
//         }
//         Err(e) => Err(RuntimeError::NetworkError(format!("Erro ao aceitar conexão: {}", e))),
//     }
// }

// As demais funções do servidor podem ser implementadas seguindo o mesmo padrão.

// ========================
// Funções do Cliente
// ========================

fn native_ws_connect(args: &[RuntimeValue]) -> Result<RuntimeValue, RuntimeError> {
    let url = match &args[0] {
        RuntimeValue::String(s) => s,
        _ => return Err(RuntimeError::TypeError("native_ws_connect: argumento deve ser string".to_string())),
    };

    let (ws, _) = connect(url).map_err(|e| RuntimeError::NetworkError(e.to_string()))?;
    let mut counter = WS_COUNTER.lock().unwrap();
    let socket_id = *counter;
    *counter += 1;
    drop(counter);

    WS_CLIENTS.lock().unwrap().insert(socket_id, WsSocket::Tls(ws));
    Ok(RuntimeValue::Number(socket_id as f64))
}

// Envio de mensagem
fn native_ws_send(args: &[RuntimeValue]) -> Result<RuntimeValue, RuntimeError> {
    let socket_id = match &args[0] {
        RuntimeValue::Number(n) => *n as u64,
        _ => return Err(RuntimeError::TypeError("native_ws_send: primeiro argumento deve ser número".to_string())),
    };
    let message = match &args[1] {
        RuntimeValue::String(s) => Message::Text(s.clone()),
        RuntimeValue::Array(arr) => {
            let bytes: Vec<u8> = arr.iter().filter_map(|v| match v {
                RuntimeValue::Number(n) => Some(*n as u8),
                _ => None,
            }).collect();
            Message::Binary(bytes)
        }
        _ => return Err(RuntimeError::TypeError("native_ws_send: segundo argumento deve ser string ou array de bytes".to_string())),
    };

    let mut clients = WS_CLIENTS.lock().unwrap();
    if let Some(socket) = clients.get_mut(&socket_id) {
        match socket {
            WsSocket::Plain(ws) => ws.send(message).map_err(|e| RuntimeError::NetworkError(e.to_string()))?,
            WsSocket::Tls(ws) => ws.send(message).map_err(|e| RuntimeError::NetworkError(e.to_string()))?,
        }
        Ok(RuntimeValue::Null)
    } else {
        Err(RuntimeError::ArgumentError("Socket não encontrado".to_string()))
    }
}

// Recebimento de mensagem
fn native_ws_recv(args: &[RuntimeValue]) -> Result<RuntimeValue, RuntimeError> {
    let socket_id = match &args[0] {
        RuntimeValue::Number(n) => *n as u64,
        _ => return Err(RuntimeError::TypeError("native_ws_recv: argumento deve ser número".to_string())),
    };

    let mut clients = WS_CLIENTS.lock().unwrap();
    if let Some(ws) = clients.get_mut(&socket_id) {
        #[allow(deprecated)]
        let msg_result = match ws {
            WsSocket::Plain(ws) => ws.read_message(),
            WsSocket::Tls(ws) => ws.read_message(),
        };
        match msg_result {
            Ok(msg) => match msg {
                Message::Text(s) => Ok(RuntimeValue::String(s)),
                Message::Binary(b) => Ok(RuntimeValue::Array(b.into_iter().map(|x| RuntimeValue::Number(x as f64)).collect())),
                _ => Ok(RuntimeValue::Null),
            },
            Err(_) => Ok(RuntimeValue::Null),
        }
    } else {
        Err(RuntimeError::ArgumentError("Socket não encontrado".to_string()))
    }
}

// Fechamento de conexão
fn native_ws_close(args: &[RuntimeValue]) -> Result<RuntimeValue, RuntimeError> {
    let socket_id = match &args[0] {
        RuntimeValue::Number(n) => *n as u64,
        _ => return Err(RuntimeError::TypeError("native_ws_close: argumento deve ser número".to_string())),
    };

    let mut clients = WS_CLIENTS.lock().unwrap();
    if let Some(ws) = clients.remove(&socket_id) {
        match ws {
            WsSocket::Plain(mut ws) => { ws.close(None).ok(); }
            WsSocket::Tls(mut ws) => { ws.close(None).ok(); }
        }
        Ok(RuntimeValue::Null)
    } else {
        Err(RuntimeError::ArgumentError("Socket não encontrado".to_string()))
    }
}

// As demais funções (broadcast, timeout, nodelay, keepalive, reuseaddr, etc.) podem ser implementadas seguindo o mesmo padrão.

// ========================
// Funções utilitárias e placeholders
// ========================

fn native_ws_send_all(_args: &[RuntimeValue]) -> Result<RuntimeValue, RuntimeError> {
    // Implementação: enviar para todos os clientes conectados
    Ok(RuntimeValue::Null)
}

fn native_ws_broadcast(_args: &[RuntimeValue]) -> Result<RuntimeValue, RuntimeError> {
    Ok(RuntimeValue::Null)
}

fn native_ws_broadcast_except(_args: &[RuntimeValue]) -> Result<RuntimeValue, RuntimeError> {
    Ok(RuntimeValue::Null)
}

fn native_ws_close_all(_args: &[RuntimeValue]) -> Result<RuntimeValue, RuntimeError> {
    Ok(RuntimeValue::Null)
}

fn native_ws_close_client(_args: &[RuntimeValue]) -> Result<RuntimeValue, RuntimeError> {
    Ok(RuntimeValue::Null)
}

fn native_ws_is_connected_client(_args: &[RuntimeValue]) -> Result<RuntimeValue, RuntimeError> {
    Ok(RuntimeValue::Bool(false))
}

fn native_ws_is_connected(_args: &[RuntimeValue]) -> Result<RuntimeValue, RuntimeError> {
    Ok(RuntimeValue::Bool(false))
}

fn native_ws_set_timeout(_args: &[RuntimeValue]) -> Result<RuntimeValue, RuntimeError> {
    Ok(RuntimeValue::Null)
}

fn native_ws_set_nodelay(_args: &[RuntimeValue]) -> Result<RuntimeValue, RuntimeError> {
    Ok(RuntimeValue::Null)
}

fn native_ws_set_keepalive(_args: &[RuntimeValue]) -> Result<RuntimeValue, RuntimeError> {
    Ok(RuntimeValue::Null)
}

fn native_ws_set_reuseaddr(_args: &[RuntimeValue]) -> Result<RuntimeValue, RuntimeError> {
    Ok(RuntimeValue::Null)
}

fn native_ws_get_clients(_args: &[RuntimeValue]) -> Result<RuntimeValue, RuntimeError> {
    let clients = WS_CLIENTS.lock().unwrap();
    let ids: Vec<RuntimeValue> = clients.keys().map(|id| RuntimeValue::Number(*id as f64)).collect();
    Ok(RuntimeValue::Array(ids))
}

