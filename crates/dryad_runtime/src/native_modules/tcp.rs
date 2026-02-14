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

#[derive(Clone, Debug)]
struct ServerInstance {
    server_id: String,
    host: String,
    port: u16,
    is_running: bool,
    stop_sender: Option<mpsc::Sender<()>>,
    max_clients: usize,
}

#[derive(Clone, Debug)]
struct ClientInstance {
    client_id: String,
    host: String,
    port: u16,
    is_connected: bool,
    timeout: Option<Duration>,
}

/// Função para inicializar servidor TCP usando threads nativas
fn start_tcp_server(server_id: String, host: String, port: u16, max_clients: usize, stop_receiver: mpsc::Receiver<()>) {
    thread::spawn(move || {
        let bind_address = format!("{}:{}", host, port);
        
        match TcpListener::bind(&bind_address) {
            Ok(listener) => {
                println!("🌍 TCP Server '{}' iniciado em {}", server_id, bind_address);
                
                // Configura timeout para aceitar conexões
                if let Err(e) = listener.set_nonblocking(true) {
                    eprintln!("❌ Erro ao configurar servidor não-bloqueante: {}", e);
                    return;
                }
                
                let mut client_count = 0;
                
                loop {
                    // Verifica se deve parar
                    if stop_receiver.try_recv().is_ok() {
                        println!("🛑 TCP Server '{}' parando...", server_id);
                        break;
                    }
                    
                    // Tenta aceitar conexão
                    match listener.accept() {
                        Ok((stream, addr)) => {
                            if client_count >= max_clients {
                                println!("⚠️ TCP Server '{}': Limite de clientes ({}) atingido, rejeitando {}", 
                                        server_id, max_clients, addr);
                                continue;
                            }
                            
                            client_count += 1;
                            let server_id_clone = server_id.clone();
                            
                            // Spawn thread para cada cliente
                            thread::spawn(move || {
                                handle_tcp_client(stream, addr, server_id_clone, client_count);
                            });
                            
                            println!("🔗 TCP Server '{}': Cliente {} conectado de {}", 
                                    server_id, client_count, addr);
                        }
                        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                            // Não há conexão pendente, espera um pouco
                            thread::sleep(Duration::from_millis(100));
                        }
                        Err(e) => {
                            eprintln!("❌ TCP Server '{}': Erro ao aceitar conexão: {}", server_id, e);
                        }
                    }
                }
                
                println!("✅ TCP Server '{}' finalizado", server_id);
            }
            Err(e) => {
                eprintln!("❌ Erro ao iniciar TCP Server '{}' em {}: {}", server_id, bind_address, e);
            }
        }
    });
}

/// Função para lidar com cada cliente TCP conectado
fn handle_tcp_client(mut stream: TcpStream, addr: SocketAddr, server_id: String, client_num: usize) {
    let buffer = [0; 1024];
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    
    loop {
        match reader.read_line(&mut String::new()) {
            Ok(0) => {
                // Cliente desconectou
                println!("👋 TCP Server '{}': Cliente {} de {} desconectou", server_id, client_num, addr);
                break;
            }
            Ok(_) => {
                // Dados recebidos - por enquanto apenas echo
                let response = format!("Echo from TCP Server '{}': received data\n", server_id);
                
                if let Err(e) = stream.write_all(response.as_bytes()) {
                    eprintln!("❌ TCP Server '{}': Erro ao enviar resposta para {}: {}", server_id, addr, e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("❌ TCP Server '{}': Erro ao ler dados de {}: {}", server_id, addr, e);
                break;
            }
        }
    }
}

pub fn register_tcp_functions(functions: &mut HashMap<String, NativeFunction>) {
    // Funções de servidor TCP
    functions.insert("tcp_server_create".to_string(), native_tcp_server_create);
    functions.insert("tcp_server_start".to_string(), native_tcp_server_start);
    functions.insert("tcp_server_stop".to_string(), native_tcp_server_stop);
    functions.insert("tcp_server_status".to_string(), native_tcp_server_status);
    functions.insert("tcp_server_set_max_clients".to_string(), native_tcp_server_set_max_clients);
    
    // Funções de cliente TCP
    functions.insert("tcp_client_create".to_string(), native_tcp_client_create);
    functions.insert("tcp_client_connect".to_string(), native_tcp_client_connect);
    functions.insert("tcp_client_disconnect".to_string(), native_tcp_client_disconnect);
    functions.insert("tcp_client_send".to_string(), native_tcp_client_send);
    functions.insert("tcp_client_receive".to_string(), native_tcp_client_receive);
    functions.insert("tcp_client_status".to_string(), native_tcp_client_status);
    functions.insert("tcp_client_set_timeout".to_string(), native_tcp_client_set_timeout);
    
    // Funções utilitárias
    functions.insert("tcp_resolve_hostname".to_string(), native_tcp_resolve_hostname);
    functions.insert("tcp_get_local_ip".to_string(), native_tcp_get_local_ip);
    functions.insert("tcp_port_available".to_string(), native_tcp_port_available);
}

// ========================
// Funções de servidor TCP
// ========================

/// native_tcp_server_create(server_id, host?, port?, max_clients?) -> null
/// Cria uma nova instância de servidor TCP
fn native_tcp_server_create(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentError("tcp_server_create() requer pelo menos server_id".to_string()));
    }
    
    let server_id = match &args[0] {
        Value::String(id) => id.clone(),
        _ => return Err(RuntimeError::ArgumentError("server_id deve ser uma string".to_string())),
    };
    
    let host = if args.len() > 1 {
        match &args[1] {
            Value::String(h) => h.clone(),
            _ => "127.0.0.1".to_string(),
        }
    } else {
        "127.0.0.1".to_string()
    };
    
    let port = if args.len() > 2 {
        match &args[2] {
            Value::Number(p) => *p as u16,
            _ => 8080,
        }
    } else {
        8080
    };
    
    let max_clients = if args.len() > 3 {
        match &args[3] {
            Value::Number(mc) => *mc as usize,
            _ => 10,
        }
    } else {
        10
    };
    
    let server_instance = ServerInstance {
        server_id: server_id.clone(),
        host: host.clone(),
        port,
        is_running: false,
        stop_sender: None,
        max_clients,
    };
    
    TCP_SERVERS.lock().unwrap().insert(server_id.clone(), server_instance);
    
    println!("✅ TCP Server '{}' criado para {}:{} (máx {} clientes)", server_id, host, port, max_clients);
    Ok(Value::Null)
}

/// native_tcp_server_start(server_id) -> null
/// Inicia o servidor TCP
fn native_tcp_server_start(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentError("tcp_server_start() requer server_id".to_string()));
    }
    
    let server_id = match &args[0] {
        Value::String(id) => id.clone(),
        _ => return Err(RuntimeError::ArgumentError("server_id deve ser uma string".to_string())),
    };
    
    let mut servers = TCP_SERVERS.lock().unwrap();
    if let Some(server) = servers.get_mut(&server_id) {
        if server.is_running {
            return Err(RuntimeError::Generic(format!("TCP Server '{}' já está rodando", server_id)));
        }
        
        let (stop_sender, stop_receiver) = mpsc::channel();
        server.stop_sender = Some(stop_sender);
        server.is_running = true;
        
        let server_clone = server.clone();
        start_tcp_server(
            server_clone.server_id,
            server_clone.host,
            server_clone.port,
            server_clone.max_clients,
            stop_receiver,
        );
        
        println!("🚀 TCP Server '{}' iniciado em {}:{}", server_id, server.host, server.port);
        Ok(Value::Null)
    } else {
        Err(RuntimeError::ArgumentError(format!("TCP Server '{}' não encontrado", server_id)))
    }
}

/// native_tcp_server_stop(server_id) -> null
/// Para o servidor TCP
fn native_tcp_server_stop(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentError("tcp_server_stop() requer server_id".to_string()));
    }
    
    let server_id = match &args[0] {
        Value::String(id) => id.clone(),
        _ => return Err(RuntimeError::ArgumentError("server_id deve ser uma string".to_string())),
    };
    
    let mut servers = TCP_SERVERS.lock().unwrap();
    if let Some(server) = servers.get_mut(&server_id) {
        if !server.is_running {
            return Err(RuntimeError::Generic(format!("TCP Server '{}' não está rodando", server_id)));
        }
        
        if let Some(stop_sender) = &server.stop_sender {
            let _ = stop_sender.send(());
        }
        
        server.is_running = false;
        server.stop_sender = None;
        
        println!("🛑 TCP Server '{}' parado", server_id);
        Ok(Value::Null)
    } else {
        Err(RuntimeError::ArgumentError(format!("TCP Server '{}' não encontrado", server_id)))
    }
}

/// native_tcp_server_status(server_id) -> object
/// Retorna status do servidor TCP
fn native_tcp_server_status(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentError("tcp_server_status() requer server_id".to_string()));
    }
    
    let server_id = match &args[0] {
        Value::String(id) => id.clone(),
        _ => return Err(RuntimeError::ArgumentError("server_id deve ser uma string".to_string())),
    };
    
    let servers = TCP_SERVERS.lock().unwrap();
    if let Some(server) = servers.get(&server_id) {
        let mut status = HashMap::new();
        status.insert("server_id".to_string(), Value::String(server.server_id.clone()));
        status.insert("host".to_string(), Value::String(server.host.clone()));
        status.insert("port".to_string(), Value::Number(server.port as f64));
        status.insert("is_running".to_string(), Value::Bool(server.is_running));
        status.insert("max_clients".to_string(), Value::Number(server.max_clients as f64));
        
        let id = _heap.allocate(crate::heap::ManagedObject::Object {
            properties: status,
            methods: HashMap::new(),
        });
        
        Ok(Value::Object(id))
    } else {
        Err(RuntimeError::ArgumentError(format!("TCP Server '{}' não encontrado", server_id)))
    }
}

/// native_tcp_server_set_max_clients(server_id, max_clients) -> null
/// Define o número máximo de clientes para um servidor TCP
fn native_tcp_server_set_max_clients(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::ArgumentError("tcp_server_set_max_clients() requer server_id e max_clients".to_string()));
    }
    
    let server_id = match &args[0] {
        Value::String(id) => id.clone(),
        _ => return Err(RuntimeError::ArgumentError("server_id deve ser uma string".to_string())),
    };
    
    let max_clients = match &args[1] {
        Value::Number(mc) => *mc as usize,
        _ => return Err(RuntimeError::ArgumentError("max_clients deve ser um número".to_string())),
    };
    
    let mut servers = TCP_SERVERS.lock().unwrap();
    if let Some(server) = servers.get_mut(&server_id) {
        if server.is_running {
            return Err(RuntimeError::Generic(format!("Não é possível alterar max_clients enquanto TCP Server '{}' está rodando", server_id)));
        }
        
        server.max_clients = max_clients;
        println!("✅ TCP Server '{}' configurado para máximo {} clientes", server_id, max_clients);
        Ok(Value::Null)
    } else {
        Err(RuntimeError::ArgumentError(format!("TCP Server '{}' não encontrado", server_id)))
    }
}

// ========================
// Funções de cliente TCP
// ========================

/// native_tcp_client_create(client_id, host, port) -> null
/// Cria uma nova instância de cliente TCP
fn native_tcp_client_create(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.len() < 3 {
        return Err(RuntimeError::ArgumentError("tcp_client_create() requer client_id, host e port".to_string()));
    }
    
    let client_id = match &args[0] {
        Value::String(id) => id.clone(),
        _ => return Err(RuntimeError::ArgumentError("client_id deve ser uma string".to_string())),
    };
    
    let host = match &args[1] {
        Value::String(h) => h.clone(),
        _ => return Err(RuntimeError::ArgumentError("host deve ser uma string".to_string())),
    };
    
    let port = match &args[2] {
        Value::Number(p) => *p as u16,
        _ => return Err(RuntimeError::ArgumentError("port deve ser um número".to_string())),
    };
    
    let client_instance = ClientInstance {
        client_id: client_id.clone(),
        host: host.clone(),
        port,
        is_connected: false,
        timeout: Some(Duration::from_secs(30)),
    };
    
    TCP_CLIENTS.lock().unwrap().insert(client_id.clone(), client_instance);
    
    println!("✅ TCP Client '{}' criado para {}:{}", client_id, host, port);
    Ok(Value::Null)
}

/// native_tcp_client_connect(client_id) -> bool
/// Conecta o cliente TCP ao servidor
fn native_tcp_client_connect(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentError("tcp_client_connect() requer client_id".to_string()));
    }
    
    let client_id = match &args[0] {
        Value::String(id) => id.clone(),
        _ => return Err(RuntimeError::ArgumentError("client_id deve ser uma string".to_string())),
    };
    
    let mut clients = TCP_CLIENTS.lock().unwrap();
    if let Some(client) = clients.get_mut(&client_id) {
        if client.is_connected {
            return Ok(Value::Bool(true));
        }
        
        let address = format!("{}:{}", client.host, client.port);
        
        match TcpStream::connect_timeout(&address.parse().unwrap(), client.timeout.unwrap_or(Duration::from_secs(30))) {
            Ok(_stream) => {
                client.is_connected = true;
                println!("🔗 TCP Client '{}' conectado a {}", client_id, address);
                Ok(Value::Bool(true))
            }
            Err(e) => {
                println!("❌ TCP Client '{}' falha ao conectar a {}: {}", client_id, address, e);
                Ok(Value::Bool(false))
            }
        }
    } else {
        Err(RuntimeError::ArgumentError(format!("TCP Client '{}' não encontrado", client_id)))
    }
}

/// native_tcp_client_disconnect(client_id) -> null
/// Desconecta o cliente TCP
fn native_tcp_client_disconnect(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentError("tcp_client_disconnect() requer client_id".to_string()));
    }
    
    let client_id = match &args[0] {
        Value::String(id) => id.clone(),
        _ => return Err(RuntimeError::ArgumentError("client_id deve ser uma string".to_string())),
    };
    
    let mut clients = TCP_CLIENTS.lock().unwrap();
    if let Some(client) = clients.get_mut(&client_id) {
        if !client.is_connected {
            return Err(RuntimeError::Generic(format!("TCP Client '{}' não está conectado", client_id)));
        }
        
        client.is_connected = false;
        println!("👋 TCP Client '{}' desconectado", client_id);
        Ok(Value::Null)
    } else {
        Err(RuntimeError::ArgumentError(format!("TCP Client '{}' não encontrado", client_id)))
    }
}

/// native_tcp_client_send(client_id, data) -> bool
/// Envia dados através do cliente TCP
fn native_tcp_client_send(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::ArgumentError("tcp_client_send() requer client_id e data".to_string()));
    }
    
    let client_id = match &args[0] {
        Value::String(id) => id.clone(),
        _ => return Err(RuntimeError::ArgumentError("client_id deve ser uma string".to_string())),
    };
    
    let data = match &args[1] {
        Value::String(d) => d.clone(),
        _ => return Err(RuntimeError::ArgumentError("data deve ser uma string".to_string())),
    };
    
    let clients = TCP_CLIENTS.lock().unwrap();
    if let Some(client) = clients.get(&client_id) {
        if !client.is_connected {
            return Err(RuntimeError::Generic(format!("TCP Client '{}' não está conectado", client_id)));
        }
        
        let address = format!("{}:{}", client.host, client.port);
        
        match TcpStream::connect(&address) {
            Ok(mut stream) => {
                match stream.write_all(data.as_bytes()) {
                    Ok(_) => {
                        println!("📤 TCP Client '{}' enviou dados: {}", client_id, data);
                        Ok(Value::Bool(true))
                    }
                    Err(e) => {
                        println!("❌ TCP Client '{}' erro ao enviar dados: {}", client_id, e);
                        Ok(Value::Bool(false))
                    }
                }
            }
            Err(e) => {
                println!("❌ TCP Client '{}' erro de conexão: {}", client_id, e);
                Ok(Value::Bool(false))
            }
        }
    } else {
        Err(RuntimeError::ArgumentError(format!("TCP Client '{}' não encontrado", client_id)))
    }
}

/// native_tcp_client_receive(client_id) -> string
/// Recebe dados através do cliente TCP
fn native_tcp_client_receive(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentError("tcp_client_receive() requer client_id".to_string()));
    }
    
    let client_id = match &args[0] {
        Value::String(id) => id.clone(),
        _ => return Err(RuntimeError::ArgumentError("client_id deve ser uma string".to_string())),
    };
    
    let clients = TCP_CLIENTS.lock().unwrap();
    if let Some(client) = clients.get(&client_id) {
        if !client.is_connected {
            return Err(RuntimeError::Generic(format!("TCP Client '{}' não está conectado", client_id)));
        }
        
        let address = format!("{}:{}", client.host, client.port);
        
        match TcpStream::connect(&address) {
            Ok(mut stream) => {
                let mut buffer = [0; 1024];
                match stream.read(&mut buffer) {
                    Ok(size) => {
                        let received_data = String::from_utf8_lossy(&buffer[..size]).to_string();
                        println!("📥 TCP Client '{}' recebeu dados: {}", client_id, received_data);
                        Ok(Value::String(received_data))
                    }
                    Err(e) => {
                        println!("❌ TCP Client '{}' erro ao receber dados: {}", client_id, e);
                        Ok(Value::String("".to_string()))
                    }
                }
            }
            Err(e) => {
                println!("❌ TCP Client '{}' erro de conexão: {}", client_id, e);
                Ok(Value::String("".to_string()))
            }
        }
    } else {
        Err(RuntimeError::ArgumentError(format!("TCP Client '{}' não encontrado", client_id)))
    }
}

/// native_tcp_client_status(client_id) -> object
/// Retorna status do cliente TCP
fn native_tcp_client_status(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentError("tcp_client_status() requer client_id".to_string()));
    }
    
    let client_id = match &args[0] {
        Value::String(id) => id.clone(),
        _ => return Err(RuntimeError::ArgumentError("client_id deve ser uma string".to_string())),
    };
    
    let clients = TCP_CLIENTS.lock().unwrap();
    if let Some(client) = clients.get(&client_id) {
        let mut status = HashMap::new();
        status.insert("client_id".to_string(), Value::String(client.client_id.clone()));
        status.insert("host".to_string(), Value::String(client.host.clone()));
        status.insert("port".to_string(), Value::Number(client.port as f64));
        status.insert("is_connected".to_string(), Value::Bool(client.is_connected));
        status.insert("timeout_secs".to_string(), Value::Number(client.timeout.map(|d| d.as_secs() as f64).unwrap_or(0.0)));
        
        let id = _heap.allocate(crate::heap::ManagedObject::Object {
            properties: status,
            methods: HashMap::new(),
        });
        
        Ok(Value::Object(id))
    } else {
        Err(RuntimeError::ArgumentError(format!("TCP Client '{}' não encontrado", client_id)))
    }
}

/// native_tcp_client_set_timeout(client_id, timeout_secs) -> null
/// Define timeout para operações do cliente TCP
fn native_tcp_client_set_timeout(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::ArgumentError("tcp_client_set_timeout() requer client_id e timeout_secs".to_string()));
    }
    
    let client_id = match &args[0] {
        Value::String(id) => id.clone(),
        _ => return Err(RuntimeError::ArgumentError("client_id deve ser uma string".to_string())),
    };
    
    let timeout_secs = match &args[1] {
        Value::Number(t) => *t as u64,
        _ => return Err(RuntimeError::ArgumentError("timeout_secs deve ser um número".to_string())),
    };
    
    let mut clients = TCP_CLIENTS.lock().unwrap();
    if let Some(client) = clients.get_mut(&client_id) {
        client.timeout = Some(Duration::from_secs(timeout_secs));
        println!("✅ TCP Client '{}' timeout configurado para {} segundos", client_id, timeout_secs);
        Ok(Value::Null)
    } else {
        Err(RuntimeError::ArgumentError(format!("TCP Client '{}' não encontrado", client_id)))
    }
}

// ========================
// Funções utilitárias TCP
// ========================

/// native_tcp_resolve_hostname(hostname) -> string
/// Resolve um hostname para endereço IP
fn native_tcp_resolve_hostname(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentError("tcp_resolve_hostname() requer hostname".to_string()));
    }
    
    let hostname = match &args[0] {
        Value::String(h) => h.clone(),
        _ => return Err(RuntimeError::ArgumentError("hostname deve ser uma string".to_string())),
    };
    
    match std::net::ToSocketAddrs::to_socket_addrs(&format!("{}:80", hostname)) {
        Ok(mut addrs) => {
            if let Some(addr) = addrs.next() {
                let ip = addr.ip().to_string();
                println!("🔍 Hostname '{}' resolvido para IP: {}", hostname, ip);
                Ok(Value::String(ip))
            } else {
                Err(RuntimeError::NetworkError(format!("Não foi possível resolver hostname: {}", hostname)))
            }
        }
        Err(e) => {
            Err(RuntimeError::NetworkError(format!("Erro ao resolver hostname '{}': {}", hostname, e)))
        }
    }
}

/// native_tcp_get_local_ip() -> string
/// Retorna o endereço IP local da máquina
fn native_tcp_get_local_ip(_args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    match std::net::UdpSocket::bind("0.0.0.0:0") {
        Ok(socket) => {
            match socket.connect("8.8.8.8:80") {
                Ok(_) => {
                    match socket.local_addr() {
                        Ok(addr) => {
                            let local_ip = addr.ip().to_string();
                            println!("🌐 IP local detectado: {}", local_ip);
                            Ok(Value::String(local_ip))
                        }
                        Err(e) => {
                            Err(RuntimeError::NetworkError(format!("Erro ao obter endereço local: {}", e)))
                        }
                    }
                }
                Err(e) => {
                    Err(RuntimeError::NetworkError(format!("Erro ao conectar para detectar IP: {}", e)))
                }
            }
        }
        Err(e) => {
            Err(RuntimeError::NetworkError(format!("Erro ao criar socket para detectar IP: {}", e)))
        }
    }
}

/// native_tcp_port_available(port) -> bool
/// Verifica se uma porta está disponível para uso
fn native_tcp_port_available(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentError("tcp_port_available() requer port".to_string()));
    }
    
    let port = match &args[0] {
        Value::Number(p) => *p as u16,
        _ => return Err(RuntimeError::ArgumentError("port deve ser um número".to_string())),
    };
    
    match TcpListener::bind(format!("127.0.0.1:{}", port)) {
        Ok(_) => {
            println!("✅ Porta {} está disponível", port);
            Ok(Value::Bool(true))
        }
        Err(_) => {
            println!("❌ Porta {} está em uso", port);
            Ok(Value::Bool(false))
        }
    }
}