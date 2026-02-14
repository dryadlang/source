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

    let state = FFI_STATE
        .lock()
        .map_err(|_| RuntimeError::SystemError("Erro ao acessar estado FFI".to_string()))?;

    let library_wrapper = state.libraries.get(&library_alias).ok_or_else(|| {
        RuntimeError::Generic(format!("Biblioteca '{}' não carregada", library_alias))
    })?;

    let symbol_name_c = CString::new(symbol_name.clone())
        .map_err(|_| RuntimeError::Generic("Nome de símbolo inválido".to_string()))?;

    match return_type.as_str() {
        "void" => {
            type FfiVoidFn = unsafe extern "C" fn();
            let func: Symbol<FfiVoidFn> = unsafe {
                library_wrapper
                    .library
                    .get(symbol_name_c.as_bytes())
                    .map_err(|e| RuntimeError::Generic(format!("Símbolo não encontrado: {}", e)))?
            };
            unsafe {
                func();
            }
            Ok(Value::Null)
        }
        "i32" => {
            type FfiI32Fn = unsafe extern "C" fn() -> i32;
            let func: Symbol<FfiI32Fn> = unsafe {
                library_wrapper
                    .library
                    .get(symbol_name_c.as_bytes())
                    .map_err(|e| RuntimeError::Generic(format!("Símbolo não encontrado: {}", e)))?
            };
            let result = unsafe { func() };
            Ok(Value::Number(result as f64))
        }
        "i64" => {
            type FfiI64Fn = unsafe extern "C" fn() -> i64;
            let func: Symbol<FfiI64Fn> = unsafe {
                library_wrapper
                    .library
                    .get(symbol_name_c.as_bytes())
                    .map_err(|e| RuntimeError::Generic(format!("Símbolo não encontrado: {}", e)))?
            };
            let result = unsafe { func() };
            Ok(Value::Number(result as f64))
        }
        "f64" => {
            type FfiF64Fn = unsafe extern "C" fn() -> f64;
            let func: Symbol<FfiF64Fn> = unsafe {
                library_wrapper
                    .library
                    .get(symbol_name_c.as_bytes())
                    .map_err(|e| RuntimeError::Generic(format!("Símbolo não encontrado: {}", e)))?
            };
            let result = unsafe { func() };
            Ok(Value::Number(result))
        }
        "string" | "cstring" => {
            type FfiStringFn = unsafe extern "C" fn() -> *const c_char;
            let func: Symbol<FfiStringFn> = unsafe {
                library_wrapper
                    .library
                    .get(symbol_name_c.as_bytes())
                    .map_err(|e| RuntimeError::Generic(format!("Símbolo não encontrado: {}", e)))?
            };
            let c_str = unsafe { CStr::from_ptr(func()) };
            let rust_string = c_str.to_string_lossy().into_owned();
            Ok(Value::String(rust_string))
        }
        "pointer" => {
            type FfiPointerFn = unsafe extern "C" fn() -> *const std::os::raw::c_void;
            let func: Symbol<FfiPointerFn> = unsafe {
                library_wrapper
                    .library
                    .get(symbol_name_c.as_bytes())
                    .map_err(|e| RuntimeError::Generic(format!("Símbolo não encontrado: {}", e)))?
            };
            let ptr = unsafe { func() };
            let ptr_num = ptr as usize as f64;
            Ok(Value::Number(ptr_num))
        }
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
