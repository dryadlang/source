use crate::interpreter::RuntimeValue;
use crate::native_modules::NativeFunction;
use crate::errors::RuntimeError;
use std::collections::HashMap;
use regex::Regex;
use rand::{RngCore, SeedableRng, Rng};
use rand_chacha::ChaCha20Rng;
use std::sync::{Arc, Mutex};
use std::thread;
use std::path::Path;
use notify::{Watcher, RecursiveMode, recommended_watcher};
use std::sync::mpsc;
use lazy_static::lazy_static;

lazy_static! {
    static ref RNG: Arc<Mutex<ChaCha20Rng>> = Arc::new(Mutex::new(ChaCha20Rng::from_entropy()));
    static ref WATCHER_COUNTER: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));
}

/// Registra todas as funções nativas do módulo utils
pub fn register_utils_functions(functions: &mut HashMap<String, NativeFunction>) {
    functions.insert("native_eval".to_string(), native_eval);
    functions.insert("native_clone".to_string(), native_clone);
    functions.insert("native_watch_file".to_string(), native_watch_file);
    functions.insert("native_random_int".to_string(), native_random_int);
    functions.insert("native_random_float".to_string(), native_random_float);
    functions.insert("native_random_string".to_string(), native_random_string);
    functions.insert("native_random_bytes".to_string(), native_random_bytes);
    functions.insert("native_random_seed".to_string(), native_random_seed);
    functions.insert("native_regex_match".to_string(), native_regex_match);
    functions.insert("native_regex_replace".to_string(), native_regex_replace);
    functions.insert("native_regex_split".to_string(), native_regex_split);
    functions.insert("native_regex_test".to_string(), native_regex_test);
}

// ============================================
// EXECUÇÃO DINÂMICA
// ============================================

/// native_eval(code) -> valor
/// Executa código Dryad dinâmico passado como string
fn native_eval(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("native_eval: esperado 1 argumento (código)".to_string()));
    }
    
    let code = match &args[0] {
        RuntimeValue::String(s) => s,
        _ => return Err(RuntimeError::TypeError("native_eval: argumento deve ser string".to_string())),
    };
    
    // NOTA: Esta é uma implementação simplificada
    // Em uma implementação real, seria necessário ter acesso ao parser e interpretador
    // Por enquanto, simulamos alguns comandos básicos
    
    // Verifica se é uma expressão matemática simples
    if let Ok(result) = evaluate_simple_expression(code) {
        return Ok(RuntimeValue::Number(result));
    }
    
    // Verifica se é uma string literal
    if code.starts_with('"') && code.ends_with('"') && code.len() >= 2 {
        let string_content = &code[1..code.len()-1];
        return Ok(RuntimeValue::String(string_content.to_string()));
    }
    
    // Verifica se é um valor booleano
    match code.trim() {
        "true" => return Ok(RuntimeValue::Bool(true)),
        "false" => return Ok(RuntimeValue::Bool(false)),
        "null" => return Ok(RuntimeValue::Null),
        _ => {}
    }
    
    // Se não conseguir avaliar, retorna erro
    Err(RuntimeError::Generic(format!("native_eval: não foi possível avaliar o código: {}", code)))
}

/// Avalia expressões matemáticas simples
fn evaluate_simple_expression(expr: &str) -> Result<f64, ()> {
    let expr = expr.trim();
    
    // Operações básicas
    if let Some(pos) = expr.rfind(" + ") {
        let left = evaluate_simple_expression(&expr[..pos])?;
        let right = evaluate_simple_expression(&expr[pos + 3..])?;
        return Ok(left + right);
    }
    
    if let Some(pos) = expr.rfind(" - ") {
        let left = evaluate_simple_expression(&expr[..pos])?;
        let right = evaluate_simple_expression(&expr[pos + 3..])?;
        return Ok(left - right);
    }
    
    if let Some(pos) = expr.rfind(" * ") {
        let left = evaluate_simple_expression(&expr[..pos])?;
        let right = evaluate_simple_expression(&expr[pos + 3..])?;
        return Ok(left * right);
    }
    
    if let Some(pos) = expr.rfind(" / ") {
        let left = evaluate_simple_expression(&expr[..pos])?;
        let right = evaluate_simple_expression(&expr[pos + 3..])?;
        if right != 0.0 {
            return Ok(left / right);
        }
    }
    
    // Número simples
    expr.parse::<f64>().map_err(|_| ())
}

// ============================================
// CLONAGEM PROFUNDA
// ============================================

/// native_clone(obj) -> objeto
/// Cria uma cópia profunda de um objeto
fn native_clone(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("native_clone: esperado 1 argumento".to_string()));
    }
    
    Ok(deep_clone(&args[0]))
}

/// Implementa clonagem profunda recursiva
fn deep_clone(value: &RuntimeValue) -> RuntimeValue {
    match value {
        RuntimeValue::String(s) => RuntimeValue::String(s.clone()),
        RuntimeValue::Number(n) => RuntimeValue::Number(*n),
        RuntimeValue::Bool(b) => RuntimeValue::Bool(*b),
        RuntimeValue::Null => RuntimeValue::Null,
        RuntimeValue::Array(arr) => {
            let cloned_array: Vec<RuntimeValue> = arr.iter()
                .map(|item| deep_clone(item))
                .collect();
            RuntimeValue::Array(cloned_array)
        },
        RuntimeValue::Tuple(tuple) => {
            let cloned_tuple: Vec<RuntimeValue> = tuple.iter()
                .map(|item| deep_clone(item))
                .collect();
            RuntimeValue::Tuple(cloned_tuple)
        },
        RuntimeValue::Object { properties, methods } => {
            let mut cloned_properties = HashMap::new();
            for (key, val) in properties {
                cloned_properties.insert(key.clone(), deep_clone(val));
            }
            RuntimeValue::Object {
                properties: cloned_properties,
                methods: methods.clone(), // Métodos são copiados como referência
            }
        },
        RuntimeValue::Instance { class_name, properties } => {
            let mut cloned_properties = HashMap::new();
            for (key, val) in properties {
                cloned_properties.insert(key.clone(), deep_clone(val));
            }
            RuntimeValue::Instance {
                class_name: class_name.clone(),
                properties: cloned_properties,
            }
        },
        // Para outros tipos, faça uma cópia simples
        _ => value.clone(),
    }
}

// ============================================
// OBSERVAÇÃO DE ARQUIVOS
// ============================================

/// native_watch_file(path) -> id
/// Observa mudanças em um arquivo em tempo real
fn native_watch_file(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("native_watch_file: esperado 1 argumento (path)".to_string()));
    }

    let path = match &args[0] {
        RuntimeValue::String(s) => s,
        _ => return Err(RuntimeError::TypeError("native_watch_file: argumento deve ser string".to_string())),
    };

    if !Path::new(path).exists() {
        return Err(RuntimeError::IoError(format!("Arquivo não encontrado: {}", path)));
    }

    // Gera um ID único para o watcher
    let mut counter = WATCHER_COUNTER.lock().unwrap();
    *counter += 1;
    let watcher_id = *counter;
    drop(counter);

    let path_string = path.clone();
    thread::spawn(move || {
        let (tx, rx) = mpsc::channel();
        let mut watcher = match recommended_watcher(tx) {
            Ok(w) => w,
            Err(e) => {
                eprintln!("Erro ao criar watcher: {}", e);
                return;
            }
        };

        if let Err(e) = watcher.watch(Path::new(&path_string), RecursiveMode::NonRecursive) {
            eprintln!("Erro ao observar arquivo: {}", e);
            return;
        }

        while let Ok(event) = rx.recv() {
            println!("Arquivo {} modificado: {:?}", path_string, event);
        }
    });

    Ok(RuntimeValue::Number(watcher_id as f64))
}

// ============================================
// GERAÇÃO DE NÚMEROS ALEATÓRIOS
// ============================================

/// native_random_int(min, max) -> número
/// Gera um número inteiro aleatório entre min e max (inclusive)
fn native_random_int(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::ArgumentError("native_random_int: esperado 2 argumentos (min, max)".to_string()));
    }
    
    let min = match &args[0] {
        RuntimeValue::Number(n) => *n as i64,
        _ => return Err(RuntimeError::TypeError("native_random_int: primeiro argumento deve ser número".to_string())),
    };
    
    let max = match &args[1] {
        RuntimeValue::Number(n) => *n as i64,
        _ => return Err(RuntimeError::TypeError("native_random_int: segundo argumento deve ser número".to_string())),
    };
    
    if min > max {
        return Err(RuntimeError::ArgumentError("native_random_int: min deve ser menor ou igual a max".to_string()));
    }
    
    let mut rng = RNG.lock().unwrap();
    let random_int = rng.gen_range(min..=max);
    
    Ok(RuntimeValue::Number(random_int as f64))
}

/// native_random_float(min, max) -> número
/// Gera um número de ponto flutuante aleatório entre min e max
fn native_random_float(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::ArgumentError("native_random_float: esperado 2 argumentos (min, max)".to_string()));
    }
    
    let min = match &args[0] {
        RuntimeValue::Number(n) => *n,
        _ => return Err(RuntimeError::TypeError("native_random_float: primeiro argumento deve ser número".to_string())),
    };
    
    let max = match &args[1] {
        RuntimeValue::Number(n) => *n,
        _ => return Err(RuntimeError::TypeError("native_random_float: segundo argumento deve ser número".to_string())),
    };
    
    if min > max {
        return Err(RuntimeError::ArgumentError("native_random_float: min deve ser menor ou igual a max".to_string()));
    }
    
    let mut rng = RNG.lock().unwrap();
    let random_float = rng.gen_range(min..=max);
    
    Ok(RuntimeValue::Number(random_float))
}

/// native_random_string(length, charset) -> string
/// Gera uma string aleatória com charset específico
fn native_random_string(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::ArgumentError("native_random_string: esperado 2 argumentos (length, charset)".to_string()));
    }
    
    let length = match &args[0] {
        RuntimeValue::Number(n) => {
            if *n < 0.0 {
                return Err(RuntimeError::ArgumentError("native_random_string: comprimento deve ser não-negativo".to_string()));
            }
            *n as usize
        },
        _ => return Err(RuntimeError::TypeError("native_random_string: primeiro argumento deve ser número".to_string())),
    };
    
    let charset = match &args[1] {
        RuntimeValue::String(s) => s,
        _ => return Err(RuntimeError::TypeError("native_random_string: segundo argumento deve ser string".to_string())),
    };
    
    if charset.is_empty() {
        return Err(RuntimeError::ArgumentError("native_random_string: charset não pode estar vazio".to_string()));
    }
    
    if length > 100000 {
        return Err(RuntimeError::ArgumentError("native_random_string: comprimento máximo é 100000".to_string()));
    }
    
    let charset_chars: Vec<char> = charset.chars().collect();
    let mut rng = RNG.lock().unwrap();
    let mut result = String::new();
    
    for _ in 0..length {
        let idx = rng.gen_range(0..charset_chars.len());
        result.push(charset_chars[idx]);
    }
    
    Ok(RuntimeValue::String(result))
}

/// native_random_bytes(length) -> array
/// Gera um array de bytes aleatórios
fn native_random_bytes(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("native_random_bytes: esperado 1 argumento (length)".to_string()));
    }
    
    let length = match &args[0] {
        RuntimeValue::Number(n) => {
            if *n < 0.0 {
                return Err(RuntimeError::ArgumentError("native_random_bytes: comprimento deve ser não-negativo".to_string()));
            }
            *n as usize
        },
        _ => return Err(RuntimeError::TypeError("native_random_bytes: argumento deve ser número".to_string())),
    };
    
    if length > 100000 {
        return Err(RuntimeError::ArgumentError("native_random_bytes: comprimento máximo é 100000".to_string()));
    }
    
    let mut rng = RNG.lock().unwrap();
    let mut bytes = vec![0u8; length];
    rng.fill_bytes(&mut bytes);
    
    let runtime_bytes: Vec<RuntimeValue> = bytes.into_iter()
        .map(|b| RuntimeValue::Number(b as f64))
        .collect();
    
    Ok(RuntimeValue::Array(runtime_bytes))
}

/// native_random_seed(seed) -> null
/// Define a semente do gerador de números aleatórios
fn native_random_seed(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("native_random_seed: esperado 1 argumento (seed)".to_string()));
    }
    
    let seed = match &args[0] {
        RuntimeValue::Number(n) => *n as u64,
        RuntimeValue::String(s) => {
            // Usa hash da string como seed
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
            let mut hasher = DefaultHasher::new();
            s.hash(&mut hasher);
            hasher.finish()
        },
        _ => return Err(RuntimeError::TypeError("native_random_seed: argumento deve ser número ou string".to_string())),
    };
    
    // Reinicializa o RNG com nova seed
    let mut rng_lock = RNG.lock().unwrap();
    *rng_lock = ChaCha20Rng::seed_from_u64(seed);
    
    Ok(RuntimeValue::Null)
}

// ============================================
// EXPRESSÕES REGULARES
// ============================================

/// native_regex_match(pattern, string) -> array ou null
/// Verifica correspondência de regex e retorna grupos capturados
fn native_regex_match(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::ArgumentError("native_regex_match: esperado 2 argumentos (pattern, string)".to_string()));
    }
    
    let pattern = match &args[0] {
        RuntimeValue::String(s) => s,
        _ => return Err(RuntimeError::TypeError("native_regex_match: primeiro argumento deve ser string".to_string())),
    };
    
    let text = match &args[1] {
        RuntimeValue::String(s) => s,
        _ => return Err(RuntimeError::TypeError("native_regex_match: segundo argumento deve ser string".to_string())),
    };
    
    match Regex::new(pattern) {
        Ok(re) => {
            if let Some(captures) = re.captures(text) {
                let mut groups = Vec::new();
                
                // Adiciona o match completo
                if let Some(full_match) = captures.get(0) {
                    groups.push(RuntimeValue::String(full_match.as_str().to_string()));
                }
                
                // Adiciona grupos capturados
                for i in 1..captures.len() {
                    if let Some(group) = captures.get(i) {
                        groups.push(RuntimeValue::String(group.as_str().to_string()));
                    } else {
                        groups.push(RuntimeValue::Null);
                    }
                }
                
                Ok(RuntimeValue::Array(groups))
            } else {
                Ok(RuntimeValue::Null)
            }
        },
        Err(e) => Err(RuntimeError::Generic(format!("Erro no padrão regex: {}", e))),
    }
}

/// native_regex_replace(pattern, replacement, string) -> string
/// Substitui ocorrências de regex em uma string
fn native_regex_replace(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    if args.len() != 3 {
        return Err(RuntimeError::ArgumentError("native_regex_replace: esperado 3 argumentos (pattern, replacement, string)".to_string()));
    }
    
    let pattern = match &args[0] {
        RuntimeValue::String(s) => s,
        _ => return Err(RuntimeError::TypeError("native_regex_replace: primeiro argumento deve ser string".to_string())),
    };
    
    let replacement = match &args[1] {
        RuntimeValue::String(s) => s,
        _ => return Err(RuntimeError::TypeError("native_regex_replace: segundo argumento deve ser string".to_string())),
    };
    
    let text = match &args[2] {
        RuntimeValue::String(s) => s,
        _ => return Err(RuntimeError::TypeError("native_regex_replace: terceiro argumento deve ser string".to_string())),
    };
    
    match Regex::new(pattern) {
        Ok(re) => {
            let result = re.replace_all(text, replacement.as_str()).to_string();
            Ok(RuntimeValue::String(result))
        },
        Err(e) => Err(RuntimeError::Generic(format!("Erro no padrão regex: {}", e))),
    }
}

/// native_regex_split(pattern, string) -> array
/// Divide uma string usando regex como delimitador
fn native_regex_split(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::ArgumentError("native_regex_split: esperado 2 argumentos (pattern, string)".to_string()));
    }
    
    let pattern = match &args[0] {
        RuntimeValue::String(s) => s,
        _ => return Err(RuntimeError::TypeError("native_regex_split: primeiro argumento deve ser string".to_string())),
    };
    
    let text = match &args[1] {
        RuntimeValue::String(s) => s,
        _ => return Err(RuntimeError::TypeError("native_regex_split: segundo argumento deve ser string".to_string())),
    };
    
    match Regex::new(pattern) {
        Ok(re) => {
            let parts: Vec<RuntimeValue> = re.split(text)
                .map(|s| RuntimeValue::String(s.to_string()))
                .collect();
            Ok(RuntimeValue::Array(parts))
        },
        Err(e) => Err(RuntimeError::Generic(format!("Erro no padrão regex: {}", e))),
    }
}

/// native_regex_test(pattern, string) -> bool
/// Testa se regex corresponde sem capturar grupos
fn native_regex_test(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::ArgumentError("native_regex_test: esperado 2 argumentos (pattern, string)".to_string()));
    }
    
    let pattern = match &args[0] {
        RuntimeValue::String(s) => s,
        _ => return Err(RuntimeError::TypeError("native_regex_test: primeiro argumento deve ser string".to_string())),
    };
    
    let text = match &args[1] {
        RuntimeValue::String(s) => s,
        _ => return Err(RuntimeError::TypeError("native_regex_test: segundo argumento deve ser string".to_string())),
    };
    
    match Regex::new(pattern) {
        Ok(re) => Ok(RuntimeValue::Bool(re.is_match(text))),
        Err(e) => Err(RuntimeError::Generic(format!("Erro no padrão regex: {}", e))),
    }
}