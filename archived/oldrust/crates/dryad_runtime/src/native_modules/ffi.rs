use crate::errors::RuntimeError;
use crate::interpreter::Value;
use crate::native_modules::NativeFunction;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use libloading::{Library, Symbol};

struct LibraryWrapper {
    library: Library,
    name: String,
}

struct FfiState {
    libraries: HashMap<String, LibraryWrapper>,
}

impl FfiState {
    fn new() -> Self {
        FfiState {
            libraries: HashMap::new(),
        }
    }
}

lazy_static::lazy_static! {
    static ref FFI_STATE: std::sync::Mutex<FfiState> = std::sync::Mutex::new(FfiState::new());
}

pub fn register_ffi_functions(functions: &mut HashMap<String, NativeFunction>) {
    functions.insert("ffi_load_library".to_string(), ffi_load_library);
    functions.insert("ffi_unload_library".to_string(), ffi_unload_library);
    functions.insert("ffi_call".to_string(), ffi_call);
    functions.insert("ffi_get_symbol".to_string(), ffi_get_symbol);
    functions.insert("ffi_list_libraries".to_string(), ffi_list_libraries);
}

pub fn test_ffi_load_library(
    args: &[Value],
    manager: &crate::native_modules::NativeModuleManager,
    heap: &mut crate::heap::Heap,
) -> Result<Value, RuntimeError> {
    ffi_load_library(args, manager, heap)
}

pub fn test_ffi_unload_library(
    args: &[Value],
    manager: &crate::native_modules::NativeModuleManager,
    heap: &mut crate::heap::Heap,
) -> Result<Value, RuntimeError> {
    ffi_unload_library(args, manager, heap)
}

pub fn test_ffi_call(
    args: &[Value],
    manager: &crate::native_modules::NativeModuleManager,
    heap: &mut crate::heap::Heap,
) -> Result<Value, RuntimeError> {
    ffi_call(args, manager, heap)
}

pub fn test_ffi_get_symbol(
    args: &[Value],
    manager: &crate::native_modules::NativeModuleManager,
    heap: &mut crate::heap::Heap,
) -> Result<Value, RuntimeError> {
    ffi_get_symbol(args, manager, heap)
}

pub fn test_ffi_list_libraries(
    args: &[Value],
    manager: &crate::native_modules::NativeModuleManager,
    heap: &mut crate::heap::Heap,
) -> Result<Value, RuntimeError> {
    ffi_list_libraries(args, manager, heap)
}

fn ffi_load_library(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    _heap: &mut crate::heap::Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 1 && args.len() != 2 {
        return Err(RuntimeError::ArgumentError(
            "ffi_load_library requer 1 ou 2 argumentos: path, alias (opcional)".to_string(),
        ));
    }

    let path = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(RuntimeError::TypeError(
                "Primeiro argumento deve ser string (caminho da biblioteca)".to_string(),
            ))
        }
    };

    let alias = if args.len() > 1 {
        match &args[1] {
            Value::String(s) => s.clone(),
            _ => {
                return Err(RuntimeError::TypeError(
                    "Segundo argumento deve ser string (alias)".to_string(),
                ))
            }
        }
    } else {
        path.clone()
    };

    let mut state = FFI_STATE
        .lock()
        .map_err(|_| RuntimeError::SystemError("Erro ao acessar estado FFI".to_string()))?;

    if state.libraries.contains_key(&alias) {
        return Err(RuntimeError::Generic(format!(
            "Biblioteca '{}' já carregada",
            alias
        )));
    }

    let library = match unsafe { Library::new(&path) } {
        Ok(lib) => lib,
        Err(e) => {
            return Err(RuntimeError::IoError(format!(
                "Erro ao carregar biblioteca '{}': {}",
                path, e
            )))
        }
    };

    state.libraries.insert(
        alias.clone(),
        LibraryWrapper {
            library,
            name: path,
        },
    );

    Ok(Value::Bool(true))
}

fn ffi_unload_library(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    _heap: &mut crate::heap::Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError(
            "ffi_unload_library requer 1 argumento: alias".to_string(),
        ));
    }

    let alias = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(RuntimeError::TypeError(
                "Argumento deve ser string (alias)".to_string(),
            ))
        }
    };

    let mut state = FFI_STATE
        .lock()
        .map_err(|_| RuntimeError::SystemError("Erro ao acessar estado FFI".to_string()))?;

    match state.libraries.remove(&alias) {
        Some(_) => {
            println!("📦 Biblioteca '{}' descargada", alias);
            Ok(Value::Bool(true))
        }
        None => Err(RuntimeError::Generic(format!(
            "Biblioteca '{}' não encontrada",
            alias
        ))),
    }
}

fn ffi_call(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    _heap: &mut crate::heap::Heap,
) -> Result<Value, RuntimeError> {
    if args.len() < 3 {
        return Err(RuntimeError::ArgumentError(
            "ffi_call requer pelo menos 3 argumentos: library_alias, symbol_name, return_type"
                .to_string(),
        ));
    }

    let library_alias = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(RuntimeError::TypeError(
                "Primeiro argumento deve ser string (alias da biblioteca)".to_string(),
            ))
        }
    };

    let symbol_name = match &args[1] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(RuntimeError::TypeError(
                "Segundo argumento deve ser string (nome do símbolo)".to_string(),
            ))
        }
    };

    let return_type = match &args[2] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(RuntimeError::TypeError(
                "Terceiro argumento deve ser string (tipo de retorno)".to_string(),
            ))
        }
    };

    let ffi_args = &args[3..];
    if ffi_args.len() > 6 {
        return Err(RuntimeError::ArgumentError(
            "ffi_call suporta no máximo 6 argumentos".to_string(),
        ));
    }

    let mut arg_values = Vec::new();
    let mut _string_holders = Vec::new();

    for val in ffi_args {
        match val {
            Value::Number(n) => arg_values.push(*n as usize),
            Value::String(s) => {
                let c_str = CString::new(s.clone()).map_err(|_| {
                    RuntimeError::Generic("Erro ao converter string para C".to_string())
                })?;
                arg_values.push(c_str.as_ptr() as usize);
                _string_holders.push(c_str);
            }
            Value::Bool(b) => arg_values.push(if *b { 1 } else { 0 }),
            Value::Null => arg_values.push(0),
            _ => {
                return Err(RuntimeError::TypeError(
                    "Tipo de argumento não suportado para FFI".to_string(),
                ))
            }
        }
    }

    let state = FFI_STATE
        .lock()
        .map_err(|_| RuntimeError::SystemError("Erro ao acessar estado FFI".to_string()))?;

    let library_wrapper = state.libraries.get(&library_alias).ok_or_else(|| {
        RuntimeError::Generic(format!("Biblioteca '{}' não carregada", library_alias))
    })?;

    let symbol_name_c = CString::new(symbol_name.clone())
        .map_err(|_| RuntimeError::Generic("Nome de símbolo inválido".to_string()))?;

    macro_rules! dispatch_ffi {
        ($ret_dryad:ident, $ret_type:ty, $conv:expr) => {{
            match arg_values.len() {
                0 => {
                    let func: Symbol<unsafe extern "C" fn() -> $ret_type> = unsafe {
                        library_wrapper.library.get(symbol_name_c.as_bytes())
                            .map_err(|e| RuntimeError::Generic(format!("Símbolo não encontrado: {}", e)))?
                    };
                    let res = unsafe { func() };
                    Ok($conv(res))
                }
                1 => {
                    let func: Symbol<unsafe extern "C" fn(usize) -> $ret_type> = unsafe {
                        library_wrapper.library.get(symbol_name_c.as_bytes())
                            .map_err(|e| RuntimeError::Generic(format!("Símbolo não encontrado: {}", e)))?
                    };
                    let res = unsafe { func(arg_values[0]) };
                    Ok($conv(res))
                }
                2 => {
                    let func: Symbol<unsafe extern "C" fn(usize, usize) -> $ret_type> = unsafe {
                        library_wrapper.library.get(symbol_name_c.as_bytes())
                            .map_err(|e| RuntimeError::Generic(format!("Símbolo não encontrado: {}", e)))?
                    };
                    let res = unsafe { func(arg_values[0], arg_values[1]) };
                    Ok($conv(res))
                }
                3 => {
                    let func: Symbol<unsafe extern "C" fn(usize, usize, usize) -> $ret_type> = unsafe {
                        library_wrapper.library.get(symbol_name_c.as_bytes())
                            .map_err(|e| RuntimeError::Generic(format!("Símbolo não encontrado: {}", e)))?
                    };
                    let res = unsafe { func(arg_values[0], arg_values[1], arg_values[2]) };
                    Ok($conv(res))
                }
                4 => {
                    let func: Symbol<unsafe extern "C" fn(usize, usize, usize, usize) -> $ret_type> = unsafe {
                        library_wrapper.library.get(symbol_name_c.as_bytes())
                            .map_err(|e| RuntimeError::Generic(format!("Símbolo não encontrado: {}", e)))?
                    };
                    let res = unsafe { func(arg_values[0], arg_values[1], arg_values[2], arg_values[3]) };
                    Ok($conv(res))
                }
                5 => {
                    let func: Symbol<unsafe extern "C" fn(usize, usize, usize, usize, usize) -> $ret_type> = unsafe {
                        library_wrapper.library.get(symbol_name_c.as_bytes())
                            .map_err(|e| RuntimeError::Generic(format!("Símbolo não encontrado: {}", e)))?
                    };
                    let res = unsafe { func(arg_values[0], arg_values[1], arg_values[2], arg_values[3], arg_values[4]) };
                    Ok($conv(res))
                }
                6 => {
                    let func: Symbol<unsafe extern "C" fn(usize, usize, usize, usize, usize, usize) -> $ret_type> = unsafe {
                        library_wrapper.library.get(symbol_name_c.as_bytes())
                            .map_err(|e| RuntimeError::Generic(format!("Símbolo não encontrado: {}", e)))?
                    };
                    let res = unsafe { func(arg_values[0], arg_values[1], arg_values[2], arg_values[3], arg_values[4], arg_values[5]) };
                    Ok($conv(res))
                }
                _ => unreachable!(),
            }
        }};
    }

    match return_type.as_str() {
        "void" => dispatch_ffi!(void, (), |_| Value::Null),
        "i32" => dispatch_ffi!(i32, i32, |r| Value::Number(r as f64)),
        "i64" => dispatch_ffi!(i64, i64, |r| Value::Number(r as f64)),
        "f64" => dispatch_ffi!(f64, f64, |r| Value::Number(r)),
        "string" | "cstring" => dispatch_ffi!(string, *const c_char, |r| {
            let c_str = unsafe { CStr::from_ptr(r) };
            Value::String(c_str.to_string_lossy().into_owned())
        }),
        "pointer" => dispatch_ffi!(pointer, *const std::os::raw::c_void, |r| {
            Value::Number(r as usize as f64)
        }),
        _ => Err(RuntimeError::Generic(format!(
            "Tipo de retorno não suportado: {}",
            return_type
        ))),
    }
}

fn ffi_get_symbol(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    _heap: &mut crate::heap::Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::ArgumentError(
            "ffi_get_symbol requer 2 argumentos: library_alias, symbol_name".to_string(),
        ));
    }

    let library_alias = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(RuntimeError::TypeError(
                "Primeiro argumento deve ser string (alias da biblioteca)".to_string(),
            ))
        }
    };

    let symbol_name = match &args[1] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(RuntimeError::TypeError(
                "Segundo argumento deve ser string (nome do símbolo)".to_string(),
            ))
        }
    };

    let state = FFI_STATE
        .lock()
        .map_err(|_| RuntimeError::SystemError("Erro ao acessar estado FFI".to_string()))?;

    let library_wrapper = state.libraries.get(&library_alias).ok_or_else(|| {
        RuntimeError::Generic(format!("Biblioteca '{}' não carregada", library_alias))
    })?;

    let symbol_name_c = CString::new(symbol_name.clone())
        .map_err(|_| RuntimeError::Generic("Nome de símbolo inválido".to_string()))?;

    unsafe {
        match library_wrapper.library.get::<()>(symbol_name_c.as_bytes()) {
            Ok(_) => Ok(Value::Bool(true)),
            Err(_) => Ok(Value::Bool(false)),
        }
    }
}

fn ffi_list_libraries(
    _args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    _heap: &mut crate::heap::Heap,
) -> Result<Value, RuntimeError> {
    let state = FFI_STATE
        .lock()
        .map_err(|_| RuntimeError::SystemError("Erro ao acessar estado FFI".to_string()))?;

    let library_names: Vec<Value> = state
        .libraries
        .keys()
        .map(|k| Value::String(k.clone()))
        .collect();

    let id = _heap.allocate(crate::heap::ManagedObject::Array(library_names));
    Ok(Value::Array(id))
}
