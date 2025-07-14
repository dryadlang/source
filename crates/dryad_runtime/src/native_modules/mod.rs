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

// Módulos futuros:
// pub mod http;
// pub mod websocket;
// pub mod tcp;
// pub mod udp;
// pub mod web_server;

use crate::interpreter::Value;
use crate::errors::RuntimeError;
use std::collections::HashMap;

/// Tipo para funções nativas
pub type NativeFunction = fn(&[Value]) -> Result<Value, RuntimeError>;

/// Gerenciador de módulos nativos
pub struct NativeModuleManager {
    /// Funções registradas por categoria
    categories: HashMap<String, HashMap<String, NativeFunction>>,
    /// Categorias ativas (carregadas através de diretivas)
    active_categories: std::collections::HashSet<String>,
}

impl NativeModuleManager {
    pub fn new() -> Self {
        let mut manager = Self {
            categories: HashMap::new(),
            active_categories: std::collections::HashSet::new(),
        };
        
        // Registra todas as categorias disponíveis
        manager.register_all_categories();
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
        file_io::register_file_io_functions(&mut file_io_functions);
        self.categories.insert("file_io".to_string(), file_io_functions);
        
        // Registra time
        let mut time_functions = HashMap::new();
        time::register_time_functions(&mut time_functions);
        self.categories.insert("time".to_string(), time_functions);
        
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
        
        // Futuramente adicionar outras categorias aqui
    }
    
    /// Ativa uma categoria específica através de diretiva #<categoria>
    pub fn activate_category(&mut self, category: &str) -> Result<(), String> {
        if !self.categories.contains_key(category) {
            return Err(format!("Categoria '{}' não encontrada", category));
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
    pub fn get_function(&self, function_name: &str) -> Option<&NativeFunction> {
        for (category, functions) in &self.categories {
            if self.active_categories.contains(category) {
                if let Some(func) = functions.get(function_name) {
                    return Some(func);
                }
            }
        }
        None
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
}
