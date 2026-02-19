/// Módulos de funções nativas organizados por categoria
/// 
/// Cada módulo implementa uma categoria específica de funções nativas
/// que podem ser carregadas individualmente através de diretivas #<categoria>

pub mod console_io;
pub mod terminal_ansi;
pub mod binary_io;
pub mod file_io;
pub mod time;
pub mod system_env;
pub mod encode_decode;
pub mod crypto;
pub mod debug;
pub mod utils; 
pub mod http_client;
pub mod http_server;
pub mod tcp;
pub mod udp;
pub mod ffi;
pub mod json_stream;
pub mod websocket;
pub mod database;

// Módulos futuros:
// pub mod websocket;

use crate::heap::{Heap, HeapId};
use crate::interpreter::Value;
use crate::errors::RuntimeError;
use std::collections::{HashMap, HashSet};
use std::future::Future;
use std::pin::Pin;

/// Tipo para funções nativas
pub type NativeFunction = fn(&[Value], &NativeModuleManager, &mut Heap) -> Result<Value, RuntimeError>;

/// Tipo para funções nativas assíncronas
pub type AsyncNativeFunction = fn(Vec<Value>, &NativeModuleManager, &mut Heap) -> Pin<Box<dyn Future<Output = Result<Value, RuntimeError>> + Send + 'static>>;

/// Gerenciador de módulos nativos
pub struct NativeModuleManager {
    /// Funções registradas por categoria (síncronas)
    categories: HashMap<String, HashMap<String, NativeFunction>>,
    /// Funções registradas por categoria (assíncronas)
    async_categories: HashMap<String, HashMap<String, AsyncNativeFunction>>,
    /// Categorias ativas (carregadas através de diretivas)
    active_categories: HashSet<String>,
    /// Flag para permitir operações inseguras (ex: native_set_env)
    allow_unsafe: bool,
    /// Flag para permitir execução de comandos (native_exec, native_exec_output)
    allow_exec: bool,
    /// Diretório raiz para o sandbox de arquivos (se None, usa o diretório atual como base)
    sandbox_root: Option<std::path::PathBuf>,
}

impl NativeModuleManager {
    pub fn new() -> Self {
        let mut manager = Self {
            categories: HashMap::new(),
            async_categories: HashMap::new(),
            active_categories: HashSet::new(),
            allow_unsafe: false,
            allow_exec: false,
            sandbox_root: None,
        };
        
        // Registra todas as categorias disponíveis
        manager.register_all_categories();
        
        // Ativa console_io por padrão (print, println, input, etc.)
        // let _ = manager.activate_category("console_io");
        
        manager
    }
    
    /// Registra todas as categorias de funções nativas
    fn register_all_categories(&mut self) {
        // Registra console_io
        let mut console_io_functions = HashMap::new();
        console_io::register_console_io_functions(&mut console_io_functions);
        self.categories.insert("console_io".to_string(), console_io_functions);
        
        // Registra terminal_ansi
        let mut terminal_ansi_functions = HashMap::new();
        terminal_ansi::register_terminal_ansi_functions(&mut terminal_ansi_functions);
        self.categories.insert("terminal_ansi".to_string(), terminal_ansi_functions);
        
        // Registra binary_io
        let mut binary_io_functions = HashMap::new();
        binary_io::register_binary_io_functions(&mut binary_io_functions);
        self.categories.insert("binary_io".to_string(), binary_io_functions);
        
        // Registra file_io
        let mut file_io_functions = HashMap::new();
        let mut file_io_async_functions = HashMap::new();
        file_io::register_file_io_functions(&mut file_io_functions);
        file_io::register_file_io_async_functions(&mut file_io_async_functions);
        self.categories.insert("file_io".to_string(), file_io_functions);
        self.async_categories.insert("file_io".to_string(), file_io_async_functions);
        
        // Registra time
        let mut time_functions = HashMap::new();
        time::register_time_functions(&mut time_functions);
        self.categories.insert("time".to_string(), time_functions.clone());
        self.categories.insert("date_time".to_string(), time_functions); // Alias para compatibilidade
        
        // Registra system_env
        let mut system_env_functions = HashMap::new();
        system_env::register_system_env_functions(&mut system_env_functions);
        self.categories.insert("system_env".to_string(), system_env_functions);
        
        // Registra encode_decode
        let mut encode_decode_functions = HashMap::new();
        encode_decode::register_encode_decode_functions(&mut encode_decode_functions);
        self.categories.insert("encode_decode".to_string(), encode_decode_functions);
        
        // Registra crypto
        let mut crypto_functions = HashMap::new();
        crypto::register_crypto_functions(&mut crypto_functions);
        self.categories.insert("crypto".to_string(), crypto_functions);
        
        // Registra debug
        let mut debug_functions = HashMap::new();
        debug::register_debug_functions(&mut debug_functions);
        self.categories.insert("debug".to_string(), debug_functions);

        // Registra utils
        let mut utils_functions = HashMap::new();
        utils::register_utils_functions(&mut utils_functions);
        self.categories.insert("utils".to_string(), utils_functions);

        // Registra http_client
        let mut http_client_functions = HashMap::new();
        http_client::register_http_client_functions(&mut http_client_functions);
        self.categories.insert("http_client".to_string(), http_client_functions);
        
        // Registra http_server
        let mut http_server_functions = HashMap::new();
        http_server::register_http_server_functions(&mut http_server_functions);
        self.categories.insert("http_server".to_string(), http_server_functions);
        
        // Registra tcp
        let mut tcp_functions = HashMap::new();
        tcp::register_tcp_functions(&mut tcp_functions);
        self.categories.insert("tcp".to_string(), tcp_functions);
        
        // Registra UDP
        let mut udp_functions = HashMap::new();
        udp::register_udp_functions(&mut udp_functions);
        self.categories.insert("udp".to_string(), udp_functions);

        // Registra FFI
        let mut ffi_functions = HashMap::new();
        ffi::register_ffi_functions(&mut ffi_functions);
        self.categories.insert("ffi".to_string(), ffi_functions);

        // Registra JSON Stream
        let mut json_stream_functions = HashMap::new();
        json_stream::register_json_stream_functions(&mut json_stream_functions);
        self.categories.insert("json_stream".to_string(), json_stream_functions);

        // Registra WebSocket
        let mut websocket_functions = HashMap::new();
        websocket::register_websocket_functions(&mut websocket_functions);
        self.categories.insert("websocket".to_string(), websocket_functions);

        // Registra Database
        let mut database_functions = HashMap::new();
        database::register_database_functions(&mut database_functions);
        self.categories.insert("database".to_string(), database_functions);
    }
    
    /// Ativa uma categoria específica através de diretiva #<categoria>
    pub fn activate_category(&mut self, category: &str) -> Result<(), String> {
        if !self.categories.contains_key(category) && !self.async_categories.contains_key(category) {
            return Err(format!("Módulo nativo desconhecido: {}", category));
        }
        
        self.active_categories.insert(category.to_string());
        Ok(())
    }
    
    /// Desativa uma categoria
    pub fn deactivate_category(&mut self, category: &str) {
        self.active_categories.remove(category);
    }
    
    /// Verifica se uma categoria está ativa
    pub fn is_category_active(&self, category: &str) -> bool {
        self.active_categories.contains(category)
    }
    
    /// Obtém uma função nativa se sua categoria estiver ativa
    pub fn get_function(&self, function_name: &str) -> Option<NativeFunction> {
        for (category, functions) in &self.categories {
            if self.active_categories.contains(category) {
                if let Some(func) = functions.get(function_name) {
                    return Some(*func);
                }
            }
        }
        None
    }

    /// Obtém uma função nativa assíncrona se sua categoria estiver ativa
    pub fn get_async_function(&self, function_name: &str) -> Option<AsyncNativeFunction> {
        for (category, functions) in &self.async_categories {
            if self.active_categories.contains(category) {
                if let Some(func) = functions.get(function_name) {
                    return Some(*func);
                }
            }
        }
        None
    }

    pub fn set_allow_unsafe(&mut self, allow: bool) {
        self.allow_unsafe = allow;
    }

    pub fn allow_unsafe(&self) -> bool {
        self.allow_unsafe
    }

    pub fn set_allow_exec(&mut self, allow: bool) {
        self.allow_exec = allow;
    }

    pub fn allow_exec(&self) -> bool {
        self.allow_exec
    }

    pub fn set_sandbox_root(&mut self, root: std::path::PathBuf) {
        self.sandbox_root = Some(root);
    }

    pub fn sandbox_root(&self) -> Option<&std::path::PathBuf> {
        self.sandbox_root.as_ref()
    }
    
    /// Lista todas as funções ativas (de categorias carregadas)
    pub fn list_active_functions(&self) -> Vec<String> {
        let mut functions = Vec::new();
        
        for (category, category_functions) in &self.categories {
            if self.active_categories.contains(category) {
                for function_name in category_functions.keys() {
                    functions.push(function_name.clone());
                }
            }
        }
        
        functions.sort();
        functions
    }
    
    /// Lista todas as categorias disponíveis
    pub fn list_categories(&self) -> Vec<String> {
        self.categories.keys().cloned().collect()
    }
    
    /// Lista categorias ativas
    pub fn list_active_categories(&self) -> Vec<String> {
        self.active_categories.iter().cloned().collect()
    }
    
    /// Obtém informações sobre uma categoria
    pub fn get_category_info(&self, category: &str) -> Option<(bool, Vec<String>)> {
        if let Some(functions) = self.categories.get(category) {
            let is_active = self.active_categories.contains(category);
            let function_names: Vec<String> = functions.keys().cloned().collect();
            Some((is_active, function_names))
        } else {
            None
        }
    }
    
    /// Verifica se uma função nativa existe em qualquer categoria (mesmo que inativa)
    /// Retorna o nome da categoria se encontrada, ou None se não existir
    pub fn find_function_category(&self, function_name: &str) -> Option<String> {
        // Verificar em categorias síncronas
        for (category, functions) in &self.categories {
            if functions.contains_key(function_name) {
                return Some(category.clone());
            }
        }
        
        // Verificar em categorias assíncronas
        for (category, functions) in &self.async_categories {
            if functions.contains_key(function_name) {
                return Some(category.clone());
            }
        }
        
        None
    }
    
    /// Verifica se uma função existe mas está em uma categoria inativa
    pub fn is_function_in_inactive_category(&self, function_name: &str) -> bool {
        if let Some(category) = self.find_function_category(function_name) {
            !self.active_categories.contains(&category)
        } else {
            false
        }
    }
}
