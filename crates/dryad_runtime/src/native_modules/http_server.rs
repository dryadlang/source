use crate::interpreter::RuntimeValue;
use crate::native_modules::NativeFunction;
use crate::errors::RuntimeError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;

lazy_static! {
    static ref HTTP_SERVERS: Arc<Mutex<HashMap<String, ServerInstance>>> = Arc::new(Mutex::new(HashMap::new()));
    static ref ROUTE_HANDLERS: Arc<Mutex<HashMap<String, HashMap<String, RouteHandler>>>> = Arc::new(Mutex::new(HashMap::new()));
    static ref STATIC_CONTENT: Arc<Mutex<HashMap<String, HashMap<String, StaticContent>>>> = Arc::new(Mutex::new(HashMap::new()));
    static ref SERVER_THREADS: Arc<Mutex<HashMap<String, thread::JoinHandle<()>>>> = Arc::new(Mutex::new(HashMap::new()));
}

#[derive(Clone, Debug)]
struct ServerInstance {
    port: u16,
    host: String,
    is_running: bool,
    stop_sender: Option<mpsc::Sender<()>>,
}

#[derive(Clone, Debug)]
struct RouteHandler {
    method: String,
    path: String,
    response_body: String,
    response_headers: HashMap<String, String>,
    status_code: u16,
}

#[derive(Clone, Debug)]
struct StaticContent {
    content: Vec<u8>,
    content_type: String,
}

// Função para inicializar servidor HTTP simples usando threads nativas
fn start_simple_http_server(server_id: String, host: String, port: u16, stop_receiver: mpsc::Receiver<()>) {
    use std::net::TcpListener;
    
    
    let addr = format!("{}:{}", host, port);
    match TcpListener::bind(&addr) {
        Ok(listener) => {
            println!("🌐 Servidor HTTP '{}' ouvindo em http://{}", server_id, addr);
            
            // Configura para não bloquear indefinidamente
            listener.set_nonblocking(true).expect("Cannot set nonblocking");
            
            loop {
                // Verifica se deve parar
                if stop_receiver.try_recv().is_ok() {
                    println!("🛑 Parando servidor '{}'...", server_id);
                    break;
                }
                
                // Aceita conexões
                match listener.accept() {
                    Ok((stream, _)) => {
                        let server_id_clone = server_id.clone();
                        thread::spawn(move || {
                            handle_http_connection(stream, server_id_clone);
                        });
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        // Não há conexões, aguarda um pouco
                        thread::sleep(Duration::from_millis(50));
                        continue;
                    }
                    Err(e) => {
                        eprintln!("❌ Erro ao aceitar conexão: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Erro ao iniciar servidor '{}': {}", server_id, e);
        }
    }
}

// Função para lidar com conexões HTTP
fn handle_http_connection(mut stream: std::net::TcpStream, server_id: String) {
    use std::io::prelude::*;
    
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(_) => {
            let request = String::from_utf8_lossy(&buffer[..]);
            let lines: Vec<&str> = request.lines().collect();
            
            if let Some(request_line) = lines.first() {
                let parts: Vec<&str> = request_line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let method = parts[0];
                    let path = parts[1];
                    
                    println!("📨 {} {} (servidor: {})", method, path, server_id);
                    
                    let response = generate_response(&server_id, method, path);
                    
                    if stream.write_all(response.as_bytes()).is_err() {
                        eprintln!("❌ Erro ao enviar resposta");
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Erro ao ler request: {}", e);
        }
    }
}

// Gera resposta HTTP
fn generate_response(server_id: &str, method: &str, path: &str) -> String {
    let route_key = format!("{}:{}", method, path);
    
    // Verifica rotas primeiro
    if let Some(handler) = ROUTE_HANDLERS.lock().unwrap()
        .get(server_id)
        .and_then(|routes| routes.get(&route_key)) {
        
        return format!(
            "HTTP/1.1 {} OK\r\nContent-Type: text/html; charset=utf-8\r\n\r\n{}",
            handler.status_code,
            handler.response_body
        );
    }
    
    // Verifica conteúdo estático
    if let Some(static_content) = STATIC_CONTENT.lock().unwrap()
        .get(server_id)
        .and_then(|content| content.get(path)) {
        
        return format!(
            "HTTP/1.1 200 OK\r\nContent-Type: {}\r\n\r\n{}",
            static_content.content_type,
            String::from_utf8_lossy(&static_content.content)
        );
    }
    
    // 404 - Página não encontrada
    let not_found_html = format!(r#"
<!DOCTYPE html>
<html>
<head>
    <title>404 - Página Não Encontrada</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        .error {{ color: #d32f2f; }}
        .info {{ color: #666; margin-top: 20px; }}
    </style>
</head>
<body>
    <h1 class="error">404 - Página Não Encontrada</h1>
    <p>O caminho <strong>{}</strong> não foi encontrado neste servidor.</p>
    <p class="info">Servidor: {} | Método: {}</p>
    <hr>
    <small>Dryad WebServer v1.0</small>
</body>
</html>"#, path, server_id, method);

    format!("HTTP/1.1 404 Not Found\r\nContent-Type: text/html; charset=utf-8\r\n\r\n{}", not_found_html)
}

pub fn register_http_server_functions(functions: &mut HashMap<String, NativeFunction>) {
    // Gerenciamento de servidor
    functions.insert("native_http_server_create".to_string(), native_http_server_create);
    functions.insert("native_http_server_start".to_string(), native_http_server_start);
    functions.insert("native_http_server_stop".to_string(), native_http_server_stop);
    functions.insert("native_http_server_status".to_string(), native_http_server_status);
    
    // Configuração de rotas
    functions.insert("native_http_server_route".to_string(), native_http_server_route);
    functions.insert("native_http_server_get".to_string(), native_http_server_get);
    functions.insert("native_http_server_post".to_string(), native_http_server_post);
    functions.insert("native_http_server_put".to_string(), native_http_server_put);
    functions.insert("native_http_server_delete".to_string(), native_http_server_delete);
    
    // Conteúdo estático
    functions.insert("native_http_server_static".to_string(), native_http_server_static);
    functions.insert("native_http_server_file".to_string(), native_http_server_file);
    functions.insert("native_http_server_html".to_string(), native_http_server_html);
    functions.insert("native_http_server_json".to_string(), native_http_server_json);
    
    // Configurações avançadas
    functions.insert("native_http_server_cors".to_string(), native_http_server_cors);
    functions.insert("native_http_server_middleware".to_string(), native_http_server_middleware);
}

// ========================
// Funções de gerenciamento de servidor
// ========================

/// native_http_server_create(server_id, host?, port?) -> null
/// Cria uma nova instância de servidor HTTP
fn native_http_server_create(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    let server_id = match args.get(0) {
        Some(RuntimeValue::String(s)) => s.clone(),
        Some(_) => return Err(RuntimeError::TypeError("Primeiro argumento deve ser string (server_id)".to_string())),
        None => return Err(RuntimeError::ArgumentError("native_http_server_create espera pelo menos 1 argumento".to_string())),
    };
    
    let host = match args.get(1) {
        Some(RuntimeValue::String(s)) => s.clone(),
        Some(_) => return Err(RuntimeError::TypeError("Segundo argumento deve ser string (host)".to_string())),
        None => "127.0.0.1".to_string(),
    };
    
    let port = match args.get(2) {
        Some(RuntimeValue::Number(n)) => *n as u16,
        Some(_) => return Err(RuntimeError::TypeError("Terceiro argumento deve ser número (port)".to_string())),
        None => 8080,
    };
    
    let instance = ServerInstance {
        port,
        host,
        is_running: false,
        stop_sender: None,
    };
    
    HTTP_SERVERS.lock().unwrap().insert(server_id.clone(), instance);
    ROUTE_HANDLERS.lock().unwrap().insert(server_id.clone(), HashMap::new());
    STATIC_CONTENT.lock().unwrap().insert(server_id, HashMap::new());
    
    Ok(RuntimeValue::Null)
}

/// native_http_server_start(server_id) -> null
/// Inicia o servidor HTTP
fn native_http_server_start(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    let server_id = match args.get(0) {
        Some(RuntimeValue::String(s)) => s.clone(),
        Some(_) => return Err(RuntimeError::TypeError("Argumento deve ser string (server_id)".to_string())),
        None => return Err(RuntimeError::ArgumentError("native_http_server_start espera 1 argumento".to_string())),
    };
    
    let (host, port) = {
        let servers = HTTP_SERVERS.lock().unwrap();
        let server = servers.get(&server_id)
            .ok_or_else(|| RuntimeError::ArgumentError(format!("Servidor '{}' não encontrado", server_id)))?;
        
        if server.is_running {
            return Err(RuntimeError::Generic(format!("Servidor '{}' já está rodando", server_id)));
        }
        
        (server.host.clone(), server.port)
    };
    
    // Cria canal para parar o servidor
    let (stop_sender, stop_receiver) = mpsc::channel();
    
    // Inicia servidor em thread separada
    let server_id_clone = server_id.clone();
    let handle = thread::spawn(move || {
        start_simple_http_server(server_id_clone, host, port, stop_receiver);
    });
    
    // Armazena o handle da thread
    SERVER_THREADS.lock().unwrap().insert(server_id.clone(), handle);
    
    // Aguarda um pouco para o servidor inicializar
    thread::sleep(Duration::from_millis(200));
    
    // Marca como rodando e armazena o sender
    {
        let mut servers = HTTP_SERVERS.lock().unwrap();
        if let Some(server) = servers.get_mut(&server_id) {
            server.is_running = true;
            server.stop_sender = Some(stop_sender);
        }
    }
    
    println!("🚀 Servidor HTTP '{}' iniciado em http://{}:{}", 
             server_id,
             HTTP_SERVERS.lock().unwrap().get(&server_id).unwrap().host,
             HTTP_SERVERS.lock().unwrap().get(&server_id).unwrap().port);
    
    Ok(RuntimeValue::Null)
}

/// native_http_server_stop(server_id) -> null
/// Para o servidor HTTP
fn native_http_server_stop(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    let server_id = match args.get(0) {
        Some(RuntimeValue::String(s)) => s.clone(),
        Some(_) => return Err(RuntimeError::TypeError("Argumento deve ser string (server_id)".to_string())),
        None => return Err(RuntimeError::ArgumentError("native_http_server_stop espera 1 argumento".to_string())),
    };
    
    // Para o servidor enviando sinal
    {
        let mut servers = HTTP_SERVERS.lock().unwrap();
        if let Some(server) = servers.get_mut(&server_id) {
            if !server.is_running {
                return Err(RuntimeError::Generic(format!("Servidor '{}' não está rodando", server_id)));
            }
            
            if let Some(sender) = server.stop_sender.take() {
                let _ = sender.send(()); // Envia sinal para parar
            }
            
            server.is_running = false;
        } else {
            return Err(RuntimeError::ArgumentError(format!("Servidor '{}' não encontrado", server_id)));
        }
    }
    
    println!("🛑 Servidor HTTP '{}' parado", server_id);
    Ok(RuntimeValue::Null)
}

/// native_http_server_status(server_id) -> object
/// Retorna status do servidor
fn native_http_server_status(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    let server_id = match args.get(0) {
        Some(RuntimeValue::String(s)) => s.clone(),
        Some(_) => return Err(RuntimeError::TypeError("Argumento deve ser string (server_id)".to_string())),
        None => return Err(RuntimeError::ArgumentError("native_http_server_status espera 1 argumento".to_string())),
    };
    
    let servers = HTTP_SERVERS.lock().unwrap();
    let server = servers.get(&server_id)
        .ok_or_else(|| RuntimeError::ArgumentError(format!("Servidor '{}' não encontrado", server_id)))?;
    
    // Retorna informações do servidor como string JSON (simplificado)
    let status = format!(
        r#"{{"server_id": "{}", "host": "{}", "port": {}, "running": {}}}"#,
        server_id, server.host, server.port, server.is_running
    );
    
    Ok(RuntimeValue::String(status))
}

// ========================
// Funções de configuração de rotas
// ========================

/// native_http_server_route(server_id, method, path, response_body, status_code?) -> null
/// Define uma rota genérica
fn native_http_server_route(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    let server_id = match args.get(0) {
        Some(RuntimeValue::String(s)) => s.clone(),
        _ => return Err(RuntimeError::ArgumentError("Primeiro argumento deve ser string (server_id)".to_string())),
    };
    
    let method = match args.get(1) {
        Some(RuntimeValue::String(s)) => s.to_uppercase(),
        _ => return Err(RuntimeError::ArgumentError("Segundo argumento deve ser string (method)".to_string())),
    };
    
    let path = match args.get(2) {
        Some(RuntimeValue::String(s)) => s.clone(),
        _ => return Err(RuntimeError::ArgumentError("Terceiro argumento deve ser string (path)".to_string())),
    };
    
    let response_body = match args.get(3) {
        Some(RuntimeValue::String(s)) => s.clone(),
        _ => return Err(RuntimeError::ArgumentError("Quarto argumento deve ser string (response_body)".to_string())),
    };
    
    let status_code = match args.get(4) {
        Some(RuntimeValue::Number(n)) => *n as u16,
        _ => 200,
    };
    
    let route_key = format!("{}:{}", method, path);
    let handler = RouteHandler {
        method,
        path,
        response_body,
        response_headers: HashMap::new(),
        status_code,
    };
    
    ROUTE_HANDLERS.lock().unwrap()
        .get_mut(&server_id)
        .ok_or_else(|| RuntimeError::ArgumentError(format!("Servidor '{}' não encontrado", server_id)))?
        .insert(route_key, handler);
    
    Ok(RuntimeValue::Null)
}

/// native_http_server_get(server_id, path, response_body) -> null
/// Define uma rota GET
fn native_http_server_get(args: &[RuntimeValue], manager: &crate::native_modules::NativeModuleManager) -> Result<RuntimeValue, RuntimeError> {
    if args.len() < 3 {
        return Err(RuntimeError::ArgumentError("native_http_server_get espera 3 argumentos".to_string()));
    }
    
    native_http_server_route(args, manager)
}

/// native_http_server_post(server_id, path, response_body) -> null
/// Define uma rota POST
fn native_http_server_post(args: &[RuntimeValue], manager: &crate::native_modules::NativeModuleManager) -> Result<RuntimeValue, RuntimeError> {
    if args.len() < 3 {
        return Err(RuntimeError::ArgumentError("native_http_server_post espera 3 argumentos".to_string()));
    }
    
    native_http_server_route(args, manager)
}

/// native_http_server_put(server_id, path, response_body) -> null
/// Define uma rota PUT
fn native_http_server_put(args: &[RuntimeValue], manager: &crate::native_modules::NativeModuleManager) -> Result<RuntimeValue, RuntimeError> {
    if args.len() < 3 {
        return Err(RuntimeError::ArgumentError("native_http_server_put espera 3 argumentos".to_string()));
    }
    
    native_http_server_route(args, manager)
}

/// native_http_server_delete(server_id, path, response_body) -> null
/// Define uma rota DELETE
fn native_http_server_delete(args: &[RuntimeValue], manager: &crate::native_modules::NativeModuleManager) -> Result<RuntimeValue, RuntimeError> {
    if args.len() < 3 {
        return Err(RuntimeError::ArgumentError("native_http_server_delete espera 3 argumentos".to_string()));
    }
    
    native_http_server_route(args, manager)
}

// ========================
// Funções de conteúdo estático
// ========================

/// native_http_server_static(server_id, path, file_path) -> null
/// Serve arquivo estático
fn native_http_server_static(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    if args.len() != 3 {
        return Err(RuntimeError::ArgumentError("native_http_server_static espera 3 argumentos".to_string()));
    }
    
    let server_id = match &args[0] {
        RuntimeValue::String(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("Primeiro argumento deve ser string (server_id)".to_string())),
    };
    
    let path = match &args[1] {
        RuntimeValue::String(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("Segundo argumento deve ser string (path)".to_string())),
    };
    
    let file_path = match &args[2] {
        RuntimeValue::String(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("Terceiro argumento deve ser string (file_path)".to_string())),
    };
    
    // Lê o arquivo
    let content = fs::read(&file_path)
        .map_err(|e| RuntimeError::IoError(format!("Erro ao ler arquivo '{}': {}", file_path, e)))?;
    
    let content_type = get_content_type(&file_path);
    
    let static_content = StaticContent {
        content,
        content_type,
    };
    
    STATIC_CONTENT.lock().unwrap()
        .get_mut(&server_id)
        .ok_or_else(|| RuntimeError::ArgumentError(format!("Servidor '{}' não encontrado", server_id)))?
        .insert(path, static_content);
    
    Ok(RuntimeValue::Null)
}

/// native_http_server_file(server_id, path, file_path) -> null
/// Alias para static
fn native_http_server_file(args: &[RuntimeValue], manager: &crate::native_modules::NativeModuleManager) -> Result<RuntimeValue, RuntimeError> {
    native_http_server_static(args, manager)
}

/// native_http_server_html(server_id, path, html_content) -> null
/// Define resposta HTML
fn native_http_server_html(args: &[RuntimeValue], manager: &crate::native_modules::NativeModuleManager) -> Result<RuntimeValue, RuntimeError> {
    if args.len() != 3 {
        return Err(RuntimeError::ArgumentError("native_http_server_html espera 3 argumentos".to_string()));
    }
    
    let new_args = vec![
        args[0].clone(),
        RuntimeValue::String("GET".to_string()),
        args[1].clone(),
        args[2].clone(),
    ];
    
    // Define como rota GET primeiro
    native_http_server_route(&new_args, manager)?;
    
    // Depois define o content-type como HTML (simplificado por enquanto)
    Ok(RuntimeValue::Null)
}

/// native_http_server_json(server_id, path, json_content) -> null
/// Define resposta JSON
fn native_http_server_json(args: &[RuntimeValue], manager: &crate::native_modules::NativeModuleManager) -> Result<RuntimeValue, RuntimeError> {
    if args.len() != 3 {
        return Err(RuntimeError::ArgumentError("native_http_server_json espera 3 argumentos".to_string()));
    }
    
    let new_args = vec![
        args[0].clone(),
        RuntimeValue::String("GET".to_string()),
        args[1].clone(),
        args[2].clone(),
    ];
    
    // Define como rota GET primeiro
    native_http_server_route(&new_args, manager)?;
    
    // Depois define o content-type como JSON (simplificado por enquanto)
    Ok(RuntimeValue::Null)
}

// ========================
// Funções avançadas
// ========================

/// native_http_server_cors(server_id, origin?) -> null
/// Configura CORS
fn native_http_server_cors(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    // Por enquanto, apenas aceita os argumentos mas não implementa CORS real
    if args.is_empty() {
        return Err(RuntimeError::ArgumentError("native_http_server_cors espera pelo menos 1 argumento".to_string()));
    }
    
    println!("📋 CORS configurado (implementação simplificada)");
    Ok(RuntimeValue::Null)
}

/// native_http_server_middleware(server_id, middleware_fn) -> null
/// Adiciona middleware (futuro)
fn native_http_server_middleware(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    // Por enquanto, apenas aceita os argumentos mas não implementa middleware real
    if args.len() != 2 {
        return Err(RuntimeError::ArgumentError("native_http_server_middleware espera 2 argumentos".to_string()));
    }
    
    println!("📋 Middleware adicionado (implementação simplificada)");
    Ok(RuntimeValue::Null)
}

// ========================
// Funções auxiliares (implementação antiga com Tokio - desabilitada)
// ========================

/*
/// Inicia o servidor HTTP real (versão Tokio/Hyper - desabilitada)
async fn start_http_server(server_id: String, host: String, port: u16) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Implementação comentada para evitar conflitos com threads do Tokio
    // ... código original aqui ...
}

/// Manipula requisições HTTP (versão Tokio/Hyper - desabilitada)  
async fn handle_request(
    req: Request<hyper::body::Incoming>,
    server_id: String,
) -> Result<Response<Full<Bytes>>, Infallible> {
    // Implementação comentada para evitar conflitos com threads do Tokio
    // ... código original aqui ...
}
*/

/// Determina tipo de conteúdo baseado na extensão do arquivo
fn get_content_type(file_path: &str) -> String {
    match Path::new(file_path).extension().and_then(|ext| ext.to_str()) {
        Some("html") | Some("htm") => "text/html; charset=utf-8".to_string(),
        Some("css") => "text/css".to_string(),
        Some("js") => "text/javascript".to_string(),
        Some("json") => "application/json".to_string(),
        Some("xml") => "application/xml".to_string(),
        Some("png") => "image/png".to_string(),
        Some("jpg") | Some("jpeg") => "image/jpeg".to_string(),
        Some("gif") => "image/gif".to_string(),
        Some("svg") => "image/svg+xml".to_string(),
        Some("ico") => "image/x-icon".to_string(),
        Some("pdf") => "application/pdf".to_string(),
        Some("txt") => "text/plain; charset=utf-8".to_string(),
        Some("md") => "text/markdown; charset=utf-8".to_string(),
        _ => "application/octet-stream".to_string(),
    }
}