use crate::interpreter::Value;
use crate::errors::RuntimeError;
use std::path::{Path, PathBuf};
use tokio::fs as tfs;
use tokio::io::AsyncWriteExt;
use std::future::Future;
use std::pin::Pin;

/// Verifica se um caminho de arquivo é seguro (dentro do sandbox)
/// 
/// Esta função implementa sandboxing de filesystem com as seguintes regras:
/// 1. Caminhos absolutos devem estar dentro do sandbox_root
/// 2. Caminhos relativos são resolvidos contra o sandbox_root
/// 3. Tentativas de path traversal (../) são bloqueadas
/// 4. Symlinks são verificados para não escapar do sandbox
fn is_path_safe(path_str: &str, manager: &crate::native_modules::NativeModuleManager) -> bool {
    // Obter o diretório base do sandbox (diretório atual se não definido)
    let sandbox_root = manager.sandbox_root()
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    
    // Normalizar o caminho de entrada
    let input_path = Path::new(path_str);
    
    // Rejeitar caminhos vazios
    if path_str.is_empty() {
        return false;
    }
    
    // Rejeitar caminhos nulos (caracteres nulos)
    if path_str.contains('\0') {
        return false;
    }
    
    // Resolver o caminho completo contra o sandbox_root
    let resolved_path = if input_path.is_absolute() {
        // Se for absoluto, verificar se está dentro do sandbox
        // Primeiro normalizar para remover . e ..
        match input_path.canonicalize() {
            Ok(canonical) => canonical,
            Err(_) => {
                // Se não conseguir canonicalizar (arquivo não existe), 
                // verificar pelo menos se o caminho começa com o root
                return input_path.starts_with(&sandbox_root);
            }
        }
    } else {
        // Caminho relativo - resolver contra o sandbox_root
        let full_path = sandbox_root.join(input_path);
        match full_path.canonicalize() {
            Ok(canonical) => canonical,
            Err(_) => {
                // Se não existe, verificar se o caminho normalizado está dentro do sandbox
                // Remover componentes . e ..
                let normalized = normalize_path(&full_path);
                return normalized.starts_with(&sandbox_root);
            }
        }
    };
    
    // Verificar se o caminho resolvido está dentro do sandbox_root
    // Usar canonicalize no sandbox_root também para comparação justa
    let canonical_root = match sandbox_root.canonicalize() {
        Ok(root) => root,
        Err(_) => sandbox_root.clone(),
    };
    
    resolved_path.starts_with(&canonical_root)
}

/// Normaliza um caminho removendo . e .. sem acessar o filesystem
fn normalize_path(path: &Path) -> PathBuf {
    let mut components = Vec::new();
    
    for component in path.components() {
        match component {
            std::path::Component::Prefix(_) => {
                // Manter prefixos (Windows)
                components.push(component.as_os_str().to_os_string());
            }
            std::path::Component::RootDir => {
                // Manter root
                components.push(component.as_os_str().to_os_string());
            }
            std::path::Component::CurDir => {
                // Ignorar .
                continue;
            }
            std::path::Component::ParentDir => {
                // Remover último componente se não for root
                if !components.is_empty() && 
                   components.last() != Some(&std::ffi::OsString::from("/")) &&
                   components.last() != Some(&std::ffi::OsString::from("\\")) {
                    components.pop();
                }
            }
            std::path::Component::Normal(name) => {
                components.push(name.to_os_string());
            }
        }
    }
    
    // Reconstruir o caminho
    let mut result = PathBuf::new();
    for (i, component) in components.iter().enumerate() {
        if i == 0 && path.has_root() {
            result.push(component);
        } else if i == 0 {
            result.push(component);
        } else {
            result.push(component);
        }
    }
    
    result
}

pub fn register_file_io_functions(functions: &mut std::collections::HashMap<String, crate::native_modules::NativeFunction>) {
    functions.insert("native_read_file".to_string(), native_read_file);
    functions.insert("native_write_file".to_string(), native_write_file);
    functions.insert("native_append_file".to_string(), native_append_file);
    functions.insert("native_file_exists".to_string(), native_file_exists);
    functions.insert("native_is_dir".to_string(), native_is_dir);
    functions.insert("native_list_dir".to_string(), native_list_dir);
    functions.insert("native_mkdir".to_string(), native_mkdir);
    functions.insert("native_remove_file".to_string(), native_remove_file);
    functions.insert("native_remove_dir".to_string(), native_remove_dir);
    
    // Convenientes aliases
    functions.insert("read_file".to_string(), native_read_file);
    functions.insert("write_file".to_string(), native_write_file);
    functions.insert("file_exists".to_string(), native_file_exists);
    functions.insert("is_dir".to_string(), native_is_dir);
    functions.insert("list_dir".to_string(), native_list_dir);
    functions.insert("mkdir".to_string(), native_mkdir);
    functions.insert("remove_file".to_string(), native_remove_file);
    functions.insert("remove_dir".to_string(), native_remove_dir);
}

pub fn register_file_io_async_functions(functions: &mut std::collections::HashMap<String, crate::native_modules::AsyncNativeFunction>) {
    functions.insert("async_read_file".to_string(), async_read_file);
    functions.insert("async_write_file".to_string(), async_write_file);
    functions.insert("async_append_file".to_string(), async_append_file);
}

// Implementações síncronas
fn native_read_file(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentError("readFile espera pelo menos 1 argumento".to_string()));
    }
    
    let path_str = match &args[0] {
        Value::String(s) => s.as_str(),
        _ => return Err(RuntimeError::TypeError("Argumento de caminho deve ser string".to_string())),
    };
    
    if !is_path_safe(path_str, _manager) {
        return Err(RuntimeError::SystemError(format!("Acesso negado: o caminho '{}' está fora do sandbox permitido.", path_str)));
    }
    
    match std::fs::read_to_string(path_str) {
        Ok(content) => Ok(Value::String(content)),
        Err(e) => Err(RuntimeError::IoError(format!("Erro ao ler arquivo: {}", e))),
    }
}

fn native_write_file(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::ArgumentError("writeFile espera 2 argumentos: path, content".to_string()));
    }
    
    let path_str = match &args[0] {
        Value::String(s) => s.as_str(),
        _ => return Err(RuntimeError::TypeError("Argumento de caminho deve ser string".to_string())),
    };
    
    if !is_path_safe(path_str, _manager) {
        return Err(RuntimeError::SystemError(format!("Acesso negado: o caminho '{}' está fora do sandbox permitido.", path_str)));
    }
    
    let content = args[1].to_string();
    
    match std::fs::write(path_str, content) {
        Ok(_) => Ok(Value::Null),
        Err(e) => Err(RuntimeError::IoError(format!("Erro ao escrever arquivo: {}", e))),
    }
}

fn native_append_file(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::ArgumentError("appendFile espera 2 argumentos: path, content".to_string()));
    }
    
    let path_str = match &args[0] {
        Value::String(s) => s.as_str(),
        _ => return Err(RuntimeError::TypeError("Argumento de caminho deve ser string".to_string())),
    };
    
    if !is_path_safe(path_str, _manager) {
        return Err(RuntimeError::SystemError(format!("Acesso negado: o caminho '{}' está fora do sandbox permitido.", path_str)));
    }
    
    let content = args[1].to_string();
    
    use std::io::Write;
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path_str)
        .map_err(|e| RuntimeError::IoError(format!("Erro ao abrir arquivo: {}", e)))?;
        
    file.write_all(content.as_bytes())
        .map_err(|e| RuntimeError::IoError(format!("Erro ao anexar ao arquivo: {}", e)))?;
        
    Ok(Value::Null)
}

fn native_file_exists(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.is_empty() { return Err(RuntimeError::ArgumentError("fileExists espera 1 argumento".to_string())); }
    let path = match &args[0] { Value::String(s) => s, _ => return Err(RuntimeError::TypeError("Caminho deve ser string".to_string())) };
    if !is_path_safe(path, _manager) { return Ok(Value::Bool(false)); }
    Ok(Value::Bool(Path::new(path).exists()))
}

fn native_is_dir(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.is_empty() { return Err(RuntimeError::ArgumentError("isDir espera 1 argumento".to_string())); }
    let path = match &args[0] { Value::String(s) => s, _ => return Err(RuntimeError::TypeError("Caminho deve ser string".to_string())) };
    if !is_path_safe(path, _manager) { return Ok(Value::Bool(false)); }
    Ok(Value::Bool(Path::new(path).is_dir()))
}

fn native_list_dir(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.is_empty() { return Err(RuntimeError::ArgumentError("listDir espera 1 argumento".to_string())); }
    let path_str = match &args[0] { Value::String(s) => s, _ => return Err(RuntimeError::TypeError("Caminho deve ser string".to_string())) };
    if !is_path_safe(path_str, _manager) { return Err(RuntimeError::SystemError("Acesso negado".to_string())); }
    
    let entries = std::fs::read_dir(path_str).map_err(|e| RuntimeError::IoError(e.to_string()))?;
    let mut file_names = Vec::new();
    for entry in entries {
        if let Ok(e) = entry {
            file_names.push(Value::String(e.file_name().to_string_lossy().to_string()));
        }
    }
    
    let id = _heap.allocate(crate::heap::ManagedObject::Array(file_names));
    Ok(Value::Array(id))
}

fn native_mkdir(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.is_empty() { return Err(RuntimeError::ArgumentError("mkdir espera 1 argumento".to_string())); }
    let path = match &args[0] { Value::String(s) => s, _ => return Err(RuntimeError::TypeError("Caminho deve ser string".to_string())) };
    if !is_path_safe(path, _manager) { return Err(RuntimeError::SystemError("Acesso negado".to_string())); }
    std::fs::create_dir_all(path).map_err(|e| RuntimeError::IoError(e.to_string()))?;
    Ok(Value::Null)
}

fn native_remove_file(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.is_empty() { return Err(RuntimeError::ArgumentError("removeFile espera 1 argumento".to_string())); }
    let path = match &args[0] { Value::String(s) => s, _ => return Err(RuntimeError::TypeError("Caminho deve ser string".to_string())) };
    if !is_path_safe(path, _manager) { return Err(RuntimeError::SystemError("Acesso negado".to_string())); }
    std::fs::remove_file(path).map_err(|e| RuntimeError::IoError(e.to_string()))?;
    Ok(Value::Null)
}

fn native_remove_dir(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.is_empty() { return Err(RuntimeError::ArgumentError("removeDir espera 1 argumento".to_string())); }
    let path = match &args[0] { Value::String(s) => s, _ => return Err(RuntimeError::TypeError("Caminho deve ser string".to_string())) };
    if !is_path_safe(path, _manager) { return Err(RuntimeError::SystemError("Acesso negado".to_string())); }
    
    let recursive = args.len() > 1 && match &args[1] { Value::Bool(b) => *b, _ => false };
    if recursive {
        std::fs::remove_dir_all(path).map_err(|e| RuntimeError::IoError(e.to_string()))?;
    } else {
        std::fs::remove_dir(path).map_err(|e| RuntimeError::IoError(e.to_string()))?;
    }
    Ok(Value::Null)
}

/// Versões assíncronas
fn async_read_file(args: Vec<Value>, _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Pin<Box<dyn Future<Output = Result<Value, RuntimeError>> + Send + 'static>> {
    if args.len() != 1 {
        return Box::pin(async { Err(RuntimeError::ArgumentError("readFile espera 1 argumento: path".to_string())) });
    }
    
    let path = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Box::pin(async { Err(RuntimeError::TypeError("Argumento deve ser uma string".to_string())) })
    };
    
    if !is_path_safe(&path, _manager) {
        return Box::pin(async move {
            Err(RuntimeError::SystemError(
                format!("Acesso negado: o caminho '{}' está fora do sandbox permitido.", path)
            ))
        });
    }
    
    Box::pin(async move {
        match tfs::read_to_string(&path).await {
            Ok(content) => Ok(Value::String(content)),
            Err(e) => Err(RuntimeError::IoError(format!("Erro ao ler arquivo '{}': {}", path, e)))
        }
    })
}

fn async_write_file(args: Vec<Value>, _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Pin<Box<dyn Future<Output = Result<Value, RuntimeError>> + Send + 'static>> {
    if args.len() != 2 {
        return Box::pin(async { Err(RuntimeError::ArgumentError("writeFile espera 2 argumentos: path, data".to_string())) });
    }
    
    let path = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Box::pin(async { Err(RuntimeError::TypeError("Primeiro argumento deve ser uma string".to_string())) })
    };
    
    if !is_path_safe(&path, _manager) {
        return Box::pin(async move {
            Err(RuntimeError::SystemError(
                format!("Acesso negado: o caminho '{}' está fora do sandbox permitido.", path)
            ))
        });
    }
    
    let data = args[1].to_string();
    
    Box::pin(async move {
        match tfs::write(&path, data).await {
            Ok(_) => Ok(Value::Null),
            Err(e) => Err(RuntimeError::IoError(format!("Erro ao escrever arquivo '{}': {}", path, e)))
        }
    })
}

fn async_append_file(args: Vec<Value>, _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Pin<Box<dyn Future<Output = Result<Value, RuntimeError>> + Send + 'static>> {
    if args.len() != 2 {
        return Box::pin(async { Err(RuntimeError::ArgumentError("appendFile espera 2 argumentos: path, data".to_string())) });
    }
    
    let path = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Box::pin(async { Err(RuntimeError::TypeError("Primeiro argumento deve ser uma string".to_string())) })
    };
    
    if !is_path_safe(&path, _manager) {
        return Box::pin(async move {
            Err(RuntimeError::SystemError(
                format!("Acesso negado: o caminho '{}' está fora do sandbox permitido.", path)
            ))
        });
    }
    
    let data = args[1].to_string();
    
    Box::pin(async move {
        let mut file = match tfs::OpenOptions::new().create(true).append(true).open(&path).await {
            Ok(f) => f,
            Err(e) => return Err(RuntimeError::IoError(format!("Erro ao abrir arquivo '{}': {}", path, e)))
        };
        
        match file.write_all(data.as_bytes()).await {
            Ok(_) => Ok(Value::Null),
            Err(e) => Err(RuntimeError::IoError(format!("Erro ao adicionar ao arquivo '{}': {}", path, e)))
        }
    })
}
