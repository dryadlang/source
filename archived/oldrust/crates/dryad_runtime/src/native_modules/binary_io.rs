use crate::interpreter::Value;
use crate::native_modules::NativeFunction;
use crate::errors::RuntimeError;
use crate::heap::{Heap, ManagedObject};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom};

pub fn register_binary_io_functions(functions: &mut HashMap<String, NativeFunction>) {
    functions.insert("native_write_bytes".to_string(), native_write_bytes);
    functions.insert("native_append_bytes".to_string(), native_append_bytes);
    functions.insert("native_overwrite_chunk".to_string(), native_overwrite_chunk);
    functions.insert("native_read_bytes".to_string(), native_read_bytes);
    functions.insert("native_read_chunk".to_string(), native_read_chunk);
    functions.insert("native_file_size".to_string(), native_file_size);
    functions.insert("to_hex".to_string(), native_to_hex);
}

/// Escreve um array de bytes em um arquivo
/// Entrada: path (string), bytes (array)
/// Retorna: null
fn native_write_bytes(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::ArgumentError(
            "native_write_bytes espera 2 argumentos: path, bytes".to_string()
        ));
    }
    
    let path = match &args[0] {
        Value::String(s) => s,
        _ => return Err(RuntimeError::TypeError(
            "Primeiro argumento deve ser uma string (caminho do arquivo)".to_string()
        ))
    };
    
    let bytes = extract_bytes_from_value(&args[1], _heap)?;
    
    match std::fs::write(path, bytes) {
        Ok(_) => Ok(Value::Null),
        Err(e) => Err(RuntimeError::IoError(
            format!("Erro ao escrever arquivo '{}': {}", path, e)
        ))
    }
}

/// To hex
/// Converte um array de números (bytes) para uma string hexadecimal
fn native_to_hex(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("native_to_hex espera 1 argumento".to_string()));
    }
    let bytes = extract_bytes_from_value(&args[0], _heap)?;
    let hex_string: String = bytes.iter().map(|b| format!("{:02x}", b)).collect();
    Ok(Value::String(hex_string))
}

/// Adiciona bytes ao final de um arquivo existente
/// Entrada: path (string), bytes (array)
/// Retorna: null
fn native_append_bytes(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::ArgumentError(
            "native_append_bytes espera 2 argumentos: path, bytes".to_string()
        ));
    }
    
    let path = match &args[0] {
        Value::String(s) => s,
        _ => return Err(RuntimeError::TypeError(
            "Primeiro argumento deve ser uma string (caminho do arquivo)".to_string()
        ))
    };
    
    let bytes = extract_bytes_from_value(&args[1], _heap)?;
    
    let mut file = match OpenOptions::new().create(true).append(true).open(path) {
        Ok(f) => f,
        Err(e) => return Err(RuntimeError::IoError(
            format!("Erro ao abrir arquivo '{}' para append: {}", path, e)
        ))
    };
    
    match file.write_all(&bytes) {
        Ok(_) => Ok(Value::Null),
        Err(e) => Err(RuntimeError::IoError(
            format!("Erro ao adicionar bytes ao arquivo '{}': {}", path, e)
        ))
    }
}

/// Sobrescreve uma parte específica de um arquivo com bytes
/// Entrada: path (string), offset (number), bytes (array)
/// Retorna: null
fn native_overwrite_chunk(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.len() != 3 {
        return Err(RuntimeError::ArgumentError(
            "native_overwrite_chunk espera 3 argumentos: path, offset, bytes".to_string()
        ));
    }
    
    let path = match &args[0] {
        Value::String(s) => s,
        _ => return Err(RuntimeError::TypeError(
            "Primeiro argumento deve ser uma string (caminho do arquivo)".to_string()
        ))
    };
    
    let offset = match &args[1] {
        Value::Number(n) => {
            if *n < 0.0 || n.fract() != 0.0 {
                return Err(RuntimeError::ArgumentError(
                    "Offset deve ser um número inteiro não-negativo".to_string()
                ));
            }
            *n as u64
        },
        _ => return Err(RuntimeError::TypeError(
            "Segundo argumento deve ser um número (offset)".to_string()
        ))
    };
    
    let bytes = extract_bytes_from_value(&args[2], _heap)?;
    
    let mut file = match OpenOptions::new().write(true).open(path) {
        Ok(f) => f,
        Err(e) => return Err(RuntimeError::IoError(
            format!("Erro ao abrir arquivo '{}' para escrita: {}", path, e)
        ))
    };
    
    // Move para a posição específica
    if let Err(e) = file.seek(SeekFrom::Start(offset)) {
        return Err(RuntimeError::IoError(
            format!("Erro ao posicionar no offset {} do arquivo '{}': {}", offset, path, e)
        ));
    }
    
    match file.write_all(&bytes) {
        Ok(_) => Ok(Value::Null),
        Err(e) => Err(RuntimeError::IoError(
            format!("Erro ao sobrescrever chunk no arquivo '{}': {}", path, e)
        ))
    }
}

/// Lê o conteúdo de um arquivo como um array de bytes
/// Entrada: path (string)
/// Retorna: array de bytes
fn native_read_bytes(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError(
            "native_read_bytes espera 1 argumento: path".to_string()
        ));
    }
    
    let path = match &args[0] {
        Value::String(s) => s,
        _ => return Err(RuntimeError::TypeError(
            "Argumento deve ser uma string (caminho do arquivo)".to_string()
        ))
    };
    
    match std::fs::read(path) {
        Ok(bytes) => {
            // Converte bytes para array de Values (números)
            let byte_values: Vec<Value> = bytes.into_iter()
                .map(|b| Value::Number(b as f64))
                .collect();
            let id = _heap.allocate(ManagedObject::Array(byte_values));
            Ok(Value::Array(id))
        },
        Err(e) => Err(RuntimeError::IoError(
            format!("Erro ao ler arquivo '{}': {}", path, e)
        ))
    }
}

/// Lê uma parte específica de um arquivo como um array de bytes
/// Entrada: path (string), offset (number), size (number)
/// Retorna: array de bytes
fn native_read_chunk(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.len() != 3 {
        return Err(RuntimeError::ArgumentError(
            "native_read_chunk espera 3 argumentos: path, offset, size".to_string()
        ));
    }
    
    let path = match &args[0] {
        Value::String(s) => s,
        _ => return Err(RuntimeError::TypeError(
            "Primeiro argumento deve ser uma string (caminho do arquivo)".to_string()
        ))
    };
    
    let offset = match &args[1] {
        Value::Number(n) => {
            if *n < 0.0 || n.fract() != 0.0 {
                return Err(RuntimeError::ArgumentError(
                    "Offset deve ser um número inteiro não-negativo".to_string()
                ));
            }
            *n as u64
        },
        _ => return Err(RuntimeError::TypeError(
            "Segundo argumento deve ser um número (offset)".to_string()
        ))
    };
    
    let size = match &args[2] {
        Value::Number(n) => {
            if *n < 0.0 || n.fract() != 0.0 {
                return Err(RuntimeError::ArgumentError(
                    "Size deve ser um número inteiro não-negativo".to_string()
                ));
            }
            *n as usize
        },
        _ => return Err(RuntimeError::TypeError(
            "Terceiro argumento deve ser um número (tamanho)".to_string()
        ))
    };
    
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => return Err(RuntimeError::IoError(
            format!("Erro ao abrir arquivo '{}': {}", path, e)
        ))
    };
    
    // Move para a posição específica
    if let Err(e) = file.seek(SeekFrom::Start(offset)) {
        return Err(RuntimeError::IoError(
            format!("Erro ao posicionar no offset {} do arquivo '{}': {}", offset, path, e)
        ));
    }
    
    // Lê o número específico de bytes
    let mut buffer = vec![0u8; size];
    let bytes_read = match file.read(&mut buffer) {
        Ok(n) => n,
        Err(e) => return Err(RuntimeError::IoError(
            format!("Erro ao ler chunk do arquivo '{}': {}", path, e)
        ))
    };
    
    // Ajusta o buffer para o número real de bytes lidos
    buffer.truncate(bytes_read);
    
    // Converte bytes para array de Values (números)
    let byte_values: Vec<Value> = buffer.into_iter()
                .map(|b| Value::Number(b as f64))
                .collect();
    let id = _heap.allocate(ManagedObject::Array(byte_values));
    Ok(Value::Array(id))
}

/// Retorna o tamanho de um arquivo em bytes
/// Entrada: path (string)
/// Retorna: número inteiro representando o tamanho do arquivo
fn native_file_size(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError(
            "native_file_size espera 1 argumento: path".to_string()
        ));
    }
    
    let path = match &args[0] {
        Value::String(s) => s,
        _ => return Err(RuntimeError::TypeError(
            "Argumento deve ser uma string (caminho do arquivo)".to_string()
        ))
    };
    
    match std::fs::metadata(path) {
        Ok(metadata) => Ok(Value::Number(metadata.len() as f64)),
        Err(e) => Err(RuntimeError::IoError(
            format!("Erro ao obter tamanho do arquivo '{}': {}", path, e)
        ))
    }
}

/// Função auxiliar para extrair bytes de um Value
/// Aceita tanto arrays de números quanto strings
fn extract_bytes_from_value(value: &Value, heap: &Heap) -> Result<Vec<u8>, RuntimeError> {
    match value {
        Value::Array(id) => {
            let obj = heap.get(*id).ok_or_else(|| RuntimeError::HeapError("Array reference not found".to_string()))?;
            if let ManagedObject::Array(arr) = obj {
                arr.iter().map(|v| {
                    if let Value::Number(n) = v {
                        Ok(*n as u8)
                    } else {
                        Err(RuntimeError::TypeError("Array deve conter apenas números".to_string()))
                    }
                }).collect()
            } else {
                Err(RuntimeError::TypeError("Expected array in heap".to_string()))
            }
        },
        Value::String(s) => Ok(s.as_bytes().to_vec()),
        Value::Number(n) => Ok(vec![*n as u8]),
        _ => Err(RuntimeError::TypeError("Bytes devem ser um array de números ou uma string".to_string())),
    }
}
