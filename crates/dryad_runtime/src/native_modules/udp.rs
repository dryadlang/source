use crate::interpreter::Value;
use crate::native_modules::NativeFunction;
use crate::errors::RuntimeError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use std::net::{UdpSocket, ToSocketAddrs};
use std::time::Duration;
use std::thread;
use std::sync::mpsc;

lazy_static! {
    static ref UDP_SERVERS: Arc<Mutex<HashMap<String, ServerInstance>>> = Arc::new(Mutex::new(HashMap::new()));
    static ref UDP_CLIENTS: Arc<Mutex<HashMap<String, ClientInstance>>> = Arc::new(Mutex::new(HashMap::new()));
}

#[derive(Clone, Debug)]
struct ServerInstance {
    server_id: String,
    host: String,
    port: u16,
    is_running: bool,
    stop_sender: Option<mpsc::Sender<()>>,
}

#[derive(Clone, Debug)]
struct ClientInstance {
    client_id: String,
    host: String,
    port: u16,
    socket: Option<Arc<UdpSocket>>,
    timeout_secs: u64,
    is_bound: bool,
}

/// Função para inicializar servidor UDP usando threads nativas
fn start_udp_server(server_id: String, host: String, port: u16, stop_receiver: mpsc::Receiver<()>) {
    let bind_addr = format!("{}:{}", host, port);
    
    match UdpSocket::bind(&bind_addr) {
        Ok(socket) => {
            println!("🌐 UDP Server '{}' iniciado em {}", server_id, bind_addr);
            
            // Configurar timeout para operações não-bloqueantes
            socket.set_read_timeout(Some(Duration::from_millis(100))).ok();
            
            let mut buffer = [0; 1024];
            
            loop {
                // Verificar se deve parar o servidor
                if stop_receiver.try_recv().is_ok() {
                    println!("🛑 UDP Server '{}' parando...", server_id);
                    break;
                }
                
                // Tentar receber dados
                match socket.recv_from(&mut buffer) {
                    Ok((size, addr)) => {
                        let data = String::from_utf8_lossy(&buffer[..size]);
                        println!("📦 UDP Server '{}': Recebido de {}: {}", server_id, addr, data);
                        
                        // Echo - responder com os mesmos dados
                        let response = format!("Echo: {}", data);
                        if let Err(e) = socket.send_to(response.as_bytes(), addr) {
                            eprintln!("❌ UDP Server '{}': Erro ao enviar resposta para {}: {}", server_id, addr, e);
                        } else {
                            println!("📤 UDP Server '{}': Enviado para {}: {}", server_id, addr, response);
                        }
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock || e.kind() == std::io::ErrorKind::TimedOut => {
                        // Timeout normal, continuar
                        continue;
                    }
                    Err(e) => {
                        eprintln!("❌ UDP Server '{}': Erro ao receber dados: {}", server_id, e);
                        break;
                    }
                }
            }
            
            println!("👋 UDP Server '{}' finalizado", server_id);
        }
        Err(e) => {
            eprintln!("❌ UDP Server '{}': Erro ao fazer bind em {}: {}", server_id, bind_addr, e);
        }
    }
}

pub fn register_udp_functions(functions: &mut HashMap<String, NativeFunction>) {
    // Servidor UDP
    functions.insert("udp_server_create".to_string(), native_udp_server_create);
    functions.insert("udp_server_start".to_string(), native_udp_server_start);
    functions.insert("udp_server_stop".to_string(), native_udp_server_stop);
    functions.insert("udp_server_status".to_string(), native_udp_server_status);
    
    // Cliente UDP
    functions.insert("udp_client_create".to_string(), native_udp_client_create);
    functions.insert("udp_client_bind".to_string(), native_udp_client_bind);
    functions.insert("udp_client_send".to_string(), native_udp_client_send);
    functions.insert("udp_client_receive".to_string(), native_udp_client_receive);
    functions.insert("udp_client_send_to".to_string(), native_udp_client_send_to);
    functions.insert("udp_client_receive_from".to_string(), native_udp_client_receive_from);
    functions.insert("udp_client_status".to_string(), native_udp_client_status);
    functions.insert("udp_client_set_timeout".to_string(), native_udp_client_set_timeout);
    functions.insert("udp_client_close".to_string(), native_udp_client_close);
    
    // Utilitários
    functions.insert("udp_resolve_hostname".to_string(), native_udp_resolve_hostname);
    functions.insert("udp_get_local_ip".to_string(), native_udp_get_local_ip);
    functions.insert("udp_port_available".to_string(), native_udp_port_available);
}

// ========================
// Funções de servidor UDP
// ========================

/// native_udp_server_create(server_id, host?, port?) -> null
/// Cria uma nova instância de servidor UDP
fn native_udp_server_create(args: &[Value], _manager: &crate::native_modules::NativeModuleManager) -> Result<Value, RuntimeError> {
    if args.is_empty() || args.len() > 3 {
        return Err(RuntimeError::ArgumentError("udp_server_create requer 1-3 argumentos: server_id, host (opcional), port (opcional)".to_string()));
    }

    let server_id = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("server_id deve ser uma string".to_string())),
    };

    let host = if args.len() > 1 {
        match &args[1] {
            Value::String(s) => s.clone(),
            _ => return Err(RuntimeError::TypeError("host deve ser uma string".to_string())),
        }
    } else {
        "127.0.0.1".to_string()
    };

    let port = if args.len() > 2 {
        match &args[2] {
            Value::Number(n) => *n as u16,
            _ => return Err(RuntimeError::TypeError("port deve ser um número".to_string())),
        }
    } else {
        8080
    };

    let server = ServerInstance {
        server_id: server_id.clone(),
        host: host.clone(),
        port,
        is_running: false,
        stop_sender: None,
    };

    let mut servers = UDP_SERVERS.lock().unwrap();
    servers.insert(server_id.clone(), server);

    println!("✅ UDP Server '{}' criado para {}:{}", server_id, host, port);
    Ok(Value::Null)
}

/// native_udp_server_start(server_id) -> null  
/// Inicia o servidor UDP especificado
fn native_udp_server_start(args: &[Value], _manager: &crate::native_modules::NativeModuleManager) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("udp_server_start requer exatamente 1 argumento: server_id".to_string()));
    }

    let server_id = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("server_id deve ser uma string".to_string())),
    };

    let (host, port, stop_sender) = {
        let mut servers = UDP_SERVERS.lock().unwrap();
        match servers.get_mut(&server_id) {
            Some(server) => {
                if server.is_running {
                    return Err(RuntimeError::NetworkError(format!("UDP Server '{}' já está rodando", server_id)));
                }

                let (sender, receiver) = mpsc::channel();
                server.stop_sender = Some(sender);
                server.is_running = true;
                
                (server.host.clone(), server.port, receiver)
            }
            None => return Err(RuntimeError::NetworkError(format!("UDP Server '{}' não encontrado", server_id))),
        }
    };

    // Iniciar servidor em thread separada
    let server_id_clone = server_id.clone();
    thread::spawn(move || {
        start_udp_server(server_id_clone, host, port, stop_sender);
    });

    Ok(Value::Null)
}

/// native_udp_server_stop(server_id) -> null
/// Para o servidor UDP especificado
fn native_udp_server_stop(args: &[Value], _manager: &crate::native_modules::NativeModuleManager) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("udp_server_stop requer exatamente 1 argumento: server_id".to_string()));
    }

    let server_id = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("server_id deve ser uma string".to_string())),
    };

    let mut servers = UDP_SERVERS.lock().unwrap();
    match servers.get_mut(&server_id) {
        Some(server) => {
            if !server.is_running {
                return Err(RuntimeError::NetworkError(format!("UDP Server '{}' não está rodando", server_id)));
            }

            if let Some(sender) = server.stop_sender.take() {
                sender.send(()).ok();
            }
            server.is_running = false;
            
            println!("🛑 UDP Server '{}' parado", server_id);
            Ok(Value::Null)
        }
        None => Err(RuntimeError::NetworkError(format!("UDP Server '{}' não encontrado", server_id))),
    }
}

/// native_udp_server_status(server_id) -> objeto
/// Retorna o status do servidor UDP
fn native_udp_server_status(args: &[Value], _manager: &crate::native_modules::NativeModuleManager) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("udp_server_status requer exatamente 1 argumento: server_id".to_string()));
    }

    let server_id = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("server_id deve ser uma string".to_string())),
    };

    let servers = UDP_SERVERS.lock().unwrap();
    match servers.get(&server_id) {
        Some(server) => {
            let mut status = HashMap::new();
            status.insert("server_id".to_string(), Value::String(server.server_id.clone()));
            status.insert("host".to_string(), Value::String(server.host.clone()));
            status.insert("port".to_string(), Value::Number(server.port as f64));
            status.insert("is_running".to_string(), Value::Bool(server.is_running));
            
            Ok(Value::Object { properties: status, methods: HashMap::new() })
        }
        None => Err(RuntimeError::NetworkError(format!("UDP Server '{}' não encontrado", server_id))),
    }
}

// ========================
// Funções de cliente UDP
// ========================

/// native_udp_client_create(client_id, host?, port?) -> null
/// Cria uma nova instância de cliente UDP
fn native_udp_client_create(args: &[Value], _manager: &crate::native_modules::NativeModuleManager) -> Result<Value, RuntimeError> {
    if args.is_empty() || args.len() > 3 {
        return Err(RuntimeError::ArgumentError("udp_client_create requer 1-3 argumentos: client_id, host (opcional), port (opcional)".to_string()));
    }

    let client_id = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("client_id deve ser uma string".to_string())),
    };

    let host = if args.len() > 1 {
        match &args[1] {
            Value::String(s) => s.clone(),
            _ => return Err(RuntimeError::TypeError("host deve ser uma string".to_string())),
        }
    } else {
        "127.0.0.1".to_string()
    };

    let port = if args.len() > 2 {
        match &args[2] {
            Value::Number(n) => *n as u16,
            _ => return Err(RuntimeError::TypeError("port deve ser um número".to_string())),
        }
    } else {
        8080
    };

    let client = ClientInstance {
        client_id: client_id.clone(),
        host: host.clone(),
        port,
        socket: None,
        timeout_secs: 5,
        is_bound: false,
    };

    let mut clients = UDP_CLIENTS.lock().unwrap();
    clients.insert(client_id.clone(), client);

    println!("✅ UDP Client '{}' criado para {}:{}", client_id, host, port);
    Ok(Value::Null)
}

/// native_udp_client_bind(client_id, local_port?) -> bool
/// Faz bind do cliente UDP a uma porta local
fn native_udp_client_bind(args: &[Value], _manager: &crate::native_modules::NativeModuleManager) -> Result<Value, RuntimeError> {
    if args.is_empty() || args.len() > 2 {
        return Err(RuntimeError::ArgumentError("udp_client_bind requer 1-2 argumentos: client_id, local_port (opcional)".to_string()));
    }

    let client_id = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("client_id deve ser uma string".to_string())),
    };

    let local_port = if args.len() > 1 {
        match &args[1] {
            Value::Number(n) => *n as u16,
            _ => return Err(RuntimeError::TypeError("local_port deve ser um número".to_string())),
        }
    } else {
        0 // Deixar o sistema escolher
    };

    let mut clients = UDP_CLIENTS.lock().unwrap();
    match clients.get_mut(&client_id) {
        Some(client) => {
            let bind_addr = format!("0.0.0.0:{}", local_port);
            
            match UdpSocket::bind(&bind_addr) {
                Ok(socket) => {
                    socket.set_read_timeout(Some(Duration::from_secs(client.timeout_secs))).ok();
                    client.socket = Some(Arc::new(socket));
                    client.is_bound = true;
                    
                    println!("🔗 UDP Client '{}' vinculado à porta local {}", client_id, local_port);
                    Ok(Value::Bool(true))
                }
                Err(e) => {
                    eprintln!("❌ UDP Client '{}': Erro ao fazer bind: {}", client_id, e);
                    Ok(Value::Bool(false))
                }
            }
        }
        None => Err(RuntimeError::NetworkError(format!("UDP Client '{}' não encontrado", client_id))),
    }
}

/// native_udp_client_send(client_id, message) -> bool
/// Envia dados para o servidor configurado
fn native_udp_client_send(args: &[Value], _manager: &crate::native_modules::NativeModuleManager) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::ArgumentError("udp_client_send requer exatamente 2 argumentos: client_id, message".to_string()));
    }

    let client_id = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("client_id deve ser uma string".to_string())),
    };

    let message = match &args[1] {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        _ => return Err(RuntimeError::TypeError("message deve ser uma string, número ou bool".to_string())),
    };

    let clients = UDP_CLIENTS.lock().unwrap();
    match clients.get(&client_id) {
        Some(client) => {
            if let Some(socket) = &client.socket {
                let addr = format!("{}:{}", client.host, client.port);
                match socket.send_to(message.as_bytes(), &addr) {
                    Ok(_) => {
                        println!("📤 UDP Client '{}' enviou dados: {}", client_id, message);
                        Ok(Value::Bool(true))
                    }
                    Err(e) => {
                        eprintln!("❌ UDP Client '{}': Erro ao enviar dados: {}", client_id, e);
                        Ok(Value::Bool(false))
                    }
                }
            } else {
                Err(RuntimeError::NetworkError(format!("UDP Client '{}' não está vinculado", client_id)))
            }
        }
        None => Err(RuntimeError::NetworkError(format!("UDP Client '{}' não encontrado", client_id))),
    }
}

/// native_udp_client_receive(client_id) -> string
/// Recebe dados do socket UDP (do último remetente)
fn native_udp_client_receive(args: &[Value], _manager: &crate::native_modules::NativeModuleManager) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("udp_client_receive requer exatamente 1 argumento: client_id".to_string()));
    }

    let client_id = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("client_id deve ser uma string".to_string())),
    };

    let clients = UDP_CLIENTS.lock().unwrap();
    match clients.get(&client_id) {
        Some(client) => {
            if let Some(socket) = &client.socket {
                let mut buffer = [0; 1024];
                match socket.recv_from(&mut buffer) {
                    Ok((size, _addr)) => {
                        let data = String::from_utf8_lossy(&buffer[..size]).to_string();
                        println!("📥 UDP Client '{}' recebeu dados: {}", client_id, data);
                        Ok(Value::String(data))
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock || e.kind() == std::io::ErrorKind::TimedOut => {
                        Ok(Value::String("".to_string()))
                    }
                    Err(e) => {
                        eprintln!("❌ UDP Client '{}': Erro ao receber dados: {}", client_id, e);
                        Ok(Value::String("".to_string()))
                    }
                }
            } else {
                Err(RuntimeError::NetworkError(format!("UDP Client '{}' não está vinculado", client_id)))
            }
        }
        None => Err(RuntimeError::NetworkError(format!("UDP Client '{}' não encontrado", client_id))),
    }
}

/// native_udp_client_send_to(client_id, message, host, port) -> bool
/// Envia dados para um endereço específico
fn native_udp_client_send_to(args: &[Value], _manager: &crate::native_modules::NativeModuleManager) -> Result<Value, RuntimeError> {
    if args.len() != 4 {
        return Err(RuntimeError::ArgumentError("udp_client_send_to requer exatamente 4 argumentos: client_id, message, host, port".to_string()));
    }

    let client_id = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("client_id deve ser uma string".to_string())),
    };

    let message = match &args[1] {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        _ => return Err(RuntimeError::TypeError("message deve ser uma string, número ou bool".to_string())),
    };

    let host = match &args[2] {
        Value::String(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("host deve ser uma string".to_string())),
    };

    let port = match &args[3] {
        Value::Number(n) => *n as u16,
        _ => return Err(RuntimeError::TypeError("port deve ser um número".to_string())),
    };

    let clients = UDP_CLIENTS.lock().unwrap();
    match clients.get(&client_id) {
        Some(client) => {
            if let Some(socket) = &client.socket {
                let addr = format!("{}:{}", host, port);
                match socket.send_to(message.as_bytes(), &addr) {
                    Ok(_) => {
                        println!("📤 UDP Client '{}' enviou dados para {}: {}", client_id, addr, message);
                        Ok(Value::Bool(true))
                    }
                    Err(e) => {
                        eprintln!("❌ UDP Client '{}': Erro ao enviar dados para {}: {}", client_id, addr, e);
                        Ok(Value::Bool(false))
                    }
                }
            } else {
                Err(RuntimeError::NetworkError(format!("UDP Client '{}' não está vinculado", client_id)))
            }
        }
        None => Err(RuntimeError::NetworkError(format!("UDP Client '{}' não encontrado", client_id))),
    }
}

/// native_udp_client_receive_from(client_id) -> objeto
/// Recebe dados e retorna objeto com dados e informações do remetente
fn native_udp_client_receive_from(args: &[Value], _manager: &crate::native_modules::NativeModuleManager) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("udp_client_receive_from requer exatamente 1 argumento: client_id".to_string()));
    }

    let client_id = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("client_id deve ser uma string".to_string())),
    };

    let clients = UDP_CLIENTS.lock().unwrap();
    match clients.get(&client_id) {
        Some(client) => {
            if let Some(socket) = &client.socket {
                let mut buffer = [0; 1024];
                match socket.recv_from(&mut buffer) {
                    Ok((size, addr)) => {
                        let data = String::from_utf8_lossy(&buffer[..size]).to_string();
                        let addr_str = addr.to_string();
                        
                        println!("📥 UDP Client '{}' recebeu de {}: {}", client_id, addr, data);
                        
                        let mut result = HashMap::new();
                        result.insert("data".to_string(), Value::String(data));
                        result.insert("sender".to_string(), Value::String(addr_str));
                        
                        Ok(Value::Object { properties: result, methods: HashMap::new() })
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock || e.kind() == std::io::ErrorKind::TimedOut => {
                        let mut result = HashMap::new();
                        result.insert("data".to_string(), Value::String("".to_string()));
                        result.insert("sender".to_string(), Value::String("".to_string()));
                        Ok(Value::Object { properties: result, methods: HashMap::new() })
                    }
                    Err(e) => {
                        eprintln!("❌ UDP Client '{}': Erro ao receber dados: {}", client_id, e);
                        let mut result = HashMap::new();
                        result.insert("data".to_string(), Value::String("".to_string()));
                        result.insert("sender".to_string(), Value::String("".to_string()));
                        Ok(Value::Object { properties: result, methods: HashMap::new() })
                    }
                }
            } else {
                Err(RuntimeError::NetworkError(format!("UDP Client '{}' não está vinculado", client_id)))
            }
        }
        None => Err(RuntimeError::NetworkError(format!("UDP Client '{}' não encontrado", client_id))),
    }
}

/// native_udp_client_status(client_id) -> objeto
/// Retorna o status do cliente UDP
fn native_udp_client_status(args: &[Value], _manager: &crate::native_modules::NativeModuleManager) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("udp_client_status requer exatamente 1 argumento: client_id".to_string()));
    }

    let client_id = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("client_id deve ser uma string".to_string())),
    };

    let clients = UDP_CLIENTS.lock().unwrap();
    match clients.get(&client_id) {
        Some(client) => {
            let mut status = HashMap::new();
            status.insert("client_id".to_string(), Value::String(client.client_id.clone()));
            status.insert("host".to_string(), Value::String(client.host.clone()));
            status.insert("port".to_string(), Value::Number(client.port as f64));
            status.insert("timeout_secs".to_string(), Value::Number(client.timeout_secs as f64));
            status.insert("is_bound".to_string(), Value::Bool(client.is_bound));
            
            Ok(Value::Object { properties: status, methods: HashMap::new() })
        }
        None => Err(RuntimeError::NetworkError(format!("UDP Client '{}' não encontrado", client_id))),
    }
}

/// native_udp_client_set_timeout(client_id, timeout_secs) -> null
/// Define o timeout para operações de recepção
fn native_udp_client_set_timeout(args: &[Value], _manager: &crate::native_modules::NativeModuleManager) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::ArgumentError("udp_client_set_timeout requer exatamente 2 argumentos: client_id, timeout_secs".to_string()));
    }

    let client_id = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("client_id deve ser uma string".to_string())),
    };

    let timeout_secs = match &args[1] {
        Value::Number(n) => *n as u64,
        _ => return Err(RuntimeError::TypeError("timeout_secs deve ser um número".to_string())),
    };

    let mut clients = UDP_CLIENTS.lock().unwrap();
    match clients.get_mut(&client_id) {
        Some(client) => {
            client.timeout_secs = timeout_secs;
            
            // Atualizar timeout do socket se existir
            if let Some(socket) = &client.socket {
                socket.set_read_timeout(Some(Duration::from_secs(timeout_secs))).ok();
            }
            
            println!("✅ UDP Client '{}' timeout configurado para {} segundos", client_id, timeout_secs);
            Ok(Value::Null)
        }
        None => Err(RuntimeError::NetworkError(format!("UDP Client '{}' não encontrado", client_id))),
    }
}

/// native_udp_client_close(client_id) -> null
/// Fecha o cliente UDP
fn native_udp_client_close(args: &[Value], _manager: &crate::native_modules::NativeModuleManager) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("udp_client_close requer exatamente 1 argumento: client_id".to_string()));
    }

    let client_id = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("client_id deve ser uma string".to_string())),
    };

    let mut clients = UDP_CLIENTS.lock().unwrap();
    match clients.remove(&client_id) {
        Some(_) => {
            println!("👋 UDP Client '{}' fechado", client_id);
            Ok(Value::Null)
        }
        None => Err(RuntimeError::NetworkError(format!("UDP Client '{}' não encontrado", client_id))),
    }
}

// ========================
// Funções utilitárias
// ========================

/// native_udp_resolve_hostname(hostname) -> string
/// Resolve um hostname para endereço IP
fn native_udp_resolve_hostname(args: &[Value], _manager: &crate::native_modules::NativeModuleManager) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("udp_resolve_hostname requer exatamente 1 argumento: hostname".to_string()));
    }

    let hostname = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("hostname deve ser uma string".to_string())),
    };

    match format!("{}:0", hostname).to_socket_addrs() {
        Ok(mut addrs) => {
            if let Some(addr) = addrs.next() {
                let ip = addr.ip().to_string();
                println!("🔍 Hostname '{}' resolvido para IP: {}", hostname, ip);
                Ok(Value::String(ip))
            } else {
                Err(RuntimeError::NetworkError(format!("Não foi possível resolver o hostname: {}", hostname)))
            }
        }
        Err(e) => Err(RuntimeError::NetworkError(format!("Erro ao resolver hostname '{}': {}", hostname, e))),
    }
}

/// native_udp_get_local_ip() -> string
/// Retorna o IP local da máquina
fn native_udp_get_local_ip(args: &[Value], _manager: &crate::native_modules::NativeModuleManager) -> Result<Value, RuntimeError> {
    if !args.is_empty() {
        return Err(RuntimeError::ArgumentError("udp_get_local_ip não requer argumentos".to_string()));
    }

    // Conectar a um endereço externo para descobrir IP local
    match UdpSocket::bind("0.0.0.0:0") {
        Ok(socket) => {
            match socket.connect("8.8.8.8:80") {
                Ok(_) => {
                    match socket.local_addr() {
                        Ok(addr) => {
                            let ip = addr.ip().to_string();
                            Ok(Value::String(ip))
                        }
                        Err(e) => Err(RuntimeError::NetworkError(format!("Erro ao obter endereço local: {}", e))),
                    }
                }
                Err(e) => Err(RuntimeError::NetworkError(format!("Erro ao conectar para descobrir IP local: {}", e))),
            }
        }
        Err(e) => Err(RuntimeError::NetworkError(format!("Erro ao criar socket UDP: {}", e))),
    }
}

/// native_udp_port_available(port) -> bool
/// Verifica se uma porta está disponível para bind
fn native_udp_port_available(args: &[Value], _manager: &crate::native_modules::NativeModuleManager) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("udp_port_available requer exatamente 1 argumento: port".to_string()));
    }

    let port = match &args[0] {
        Value::Number(n) => *n as u16,
        _ => return Err(RuntimeError::TypeError("port deve ser um número".to_string())),
    };

    let addr = format!("127.0.0.1:{}", port);
    match UdpSocket::bind(&addr) {
        Ok(_) => {
            println!("✅ Porta {} está disponível", port);
            Ok(Value::Bool(true))
        }
        Err(_) => {
            println!("❌ Porta {} não está disponível", port);
            Ok(Value::Bool(false))
        }
    }
}
