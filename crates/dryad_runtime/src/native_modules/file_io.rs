use crate::interpreter::RuntimeValue;
use crate::native_modules::{NativeFunction, AsyncNativeFunction};
use crate::errors::RuntimeError;
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::env;
use std::future::Future;
use std::pin::Pin;
use tokio::fs as tfs;
use tokio::io::AsyncWriteExt;

pub fn register_file_io_functions(functions: &mut HashMap<String, NativeFunction>) {
    functions.insert("file_exists".to_string(), native_file_exists);
    functions.insert("native_read_file".to_string(), native_read_file);
    functions.insert("native_write_file".to_string(), native_write_file);
    functions.insert("native_append_file".to_string(), native_append_file);
    functions.insert("native_delete_file".to_string(), native_delete_file);
    functions.insert("native_list_dir".to_string(), native_list_dir);
    functions.insert("native_copy_file".to_string(), native_copy_file);
    functions.insert("native_move_file".to_string(), native_move_file);
    functions.insert("native_file_exists".to_string(), native_file_exists);
    functions.insert("native_is_dir".to_string(), native_is_dir);
    functions.insert("native_mkdir".to_string(), native_mkdir);
    functions.insert("native_getcwd".to_string(), native_getcwd);
    functions.insert("native_setcwd".to_string(), native_setcwd);
    functions.insert("native_get_file_info".to_string(), native_get_file_info);
    functions.insert("native_read_file_content".to_string(), native_read_file_content);
}

pub fn register_file_io_async_functions(functions: &mut HashMap<String, AsyncNativeFunction>) {
    functions.insert("readFile".to_string(), async_read_file);
    functions.insert("writeFile".to_string(), async_write_file);
    functions.insert("appendFile".to_string(), async_append_file);
}

/// Lê o conteúdo de um arquivo como uma string
/// Entrada: path (string)
/// Retorna: string com o conteúdo do arquivo
fn native_read_file(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError(
            "native_read_file espera 1 argumento: path".to_string()
        ));
    }
    
    let path = match &args[0] {
        RuntimeValue::String(s) => s,
        _ => return Err(RuntimeError::TypeError(
            "Argumento deve ser uma string (caminho do arquivo)".to_string()
        ))
    };
    
    match fs::read_to_string(path) {
        Ok(content) => Ok(RuntimeValue::String(content)),
        Err(e) => Err(RuntimeError::IoError(
            format!("Erro ao ler arquivo '{}': {}", path, e)
        ))
    }
}

/// Escreve uma string em um arquivo, sobrescrevendo o conteúdo existente
/// Entrada: path (string), data (string)
/// Retorna: null
fn native_write_file(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::ArgumentError(
            "native_write_file espera 2 argumentos: path, data".to_string()
        ));
    }
    
    let path = match &args[0] {
        RuntimeValue::String(s) => s,
        _ => return Err(RuntimeError::TypeError(
            "Primeiro argumento deve ser uma string (caminho do arquivo)".to_string()
        ))
    };
    
    let data = match &args[1] {
        RuntimeValue::String(s) => s,
        RuntimeValue::Number(n) => &n.to_string(),
        RuntimeValue::Bool(b) => if *b { "true" } else { "false" },
        RuntimeValue::Null => "null",
        _ => return Err(RuntimeError::TypeError(
            "Segundo argumento deve ser uma string ou valor convertível".to_string()
        ))
    };
    
    match fs::write(path, data) {
        Ok(_) => Ok(RuntimeValue::Null),
        Err(e) => Err(RuntimeError::IoError(
            format!("Erro ao escrever arquivo '{}': {}", path, e)
        ))
    }
}

/// Adiciona uma string ao final de um arquivo existente
/// Entrada: path (string), data (string)
/// Retorna: null
fn native_append_file(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::ArgumentError(
            "native_append_file espera 2 argumentos: path, data".to_string()
        ));
    }
    
    let path = match &args[0] {
        RuntimeValue::String(s) => s,
        _ => return Err(RuntimeError::TypeError(
            "Primeiro argumento deve ser uma string (caminho do arquivo)".to_string()
        ))
    };
    
    let data = match &args[1] {
        RuntimeValue::String(s) => s,
        RuntimeValue::Number(n) => &n.to_string(),
        RuntimeValue::Bool(b) => if *b { "true" } else { "false" },
        RuntimeValue::Null => "null",
        _ => return Err(RuntimeError::TypeError(
            "Segundo argumento deve ser uma string ou valor convertível".to_string()
        ))
    };
    
    let mut file = match OpenOptions::new().create(true).append(true).open(path) {
        Ok(f) => f,
        Err(e) => return Err(RuntimeError::IoError(
            format!("Erro ao abrir arquivo '{}' para append: {}", path, e)
        ))
    };
    
    match file.write_all(data.as_bytes()) {
        Ok(_) => Ok(RuntimeValue::Null),
        Err(e) => Err(RuntimeError::IoError(
            format!("Erro ao adicionar conteúdo ao arquivo '{}': {}", path, e)
        ))
    }
}

/// Deleta um arquivo do sistema
/// Entrada: path (string)
/// Retorna: null
fn native_delete_file(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError(
            "native_delete_file espera 1 argumento: path".to_string()
        ));
    }
    
    let path = match &args[0] {
        RuntimeValue::String(s) => s,
        _ => return Err(RuntimeError::TypeError(
            "Argumento deve ser uma string (caminho do arquivo)".to_string()
        ))
    };
    
    match fs::remove_file(path) {
        Ok(_) => Ok(RuntimeValue::Null),
        Err(e) => Err(RuntimeError::IoError(
            format!("Erro ao deletar arquivo '{}': {}", path, e)
        ))
    }
}

/// Lista os arquivos e pastas em um diretório
/// Entrada: path (string)
/// Retorna: array de strings com os nomes dos arquivos e pastas
fn native_list_dir(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError(
            "native_list_dir espera 1 argumento: path".to_string()
        ));
    }
    
    let path = match &args[0] {
        RuntimeValue::String(s) => s,
        _ => return Err(RuntimeError::TypeError(
            "Argumento deve ser uma string (caminho do diretório)".to_string()
        ))
    };
    
    match fs::read_dir(path) {
        Ok(entries) => {
            let mut files = Vec::new();
            for entry in entries {
                match entry {
                    Ok(dir_entry) => {
                        if let Some(name) = dir_entry.file_name().to_str() {
                            files.push(RuntimeValue::String(name.to_string()));
                        }
                    },
                    Err(_) => continue, // Ignora entradas com erro
                }
            }
            Ok(RuntimeValue::Array(files))
        },
        Err(e) => Err(RuntimeError::IoError(
            format!("Erro ao listar diretório '{}': {}", path, e)
        ))
    }
}

/// Copia um arquivo de um local para outro
/// Entrada: from (string), to (string)
/// Retorna: null
fn native_copy_file(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::ArgumentError(
            "native_copy_file espera 2 argumentos: from, to".to_string()
        ));
    }
    
    let from = match &args[0] {
        RuntimeValue::String(s) => s,
        _ => return Err(RuntimeError::TypeError(
            "Primeiro argumento deve ser uma string (arquivo de origem)".to_string()
        ))
    };
    
    let to = match &args[1] {
        RuntimeValue::String(s) => s,
        _ => return Err(RuntimeError::TypeError(
            "Segundo argumento deve ser uma string (arquivo de destino)".to_string()
        ))
    };
    
    match fs::copy(from, to) {
        Ok(_) => Ok(RuntimeValue::Null),
        Err(e) => Err(RuntimeError::IoError(
            format!("Erro ao copiar arquivo de '{}' para '{}': {}", from, to, e)
        ))
    }
}

/// Move um arquivo de um local para outro
/// Entrada: from (string), to (string)
/// Retorna: null
fn native_move_file(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::ArgumentError(
            "native_move_file espera 2 argumentos: from, to".to_string()
        ));
    }
    
    let from = match &args[0] {
        RuntimeValue::String(s) => s,
        _ => return Err(RuntimeError::TypeError(
            "Primeiro argumento deve ser uma string (arquivo de origem)".to_string()
        ))
    };
    
    let to = match &args[1] {
        RuntimeValue::String(s) => s,
        _ => return Err(RuntimeError::TypeError(
            "Segundo argumento deve ser uma string (arquivo de destino)".to_string()
        ))
    };
    
    match fs::rename(from, to) {
        Ok(_) => Ok(RuntimeValue::Null),
        Err(e) => Err(RuntimeError::IoError(
            format!("Erro ao mover arquivo de '{}' para '{}': {}", from, to, e)
        ))
    }
}

/// Verifica se um arquivo existe
/// Entrada: path (string)
/// Retorna: booleano (true se o arquivo existir, false caso contrário)
fn native_file_exists(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError(
            "native_file_exists espera 1 argumento: path".to_string()
        ));
    }
    
    let path = match &args[0] {
        RuntimeValue::String(s) => s,
        _ => return Err(RuntimeError::TypeError(
            "Argumento deve ser uma string (caminho do arquivo)".to_string()
        ))
    };
    
    Ok(RuntimeValue::Bool(Path::new(path).exists()))
}

/// Verifica se um caminho é um diretório
/// Entrada: path (string)
/// Retorna: booleano (true se for um diretório, false caso contrário)
fn native_is_dir(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError(
            "native_is_dir espera 1 argumento: path".to_string()
        ));
    }
    
    let path = match &args[0] {
        RuntimeValue::String(s) => s,
        _ => return Err(RuntimeError::TypeError(
            "Argumento deve ser uma string (caminho)".to_string()
        ))
    };
    
    Ok(RuntimeValue::Bool(Path::new(path).is_dir()))
}

/// Cria um diretório
/// Entrada: path (string)
/// Retorna: null
fn native_mkdir(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError(
            "native_mkdir espera 1 argumento: path".to_string()
        ));
    }
    
    let path = match &args[0] {
        RuntimeValue::String(s) => s,
        _ => return Err(RuntimeError::TypeError(
            "Argumento deve ser uma string (caminho do diretório)".to_string()
        ))
    };
    
    match fs::create_dir_all(path) {
        Ok(_) => Ok(RuntimeValue::Null),
        Err(e) => Err(RuntimeError::IoError(
            format!("Erro ao criar diretório '{}': {}", path, e)
        ))
    }
}

/// Retorna o diretório de trabalho atual como uma string
/// Entrada: nenhum
/// Retorna: string com o caminho do diretório atual
fn native_getcwd(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    if !args.is_empty() {
        return Err(RuntimeError::ArgumentError(
            "native_getcwd não espera argumentos".to_string()
        ));
    }
    
    match env::current_dir() {
        Ok(path) => {
            if let Some(path_str) = path.to_str() {
                Ok(RuntimeValue::String(path_str.to_string()))
            } else {
                Err(RuntimeError::IoError(
                    "Não foi possível converter o caminho atual para string".to_string()
                ))
            }
        },
        Err(e) => Err(RuntimeError::IoError(
            format!("Erro ao obter diretório atual: {}", e)
        ))
    }
}

/// Muda o diretório de trabalho atual para o especificado
/// Entrada: path (string)
/// Retorna: null
fn native_setcwd(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError(
            "native_setcwd espera 1 argumento: path".to_string()
        ));
    }
    
    let path = match &args[0] {
        RuntimeValue::String(s) => s,
        _ => return Err(RuntimeError::TypeError(
            "Argumento deve ser uma string (caminho do diretório)".to_string()
        ))
    };
    
    match env::set_current_dir(path) {
        Ok(_) => Ok(RuntimeValue::Null),
        Err(e) => Err(RuntimeError::IoError(
            format!("Erro ao mudar para diretório '{}': {}", path, e)
        ))
    }
}

/// Retorna informações sobre um arquivo, como tamanho, data de modificação, etc.
/// Entrada: path (string)
/// Retorna: um objeto com as informações do arquivo
fn native_get_file_info(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError(
            "native_get_file_info espera 1 argumento: path".to_string()
        ));
    }
    
    let path = match &args[0] {
        RuntimeValue::String(s) => s,
        _ => return Err(RuntimeError::TypeError(
            "Argumento deve ser uma string (caminho do arquivo)".to_string()
        ))
    };
    
    match fs::metadata(path) {
        Ok(metadata) => {
            // Cria um array com as informações do arquivo
            // [tamanho, é_diretório, é_arquivo, é_somente_leitura]
            let mut info = Vec::new();
            info.push(RuntimeValue::Number(metadata.len() as f64));
            info.push(RuntimeValue::Bool(metadata.is_dir()));
            info.push(RuntimeValue::Bool(metadata.is_file()));
            info.push(RuntimeValue::Bool(metadata.permissions().readonly()));
            
            Ok(RuntimeValue::Array(info))
        },
        Err(e) => Err(RuntimeError::IoError(
            format!("Erro ao obter informações do arquivo '{}': {}", path, e)
        ))
    }
}

/// Lê o conteúdo de um arquivo como uma string, sem quebra de linha
/// Entrada: path (string)
/// Retorna: string com o conteúdo do arquivo (uma única linha)
fn native_read_file_content(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError(
            "native_read_file_content espera 1 argumento: path".to_string()
        ));
    }
    
    let path = match &args[0] {
        RuntimeValue::String(s) => s,
        _ => return Err(RuntimeError::TypeError(
            "Argumento deve ser uma string (caminho do arquivo)".to_string()
        ))
    };
    
    match fs::read_to_string(path) {
        Ok(content) => {
            // Remove quebras de linha e espaços no início/fim
            let single_line = content.trim().replace('\n', " ").replace('\r', "");
            Ok(RuntimeValue::String(single_line))
        },
        Err(e) => Err(RuntimeError::IoError(
            format!("Erro ao ler conteúdo do arquivo '{}': {}", path, e)
        ))
    }
}

/// Versões assíncronas
fn async_read_file(args: Vec<RuntimeValue>, _manager: &crate::native_modules::NativeModuleManager) -> Pin<Box<dyn Future<Output = Result<RuntimeValue, RuntimeError>> + Send>> {
    Box::pin(async move {
        if args.len() != 1 {
            return Err(RuntimeError::ArgumentError("readFile espera 1 argumento: path".to_string()));
        }
        
        let path = match &args[0] {
            RuntimeValue::String(s) => s.clone(),
            _ => return Err(RuntimeError::TypeError("Argumento deve ser uma string".to_string()))
        };
        
        match tfs::read_to_string(&path).await {
            Ok(content) => Ok(RuntimeValue::String(content)),
            Err(e) => Err(RuntimeError::IoError(format!("Erro ao ler arquivo '{}': {}", path, e)))
        }
    })
}

fn async_write_file(args: Vec<RuntimeValue>, _manager: &crate::native_modules::NativeModuleManager) -> Pin<Box<dyn Future<Output = Result<RuntimeValue, RuntimeError>> + Send>> {
    Box::pin(async move {
        if args.len() != 2 {
            return Err(RuntimeError::ArgumentError("writeFile espera 2 argumentos: path, data".to_string()));
        }
        
        let path = match &args[0] {
            RuntimeValue::String(s) => s.clone(),
            _ => return Err(RuntimeError::TypeError("Primeiro argumento deve ser uma string".to_string()))
        };
        
        let data = args[1].to_string(); // Usa o método to_string() do Value
        
        match tfs::write(&path, data).await {
            Ok(_) => Ok(RuntimeValue::Null),
            Err(e) => Err(RuntimeError::IoError(format!("Erro ao escrever arquivo '{}': {}", path, e)))
        }
    })
}

fn async_append_file(args: Vec<RuntimeValue>, _manager: &crate::native_modules::NativeModuleManager) -> Pin<Box<dyn Future<Output = Result<RuntimeValue, RuntimeError>> + Send>> {
    Box::pin(async move {
        if args.len() != 2 {
            return Err(RuntimeError::ArgumentError("appendFile espera 2 argumentos: path, data".to_string()));
        }
        
        let path = match &args[0] {
            RuntimeValue::String(s) => s.clone(),
            _ => return Err(RuntimeError::TypeError("Primeiro argumento deve ser uma string".to_string()))
        };
        
        let data = args[1].to_string();
        
        let mut file = match tfs::OpenOptions::new().create(true).append(true).open(&path).await {
            Ok(f) => f,
            Err(e) => return Err(RuntimeError::IoError(format!("Erro ao abrir arquivo '{}': {}", path, e)))
        };
        
        match file.write_all(data.as_bytes()).await {
            Ok(_) => Ok(RuntimeValue::Null),
            Err(e) => Err(RuntimeError::IoError(format!("Erro ao adicionar ao arquivo '{}': {}", path, e)))
        }
    })
}
