use crate::interpreter::RuntimeValue;
use crate::errors::RuntimeError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use reqwest::{blocking::Client, header::HeaderMap};
use std::fs::File;
use std::io::Write;

lazy_static! {
    static ref HTTP_CONFIG: Arc<Mutex<HashMap<String, HttpConfig>>> = Arc::new(Mutex::new(HashMap::new()));
}

#[derive(Clone, Default)]
struct HttpConfig {
    timeout_ms: Option<u64>,
    headers: Option<HashMap<String, String>>,
    user_agent: Option<String>,
    proxy: Option<String>,
    auth: Option<(String, String)>,
    follow_redirects: Option<bool>,
    cache: Option<bool>,
    compression: Option<bool>,
    max_redirects: Option<usize>,
    retry: Option<usize>,
    cookies: Option<HashMap<String, String>>,
    keepalive: Option<bool>,
    reuseaddr: Option<bool>,
    nodelay: Option<bool>,
    ssl_verify: Option<bool>,
    ssl_cert: Option<String>,
    ssl_key: Option<String>,
    ssl_ca: Option<String>,
    ssl_sni: Option<String>,
    ssl_protocols: Option<String>,
    ssl_ciphers: Option<String>,
    ssl_session: Option<String>,
}

pub fn register_http_functions(functions: &mut HashMap<String, crate::native_modules::NativeFunction>) {
    functions.insert("native_http_get".to_string(), native_http_get);
    functions.insert("native_http_post".to_string(), native_http_post);
    functions.insert("native_http_headers".to_string(), native_http_headers);
    functions.insert("native_http_download".to_string(), native_http_download);
    functions.insert("native_http_status".to_string(), native_http_status);
    functions.insert("native_http_json".to_string(), native_http_json);

    functions.insert("native_http_set_timeout".to_string(), native_http_set_timeout);
    functions.insert("native_http_set_headers".to_string(), native_http_set_headers);
    functions.insert("native_http_set_user_agent".to_string(), native_http_set_user_agent);
    functions.insert("native_http_set_proxy".to_string(), native_http_set_proxy);
    functions.insert("native_http_set_auth".to_string(), native_http_set_auth);
    functions.insert("native_http_set_follow_redirects".to_string(), native_http_set_follow_redirects);
    functions.insert("native_http_set_cache".to_string(), native_http_set_cache);
    functions.insert("native_http_set_compression".to_string(), native_http_set_compression);
    functions.insert("native_http_set_max_redirects".to_string(), native_http_set_max_redirects);
    functions.insert("native_http_set_retry".to_string(), native_http_set_retry);
    functions.insert("native_http_set_cookies".to_string(), native_http_set_cookies);
    functions.insert("native_http_set_keepalive".to_string(), native_http_set_keepalive);
    functions.insert("native_http_set_reuseaddr".to_string(), native_http_set_reuseaddr);
    functions.insert("native_http_set_nodelay".to_string(), native_http_set_nodelay);
    functions.insert("native_http_set_ssl_verify".to_string(), native_http_set_ssl_verify);
    functions.insert("native_http_set_ssl_cert".to_string(), native_http_set_ssl_cert);
    functions.insert("native_http_set_ssl_key".to_string(), native_http_set_ssl_key);
    functions.insert("native_http_set_ssl_ca".to_string(), native_http_set_ssl_ca);
    functions.insert("native_http_set_ssl_sni".to_string(), native_http_set_ssl_sni);
    functions.insert("native_http_set_ssl_protocols".to_string(), native_http_set_ssl_protocols);
    functions.insert("native_http_set_ssl_ciphers".to_string(), native_http_set_ssl_ciphers);
    functions.insert("native_http_set_ssl_session".to_string(), native_http_set_ssl_session);
}

// Função auxiliar para obter config
fn get_config(url: &str) -> HttpConfig {
    HTTP_CONFIG.lock().unwrap().get(url).cloned().unwrap_or_default()
}

// Função auxiliar para atualizar config
fn update_config<F: FnOnce(&mut HttpConfig)>(url: &str, f: F) {
    let mut map = HTTP_CONFIG.lock().unwrap();
    let entry = map.entry(url.to_string()).or_insert_with(HttpConfig::default);
    f(entry);
}

// ========================
// Funções principais HTTP
// ========================

fn native_http_get(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager) -> Result<RuntimeValue, RuntimeError> {
    let url = match &args[0] {
        RuntimeValue::String(s) => s,
        _ => return Err(RuntimeError::TypeError("native_http_get: argumento deve ser string".to_string())),
    };
    let config = get_config(url);
    let client = build_client(&config)?;

    let mut request = client.get(url);
    if let Some((ref user, ref pass)) = config.auth {
        request = request.basic_auth(user, Some(pass));
    }

    match request.send() {
        Ok(resp) => match resp.text() {
            Ok(text) => Ok(RuntimeValue::String(text)),
            Err(e) => Err(RuntimeError::IoError(format!("Erro ao ler resposta: {}", e))),
        },
        Err(e) => Err(RuntimeError::IoError(format!("Erro na requisição GET: {}", e))),
    }
}

fn native_http_post(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager) -> Result<RuntimeValue, RuntimeError> {
    let url = match &args[0] {
        RuntimeValue::String(s) => s,
        _ => return Err(RuntimeError::TypeError("native_http_post: primeiro argumento deve ser string".to_string())),
    };
    let body = match &args[1] {
        RuntimeValue::String(s) => s,
        _ => return Err(RuntimeError::TypeError("native_http_post: segundo argumento deve ser string".to_string())),
    };
    let config = get_config(url);
    let client = build_client(&config)?;

    match client.post(url).body(body.clone()).send() {
        Ok(resp) => match resp.text() {
            Ok(text) => Ok(RuntimeValue::String(text)),
            Err(e) => Err(RuntimeError::IoError(format!("Erro ao ler resposta: {}", e))),
        },
        Err(e) => Err(RuntimeError::IoError(format!("Erro na requisição POST: {}", e))),
    }
}

fn native_http_headers(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager) -> Result<RuntimeValue, RuntimeError> {
    let url = match &args[0] {
        RuntimeValue::String(s) => s,
        _ => return Err(RuntimeError::TypeError("native_http_headers: argumento deve ser string".to_string())),
    };
    let config = get_config(url);
    let client = build_client(&config)?;

    match client.get(url).send() {
        Ok(resp) => {
            let mut headers_map = HashMap::new();
            for (key, value) in resp.headers().iter() {
                headers_map.insert(key.to_string(), RuntimeValue::String(value.to_str().unwrap_or("").to_string()));
            }
            Ok(RuntimeValue::Object { properties: headers_map, methods: HashMap::new() })
        },
        Err(e) => Err(RuntimeError::IoError(format!("Erro ao obter headers: {}", e))),
    }
}

fn native_http_download(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager) -> Result<RuntimeValue, RuntimeError> {
    let url = match &args[0] {
        RuntimeValue::String(s) => s,
        _ => return Err(RuntimeError::TypeError("native_http_download: primeiro argumento deve ser string".to_string())),
    };
    let path = match &args[1] {
        RuntimeValue::String(s) => s,
        _ => return Err(RuntimeError::TypeError("native_http_download: segundo argumento deve ser string".to_string())),
    };
    let config = get_config(url);
    let client = build_client(&config)?;

    match client.get(url).send() {
        Ok(mut resp) => {
            let mut file = File::create(path).map_err(|e| RuntimeError::IoError(format!("Erro ao criar arquivo: {}", e)))?;
            let bytes = resp.bytes().map_err(|e| RuntimeError::IoError(format!("Erro ao baixar conteúdo: {}", e)))?;
            file.write_all(&bytes).map_err(|e| RuntimeError::IoError(format!("Erro ao salvar arquivo: {}", e)))?;
            Ok(RuntimeValue::Null)
        },
        Err(e) => Err(RuntimeError::IoError(format!("Erro na requisição download: {}", e))),
    }
}

fn native_http_status(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager) -> Result<RuntimeValue, RuntimeError> {
    let url = match &args[0] {
        RuntimeValue::String(s) => s,
        _ => return Err(RuntimeError::TypeError("native_http_status: argumento deve ser string".to_string())),
    };
    let config = get_config(url);
    let client = build_client(&config)?;

    match client.get(url).send() {
        Ok(resp) => Ok(RuntimeValue::Number(resp.status().as_u16() as f64)),
        Err(e) => Err(RuntimeError::IoError(format!("Erro ao obter status: {}", e))),
    }
}

fn native_http_json(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager) -> Result<RuntimeValue, RuntimeError> {
    let url = match &args[0] {
        RuntimeValue::String(s) => s,
        _ => return Err(RuntimeError::TypeError("native_http_json: argumento deve ser string".to_string())),
    };
    let config = get_config(url);
    let client = build_client(&config)?;

    match client.get(url).send() {
        Ok(resp) => match resp.json::<serde_json::Value>() {
            Ok(json) => Ok(crate::native_modules::encode_decode::json_to_runtime_value(&json)),
            Err(e) => Err(RuntimeError::IoError(format!("Erro ao decodificar JSON: {}", e))),
        },
        Err(e) => Err(RuntimeError::IoError(format!("Erro na requisição JSON: {}", e))),
    }
}

// ========================
// Funções de configuração
// ========================

fn native_http_set_timeout(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager) -> Result<RuntimeValue, RuntimeError> {
    let url = match &args[0] { RuntimeValue::String(s) => s, _ => return Err(RuntimeError::TypeError("native_http_set_timeout: primeiro argumento deve ser string".to_string())) };
    let ms = match &args[1] { RuntimeValue::Number(n) => *n as u64, _ => return Err(RuntimeError::TypeError("native_http_set_timeout: segundo argumento deve ser número".to_string())) };
    update_config(url, |c| c.timeout_ms = Some(ms));
    Ok(RuntimeValue::Null)
}

fn native_http_set_headers(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager) -> Result<RuntimeValue, RuntimeError> {
    let url = match &args[0] { RuntimeValue::String(s) => s, _ => return Err(RuntimeError::TypeError("native_http_set_headers: primeiro argumento deve ser string".to_string())) };
    let obj = match &args[1] {
        RuntimeValue::Object { properties, .. } => properties,
        _ => return Err(RuntimeError::TypeError("native_http_set_headers: segundo argumento deve ser objeto".to_string())),
    };
    let mut headers = HashMap::new();
    for (k, v) in obj {
        if let RuntimeValue::String(val) = v {
            headers.insert(k.clone(), val.clone());
        }
    }
    update_config(url, |c| c.headers = Some(headers));
    Ok(RuntimeValue::Null)
}

fn native_http_set_user_agent(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager) -> Result<RuntimeValue, RuntimeError> {
    let url = match &args[0] { RuntimeValue::String(s) => s, _ => return Err(RuntimeError::TypeError("native_http_set_user_agent: primeiro argumento deve ser string".to_string())) };
    let agent = match &args[1] { RuntimeValue::String(s) => s.clone(), _ => return Err(RuntimeError::TypeError("native_http_set_user_agent: segundo argumento deve ser string".to_string())) };
    update_config(url, |c| c.user_agent = Some(agent));
    Ok(RuntimeValue::Null)
}

fn native_http_set_proxy(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager) -> Result<RuntimeValue, RuntimeError> {
    let url = match &args[0] { RuntimeValue::String(s) => s, _ => return Err(RuntimeError::TypeError("native_http_set_proxy: primeiro argumento deve ser string".to_string())) };
    let proxy = match &args[1] { RuntimeValue::String(s) => s.clone(), _ => return Err(RuntimeError::TypeError("native_http_set_proxy: segundo argumento deve ser string".to_string())) };
    update_config(url, |c| c.proxy = Some(proxy));
    Ok(RuntimeValue::Null)
}

fn native_http_set_auth(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager) -> Result<RuntimeValue, RuntimeError> {
    let url = match &args[0] { RuntimeValue::String(s) => s, _ => return Err(RuntimeError::TypeError("native_http_set_auth: primeiro argumento deve ser string".to_string())) };
    let user = match &args[1] { RuntimeValue::String(s) => s.clone(), _ => return Err(RuntimeError::TypeError("native_http_set_auth: segundo argumento deve ser string".to_string())) };
    let pass = match &args[2] { RuntimeValue::String(s) => s.clone(), _ => return Err(RuntimeError::TypeError("native_http_set_auth: terceiro argumento deve ser string".to_string())) };
    update_config(url, |c| c.auth = Some((user, pass)));
    Ok(RuntimeValue::Null)
}

fn native_http_set_follow_redirects(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager) -> Result<RuntimeValue, RuntimeError> {
    let url = match &args[0] { RuntimeValue::String(s) => s, _ => return Err(RuntimeError::TypeError("native_http_set_follow_redirects: primeiro argumento deve ser string".to_string())) };
    let enable = match &args[1] { RuntimeValue::Bool(b) => *b, _ => return Err(RuntimeError::TypeError("native_http_set_follow_redirects: segundo argumento deve ser bool".to_string())) };
    update_config(url, |c| c.follow_redirects = Some(enable));
    Ok(RuntimeValue::Null)
}

fn native_http_set_cache(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager) -> Result<RuntimeValue, RuntimeError> {
    let url = match &args[0] { RuntimeValue::String(s) => s, _ => return Err(RuntimeError::TypeError("native_http_set_cache: primeiro argumento deve ser string".to_string())) };
    let enable = match &args[1] { RuntimeValue::Bool(b) => *b, _ => return Err(RuntimeError::TypeError("native_http_set_cache: segundo argumento deve ser bool".to_string())) };
    update_config(url, |c| c.cache = Some(enable));
    Ok(RuntimeValue::Null)
}

fn native_http_set_compression(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager) -> Result<RuntimeValue, RuntimeError> {
    let url = match &args[0] { RuntimeValue::String(s) => s, _ => return Err(RuntimeError::TypeError("native_http_set_compression: primeiro argumento deve ser string".to_string())) };
    let enable = match &args[1] { RuntimeValue::Bool(b) => *b, _ => return Err(RuntimeError::TypeError("native_http_set_compression: segundo argumento deve ser bool".to_string())) };
    update_config(url, |c| c.compression = Some(enable));
    Ok(RuntimeValue::Null)
}

fn native_http_set_max_redirects(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager) -> Result<RuntimeValue, RuntimeError> {
    let url = match &args[0] { RuntimeValue::String(s) => s, _ => return Err(RuntimeError::TypeError("native_http_set_max_redirects: primeiro argumento deve ser string".to_string())) };
    let max_redirects = match &args[1] { RuntimeValue::Number(n) => *n as usize, _ => return Err(RuntimeError::TypeError("native_http_set_max_redirects: segundo argumento deve ser número".to_string())) };
    update_config(url, |c| c.max_redirects = Some(max_redirects));
    Ok(RuntimeValue::Null)
}

fn native_http_set_retry(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager) -> Result<RuntimeValue, RuntimeError> {
    let url = match &args[0] { RuntimeValue::String(s) => s, _ => return Err(RuntimeError::TypeError("native_http_set_retry: primeiro argumento deve ser string".to_string())) };
    let retry = match &args[1] { RuntimeValue::Number(n) => *n as usize, _ => return Err(RuntimeError::TypeError("native_http_set_retry: segundo argumento deve ser número".to_string())) };
    update_config(url, |c| c.retry = Some(retry));
    Ok(RuntimeValue::Null)
}

fn native_http_set_cookies(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager) -> Result<RuntimeValue, RuntimeError> {
    let url = match &args[0] { RuntimeValue::String(s) => s, _ => return Err(RuntimeError::TypeError("native_http_set_cookies: primeiro argumento deve ser string".to_string())) };
    let obj = match &args[1] {
        RuntimeValue::Object { properties, .. } => properties,
        _ => return Err(RuntimeError::TypeError("native_http_set_cookies: segundo argumento deve ser objeto".to_string())),
    };
    let mut cookies = HashMap::new();
    for (k, v) in obj {
        if let RuntimeValue::String(val) = v {
            cookies.insert(k.clone(), val.clone());
        }
    }
    update_config(url, |c| c.cookies = Some(cookies));
    Ok(RuntimeValue::Null)
}

fn native_http_set_keepalive(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager) -> Result<RuntimeValue, RuntimeError> {
    let url = match &args[0] { RuntimeValue::String(s) => s, _ => return Err(RuntimeError::TypeError("native_http_set_keepalive: primeiro argumento deve ser string".to_string())) };
    let enable = match &args[1] { RuntimeValue::Bool(b) => *b, _ => return Err(RuntimeError::TypeError("native_http_set_keepalive: segundo argumento deve ser bool".to_string())) };
    update_config(url, |c| c.keepalive = Some(enable));
    Ok(RuntimeValue::Null)
}

fn native_http_set_reuseaddr(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager) -> Result<RuntimeValue, RuntimeError> {
    let url = match &args[0] { RuntimeValue::String(s) => s, _ => return Err(RuntimeError::TypeError("native_http_set_reuseaddr: primeiro argumento deve ser string".to_string())) };
    let enable = match &args[1] { RuntimeValue::Bool(b) => *b, _ => return Err(RuntimeError::TypeError("native_http_set_reuseaddr: segundo argumento deve ser bool".to_string())) };
    update_config(url, |c| c.reuseaddr = Some(enable));
    Ok(RuntimeValue::Null)
}

fn native_http_set_nodelay(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager) -> Result<RuntimeValue, RuntimeError> {
    let url = match &args[0] { RuntimeValue::String(s) => s, _ => return Err(RuntimeError::TypeError("native_http_set_nodelay: primeiro argumento deve ser string".to_string())) };
    let enable = match &args[1] { RuntimeValue::Bool(b) => *b, _ => return Err(RuntimeError::TypeError("native_http_set_nodelay: segundo argumento deve ser bool".to_string())) };
    update_config(url, |c| c.nodelay = Some(enable));
    Ok(RuntimeValue::Null)
}

fn native_http_set_ssl_verify(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager) -> Result<RuntimeValue, RuntimeError> {
    let url = match &args[0] { RuntimeValue::String(s) => s, _ => return Err(RuntimeError::TypeError("native_http_set_ssl_verify: primeiro argumento deve ser string".to_string())) };
    let verify = match &args[1] { RuntimeValue::Bool(b) => *b, _ => return Err(RuntimeError::TypeError("native_http_set_ssl_verify: segundo argumento deve ser bool".to_string())) };
    update_config(url, |c| c.ssl_verify = Some(verify));
    Ok(RuntimeValue::Null)
}

fn native_http_set_ssl_cert(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager) -> Result<RuntimeValue, RuntimeError> {
    let url = match &args[0] { RuntimeValue::String(s) => s, _ => return Err(RuntimeError::TypeError("native_http_set_ssl_cert: primeiro argumento deve ser string".to_string())) };
    let cert = match &args[1] { RuntimeValue::String(s) => s.clone(), _ => return Err(RuntimeError::TypeError("native_http_set_ssl_cert: segundo argumento deve ser string".to_string())) };
    update_config(url, |c| c.ssl_cert = Some(cert));
    Ok(RuntimeValue::Null)
}

fn native_http_set_ssl_key(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager) -> Result<RuntimeValue, RuntimeError> {
    let url = match &args[0] { RuntimeValue::String(s) => s, _ => return Err(RuntimeError::TypeError("native_http_set_ssl_key: primeiro argumento deve ser string".to_string())) };
    let key = match &args[1] { RuntimeValue::String(s) => s.clone(), _ => return Err(RuntimeError::TypeError("native_http_set_ssl_key: segundo argumento deve ser string".to_string())) };
    update_config(url, |c| c.ssl_key = Some(key));
    Ok(RuntimeValue::Null)
}

fn native_http_set_ssl_ca(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager) -> Result<RuntimeValue, RuntimeError> {
    let url = match &args[0] { RuntimeValue::String(s) => s, _ => return Err(RuntimeError::TypeError("native_http_set_ssl_ca: primeiro argumento deve ser string".to_string())) };
    let ca = match &args[1] { RuntimeValue::String(s) => s.clone(), _ => return Err(RuntimeError::TypeError("native_http_set_ssl_ca: segundo argumento deve ser string".to_string())) };
    update_config(url, |c| c.ssl_ca = Some(ca));
    Ok(RuntimeValue::Null)
}

fn native_http_set_ssl_sni(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager) -> Result<RuntimeValue, RuntimeError> {
    let url = match &args[0] { RuntimeValue::String(s) => s, _ => return Err(RuntimeError::TypeError("native_http_set_ssl_sni: primeiro argumento deve ser string".to_string())) };
    let sni = match &args[1] { RuntimeValue::String(s) => s.clone(), _ => return Err(RuntimeError::TypeError("native_http_set_ssl_sni: segundo argumento deve ser string".to_string())) };
    update_config(url, |c| c.ssl_sni = Some(sni));
    Ok(RuntimeValue::Null)
}

fn native_http_set_ssl_protocols(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager) -> Result<RuntimeValue, RuntimeError> {
    let url = match &args[0] { RuntimeValue::String(s) => s, _ => return Err(RuntimeError::TypeError("native_http_set_ssl_protocols: primeiro argumento deve ser string".to_string())) };
    let protocols = match &args[1] { RuntimeValue::String(s) => s.clone(), _ => return Err(RuntimeError::TypeError("native_http_set_ssl_protocols: segundo argumento deve ser string".to_string())) };
    update_config(url, |c| c.ssl_protocols = Some(protocols));
    Ok(RuntimeValue::Null)
}

fn native_http_set_ssl_ciphers(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager) -> Result<RuntimeValue, RuntimeError> {
    let url = match &args[0] { RuntimeValue::String(s) => s, _ => return Err(RuntimeError::TypeError("native_http_set_ssl_ciphers: primeiro argumento deve ser string".to_string())) };
    let ciphers = match &args[1] { RuntimeValue::String(s) => s.clone(), _ => return Err(RuntimeError::TypeError("native_http_set_ssl_ciphers: segundo argumento deve ser string".to_string())) };
    update_config(url, |c| c.ssl_ciphers = Some(ciphers));
    Ok(RuntimeValue::Null)
}

fn native_http_set_ssl_session(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager) -> Result<RuntimeValue, RuntimeError> {
    let url = match &args[0] { RuntimeValue::String(s) => s, _ => return Err(RuntimeError::TypeError("native_http_set_ssl_session: primeiro argumento deve ser string".to_string())) };
    let session = match &args[1] { RuntimeValue::String(s) => s.clone(), _ => return Err(RuntimeError::TypeError("native_http_set_ssl_session: segundo argumento deve ser string".to_string())) };
    update_config(url, |c| c.ssl_session = Some(session));
    Ok(RuntimeValue::Null)
}

// ========================
// Auxiliar: constrói o client
// ========================
fn build_client(config: &HttpConfig) -> Result<Client, RuntimeError> {
    let mut builder = Client::builder();

    if let Some(timeout) = config.timeout_ms {
        builder = builder.timeout(std::time::Duration::from_millis(timeout));
    }
    if let Some(ref headers_map) = config.headers {
        let mut headers = HeaderMap::new();
        for (k, v) in headers_map {
            headers.insert(
                reqwest::header::HeaderName::from_bytes(k.as_bytes()).unwrap(),
                reqwest::header::HeaderValue::from_str(v).unwrap(),
            );
        }
        builder = builder.default_headers(headers);
    }
    if let Some(ref agent) = config.user_agent {
        builder = builder.user_agent(agent);
    }
    if let Some(ref proxy) = config.proxy {
        builder = builder.proxy(reqwest::Proxy::all(proxy).map_err(|e| RuntimeError::IoError(format!("Proxy inválido: {}", e)))?);
    }
    if let Some(verify) = config.ssl_verify {
        builder = builder.danger_accept_invalid_certs(!verify);
    }
    // Outras opções podem ser implementadas conforme necessário

    builder.build().map_err(|e| RuntimeError::IoError(format!("Erro ao construir client HTTP: {}", e)))
}