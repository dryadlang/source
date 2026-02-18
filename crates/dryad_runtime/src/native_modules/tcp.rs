use crate::interpreter::Value;
use crate::native_modules::NativeFunction;
use crate::errors::RuntimeError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::{Read, Write, BufRead, BufReader};
use std::thread;
use std::time::Duration;
use std::sync::mpsc;

lazy_static! {
    static ref TCP_SERVERS: Arc<Mutex<HashMap<String, ServerInstance>>> = Arc::new(Mutex::new(HashMap::new()));
    static ref TCP_CLIENTS: Arc<Mutex<HashMap<String, ClientInstance>>> = Arc::new(Mutex::new(HashMap::new()));
}

#[derive(Clone)]
struct ServerInstance {
    server_id: String,
    host: String,
    port: u16,
    is_running: bool,
    stop_sender: Option<mpsc::Sender<()>>,
    max_clients: usize,
}

struct ClientInstance {
    client_id: String,
    host: String,
    port: u16,
    stream: Arc<Mutex<Option<TcpStream>>>,
    timeout: Option<Duration>,
}

/// Função para inicializar servidor TCP usando threads nativas
fn start_tcp_server(server_id: String, host: String, port: u16, max_clients: usize, stop_receiver: mpsc::Receiver<()>) {
    thread::spawn(move || {
        let bind_address = format!("{}:{}", host, port);
        
        match TcpListener::bind(&bind_address) {
            Ok(listener) => {
                println!("🌍 TCP Server '{}' iniciado em {}", server_id, bind_address);
                
                if let Err(e) = listener.set_nonblocking(true) {
                    eprintln!("❌ Erro ao configurar servidor não-bloqueante: {}", e);
                    return;
                }
                
                let mut client_count = 0;
                
                loop {
                    if stop_receiver.try_recv().is_ok() {
                        println!("🛑 TCP Server '{}' parando...", server_id);
                        break;
                    }
                    
                    match listener.accept() {
                        Ok((stream, addr)) => {
                            if client_count >= max_clients {
                                println!("⚠️ TCP Server '{}': Limite de clientes ({}) atingido, rejeitando {}", 
                                        server_id, max_clients, addr);
                                continue;
                            }
                            
                            client_count += 1;
                            let server_id_clone = server_id.clone();
                            
                            thread::spawn(move || {
                                handle_tcp_client(stream, addr, server_id_clone, client_count);
                            });
                            
                            println!("🔗 TCP Server '{}': Cliente {} conectado de {}", 
                                    server_id, client_count, addr);
                        }
                        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                            thread::sleep(Duration::from_millis(100));
                        }
                        Err(e) => {
                            eprintln!("❌ TCP Server '{}': Erro ao aceitar conexão: {}", server_id, e);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("❌ Erro ao iniciar TCP Server '{}' em {}: {}", server_id, bind_address, e);
            }
        }
    });
}

fn handle_tcp_client(mut stream: TcpStream, addr: SocketAddr, server_id: String, client_num: usize) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut line = String::new();
    
    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) => {
                println!("👋 TCP Server '{}': Cliente {} de {} desconectou", server_id, client_num, addr);
                break;
            }
            Ok(_) => {
                let response = format!("Echo from TCP Server '{}': received {}", server_id, line);
                if let Err(e) = stream.write_all(response.as_bytes()) {
                    eprintln!("❌ TCP Server '{}': Erro ao enviar resposta: {}", server_id, e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("❌ TCP Server '{}': Erro ao ler dados: {}", server_id, e);
                break;
            }
        }
    }
}

pub fn register_tcp_functions(functions: &mut HashMap<String, NativeFunction>) {
    functions.insert("tcp_server_create".to_string(), native_tcp_server_create);
    functions.insert("tcp_server_start".to_string(), native_tcp_server_start);
    functions.insert("tcp_server_stop".to_string(), native_tcp_server_stop);
    functions.insert("tcp_server_status".to_string(), native_tcp_server_status);
    
    functions.insert("tcp_client_create".to_string(), native_tcp_client_create);
    functions.insert("tcp_client_connect".to_string(), native_tcp_client_connect);
    functions.insert("tcp_client_disconnect".to_string(), native_tcp_client_disconnect);
    functions.insert("tcp_client_send".to_string(), native_tcp_client_send);
    functions.insert("tcp_client_receive".to_string(), native_tcp_client_receive);
    functions.insert("tcp_client_status".to_string(), native_tcp_client_status);

    functions.insert("tcp_resolve_hostname".to_string(), native_tcp_resolve_hostname);
    functions.insert("tcp_get_local_ip".to_string(), native_tcp_get_local_ip);
}

// ========================
// Funções de servidor TCP
// ========================

fn native_tcp_server_create(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentError("tcp_server_create(id, [host], [port], [max]) requerido".to_string()));
    }
    
    let server_id = match &args[0] { Value::String(id) => id.clone(), _ => return Err(RuntimeError::TypeError("id deve ser string".to_string())) };
    let host = if args.len() > 1 { match &args[1] { Value::String(h) => h.clone(), _ => "127.0.0.1".to_string() } } else { "127.0.0.1".to_string() };
    let port = if args.len() > 2 { match &args[2] { Value::Number(p) => *p as u16, _ => 8080 } } else { 8080 };
    let max_clients = if args.len() > 3 { match &args[3] { Value::Number(mc) => *mc as usize, _ => 10 } } else { 10 };
    
    let server_instance = ServerInstance {
        server_id: server_id.clone(),
        host,
        port,
        is_running: false,
        stop_sender: None,
        max_clients,
    };
    
    TCP_SERVERS.lock().unwrap().insert(server_id, server_instance);
    Ok(Value::Null)
}

fn native_tcp_server_start(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    let server_id = match &args[0] { Value::String(id) => id, _ => return Err(RuntimeError::TypeError("id deve ser string".to_string())) };
    
    let mut servers = TCP_SERVERS.lock().unwrap();
    let server = servers.get_mut(server_id).ok_or_else(|| RuntimeError::ArgumentError("Servidor não encontrado".to_string()))?;
    
    if server.is_running { return Ok(Value::Null); }
    
    let (stop_sender, stop_receiver) = mpsc::channel();
    server.stop_sender = Some(stop_sender);
    server.is_running = true;
    
    start_tcp_server(server.server_id.clone(), server.host.clone(), server.port, server.max_clients, stop_receiver);
    Ok(Value::Null)
}

fn native_tcp_server_stop(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    let server_id = match &args[0] { Value::String(id) => id, _ => return Err(RuntimeError::TypeError("id deve ser string".to_string())) };
    
    let mut servers = TCP_SERVERS.lock().unwrap();
    let server = servers.get_mut(server_id).ok_or_else(|| RuntimeError::ArgumentError("Servidor não encontrado".to_string()))?;
    
    if let Some(sender) = server.stop_sender.take() {
        let _ = sender.send(());
    }
    server.is_running = false;
    Ok(Value::Null)
}

fn native_tcp_server_status(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    let server_id = match &args[0] { Value::String(id) => id, _ => return Err(RuntimeError::TypeError("id deve ser string".to_string())) };
    
    let servers = TCP_SERVERS.lock().unwrap();
    let server = servers.get(server_id).ok_or_else(|| RuntimeError::ArgumentError("Servidor não encontrado".to_string()))?;
    
    let mut props = HashMap::new();
    props.insert("id".to_string(), Value::String(server.server_id.clone()));
    props.insert("is_running".to_string(), Value::Bool(server.is_running));
    props.insert("port".to_string(), Value::Number(server.port as f64));
    
    let id = heap.allocate(crate::heap::ManagedObject::Object { properties: props, methods: HashMap::new() });
    Ok(Value::Object(id))
}

// ========================
// Funções de cliente TCP
// ========================

fn native_tcp_client_create(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.is_empty() { return Err(RuntimeError::ArgumentError("tcp_client_create(id, host, port)".to_string())); }
    let client_id = match &args[0] { Value::String(id) => id.clone(), _ => return Err(RuntimeError::TypeError("id deve ser string".to_string())) };
    let host = match &args[1] { Value::String(h) => h.clone(), _ => "127.0.0.1".to_string() };
    let port = match &args[2] { Value::Number(p) => *p as u16, _ => 8080 };
    
    let client = ClientInstance {
        client_id: client_id.clone(),
        host,
        port,
        stream: Arc::new(Mutex::new(None)),
        timeout: Some(Duration::from_secs(30)),
    };
    
    TCP_CLIENTS.lock().unwrap().insert(client_id, client);
    Ok(Value::Null)
}

fn native_tcp_client_connect(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    let client_id = match &args[0] { Value::String(id) => id, _ => return Err(RuntimeError::TypeError("id deve ser string".to_string())) };
    let mut clients = TCP_CLIENTS.lock().unwrap();
    let client = clients.get_mut(client_id).ok_or_else(|| RuntimeError::ArgumentError("Cliente não encontrado".to_string()))?;
    
    let mut stream_lock = client.stream.lock().unwrap();
    if stream_lock.is_some() { return Ok(Value::Bool(true)); }
    
    let addr = format!("{}:{}", client.host, client.port);
    match TcpStream::connect_timeout(&addr.parse().map_err(|_| RuntimeError::IoError("Endereço inválido".to_string()))?, client.timeout.unwrap_or(Duration::from_secs(5))) {
        Ok(stream) => { *stream_lock = Some(stream); Ok(Value::Bool(true)) }
        Err(_) => Ok(Value::Bool(false)),
    }
}

fn native_tcp_client_send(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    let client_id = match &args[0] { Value::String(id) => id, _ => return Err(RuntimeError::TypeError("id deve ser string".to_string())) };
    let data = match &args[1] { Value::String(s) => s.as_bytes(), _ => return Err(RuntimeError::TypeError("data deve ser string".to_string())) };
    
    let clients = TCP_CLIENTS.lock().unwrap();
    let client = clients.get(client_id).ok_or_else(|| RuntimeError::ArgumentError("Cliente não encontrado".to_string()))?;
    
    let mut stream_lock = client.stream.lock().unwrap();
    if let Some(ref mut stream) = *stream_lock {
        stream.write_all(data).map(|_| Value::Bool(true)).map_err(|e| RuntimeError::IoError(e.to_string()))
    } else {
        Err(RuntimeError::IoError("Não conectado".to_string()))
    }
}

fn native_tcp_client_receive(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    let client_id = match &args[0] { Value::String(id) => id, _ => return Err(RuntimeError::TypeError("id deve ser string".to_string())) };
    let clients = TCP_CLIENTS.lock().unwrap();
    let client = clients.get(client_id).ok_or_else(|| RuntimeError::ArgumentError("Cliente não encontrado".to_string()))?;
    
    let mut stream_lock = client.stream.lock().unwrap();
    if let Some(ref mut stream) = *stream_lock {
        let mut buffer = [0; 4096];
        match stream.read(&mut buffer) {
            Ok(0) => { *stream_lock = None; Ok(Value::Null) }
            Ok(n) => Ok(Value::String(String::from_utf8_lossy(&buffer[..n]).to_string())),
            Err(e) => Err(RuntimeError::IoError(e.to_string())),
        }
    } else {
        Err(RuntimeError::IoError("Não conectado".to_string()))
    }
}

fn native_tcp_client_disconnect(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    let client_id = match &args[0] { Value::String(id) => id, _ => return Err(RuntimeError::TypeError("id deve ser string".to_string())) };
    let clients = TCP_CLIENTS.lock().unwrap();
    if let Some(client) = clients.get(client_id) {
        let mut stream_lock = client.stream.lock().unwrap();
        *stream_lock = None;
        Ok(Value::Bool(true))
    } else {
        Ok(Value::Bool(false))
    }
}

fn native_tcp_client_status(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    let client_id = match &args[0] { Value::String(id) => id, _ => return Err(RuntimeError::TypeError("id deve ser string".to_string())) };
    let clients = TCP_CLIENTS.lock().unwrap();
    let client = clients.get(client_id).ok_or_else(|| RuntimeError::ArgumentError("Cliente não encontrado".to_string()))?;
    
    let stream_lock = client.stream.lock().unwrap();
    let mut props = HashMap::new();
    props.insert("id".to_string(), Value::String(client.client_id.clone()));
    props.insert("is_connected".to_string(), Value::Bool(stream_lock.is_some()));
    
    let id = heap.allocate(crate::heap::ManagedObject::Object { properties: props, methods: HashMap::new() });
    Ok(Value::Object(id))
}

// ========================
// Funções utilitárias TCP
// ========================

fn native_tcp_resolve_hostname(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    let hostname = match &args[0] { Value::String(h) => h, _ => return Err(RuntimeError::TypeError("host deve ser string".to_string())) };
    match std::net::ToSocketAddrs::to_socket_addrs(&format!("{}:80", hostname)) {
        Ok(mut addrs) => {
            if let Some(addr) = addrs.next() { Ok(Value::String(addr.ip().to_string())) }
            else { Err(RuntimeError::NetworkError("Falha ao resolver".to_string())) }
        }
        Err(e) => Err(RuntimeError::NetworkError(e.to_string())),
    }
}

fn native_tcp_get_local_ip(_args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    let socket = std::net::UdpSocket::bind("0.0.0.0:0").map_err(|e| RuntimeError::NetworkError(e.to_string()))?;
    socket.connect("8.8.8.8:80").map_err(|e| RuntimeError::NetworkError(e.to_string()))?;
    let addr = socket.local_addr().map_err(|e| RuntimeError::NetworkError(e.to_string()))?;
    Ok(Value::String(addr.ip().to_string()))
}