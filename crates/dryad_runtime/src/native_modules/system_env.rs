use crate::interpreter::RuntimeValue;
use crate::native_modules::NativeFunction;
use crate::errors::RuntimeError;
use std::collections::HashMap;
use std::env;
use std::process::{Command, exit};

/// Registra todas as funções nativas do módulo system_env
pub fn register_system_env_functions(map: &mut HashMap<String, NativeFunction>) {
    map.insert("native_platform".to_string(), native_platform);
    map.insert("native_arch".to_string(), native_arch);
    map.insert("native_env".to_string(), native_env);
    map.insert("native_set_env".to_string(), native_set_env);
    map.insert("native_exec".to_string(), native_exec);
    map.insert("native_exec_output".to_string(), native_exec_output);
    map.insert("native_pid".to_string(), native_pid);
    map.insert("native_exit".to_string(), native_exit);
    map.insert("get_current_dir".to_string(), native_getcwd);
}

/// Retorna o sistema operacional atual
/// Entrada: nenhum
/// Retorna: uma string representando o sistema operacional
fn native_platform(_args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    let platform = if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "freebsd") {
        "freebsd"
    } else if cfg!(target_os = "openbsd") {
        "openbsd"
    } else if cfg!(target_os = "netbsd") {
        "netbsd"
    } else {
        "unknown"
    };
    
    Ok(RuntimeValue::String(platform.to_string()))
}

/// Retorna a arquitetura do sistema atual
/// Entrada: nenhum
/// Retorna: uma string representando a arquitetura do sistema
fn native_arch(_args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    let arch = if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else if cfg!(target_arch = "x86") {
        "x86"
    } else if cfg!(target_arch = "aarch64") {
        "aarch64"
    } else if cfg!(target_arch = "arm") {
        "arm"
    } else if cfg!(target_arch = "mips") {
        "mips"
    } else if cfg!(target_arch = "mips64") {
        "mips64"
    } else if cfg!(target_arch = "powerpc") {
        "powerpc"
    } else if cfg!(target_arch = "powerpc64") {
        "powerpc64"
    } else if cfg!(target_arch = "riscv64") {
        "riscv64"
    } else if cfg!(target_arch = "s390x") {
        "s390x"
    } else {
        "unknown"
    };
    
    Ok(RuntimeValue::String(arch.to_string()))
}

/// Busca o valor de uma variável de ambiente
/// Entrada: uma string representando o nome da variável de ambiente
/// Retorna: uma string com o valor da variável ou null se não existir
fn native_env(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("native_env requer exatamente 1 argumento".to_string()));
    }

    let key = match &args[0] {
        RuntimeValue::String(s) => s,
        _ => return Err(RuntimeError::TypeError("Argumento deve ser uma string".to_string())),
    };

    match env::var(key) {
        Ok(value) => Ok(RuntimeValue::String(value)),
        Err(_) => Ok(RuntimeValue::Null),
    }
}

/// Define o valor de uma variável de ambiente
/// Entrada: duas strings, a primeira é o nome da variável e a segunda é o valor
/// Retorna: nenhum
fn native_set_env(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::ArgumentError("native_set_env requer exatamente 2 argumentos".to_string()));
    }

    let key = match &args[0] {
        RuntimeValue::String(s) => s,
        _ => return Err(RuntimeError::TypeError("Primeiro argumento deve ser uma string".to_string())),
    };

    let value = match &args[1] {
        RuntimeValue::String(s) => s,
        _ => return Err(RuntimeError::TypeError("Segundo argumento deve ser uma string".to_string())),
    };

    env::set_var(key, value);
    Ok(RuntimeValue::Null)
}

/// Executa um comando no shell e retorna o status de saída
/// Entrada: uma string representando o comando a ser executado
/// Retorna: um número inteiro representando o status de saída do comando
fn native_exec(args: &[RuntimeValue], manager: &crate::native_modules::NativeModuleManager) -> Result<RuntimeValue, RuntimeError> {
    if !manager.allow_unsafe() {
        return Err(RuntimeError::SystemError("native_exec está desabilitado. Use --allow-unsafe para habilitar.".to_string()));
    }
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("native_exec requer exatamente 1 argumento".to_string()));
    }

    let cmd_str = match &args[0] {
        RuntimeValue::String(s) => s,
        _ => return Err(RuntimeError::TypeError("Argumento deve ser uma string".to_string())),
    };

    // Detectar o shell apropriado baseado no sistema operacional
    let (shell, flag) = if cfg!(target_os = "windows") {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    };

    match Command::new(shell)
        .arg(flag)
        .arg(cmd_str)
        .status()
    {
        Ok(status) => {
            let code = status.code().unwrap_or(-1);
            Ok(RuntimeValue::Number(code as f64))
        }
        Err(e) => Err(RuntimeError::IoError(format!("Erro ao executar comando: {}", e))),
    }
}

/// Executa um comando no shell e retorna sua saída padrão
/// Entrada: uma string representando o comando a ser executado
/// Retorna: uma string com a saída do comando
fn native_exec_output(args: &[RuntimeValue], manager: &crate::native_modules::NativeModuleManager) -> Result<RuntimeValue, RuntimeError> {
    if !manager.allow_unsafe() {
        return Err(RuntimeError::SystemError("native_exec_output está desabilitado. Use --allow-unsafe para habilitar.".to_string()));
    }
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("native_exec_output requer exatamente 1 argumento".to_string()));
    }

    let cmd_str = match &args[0] {
        RuntimeValue::String(s) => s,
        _ => return Err(RuntimeError::TypeError("Argumento deve ser uma string".to_string())),
    };

    // Detectar o shell apropriado baseado no sistema operacional
    let (shell, flag) = if cfg!(target_os = "windows") {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    };

    match Command::new(shell)
        .arg(flag)
        .arg(cmd_str)
        .output()
    {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            Ok(RuntimeValue::String(stdout.trim().to_string()))
        }
        Err(e) => Err(RuntimeError::IoError(format!("Erro ao executar comando: {}", e))),
    }
}

/// Retorna o ID do processo atual
/// Entrada: nenhum
/// Retorna: um número inteiro representando o ID do processo
fn native_pid(_args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    let pid = std::process::id();
    Ok(RuntimeValue::Number(pid as f64))
}

/// Encerra a execução do programa com um código de saída
/// Entrada: um número inteiro representando o código de saída (0 para sucesso, outros valores para erro)
/// Retorna: nenhum (nunca retorna, pois encerra o programa)
fn native_exit(args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    let exit_code = if args.is_empty() {
        0
    } else {
        match &args[0] {
            RuntimeValue::Number(n) => *n as i32,
            _ => return Err(RuntimeError::TypeError("Código de saída deve ser um número".to_string())),
        }
    };

    exit(exit_code);
}

/// Retorna o diretório de trabalho atual
/// Entrada: nenhum
/// Retorna: uma string representando o caminho do diretório atual
fn native_getcwd(_args: &[RuntimeValue], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<RuntimeValue, RuntimeError> {
    match env::current_dir() {
        Ok(path) => Ok(RuntimeValue::String(path.to_string_lossy().into_owned())),
        Err(e) => Err(RuntimeError::IoError(format!("Erro ao obter diretório atual: {}", e))),
    }
}
