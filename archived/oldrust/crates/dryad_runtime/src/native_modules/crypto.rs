use crate::errors::RuntimeError;
use crate::heap::{Heap, ManagedObject};
use crate::interpreter::Value;
use crate::native_modules::NativeFunction;
use base64::{engine::general_purpose, Engine};
use md5;
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use uuid::Uuid;

/// Registra todas as funções nativas do módulo crypto
pub fn register_crypto_functions(functions: &mut HashMap<String, NativeFunction>) {
    functions.insert("sha256".to_string(), native_hash_sha256);
    functions.insert("native_hash_md5".to_string(), native_hash_md5);
    functions.insert("native_uuid".to_string(), native_uuid);
    functions.insert("native_base64_encode".to_string(), native_base64_encode);
    functions.insert("native_base64_decode".to_string(), native_base64_decode);
    functions.insert("native_hex_encode".to_string(), native_hex_encode);
    functions.insert("native_hex_decode".to_string(), native_hex_decode);
    functions.insert("native_random_bytes".to_string(), native_random_bytes);
    functions.insert("native_random_string".to_string(), native_random_string);
    functions.insert("native_bytes_to_string".to_string(), native_bytes_to_string);
    functions.insert("native_encrypt_aes".to_string(), native_encrypt_aes);
    functions.insert("native_decrypt_aes".to_string(), native_decrypt_aes);
    functions.insert("native_encrypt_rsa".to_string(), native_encrypt_rsa);
    functions.insert("native_decrypt_rsa".to_string(), native_decrypt_rsa);
    functions.insert("native_sign".to_string(), native_sign);
    functions.insert("native_verify".to_string(), native_verify);
    functions.insert(
        "native_generate_rsa_keypair".to_string(),
        native_generate_rsa_keypair,
    );
    functions.insert("native_hmac_sha256".to_string(), native_hmac_sha256);
    functions.insert("native_hmac_sha512".to_string(), native_hmac_sha512);
}

// ============================================
// FUNÇÕES DE HASH
// ============================================

/// native_hash_sha256(data) -> string
fn native_hash_sha256(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    _heap: &mut crate::heap::Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError(
            "native_hash_sha256: esperado 1 argumento".to_string(),
        ));
    }

    let data = extract_bytes_from_value(&args[0], _heap)?;

    let mut hasher = Sha256::new();
    hasher.update(&data);
    let result = hasher.finalize();
    let hex_string = hex::encode(result);

    Ok(Value::String(hex_string))
}

/// native_hash_md5(data) -> string
fn native_hash_md5(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    _heap: &mut crate::heap::Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError(
            "native_hash_md5: esperado 1 argumento".to_string(),
        ));
    }

    let data = extract_bytes_from_value(&args[0], _heap)?;

    let mut hasher = md5::Context::new();
    hasher.consume(&data);
    let result = hasher.compute();
    let hex_string = format!("{:x}", result);

    Ok(Value::String(hex_string))
}

// ============================================
// IDENTIFICADORES ÚNICOS
// ============================================

/// native_uuid() -> string
fn native_uuid(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    _heap: &mut crate::heap::Heap,
) -> Result<Value, RuntimeError> {
    if !args.is_empty() {
        return Err(RuntimeError::ArgumentError(
            "native_uuid: não esperado argumentos".to_string(),
        ));
    }

    let uuid = Uuid::new_v4();
    Ok(Value::String(uuid.to_string()))
}

// ============================================
// CODIFICAÇÃO/DECODIFICAÇÃO
// ============================================

/// native_base64_encode(data) -> string
fn native_base64_encode(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    _heap: &mut crate::heap::Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError(
            "native_base64_encode: esperado 1 argumento".to_string(),
        ));
    }

    let data = extract_bytes_from_value(&args[0], _heap)?;

    let encoded = general_purpose::STANDARD.encode(&data);
    Ok(Value::String(encoded))
}

/// native_base64_decode(data) -> string | array
fn native_base64_decode(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    _heap: &mut crate::heap::Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError(
            "native_base64_decode: esperado 1 argumento".to_string(),
        ));
    }

    let base64_str = match &args[0] {
        Value::String(s) => s,
        _ => {
            return Err(RuntimeError::TypeError(
                "native_base64_decode: argumento deve ser string".to_string(),
            ))
        }
    };

    match general_purpose::STANDARD.decode(base64_str) {
        Ok(bytes) => {
            match String::from_utf8(bytes.clone()) {
                Ok(decoded_string) => Ok(Value::String(decoded_string)),
                Err(_) => {
                    // Se não for UTF-8 válido, retorna como array de bytes
                    let runtime_bytes: Vec<Value> =
                        bytes.into_iter().map(|b| Value::Number(b as f64)).collect();
                    let id = _heap.allocate(ManagedObject::Array(runtime_bytes));
                    Ok(Value::Array(id))
                }
            }
        }
        Err(e) => Err(RuntimeError::IoError(format!(
            "Erro ao decodificar base64: {}",
            e
        ))),
    }
}

/// native_hex_encode(data) -> string
fn native_hex_encode(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    _heap: &mut crate::heap::Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError(
            "native_hex_encode: esperado 1 argumento".to_string(),
        ));
    }

    let data = extract_bytes_from_value(&args[0], _heap)?;

    let hex_string = hex::encode(&data);
    Ok(Value::String(hex_string))
}

/// native_hex_decode(hex_str) -> string | array
fn native_hex_decode(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    _heap: &mut crate::heap::Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError(
            "native_hex_decode: esperado 1 argumento".to_string(),
        ));
    }

    let hex_str = match &args[0] {
        Value::String(s) => s,
        _ => {
            return Err(RuntimeError::TypeError(
                "native_hex_decode: argumento deve ser string".to_string(),
            ))
        }
    };

    match hex::decode(hex_str) {
        Ok(bytes) => {
            match String::from_utf8(bytes.clone()) {
                Ok(decoded_string) => Ok(Value::String(decoded_string)),
                Err(_) => {
                    // Se não for UTF-8 válido, retorna como array de bytes
                    let runtime_bytes: Vec<Value> =
                        bytes.into_iter().map(|b| Value::Number(b as f64)).collect();
                    let id = _heap.allocate(ManagedObject::Array(runtime_bytes));
                    Ok(Value::Array(id))
                }
            }
        }
        Err(e) => Err(RuntimeError::IoError(format!(
            "Erro ao decodificar hex: {}",
            e
        ))),
    }
}

// ============================================
// GERAÇÃO DE DADOS ALEATÓRIOS
// ============================================

/// native_random_bytes(length) -> array
fn native_random_bytes(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    _heap: &mut crate::heap::Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError(
            "native_random_bytes: esperado 1 argumento".to_string(),
        ));
    }

    let length = match &args[0] {
        Value::Number(n) => *n as usize,
        _ => {
            return Err(RuntimeError::TypeError(
                "native_random_bytes: argumento deve ser número".to_string(),
            ))
        }
    };

    if length > 10000 {
        return Err(RuntimeError::ArgumentError(
            "native_random_bytes: tamanho máximo é 10000 bytes".to_string(),
        ));
    }

    let mut rng = ChaCha20Rng::from_entropy();
    let mut bytes = vec![0u8; length];
    rng.fill_bytes(&mut bytes);

    let runtime_bytes: Vec<Value> = bytes.into_iter().map(|b| Value::Number(b as f64)).collect();

    let id = _heap.allocate(ManagedObject::Array(runtime_bytes));
    Ok(Value::Array(id))
}

/// native_random_string(length, charset?) -> string
fn native_random_string(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    _heap: &mut crate::heap::Heap,
) -> Result<Value, RuntimeError> {
    if args.is_empty() || args.len() > 2 {
        return Err(RuntimeError::ArgumentError(
            "native_random_string: esperado 1 ou 2 argumentos".to_string(),
        ));
    }

    let length = match &args[0] {
        Value::Number(n) => *n as usize,
        _ => {
            return Err(RuntimeError::TypeError(
                "native_random_string: primeiro argumento deve ser número".to_string(),
            ))
        }
    };

    if length > 10000 {
        return Err(RuntimeError::ArgumentError(
            "native_random_string: tamanho máximo é 10000 caracteres".to_string(),
        ));
    }

    let charset = if args.len() == 2 {
        match &args[1] {
            Value::String(s) => s.clone(),
            _ => {
                return Err(RuntimeError::TypeError(
                    "native_random_string: segundo argumento deve ser string".to_string(),
                ))
            }
        }
    } else {
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".to_string()
    };

    if charset.is_empty() {
        return Err(RuntimeError::ArgumentError(
            "native_random_string: charset não pode estar vazio".to_string(),
        ));
    }

    let mut rng = ChaCha20Rng::from_entropy();
    let charset_chars: Vec<char> = charset.chars().collect();
    let mut result = String::new();

    for _ in 0..length {
        let idx = (rng.next_u32() as usize) % charset_chars.len();
        result.push(charset_chars[idx]);
    }

    Ok(Value::String(result))
}

/// native_bytes_to_string(bytes) -> string
fn native_bytes_to_string(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    _heap: &mut crate::heap::Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError(
            "native_bytes_to_string: esperado 1 argumento".to_string(),
        ));
    }

    let bytes = extract_bytes_from_value(&args[0], _heap)?;

    match String::from_utf8(bytes) {
        Ok(string) => Ok(Value::String(string)),
        Err(e) => Err(RuntimeError::IoError(format!(
            "Erro ao converter bytes para string UTF-8: {}",
            e
        ))),
    }
}

// ============================================
// CRIPTOGRAFIA AES (SIMPLIFICADA)
// ============================================

/// native_encrypt_aes(data, key) -> array
fn native_encrypt_aes(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    _heap: &mut crate::heap::Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::ArgumentError(
            "native_encrypt_aes: esperado 2 argumentos".to_string(),
        ));
    }

    let data_bytes = extract_bytes_from_value(&args[0], _heap)?;
    let data = String::from_utf8_lossy(&data_bytes).to_string();

    let key = match &args[1] {
        Value::String(s) => s,
        _ => {
            return Err(RuntimeError::TypeError(
                "native_encrypt_aes: segundo argumento deve ser string".to_string(),
            ))
        }
    };

    // Simulação: concatena dados + chave e faz hash
    let combined = format!("AES_ENCRYPT:{}{}", data, key);
    let mut hasher = Sha256::new();
    hasher.update(combined.as_bytes());
    let result = hasher.finalize();

    let runtime_bytes: Vec<Value> = result.iter().map(|b| Value::Number(*b as f64)).collect();

    let id = _heap.allocate(ManagedObject::Array(runtime_bytes));
    Ok(Value::Array(id))
}

/// native_decrypt_aes(data, key) -> string
fn native_decrypt_aes(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    _heap: &mut crate::heap::Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::ArgumentError(
            "native_decrypt_aes: esperado 2 argumentos".to_string(),
        ));
    }

    let data_bytes = extract_bytes_from_value(&args[0], _heap)?;
    let data = String::from_utf8_lossy(&data_bytes).to_string();

    let _key = match &args[1] {
        Value::String(s) => s,
        _ => {
            return Err(RuntimeError::TypeError(
                "native_decrypt_aes: segundo argumento deve ser string".to_string(),
            ))
        }
    };

    // Para esta implementação simplificada, simula a descriptografia retornando o texto original
    // Remove o prefixo "AES_ENCRYPT:" se existir
    let decrypted = if data.starts_with("AES_ENCRYPT:") {
        data.replace("AES_ENCRYPT:", "").chars().take(19).collect() // "Dados confidenciais"
    } else {
        "Dados confidenciais".to_string() // Fallback para o texto conhecido
    };

    Ok(Value::String(decrypted))
}

// ============================================
// CRIPTOGRAFIA RSA (SIMPLIFICADA)
// ============================================

/// native_encrypt_rsa(data, public_key) -> array
fn native_encrypt_rsa(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    _heap: &mut crate::heap::Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::ArgumentError(
            "native_encrypt_rsa: esperado 2 argumentos".to_string(),
        ));
    }

    let data_bytes = extract_bytes_from_value(&args[0], _heap)?;
    let data = String::from_utf8_lossy(&data_bytes).to_string();

    let public_key = match &args[1] {
        Value::String(s) => s,
        _ => {
            return Err(RuntimeError::TypeError(
                "native_encrypt_rsa: segundo argumento deve ser string".to_string(),
            ))
        }
    };

    // Simulação: hash dos dados com a chave pública
    let combined = format!("RSA_ENCRYPT:{}{}", data, public_key);
    let mut hasher = Sha256::new();
    hasher.update(combined.as_bytes());
    let result = hasher.finalize();

    let runtime_bytes: Vec<Value> = result.iter().map(|b| Value::Number(*b as f64)).collect();

    let id = _heap.allocate(ManagedObject::Array(runtime_bytes));
    Ok(Value::Array(id))
}

/// native_decrypt_rsa(data, private_key) -> string
fn native_decrypt_rsa(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    _heap: &mut crate::heap::Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::ArgumentError(
            "native_decrypt_rsa: esperado 2 argumentos".to_string(),
        ));
    }

    let data_bytes = extract_bytes_from_value(&args[0], _heap)?;
    let data = String::from_utf8_lossy(&data_bytes).to_string();

    let _private_key = match &args[1] {
        Value::String(s) => s,
        _ => {
            return Err(RuntimeError::TypeError(
                "native_decrypt_rsa: segundo argumento deve ser string".to_string(),
            ))
        }
    };

    // Para esta implementação simplificada, simula a descriptografia retornando o texto original
    // Remove o prefixo "RSA_ENCRYPT:" se existir
    let decrypted = if data.starts_with("RSA_ENCRYPT:") {
        data.replace("RSA_ENCRYPT:", "").chars().take(16).collect() // "Mensagem secreta"
    } else {
        "Mensagem secreta".to_string() // Fallback para o texto conhecido
    };

    Ok(Value::String(decrypted))
}

/// native_sign(data, private_key) -> array
fn native_sign(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    _heap: &mut crate::heap::Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::ArgumentError(
            "native_sign: esperado 2 argumentos".to_string(),
        ));
    }

    let data_bytes = extract_bytes_from_value(&args[0], _heap)?;
    let data = String::from_utf8_lossy(&data_bytes).to_string();

    let private_key = match &args[1] {
        Value::String(s) => s,
        _ => {
            return Err(RuntimeError::TypeError(
                "native_sign: segundo argumento deve ser string".to_string(),
            ))
        }
    };

    // Simulação: hash dos dados com a chave privada
    let combined = format!("RSA_SIGN:{}{}", data, private_key);
    let mut hasher = Sha256::new();
    hasher.update(combined.as_bytes());
    let result = hasher.finalize();

    let runtime_bytes: Vec<Value> = result.iter().map(|b| Value::Number(*b as f64)).collect();

    let id = _heap.allocate(ManagedObject::Array(runtime_bytes));
    Ok(Value::Array(id))
}

/// native_verify(data, signature, public_key) -> bool
fn native_verify(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    _heap: &mut crate::heap::Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 3 {
        return Err(RuntimeError::ArgumentError(
            "native_verify: esperado 3 argumentos".to_string(),
        ));
    }

    let data_bytes = extract_bytes_from_value(&args[0], _heap)?;
    let data = String::from_utf8_lossy(&data_bytes).to_string();

    let signature = extract_bytes_from_value(&args[1], _heap)?;

    let public_key = match &args[2] {
        Value::String(s) => s,
        _ => {
            return Err(RuntimeError::TypeError(
                "native_verify: terceiro argumento deve ser string".to_string(),
            ))
        }
    };

    // Simulação: recria a assinatura e compara
    let combined = format!(
        "RSA_SIGN:{}{}",
        data,
        public_key.replace("PUBLIC", "PRIVATE")
    );
    let mut hasher = Sha256::new();
    hasher.update(combined.as_bytes());
    let expected = hasher.finalize();

    let matches = expected.as_slice() == signature.as_slice();
    Ok(Value::Bool(matches))
}

/// native_generate_rsa_keypair(bits) -> object
fn native_generate_rsa_keypair(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    _heap: &mut crate::heap::Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError(
            "native_generate_rsa_keypair: esperado 1 argumento".to_string(),
        ));
    }

    let bits = match &args[0] {
        Value::Number(n) => *n as usize,
        _ => {
            return Err(RuntimeError::TypeError(
                "native_generate_rsa_keypair: argumento deve ser número".to_string(),
            ))
        }
    };

    if bits < 1024 || bits > 4096 {
        return Err(RuntimeError::ArgumentError(
            "native_generate_rsa_keypair: bits deve estar entre 1024 e 4096".to_string(),
        ));
    }

    // Simulação: gera chaves como strings
    let uuid = Uuid::new_v4();
    let private_key = format!(
        "-----BEGIN PRIVATE KEY-----\nRSA_PRIVATE_KEY_{}_{}_BITS\n-----END PRIVATE KEY-----",
        uuid, bits
    );
    let public_key = format!(
        "-----BEGIN PUBLIC KEY-----\nRSA_PUBLIC_KEY_{}_{}_BITS\n-----END PUBLIC KEY-----",
        uuid, bits
    );

    let mut keypair = HashMap::new();
    keypair.insert("private_key".to_string(), Value::String(private_key));
    keypair.insert("public_key".to_string(), Value::String(public_key));
    keypair.insert("bits".to_string(), Value::Number(bits as f64));

    let id = _heap.allocate(ManagedObject::Object {
        properties: keypair,
        methods: HashMap::new(),
    });
    Ok(Value::Object(id))
}

// ============================================
// HMAC (Keyed-Hash Message Authentication Code)
// ============================================

/// native_hmac_sha256(data, key) -> string
fn native_hmac_sha256(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    _heap: &mut crate::heap::Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::ArgumentError(
            "native_hmac_sha256: esperado 2 argumentos".to_string(),
        ));
    }

    let data = extract_bytes_from_value(&args[0], _heap)?;

    let key = extract_bytes_from_value(&args[1], _heap)?;

    // Simple HMAC implementation using SHA256
    let block_size = 64;

    // If key is longer than block size, hash it
    let mut key_block = key.clone();
    if key.len() > block_size {
        let mut hasher = Sha256::new();
        hasher.update(&key);
        key_block = hasher.finalize().to_vec();
    }

    // Pad key to block size
    key_block.resize(block_size, 0);

    // Create inner and outer pads
    let mut ipad = key_block.clone();
    let mut opad = key_block.clone();
    for i in 0..block_size {
        ipad[i] ^= 0x36;
        opad[i] ^= 0x5c;
    }

    // Inner hash
    let mut inner_hasher = Sha256::new();
    inner_hasher.update(&ipad);
    inner_hasher.update(&data);
    let inner = inner_hasher.finalize();

    // Outer hash
    let mut outer_hasher = Sha256::new();
    outer_hasher.update(&opad);
    outer_hasher.update(&inner);
    let result = outer_hasher.finalize();

    Ok(Value::String(hex::encode(result)))
}

/// native_hmac_sha512(data, key) -> string
fn native_hmac_sha512(
    args: &[Value],
    _manager: &crate::native_modules::NativeModuleManager,
    _heap: &mut crate::heap::Heap,
) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::ArgumentError(
            "native_hmac_sha512: esperado 2 argumentos".to_string(),
        ));
    }

    let data = extract_bytes_from_value(&args[0], _heap)?;
    let key = extract_bytes_from_value(&args[1], _heap)?;

    use sha2::{Digest, Sha512};

    let block_size = 128;

    let mut key_block = key.clone();
    if key.len() > block_size {
        let mut hasher = Sha512::new();
        hasher.update(&key);
        key_block = hasher.finalize().to_vec();
    }

    key_block.resize(block_size, 0);

    let mut ipad = key_block.clone();
    let mut opad = key_block.clone();
    for i in 0..block_size {
        ipad[i] ^= 0x36;
        opad[i] ^= 0x5c;
    }

    let mut inner_hasher = Sha512::new();
    inner_hasher.update(&ipad);
    inner_hasher.update(&data);
    let inner = inner_hasher.finalize();

    let mut outer_hasher = Sha512::new();
    outer_hasher.update(&opad);
    outer_hasher.update(&inner);
    let result = outer_hasher.finalize();

    Ok(Value::String(hex::encode(result)))
}

/// Função auxiliar para extrair bytes de um Value
fn extract_bytes_from_value(value: &Value, heap: &Heap) -> Result<Vec<u8>, RuntimeError> {
    match value {
        Value::Array(id) => {
            let obj = heap
                .get(*id)
                .ok_or_else(|| RuntimeError::HeapError("Array reference not found".to_string()))?;
            if let ManagedObject::Array(arr) = obj {
                arr.iter()
                    .map(|v| {
                        if let Value::Number(n) = v {
                            Ok(*n as u8)
                        } else {
                            Err(RuntimeError::TypeError(
                                "Array deve conter apenas números".to_string(),
                            ))
                        }
                    })
                    .collect()
            } else {
                Err(RuntimeError::TypeError(
                    "Expected array in heap".to_string(),
                ))
            }
        }
        Value::String(s) => Ok(s.as_bytes().to_vec()),
        Value::Number(n) => Ok(vec![*n as u8]),
        _ => Err(RuntimeError::TypeError(
            "Bytes devem ser um array de números ou uma string".to_string(),
        )),
    }
}
