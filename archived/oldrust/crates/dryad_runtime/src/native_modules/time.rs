use crate::interpreter::Value;
use crate::native_modules::NativeFunction;
use crate::errors::RuntimeError;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::thread;

/// Registra todas as funções nativas do módulo time
pub fn register_time_functions(map: &mut HashMap<String, NativeFunction>) {
    map.insert("native_now".to_string(), native_now);
    map.insert("native_sleep".to_string(), native_sleep);
    map.insert("native_timestamp".to_string(), native_timestamp);
    map.insert("native_date".to_string(), native_date);
    map.insert("native_time".to_string(), native_time);
    map.insert("native_format_date".to_string(), native_format_date);
    map.insert("native_uptime".to_string(), native_uptime);
    map.insert("current_timestamp".to_string(), native_timestamp); // Alias para native_timestamp
}

// Variável global para armazenar o tempo de início da aplicação
static mut START_TIME: Option<std::time::Instant> = None;
static INIT: std::sync::Once = std::sync::Once::new();

fn get_start_time() -> std::time::Instant {
    unsafe {
        INIT.call_once(|| {
            START_TIME = Some(std::time::Instant::now());
        });
        START_TIME.unwrap()
    }
}

/// Retorna o timestamp atual em milissegundos desde a época (epoch)
/// Entrada: nenhum
/// Retorna: um número inteiro representando o timestamp atual
fn native_now(_args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => Ok(Value::Number(duration.as_millis() as f64)),
        Err(e) => Err(RuntimeError::IoError(format!("Erro ao obter timestamp: {}", e))),
    }
}

/// Pausa a execução por um número específico de milissegundos
/// Entrada: um número inteiro representando o tempo em milissegundos
/// Retorna: nenhum
fn native_sleep(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("native_sleep requer exatamente 1 argumento".to_string()));
    }

    let ms = match &args[0] {
        Value::Number(n) => {
            if *n < 0.0 {
                return Err(RuntimeError::ArgumentError("Tempo de sleep não pode ser negativo".to_string()));
            }
            *n as u64
        }
        _ => return Err(RuntimeError::TypeError("Argumento deve ser um número".to_string())),
    };

    thread::sleep(Duration::from_millis(ms));
    Ok(Value::Null)
}

/// Retorna o timestamp atual em segundos desde a época (epoch)
/// Entrada: nenhum
/// Retorna: um número inteiro representando o timestamp atual
fn native_timestamp(_args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => Ok(Value::Number(duration.as_secs() as f64)),
        Err(e) => Err(RuntimeError::IoError(format!("Erro ao obter timestamp: {}", e))),
    }
}

/// Retorna a data atual no formato "YYYY-MM-DD"
/// Entrada: nenhum
/// Retorna: uma string representando a data atual
fn native_date(_args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    use chrono::{Local, Datelike};
    
    let now = Local::now();
    let formatted = format!("{:04}-{:02}-{:02}", 
        now.year(), 
        now.month(), 
        now.day()
    );
    
    Ok(Value::String(formatted))
}

/// Retorna a hora atual no formato "HH:MM:SS"
/// Entrada: nenhum
/// Retorna: uma string representando a hora atual
fn native_time(_args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    use chrono::{Local, Timelike};
    
    let now = Local::now();
    let formatted = format!("{:02}:{:02}:{:02}", 
        now.hour(), 
        now.minute(), 
        now.second()
    );
    
    Ok(Value::String(formatted))
}

/// Formata a data atual de acordo com o formato especificado
/// Entrada: uma string representando o formato (ex: "YYYY-MM-DD HH:mm:ss")
/// Retorna: uma string com a data formatada
fn native_format_date(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("native_format_date requer exatamente 1 argumento".to_string()));
    }

    let format_str = match &args[0] {
        Value::String(s) => s,
        _ => return Err(RuntimeError::TypeError("Formato deve ser uma string".to_string())),
    };

    use chrono::{Local, Datelike, Timelike};
    
    let now = Local::now();
    
    // Substituições personalizadas para formato Dryad
    let mut result = format_str.clone();
    
    // Ano
    result = result.replace("YYYY", &format!("{:04}", now.year()));
    result = result.replace("YY", &format!("{:02}", now.year() % 100));
    
    // Mês
    result = result.replace("MM", &format!("{:02}", now.month()));
    result = result.replace("M", &format!("{}", now.month()));
    
    // Dia
    result = result.replace("DD", &format!("{:02}", now.day()));
    result = result.replace("D", &format!("{}", now.day()));
    
    // Hora (24h)
    result = result.replace("HH", &format!("{:02}", now.hour()));
    result = result.replace("H", &format!("{}", now.hour()));
    
    // Hora (12h)
    let hour_12 = if now.hour() == 0 { 12 } else if now.hour() > 12 { now.hour() - 12 } else { now.hour() };
    result = result.replace("hh", &format!("{:02}", hour_12));
    result = result.replace("h", &format!("{}", hour_12));
    
    // Minutos
    result = result.replace("mm", &format!("{:02}", now.minute()));
    
    // Evitar conflito: processar 'm' apenas se não fizer parte de 'mm'
    if !format_str.contains("mm") {
        result = result.replace("m", &format!("{}", now.minute()));
    }
    
    // Segundos
    result = result.replace("ss", &format!("{:02}", now.second()));
    result = result.replace("s", &format!("{}", now.second()));
    
    // AM/PM
    let ampm = if now.hour() < 12 { "AM" } else { "PM" };
    result = result.replace("A", ampm);
    result = result.replace("a", &ampm.to_lowercase());
    
    Ok(Value::String(result))
}

/// Retorna o tempo de execução do programa em milissegundos
/// Entrada: nenhum
/// Retorna: um número inteiro representando o tempo de execução
fn native_uptime(_args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    let start = get_start_time();
    let elapsed = start.elapsed();
    Ok(Value::Number(elapsed.as_millis() as f64))
}
