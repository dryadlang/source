use crate::interpreter::Value;
use crate::native_modules::NativeFunction;
use crate::errors::RuntimeError;
use std::io::{self, Write, Read, stdout};
use std::time::Duration;
use std::thread;
use std::sync::mpsc;

/// Módulo de funções nativas para Console/Terminal I/O
/// Categoria: #<console_io>
/// 
/// Funções disponíveis:
/// - native_input(): Lê linha do stdin (bloqueante)
/// - native_input_char(): Lê 1 caractere sem Enter
/// - native_input_bytes(count): Lê N bytes do console
/// - native_input_timeout(ms): Lê com timeout em milissegundos
/// - native_print(data): Imprime sem quebra de linha
/// - native_println(data): Imprime com quebra de linha
/// - native_write_stdout(bytes): Escrita binária direta
/// - native_flush(): Força flush do stdout

pub fn register_console_io_functions(functions: &mut std::collections::HashMap<String, NativeFunction>) {
    functions.insert("native_input".to_string(), native_input);
    functions.insert("native_input_char".to_string(), native_input_char);
    functions.insert("native_input_bytes".to_string(), native_input_bytes);
    functions.insert("native_input_timeout".to_string(), native_input_timeout);
    functions.insert("native_print".to_string(), native_print);
    functions.insert("native_println".to_string(), native_println);
    functions.insert("native_write_stdout".to_string(), native_write_stdout);
    functions.insert("native_flush".to_string(), native_flush);
    
    // Adicionar aliases convenientes
    functions.insert("print".to_string(), native_println);  // print com quebra de linha
    functions.insert("println".to_string(), native_println);
    functions.insert("input".to_string(), native_input);
}

/// native_input() - Lê linha do stdin (bloqueante)
/// Retorna: string
fn native_input(_args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    let stdin = io::stdin();
    let mut line = String::new();
    
    match stdin.read_line(&mut line) {
        Ok(_) => {
            // Remove quebra de linha do final
            if line.ends_with('\n') {
                line.pop();
                if line.ends_with('\r') {
                    line.pop();
                }
            }
            Ok(Value::String(line))
        }
        Err(e) => Err(RuntimeError::IoError(format!("Erro ao ler entrada: {}", e)))
    }
}

/// native_input_char() - Lê 1 caractere sem esperar Enter
/// Retorna: string (um caractere)
fn native_input_char(_args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    // Implementação simplificada que lê uma linha e pega o primeiro caractere
    // Uma implementação mais avançada usaria bibliotecas específicas do sistema
    let stdin = io::stdin();
    let mut line = String::new();
    
    match stdin.read_line(&mut line) {
        Ok(_) => {
            if let Some(first_char) = line.chars().next() {
                if first_char != '\n' && first_char != '\r' {
                    Ok(Value::String(first_char.to_string()))
                } else {
                    Ok(Value::String(" ".to_string())) // Espaço para Enter
                }
            } else {
                Ok(Value::String("".to_string()))
            }
        }
        Err(e) => Err(RuntimeError::IoError(format!("Erro ao ler caractere: {}", e)))
    }
}

/// native_input_bytes(count) - Lê N bytes do console
/// Args: count (número de bytes)
/// Retorna: array de bytes (como string por enquanto)
fn native_input_bytes(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("native_input_bytes() espera 1 argumento (count)".to_string()));
    }
    
    let count = match &args[0] {
        Value::Number(n) => *n as usize,
        _ => return Err(RuntimeError::ArgumentError("Argumento deve ser um número".to_string()))
    };
    
    let mut buffer = vec![0u8; count];
    match io::stdin().read_exact(&mut buffer) {
        Ok(_) => {
            // Por enquanto, retornamos como string. Futuramente, podemos implementar arrays de bytes
            let result = String::from_utf8_lossy(&buffer).to_string();
            Ok(Value::String(result))
        }
        Err(e) => Err(RuntimeError::IoError(format!("Erro ao ler {} bytes: {}", count, e)))
    }
}

/// native_input_timeout(ms) - Lê entrada com timeout
/// Args: ms (timeout em milissegundos)
/// Retorna: string ou null se timeout
fn native_input_timeout(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("native_input_timeout() espera 1 argumento (ms)".to_string()));
    }
    
    let timeout_ms = match &args[0] {
        Value::Number(n) => *n as u64,
        _ => return Err(RuntimeError::ArgumentError("Timeout deve ser um número".to_string()))
    };
    
    let (sender, receiver) = mpsc::channel();
    
    // Thread para leitura
    thread::spawn(move || {
        let stdin = io::stdin();
        let mut line = String::new();
        match stdin.read_line(&mut line) {
            Ok(_) => {
                if line.ends_with('\n') {
                    line.pop();
                    if line.ends_with('\r') {
                        line.pop();
                    }
                }
                let _ = sender.send(Some(line));
            }
            Err(_) => {
                let _ = sender.send(None);
            }
        }
    });
    
    // Aguarda com timeout
    match receiver.recv_timeout(Duration::from_millis(timeout_ms)) {
        Ok(Some(line)) => Ok(Value::String(line)),
        Ok(None) => Err(RuntimeError::IoError("Erro ao ler entrada".to_string())),
        Err(_) => Ok(Value::Null) // Timeout
    }
}

/// native_print(data) - Imprime dados sem quebra de linha
/// Args: data (qualquer tipo)
fn native_print(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("native_print() espera 1 argumento".to_string()));
    }
    
    let text = match &args[0] {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => "null".to_string(),
        _ => format!("{:?}", args[0])
    };
    
    print!("{}", text);
    let _ = stdout().flush(); // Força flush automático
    
    Ok(Value::Null)
}

/// native_println(data) - Imprime dados com quebra de linha
/// Args: data (qualquer tipo)
fn native_println(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("native_println() espera 1 argumento".to_string()));
    }
    
    let text = match &args[0] {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => "null".to_string(),
        _ => format!("{:?}", args[0])
    };
    
    println!("{}", text);
    
    Ok(Value::Null)
}

/// native_write_stdout(bytes) - Escrita binária direta no stdout
/// Args: bytes (string que representa bytes)
fn native_write_stdout(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("native_write_stdout() espera 1 argumento".to_string()));
    }
    
    let bytes = match &args[0] {
        Value::String(s) => s.as_bytes(),
        _ => return Err(RuntimeError::ArgumentError("Argumento deve ser string".to_string()))
    };
    
    match stdout().write_all(bytes) {
        Ok(_) => Ok(Value::Null),
        Err(e) => Err(RuntimeError::IoError(format!("Erro ao escrever no stdout: {}", e)))
    }
}

/// native_flush() - Força flush do stdout
fn native_flush(_args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    match stdout().flush() {
        Ok(_) => Ok(Value::Null),
        Err(e) => Err(RuntimeError::IoError(format!("Erro ao fazer flush: {}", e)))
    }
}
