use crate::interpreter::RuntimeValue;
use crate::errors::RuntimeError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use hyper::{Server, Request, Response, Body, Method};
use hyper::service::{make_service_fn, service_fn};
use std::net::SocketAddr;
use std::thread;

lazy_static! {
    static ref WEB_ROUTES: Arc<Mutex<HashMap<(Method, String), usize>>> = Arc::new(Mutex::new(HashMap::new()));
    static ref WEB_STATIC: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
    static ref WEB_DATA: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
    static ref WEB_ON_ERROR: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(None));
    static ref WEB_ON_REQUEST: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(None));
    static ref WEB_SERVER_HANDLE: Arc<Mutex<Option<thread::JoinHandle<()>>>> = Arc::new(Mutex::new(None));
}

pub fn register_webserver_functions(functions: &mut HashMap<String, fn(&[RuntimeValue]) -> Result<RuntimeValue, RuntimeError>>) {
    functions.insert("native_web_listen".to_string(), native_web_listen);
    functions.insert("native_web_route".to_string(), native_web_route);
    functions.insert("native_web_route_static".to_string(), native_web_route_static);
    functions.insert("native_web_route_data".to_string(), native_web_route_data);
    functions.insert("native_web_on_error".to_string(), native_web_on_error);
    functions.insert("native_web_on_request".to_string(), native_web_on_request);
    functions.insert("native_web_shutdown".to_string(), native_web_shutdown);
    functions.insert("native_web_send_response".to_string(), native_web_send_response);
}

// Inicia o servidor web simples
fn native_web_listen(args: &[RuntimeValue]) -> Result<RuntimeValue, RuntimeError> {
    let port = match &args[0] {
        RuntimeValue::Number(n) => *n as u16,
        _ => return Err(RuntimeError::TypeError("native_web_listen: argumento deve ser número".to_string())),
    };
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    let server_thread = thread::spawn(move || {
        let make_svc = make_service_fn(|_conn| async {
            Ok::<_, hyper::Error>(service_fn(handle_request))
        });
        let server = Server::bind(&addr).serve(make_svc);
        let _ = tokio::runtime::Runtime::new().unwrap().block_on(server);
    });

    *WEB_SERVER_HANDLE.lock().unwrap() = Some(server_thread);

    Ok(RuntimeValue::Null)
}

// Define rota e função (handler é um índice/função registrada)
fn native_web_route(args: &[RuntimeValue]) -> Result<RuntimeValue, RuntimeError> {
    let method = match &args[0] {
        RuntimeValue::String(s) => s.to_uppercase(),
        _ => return Err(RuntimeError::TypeError("native_web_route: método deve ser string".to_string())),
    };
    let path = match &args[1] {
        RuntimeValue::String(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("native_web_route: path deve ser string".to_string())),
    };
    let handler = match &args[2] {
        RuntimeValue::Number(n) => *n as usize,
        _ => return Err(RuntimeError::TypeError("native_web_route: handler deve ser número".to_string())),
    };
    let method = match method.as_str() {
        "GET" => Method::GET,
        "POST" => Method::POST,
        "PUT" => Method::PUT,
        "DELETE" => Method::DELETE,
        _ => Method::GET,
    };
    WEB_ROUTES.lock().unwrap().insert((method, path), handler);
    Ok(RuntimeValue::Null)
}

// Rota para arquivos estáticos
fn native_web_route_static(args: &[RuntimeValue]) -> Result<RuntimeValue, RuntimeError> {
    let path = match &args[0] {
        RuntimeValue::String(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("native_web_route_static: path deve ser string".to_string())),
    };
    let dir = match &args[1] {
        RuntimeValue::String(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("native_web_route_static: dir deve ser string".to_string())),
    };
    WEB_STATIC.lock().unwrap().insert(path, dir);
    Ok(RuntimeValue::Null)
}

// Rota para dados dinâmicos
fn native_web_route_data(args: &[RuntimeValue]) -> Result<RuntimeValue, RuntimeError> {
    let path = match &args[0] {
        RuntimeValue::String(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("native_web_route_data: path deve ser string".to_string())),
    };
    let data = match &args[1] {
        RuntimeValue::String(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("native_web_route_data: data deve ser string".to_string())),
    };
    WEB_DATA.lock().unwrap().insert(path, data);
    Ok(RuntimeValue::Null)
}

// Define função para erros
fn native_web_on_error(args: &[RuntimeValue]) -> Result<RuntimeValue, RuntimeError> {
    let handler = match &args[0] {
        RuntimeValue::Number(n) => *n as usize,
        _ => return Err(RuntimeError::TypeError("native_web_on_error: handler deve ser número".to_string())),
    };
    *WEB_ON_ERROR.lock().unwrap() = Some(handler);
    Ok(RuntimeValue::Null)
}

// Define função para requisições
fn native_web_on_request(args: &[RuntimeValue]) -> Result<RuntimeValue, RuntimeError> {
    let handler = match &args[0] {
        RuntimeValue::Number(n) => *n as usize,
        _ => return Err(RuntimeError::TypeError("native_web_on_request: handler deve ser número".to_string())),
    };
    *WEB_ON_REQUEST.lock().unwrap() = Some(handler);
    Ok(RuntimeValue::Null)
}

// Encerra o servidor web
fn native_web_shutdown(_args: &[RuntimeValue]) -> Result<RuntimeValue, RuntimeError> {
    // Para simplificação, apenas remove o handle (não encerra de fato o servidor)
    *WEB_SERVER_HANDLE.lock().unwrap() = None;
    Ok(RuntimeValue::Null)
}

// Envia resposta HTTP (apenas scaffold)
fn native_web_send_response(_args: &[RuntimeValue]) -> Result<RuntimeValue, RuntimeError> {
    Ok(RuntimeValue::Null)
}

// Função de tratamento de requisições (simplificada)
async fn handle_request(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let method = req.method().clone();
    let path = req.uri().path().to_string();

    // Rota para dados dinâmicos
    if let Some(data) = WEB_DATA.lock().unwrap().get(&path) {
        return Ok(Response::new(Body::from(data.clone())));
    }

    // Rota para arquivos estáticos (não implementado)
    if let Some(_dir) = WEB_STATIC.lock().unwrap().get(&path) {
        return Ok(Response::new(Body::from("Arquivo estático")));
    }

    // Rota dinâmica (handler fictício)
    if let Some(handler) = WEB_ROUTES.lock().unwrap().get(&(method, path.clone())) {
        // Aqui você pode chamar o handler registrado
        return Ok(Response::new(Body::from(format!("Handler: {}", handler))));
    }

    // Resposta padrão
    Ok(Response::new(Body::from("Dryad WebServer")))
}