// crates/dryad_runtime/src/native_functions.rs

use crate::interpreter::Value;
use dryad_errors::DryadError;
use std::collections::HashMap;
use std::io::{self, Write, Read};
use std::fs::{self, OpenOptions};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH, Instant};
use std::thread;
use std::env;
use std::process::Command;

// Conjunto de módulos nativos disponíveis
#[derive(Debug, Clone)]
pub enum NativeModule {
    ConsoleIO,
    FileIO,
    TerminalAnsi,
    BinaryIO,
    DateTime,
    SystemEnv,
    Crypto,
    Debug,
    DataStructures,
    Http,
    WebSocket,
    Tcp,
    Udp,
    WebServer,
}

impl NativeModule {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "console_io" => Some(Self::ConsoleIO),
            "file_io" => Some(Self::FileIO),
            "terminal_ansi" => Some(Self::TerminalAnsi),
            "binary_io" => Some(Self::BinaryIO),
            "date_time" => Some(Self::DateTime),
            "system_env" => Some(Self::SystemEnv),
            "crypto" => Some(Self::Crypto),
            "debug" => Some(Self::Debug),
            "http" => Some(Self::Http),
            "websocket" => Some(Self::WebSocket),
            "tcp" => Some(Self::Tcp),
            "udp" => Some(Self::Udp),
            "web_server" => Some(Self::WebServer),
            _ => None,
        }
    }
}

// Sistema de funções nativas
pub struct NativeFunctionRegistry {
    enabled_modules: Vec<NativeModule>,
    functions: HashMap<String, fn(&[Value]) -> Result<Value, DryadError>>,
    _start_time: Instant,
}

impl NativeFunctionRegistry {
    pub fn new() -> Self {
        Self {
            enabled_modules: Vec::new(),
            functions: HashMap::new(),
            _start_time: Instant::now(),
        }
    }

    pub fn enable_module(&mut self, module: NativeModule) {
        if !self.enabled_modules.contains(&module) {
            self.enabled_modules.push(module.clone());
            self.register_module_functions(&module);
        }
    }

    pub fn is_native_function(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }

    pub fn call_native_function(&self, name: &str, args: &[Value]) -> Result<Value, DryadError> {
        if let Some(func) = self.functions.get(name) {
            func(args)
        } else {
            Err(DryadError::new(3005, &format!("Função nativa '{}' não encontrada", name)))
        }
    }

    fn register_module_functions(&mut self, module: &NativeModule) {
        match module {
            NativeModule::ConsoleIO => self.register_console_io_functions(),
            NativeModule::FileIO => self.register_file_io_functions(),
            NativeModule::TerminalAnsi => self.register_terminal_ansi_functions(),
            NativeModule::BinaryIO => self.register_binary_io_functions(),
            NativeModule::DateTime => self.register_date_time_functions(),
            NativeModule::SystemEnv => self.register_system_env_functions(),
            NativeModule::Crypto => self.register_crypto_functions(),
            NativeModule::Debug => self.register_debug_functions(),
            NativeModule::DataStructures => {
                // Estruturas de dados serão implementadas no futuro
                eprintln!("Módulo DataStructures ainda não implementado");
            },
            _ => {
                // Módulos avançados (HTTP, WebSocket, etc.) serão implementados no futuro
                eprintln!("Módulo {:?} ainda não implementado", module);
            }
        }
    }

    // === MÓDULO: Console I/O ===
    fn register_console_io_functions(&mut self) {
        // Entrada do console
        self.functions.insert("native_input".to_string(), |_args| {
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => Ok(Value::String(input.trim().to_string())),
                Err(_) => Err(DryadError::new(5001, "Erro ao ler entrada do console")),
            }
        });

        self.functions.insert("native_input_char".to_string(), |_args| {
            let mut buffer = [0; 1];
            match io::stdin().read_exact(&mut buffer) {
                Ok(_) => Ok(Value::String(String::from_utf8_lossy(&buffer).to_string())),
                Err(_) => Err(DryadError::new(5001, "Erro ao ler caractere do console")),
            }
        });

        self.functions.insert("native_input_bytes".to_string(), |args| {
            if args.len() != 1 {
                return Err(DryadError::new(3004, "native_input_bytes espera 1 argumento (count)"));
            }
            
            let count = match &args[0] {
                Value::Number(n) => *n as usize,
                _ => return Err(DryadError::new(3002, "Argumento deve ser um número")),
            };

            let mut buffer = vec![0; count];
            match io::stdin().read_exact(&mut buffer) {
                Ok(_) => Ok(Value::String(String::from_utf8_lossy(&buffer).to_string())),
                Err(_) => Err(DryadError::new(5001, "Erro ao ler bytes do console")),
            }
        });

        // Saída do console
        self.functions.insert("native_print".to_string(), |args| {
            if args.is_empty() {
                return Ok(Value::Null);
            }
            print!("{}", args[0].to_string());
            let _ = io::stdout().flush();
            Ok(Value::Null)
        });

        self.functions.insert("print".to_string(), |args| {
            if args.is_empty() {
                return Ok(Value::Null);
            }
            print!("{}", args[0].to_string());
            let _ = io::stdout().flush();
            Ok(Value::Null)
        });

        self.functions.insert("native_println".to_string(), |args| {
            if args.is_empty() {
                println!();
            } else {
                println!("{}", args[0].to_string());
            }
            Ok(Value::Null)
        });

        self.functions.insert("println".to_string(), |args| {
            if args.is_empty() {
                println!();
            } else {
                println!("{}", args[0].to_string());
            }
            Ok(Value::Null)
        });

        self.functions.insert("native_flush".to_string(), |_args| {
            match io::stdout().flush() {
                Ok(_) => Ok(Value::Null),
                Err(_) => Err(DryadError::new(5002, "Erro ao fazer flush do stdout")),
            }
        });
    }

    // === MÓDULO: File I/O ===
    fn register_file_io_functions(&mut self) {
        self.functions.insert("native_read_file".to_string(), |args| {
            if args.len() != 1 {
                return Err(DryadError::new(3004, "native_read_file espera 1 argumento (path)"));
            }
            
            let path = match &args[0] {
                Value::String(s) => s,
                _ => return Err(DryadError::new(3002, "Caminho deve ser uma string")),
            };

            match fs::read_to_string(path) {
                Ok(content) => Ok(Value::String(content)),
                Err(_) => Err(DryadError::new(5003, &format!("Erro ao ler arquivo: {}", path))),
            }
        });

        self.functions.insert("native_write_file".to_string(), |args| {
            if args.len() != 2 {
                return Err(DryadError::new(3004, "native_write_file espera 2 argumentos (path, data)"));
            }
            
            let path = match &args[0] {
                Value::String(s) => s,
                _ => return Err(DryadError::new(3002, "Caminho deve ser uma string")),
            };

            let data = args[1].to_string();

            match fs::write(path, data) {
                Ok(_) => Ok(Value::Bool(true)),
                Err(_) => Err(DryadError::new(5004, &format!("Erro ao escrever arquivo: {}", path))),
            }
        });

        self.functions.insert("native_append_file".to_string(), |args| {
            if args.len() != 2 {
                return Err(DryadError::new(3004, "native_append_file espera 2 argumentos (path, data)"));
            }
            
            let path = match &args[0] {
                Value::String(s) => s,
                _ => return Err(DryadError::new(3002, "Caminho deve ser uma string")),
            };

            let data = args[1].to_string();

            match OpenOptions::new().create(true).append(true).open(path) {
                Ok(mut file) => {
                    match file.write_all(data.as_bytes()) {
                        Ok(_) => Ok(Value::Bool(true)),
                        Err(_) => Err(DryadError::new(5004, &format!("Erro ao adicionar ao arquivo: {}", path))),
                    }
                }
                Err(_) => Err(DryadError::new(5004, &format!("Erro ao abrir arquivo: {}", path))),
            }
        });

        self.functions.insert("native_delete_file".to_string(), |args| {
            if args.len() != 1 {
                return Err(DryadError::new(3004, "native_delete_file espera 1 argumento (path)"));
            }
            
            let path = match &args[0] {
                Value::String(s) => s,
                _ => return Err(DryadError::new(3002, "Caminho deve ser uma string")),
            };

            match fs::remove_file(path) {
                Ok(_) => Ok(Value::Bool(true)),
                Err(_) => Err(DryadError::new(5005, &format!("Erro ao deletar arquivo: {}", path))),
            }
        });

        self.functions.insert("native_file_exists".to_string(), |args| {
            if args.len() != 1 {
                return Err(DryadError::new(3004, "native_file_exists espera 1 argumento (path)"));
            }
            
            let path = match &args[0] {
                Value::String(s) => s,
                _ => return Err(DryadError::new(3002, "Caminho deve ser uma string")),
            };

            Ok(Value::Bool(Path::new(path).exists()))
        });

        self.functions.insert("file_exists".to_string(), |args| {
            if args.len() != 1 {
                return Err(DryadError::new(3004, "file_exists espera 1 argumento (path)"));
            }
            
            let path = match &args[0] {
                Value::String(s) => s,
                _ => return Err(DryadError::new(3002, "Caminho deve ser uma string")),
            };

            Ok(Value::Bool(Path::new(path).exists()))
        });

        self.functions.insert("native_is_dir".to_string(), |args| {
            if args.len() != 1 {
                return Err(DryadError::new(3004, "native_is_dir espera 1 argumento (path)"));
            }
            
            let path = match &args[0] {
                Value::String(s) => s,
                _ => return Err(DryadError::new(3002, "Caminho deve ser uma string")),
            };

            Ok(Value::Bool(Path::new(path).is_dir()))
        });

        self.functions.insert("native_mkdir".to_string(), |args| {
            if args.len() != 1 {
                return Err(DryadError::new(3004, "native_mkdir espera 1 argumento (path)"));
            }
            
            let path = match &args[0] {
                Value::String(s) => s,
                _ => return Err(DryadError::new(3002, "Caminho deve ser uma string")),
            };

            match fs::create_dir_all(path) {
                Ok(_) => Ok(Value::Bool(true)),
                Err(_) => Err(DryadError::new(5006, &format!("Erro ao criar diretório: {}", path))),
            }
        });

        self.functions.insert("native_getcwd".to_string(), |_args| {
            match env::current_dir() {
                Ok(path) => Ok(Value::String(path.to_string_lossy().to_string())),
                Err(_) => Err(DryadError::new(5007, "Erro ao obter diretório atual")),
            }
        });
    }

    // === MÓDULO: Terminal ANSI ===
    fn register_terminal_ansi_functions(&mut self) {
        self.functions.insert("native_clear_screen".to_string(), |_args| {
            print!("\x1B[2J\x1B[H");
            let _ = io::stdout().flush();
            Ok(Value::Null)
        });

        self.functions.insert("native_move_cursor".to_string(), |args| {
            if args.len() != 2 {
                return Err(DryadError::new(3004, "native_move_cursor espera 2 argumentos (x, y)"));
            }
            
            let x = match &args[0] {
                Value::Number(n) => *n as u32,
                _ => return Err(DryadError::new(3002, "Coordenada X deve ser um número")),
            };

            let y = match &args[1] {
                Value::Number(n) => *n as u32,
                _ => return Err(DryadError::new(3002, "Coordenada Y deve ser um número")),
            };

            print!("\x1B[{};{}H", y, x);
            let _ = io::stdout().flush();
            Ok(Value::Null)
        });

        self.functions.insert("native_hide_cursor".to_string(), |_args| {
            print!("\x1B[?25l");
            let _ = io::stdout().flush();
            Ok(Value::Null)
        });

        self.functions.insert("native_show_cursor".to_string(), |_args| {
            print!("\x1B[?25h");
            let _ = io::stdout().flush();
            Ok(Value::Null)
        });

        self.functions.insert("native_reset_style".to_string(), |_args| {
            print!("\x1B[0m");
            let _ = io::stdout().flush();
            Ok(Value::Null)
        });

        // Funções de cores ANSI
        self.functions.insert("ansi_red".to_string(), |args| {
            if args.len() != 1 {
                return Err(DryadError::new(3004, "ansi_red espera 1 argumento (text)"));
            }
            let text = args[0].to_string();
            Ok(Value::String(format!("\x1B[31m{}\x1B[0m", text)))
        });

        self.functions.insert("ansi_green".to_string(), |args| {
            if args.len() != 1 {
                return Err(DryadError::new(3004, "ansi_green espera 1 argumento (text)"));
            }
            let text = args[0].to_string();
            Ok(Value::String(format!("\x1B[32m{}\x1B[0m", text)))
        });

        self.functions.insert("ansi_yellow".to_string(), |args| {
            if args.len() != 1 {
                return Err(DryadError::new(3004, "ansi_yellow espera 1 argumento (text)"));
            }
            let text = args[0].to_string();
            Ok(Value::String(format!("\x1B[33m{}\x1B[0m", text)))
        });

        self.functions.insert("ansi_blue".to_string(), |args| {
            if args.len() != 1 {
                return Err(DryadError::new(3004, "ansi_blue espera 1 argumento (text)"));
            }
            let text = args[0].to_string();
            Ok(Value::String(format!("\x1B[34m{}\x1B[0m", text)))
        });
    }

    // === MÓDULO: Date/Time ===
    fn register_date_time_functions(&mut self) {
        self.functions.insert("native_now".to_string(), |_args| {
            match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(duration) => Ok(Value::Number(duration.as_secs_f64())),
                Err(_) => Err(DryadError::new(5008, "Erro ao obter timestamp atual")),
            }
        });

        self.functions.insert("native_timestamp".to_string(), |_args| {
            match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(duration) => Ok(Value::Number(duration.as_secs() as f64)),
                Err(_) => Err(DryadError::new(5008, "Erro ao obter timestamp unix")),
            }
        });

        self.functions.insert("native_sleep".to_string(), |args| {
            if args.len() != 1 {
                return Err(DryadError::new(3004, "native_sleep espera 1 argumento (ms)"));
            }
            
            let ms = match &args[0] {
                Value::Number(n) => *n as u64,
                _ => return Err(DryadError::new(3002, "Tempo deve ser um número")),
            };

            thread::sleep(std::time::Duration::from_millis(ms));
            Ok(Value::Null)
        });

        self.functions.insert("native_uptime".to_string(), |_args| {
            // Para simplicidade, vamos simular o uptime
            // Em uma implementação real, poderíamos usar uma referência global ao tempo de início
            Ok(Value::Number(0.0))
        });

        self.functions.insert("current_timestamp".to_string(), |_args| {
            match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(duration) => Ok(Value::Number(duration.as_secs_f64())),
                Err(_) => Err(DryadError::new(5008, "Erro ao obter timestamp atual")),
            }
        });
    }

    // === MÓDULO: System Environment ===
    fn register_system_env_functions(&mut self) {
        self.functions.insert("native_platform".to_string(), |_args| {
            let platform = if cfg!(target_os = "windows") {
                "windows"
            } else if cfg!(target_os = "macos") {
                "macos"
            } else if cfg!(target_os = "linux") {
                "linux"
            } else {
                "unknown"
            };
            Ok(Value::String(platform.to_string()))
        });

        self.functions.insert("native_arch".to_string(), |_args| {
            let arch = if cfg!(target_arch = "x86_64") {
                "x86_64"
            } else if cfg!(target_arch = "aarch64") {
                "aarch64"
            } else if cfg!(target_arch = "x86") {
                "x86"
            } else {
                "unknown"
            };
            Ok(Value::String(arch.to_string()))
        });

        self.functions.insert("native_env".to_string(), |args| {
            if args.len() != 1 {
                return Err(DryadError::new(3004, "native_env espera 1 argumento (key)"));
            }
            
            let key = match &args[0] {
                Value::String(s) => s,
                _ => return Err(DryadError::new(3002, "Chave deve ser uma string")),
            };

            match env::var(key) {
                Ok(value) => Ok(Value::String(value)),
                Err(_) => Ok(Value::Null),
            }
        });

        self.functions.insert("native_set_env".to_string(), |args| {
            if args.len() != 2 {
                return Err(DryadError::new(3004, "native_set_env espera 2 argumentos (key, value)"));
            }
            
            let key = match &args[0] {
                Value::String(s) => s,
                _ => return Err(DryadError::new(3002, "Chave deve ser uma string")),
            };

            let value = args[1].to_string();
            env::set_var(key, value);
            Ok(Value::Bool(true))
        });

        self.functions.insert("native_exec".to_string(), |args| {
            if args.len() != 1 {
                return Err(DryadError::new(3004, "native_exec espera 1 argumento (cmd)"));
            }
            
            let cmd = match &args[0] {
                Value::String(s) => s,
                _ => return Err(DryadError::new(3002, "Comando deve ser uma string")),
            };

            #[cfg(target_os = "windows")]
            let output = Command::new("cmd").args(&["/C", cmd]).output();
            
            #[cfg(not(target_os = "windows"))]
            let output = Command::new("sh").args(&["-c", cmd]).output();

            match output {
                Ok(result) => Ok(Value::Number(result.status.code().unwrap_or(-1) as f64)),
                Err(_) => Err(DryadError::new(5009, &format!("Erro ao executar comando: {}", cmd))),
            }
        });

        self.functions.insert("native_exec_output".to_string(), |args| {
            if args.len() != 1 {
                return Err(DryadError::new(3004, "native_exec_output espera 1 argumento (cmd)"));
            }
            
            let cmd = match &args[0] {
                Value::String(s) => s,
                _ => return Err(DryadError::new(3002, "Comando deve ser uma string")),
            };

            #[cfg(target_os = "windows")]
            let output = Command::new("cmd").args(&["/C", cmd]).output();
            
            #[cfg(not(target_os = "windows"))]
            let output = Command::new("sh").args(&["-c", cmd]).output();

            match output {
                Ok(result) => Ok(Value::String(String::from_utf8_lossy(&result.stdout).to_string())),
                Err(_) => Err(DryadError::new(5009, &format!("Erro ao executar comando: {}", cmd))),
            }
        });

        self.functions.insert("native_pid".to_string(), |_args| {
            Ok(Value::Number(std::process::id() as f64))
        });

        self.functions.insert("native_exit".to_string(), |args| {
            let code = if args.is_empty() {
                0
            } else {
                match &args[0] {
                    Value::Number(n) => *n as i32,
                    _ => 0,
                }
            };
            std::process::exit(code);
        });

        self.functions.insert("get_current_dir".to_string(), |_args| {
            match env::current_dir() {
                Ok(path) => Ok(Value::String(path.to_string_lossy().to_string())),
                Err(_) => Err(DryadError::new(5010, "Erro ao obter diretório atual")),
            }
        });

        self.functions.insert("native_current_dir".to_string(), |_args| {
            match env::current_dir() {
                Ok(path) => Ok(Value::String(path.to_string_lossy().to_string())),
                Err(_) => Err(DryadError::new(5010, "Erro ao obter diretório atual")),
            }
        });
    }

    // === MÓDULO: Debug ===
    fn register_debug_functions(&mut self) {
        self.functions.insert("debug".to_string(), |args| {
            if args.is_empty() {
                println!("[DEBUG]");
            } else {
                println!("[DEBUG] {:?}", args[0]);
            }
            Ok(Value::Null)
        });

        self.functions.insert("native_log".to_string(), |args| {
            if args.is_empty() {
                println!("[DEBUG]");
            } else {
                println!("[DEBUG] {:?}", args[0]);
            }
            Ok(Value::Null)
        });

        self.functions.insert("native_typeof".to_string(), |args| {
            if args.is_empty() {
                return Ok(Value::String("undefined".to_string()));
            }
            
            let type_name = match &args[0] {
                Value::Number(_) => "number",
                Value::String(_) => "string", 
                Value::Bool(_) => "boolean",
                Value::Null => "null",
                Value::Array(_) => "array",
                Value::Tuple(_) => "tuple",
                Value::Function { .. } => "function",
                Value::Exception(_) => "exception",
                Value::Lambda { .. } => "lambda",
                Value::Class { .. } => "class",
                Value::Instance { .. } => "instance",
                Value::Object { .. } => "object",
            };
            
            Ok(Value::String(type_name.to_string()))
        });

        self.functions.insert("native_memory_usage".to_string(), |_args| {
            // Simulação simples - em uma implementação real usaríamos bibliotecas de sistema
            Ok(Value::Number(0.0))
        });
    }

    // === MÓDULO: Crypto (básico) ===
    fn register_crypto_functions(&mut self) {
        self.functions.insert("native_uuid".to_string(), |_args| {
            // Implementação simples de UUID v4
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
            use std::time::SystemTime;
            
            let mut hasher = DefaultHasher::new();
            SystemTime::now().hash(&mut hasher);
            std::thread::current().id().hash(&mut hasher);
            let hash = hasher.finish();
            
            Ok(Value::String(format!("{:x}-{:x}-{:x}-{:x}", 
                hash & 0xFFFF, 
                (hash >> 16) & 0xFFFF, 
                (hash >> 32) & 0xFFFF, 
                (hash >> 48) & 0xFFFF
            )))
        });

        self.functions.insert("sha256".to_string(), |args| {
            if args.len() != 1 {
                return Err(DryadError::new(3004, "sha256 espera 1 argumento (data)"));
            }
            
            let data = args[0].to_string();
            
            // Implementação simples usando hash padrão
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
            
            let mut hasher = DefaultHasher::new();
            data.hash(&mut hasher);
            let hash = hasher.finish();
            
            Ok(Value::String(format!("{:016x}", hash)))
        });

        // Outras funções de crypto serão implementadas com bibliotecas apropriadas
    }

    // === MÓDULO: Binary I/O ===
    fn register_binary_io_functions(&mut self) {
        self.functions.insert("native_read_bytes".to_string(), |args| {
            if args.len() != 1 {
                return Err(DryadError::new(3004, "native_read_bytes espera 1 argumento (path)"));
            }
            
            let path = match &args[0] {
                Value::String(s) => s,
                _ => return Err(DryadError::new(3002, "Caminho deve ser uma string")),
            };

            match fs::read(path) {
                Ok(bytes) => {
                    // Converter bytes para array de números
                    let values: Vec<Value> = bytes.into_iter().map(|b| Value::Number(b as f64)).collect();
                    Ok(Value::Array(values))
                }
                Err(_) => Err(DryadError::new(5003, &format!("Erro ao ler bytes do arquivo: {}", path))),
            }
        });

        self.functions.insert("native_write_bytes".to_string(), |args| {
            if args.len() != 2 {
                return Err(DryadError::new(3004, "native_write_bytes espera 2 argumentos (path, bytes)"));
            }
            
            let path = match &args[0] {
                Value::String(s) => s,
                _ => return Err(DryadError::new(3002, "Caminho deve ser uma string")),
            };

            let bytes = match &args[1] {
                Value::Array(arr) => {
                    let mut byte_vec = Vec::new();
                    for val in arr {
                        match val {
                            Value::Number(n) => byte_vec.push(*n as u8),
                            _ => return Err(DryadError::new(3002, "Array deve conter apenas números")),
                        }
                    }
                    byte_vec
                }
                _ => return Err(DryadError::new(3002, "Segundo argumento deve ser um array")),
            };

            match fs::write(path, bytes) {
                Ok(_) => Ok(Value::Bool(true)),
                Err(_) => Err(DryadError::new(5004, &format!("Erro ao escrever bytes no arquivo: {}", path))),
            }
        });

        self.functions.insert("native_file_size".to_string(), |args| {
            if args.len() != 1 {
                return Err(DryadError::new(3004, "native_file_size espera 1 argumento (path)"));
            }
            
            let path = match &args[0] {
                Value::String(s) => s,
                _ => return Err(DryadError::new(3002, "Caminho deve ser uma string")),
            };

            match fs::metadata(path) {
                Ok(metadata) => Ok(Value::Number(metadata.len() as f64)),
                Err(_) => Err(DryadError::new(5003, &format!("Erro ao obter tamanho do arquivo: {}", path))),
            }
        });

        self.functions.insert("to_hex".to_string(), |args| {
            if args.len() != 1 {
                return Err(DryadError::new(3004, "to_hex espera 1 argumento (number)"));
            }
            
            let num = match &args[0] {
                Value::Number(n) => *n as u64,
                _ => return Err(DryadError::new(3002, "Argumento deve ser um número")),
            };

            Ok(Value::String(format!("{:x}", num)))
        });

        self.functions.insert("from_hex".to_string(), |args| {
            if args.len() != 1 {
                return Err(DryadError::new(3004, "from_hex espera 1 argumento (hex_string)"));
            }
            
            let hex = match &args[0] {
                Value::String(s) => s,
                _ => return Err(DryadError::new(3002, "Argumento deve ser uma string")),
            };

            match u64::from_str_radix(hex, 16) {
                Ok(num) => Ok(Value::Number(num as f64)),
                Err(_) => Err(DryadError::new(3002, "String hexadecimal inválida")),
            }
        });
    }
}

impl PartialEq for NativeModule {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}
