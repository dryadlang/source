use crate::interpreter::RuntimeValue;
use crate::native_modules::NativeFunction;
use crate::errors::RuntimeError;
use std::collections::HashMap;
use std::time::Instant;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref PERF_TIMERS: Mutex<HashMap<String, Instant>> = Mutex::new(HashMap::new());
}

fn native_debug(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("debug espera 1 argumento: mensagem".to_string()));
    }
    // Usa Debug em vez de Display
    let msg = format!("{:?}", &args[0]);
    println!("[DEBUG] {}", msg);
    Ok(RuntimeValue::Null)
}

pub fn register_debug_functions(functions: &mut HashMap<String, NativeFunction>) {
    functions.insert("debug".to_string(), native_debug);
    functions.insert("log".to_string(), native_log);
    functions.insert("native_typeof".to_string(), native_typeof);
    functions.insert("native_memory_usage".to_string(), native_memory_usage);
    functions.insert("native_stack_trace".to_string(), native_stack_trace);
    functions.insert("native_perf_start".to_string(), native_perf_start);
    functions.insert("native_perf_end".to_string(), native_perf_end);
    functions.insert("native_assert".to_string(), native_assert);
    functions.insert("native_assert_equal".to_string(), native_assert_equal);
    functions.insert("native_assert_not_equal".to_string(), native_assert_not_equal);
    functions.insert("native_assert_true".to_string(), native_assert_true);
    functions.insert("native_assert_false".to_string(), native_assert_false);
    functions.insert("native_assert_type".to_string(), native_assert_type);
    functions.insert("native_test_regex".to_string(), native_test_regex);
}

// Log simples
fn native_log(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::ArgumentError("log espera exatamente 2 argumentos: nível e mensagem".to_string()));
    }
    
    let level = match &args[0] {
        RuntimeValue::String(s) => s.as_str(),
        _ => return Err(RuntimeError::TypeError("Primeiro argumento deve ser uma string (nível)".to_string())),
    };
    
    let message = match &args[1] {
        RuntimeValue::String(s) => s.clone(),
        v => format!("{:?}", v),
    };
    
    println!("[{}] {}", level.to_uppercase(), message);
    Ok(RuntimeValue::Null)
}

// Obter tipo de uma variável
fn native_typeof(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("typeof espera exatamente 1 argumento".to_string()));
    }
    
    let type_name = match &args[0] {
        RuntimeValue::String(_) => "string",
        RuntimeValue::Number(_) => "number",
        RuntimeValue::Bool(_) => "boolean",
        RuntimeValue::Null => "null",
        RuntimeValue::Array(_) => "array",
        RuntimeValue::Tuple(_) => "tuple",
        RuntimeValue::Exception(_) => "exception",
        RuntimeValue::Function { .. } => "function",
        RuntimeValue::AsyncFunction { .. } => "async_function",
        RuntimeValue::ThreadFunction { .. } => "thread_function",
        RuntimeValue::Lambda { .. } => "lambda",
        RuntimeValue::Thread { .. } => "thread",
        RuntimeValue::Mutex { .. } => "mutex",
        RuntimeValue::Promise { .. } => "promise",
        RuntimeValue::Class { .. } => "class",
        RuntimeValue::Instance { .. } => "instance",
        RuntimeValue::Object { .. } => "object",
    };
    
    Ok(RuntimeValue::String(type_name.to_string()))
}

// Uso de memória
fn native_memory_usage(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    if !args.is_empty() {
        return Err(RuntimeError::ArgumentError("memory_usage não aceita argumentos".to_string()));
    }
    
    use sysinfo::System;
    let mut system = System::new_all();
    system.refresh_all();
    
    if let Some(process) = system.process(sysinfo::get_current_pid().unwrap()) {
        let memory_kb = process.memory();
        Ok(RuntimeValue::Number(memory_kb as f64))
    } else {
        Ok(RuntimeValue::Number(0.0))
    }
}

// Stack trace
fn native_stack_trace(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    if !args.is_empty() {
        return Err(RuntimeError::ArgumentError("stack_trace não aceita argumentos".to_string()));
    }
    
    let bt = backtrace::Backtrace::new();
    let stack_string = format!("{:?}", bt);
    Ok(RuntimeValue::String(stack_string))
}

// Iniciar cronômetro de performance
fn native_perf_start(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("perf_start espera exatamente 1 argumento (nome do timer)".to_string()));
    }
    
    let timer_name = match &args[0] {
        RuntimeValue::String(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("Argumento deve ser uma string (nome do timer)".to_string())),
    };
    
    let mut timers = PERF_TIMERS.lock().unwrap();
    timers.insert(timer_name, Instant::now());
    
    Ok(RuntimeValue::Null)
}

// Finalizar cronômetro de performance
fn native_perf_end(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("perf_end espera exatamente 1 argumento (nome do timer)".to_string()));
    }
    
    let timer_name = match &args[0] {
        RuntimeValue::String(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("Argumento deve ser uma string (nome do timer)".to_string())),
    };
    
    let mut timers = PERF_TIMERS.lock().unwrap();
    if let Some(start_time) = timers.remove(&timer_name) {
        let elapsed = start_time.elapsed();
        let elapsed_ms = elapsed.as_secs_f64() * 1000.0;
        Ok(RuntimeValue::Number(elapsed_ms))
    } else {
        Err(RuntimeError::Generic(format!("Timer '{}' não foi encontrado", timer_name)))
    }
}

// Assert genérico
fn native_assert(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    if args.len() < 1 || args.len() > 2 {
        return Err(RuntimeError::ArgumentError("assert espera 1 ou 2 argumentos: condição e mensagem opcional".to_string()));
    }
    
    let condition = match &args[0] {
        RuntimeValue::Bool(b) => *b,
        _ => return Err(RuntimeError::TypeError("Primeiro argumento deve ser um boolean".to_string())),
    };
    
    if !condition {
        let message = if args.len() > 1 {
            match &args[1] {
                RuntimeValue::String(s) => s.clone(),
                _ => "Assertion failed".to_string(),
            }
        } else {
            "Assertion failed".to_string()
        };
        
        return Err(RuntimeError::Generic(format!("Assertion Error: {}", message)));
    }
    
    Ok(RuntimeValue::Bool(true))
}

// Assert igualdade
fn native_assert_equal(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    if args.len() < 2 || args.len() > 3 {
        return Err(RuntimeError::ArgumentError("assert_equal espera 2 ou 3 argumentos: esperado, atual e mensagem opcional".to_string()));
    }
    
    let expected = &args[0];
    let actual = &args[1];
    
    let equal = match (expected, actual) {
        (RuntimeValue::String(a), RuntimeValue::String(b)) => a == b,
        (RuntimeValue::Number(a), RuntimeValue::Number(b)) => (a - b).abs() < f64::EPSILON,
        (RuntimeValue::Bool(a), RuntimeValue::Bool(b)) => a == b,
        (RuntimeValue::Null, RuntimeValue::Null) => true,
        _ => false,
    };
    
    if !equal {
        let message = if args.len() > 2 {
            match &args[2] {
                RuntimeValue::String(s) => s.clone(),
                _ => "Values are not equal".to_string(),
            }
        } else {
            format!("Expected {:?}, got {:?}", expected, actual)
        };
        
        return Err(RuntimeError::Generic(format!("Assertion Error: {}", message)));
    }
    
    Ok(RuntimeValue::Bool(true))
}

// Assert não igualdade
fn native_assert_not_equal(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    if args.len() < 2 || args.len() > 3 {
        return Err(RuntimeError::ArgumentError("assert_not_equal espera 2 ou 3 argumentos: primeiro, segundo e mensagem opcional".to_string()));
    }
    
    let first = &args[0];
    let second = &args[1];
    
    let equal = match (first, second) {
        (RuntimeValue::String(a), RuntimeValue::String(b)) => a == b,
        (RuntimeValue::Number(a), RuntimeValue::Number(b)) => (a - b).abs() < f64::EPSILON,
        (RuntimeValue::Bool(a), RuntimeValue::Bool(b)) => a == b,
        (RuntimeValue::Null, RuntimeValue::Null) => true,
        _ => false,
    };
    
    if equal {
        let message = if args.len() > 2 {
            match &args[2] {
                RuntimeValue::String(s) => s.clone(),
                _ => "Values should not be equal".to_string(),
            }
        } else {
            format!("Values {:?} and {:?} should not be equal", first, second)
        };
        
        return Err(RuntimeError::Generic(format!("Assertion Error: {}", message)));
    }
    
    Ok(RuntimeValue::Bool(true))
}

// Assert true
fn native_assert_true(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    if args.len() < 1 || args.len() > 2 {
        return Err(RuntimeError::ArgumentError("assert_true espera 1 ou 2 argumentos: valor e mensagem opcional".to_string()));
    }
    
    let value = match &args[0] {
        RuntimeValue::Bool(b) => *b,
        _ => return Err(RuntimeError::TypeError("Argumento deve ser um boolean".to_string())),
    };
    
    if !value {
        let message = if args.len() > 1 {
            match &args[1] {
                RuntimeValue::String(s) => s.clone(),
                _ => "Expected true".to_string(),
            }
        } else {
            "Expected true, got false".to_string()
        };
        
        return Err(RuntimeError::Generic(format!("Assertion Error: {}", message)));
    }
    
    Ok(RuntimeValue::Bool(true))
}

// Assert false
fn native_assert_false(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    if args.len() < 1 || args.len() > 2 {
        return Err(RuntimeError::ArgumentError("assert_false espera 1 ou 2 argumentos: valor e mensagem opcional".to_string()));
    }
    
    let value = match &args[0] {
        RuntimeValue::Bool(b) => *b,
        _ => return Err(RuntimeError::TypeError("Argumento deve ser um boolean".to_string())),
    };
    
    if value {
        let message = if args.len() > 1 {
            match &args[1] {
                RuntimeValue::String(s) => s.clone(),
                _ => "Expected false".to_string(),
            }
        } else {
            "Expected false, got true".to_string()
        };
        
        return Err(RuntimeError::Generic(format!("Assertion Error: {}", message)));
    }
    
    Ok(RuntimeValue::Bool(true))
}

// Assert tipo
fn native_assert_type(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    if args.len() < 2 || args.len() > 3 {
        return Err(RuntimeError::ArgumentError("assert_type espera 2 ou 3 argumentos: valor, tipo_esperado e mensagem opcional".to_string()));
    }
    
    let value = &args[0];
    let expected_type = match &args[1] {
        RuntimeValue::String(s) => s.as_str(),
        _ => return Err(RuntimeError::TypeError("Segundo argumento deve ser uma string (tipo)".to_string())),
    };
    
    let actual_type = match value {
        RuntimeValue::String(_) => "string",
        RuntimeValue::Number(_) => "number",
        RuntimeValue::Bool(_) => "boolean",
        RuntimeValue::Null => "null",
        RuntimeValue::Array(_) => "array",
        RuntimeValue::Tuple(_) => "tuple",
        RuntimeValue::Exception(_) => "exception",
        RuntimeValue::Function { .. } => "function",
        RuntimeValue::AsyncFunction { .. } => "async_function",
        RuntimeValue::ThreadFunction { .. } => "thread_function",
        RuntimeValue::Lambda { .. } => "lambda",
        RuntimeValue::Thread { .. } => "thread",
        RuntimeValue::Mutex { .. } => "mutex",
        RuntimeValue::Promise { .. } => "promise",
        RuntimeValue::Class { .. } => "class",
        RuntimeValue::Instance { .. } => "instance",
        RuntimeValue::Object { .. } => "object",
    };
    
    if actual_type != expected_type {
        let message = if args.len() > 2 {
            match &args[2] {
                RuntimeValue::String(s) => s.clone(),
                _ => "Type mismatch".to_string(),
            }
        } else {
            format!("Expected type '{}', got '{}'", expected_type, actual_type)
        };
        
        return Err(RuntimeError::Generic(format!("Type Assertion Error: {}", message)));
    }
    
    Ok(RuntimeValue::Bool(true))
}

// Testar regex
fn native_test_regex(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::ArgumentError("test_regex espera exatamente 2 argumentos: padrão e texto".to_string()));
    }
    
    let pattern = match &args[0] {
        RuntimeValue::String(s) => s.as_str(),
        _ => return Err(RuntimeError::TypeError("Primeiro argumento deve ser uma string (padrão regex)".to_string())),
    };
    
    let text = match &args[1] {
        RuntimeValue::String(s) => s.as_str(),
        _ => return Err(RuntimeError::TypeError("Segundo argumento deve ser uma string (texto)".to_string())),
    };
    
    match regex::Regex::new(pattern) {
        Ok(re) => Ok(RuntimeValue::Bool(re.is_match(text))),
        Err(e) => Err(RuntimeError::Generic(format!("Erro no padrão regex: {}", e))),
    }
}
