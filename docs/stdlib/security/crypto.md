# Crypto

Módulo de criptografia e utilitários de segurança.

## Ativação

```dryad
#<crypto>
```

## Funções de Hash

### sha256(data: string | bytes) -> string

Gera hash SHA-256.

```dryad
#<crypto>
let hash = sha256("Hello, World!");
```

### native_hash_md5(data: string | bytes) -> string

Gera hash MD5.

```dryad
#<crypto>
let hash = native_hash_md5("Hello");
```

## Codificação

### native_base64_encode(data: string | bytes) -> string

Codifica dados para Base64.

### native_base64_decode(data: string) -> bytes

Decodifica dados de Base64.

### native_hex_encode(data: string | bytes) -> string

Codifica dados para formato hexadecimal.

### native_hex_decode(data: string) -> bytes

Decodifica dados de formato hexadecimal.

## Números Aleatórios

### native_random_bytes(length: number) -> bytes

Gera bytes aleatórios.

```dryad
#<crypto>
let bytes = native_random_bytes(32);
```

### native_random_string(length: number) -> string

Gera uma string aleatória.

```dryad
#<crypto>
let str = native_random_string(16);
```

### native_uuid() -> string

Gera um UUID v4.

```dryad
#<crypto>
let id = native_uuid();
```

## Criptografia

### native_encrypt_aes(data: bytes, key: string) -> bytes

Criptografa dados usando AES.

### native_decrypt_aes(data: bytes, key: string) -> string

Descriptografa dados AES.

### native_encrypt_rsa(data: bytes, public_key: string) -> bytes

Criptografa dados usando RSA.

### native_decrypt_rsa(data: bytes, private_key: string) -> string

Descriptografa dados RSA.

## Assinaturas Digitais

### native_sign(data: bytes, private_key: string) -> bytes

Assina dados com chave privada RSA.

```dryad
#<crypto>
let signature = native_sign(data, private_key);
```

### native_verify(data: bytes, signature: bytes, public_key: string) -> boolean

Verifica uma assinatura digital.

```dryad
#<crypto>
let valid = native_verify(data, signature, public_key);
```

### native_generate_rsa_keypair(bits: number) -> object

Gera um par de chaves RSA.

```dryad
#<crypto>
let keys = native_generate_rsa_keypair(2048);
```

## HMAC

### native_hmac_sha256(data: string | bytes, key: string | bytes) -> string

Gera HMAC-SHA256.

```dryad
#<crypto>
let hmac = native_hmac_sha256("message", "secret_key");
```

### native_hmac_sha512(data: string | bytes, key: string | bytes) -> string

Gera HMAC-SHA512.

```dryad
#<crypto>
let hmac = native_hmac_sha512("message", "secret_key");
```

## Exemplo Completo

```dryad
#<crypto>

// Hash
let hash = sha256("password");

// UUID
let id = native_uuid();

// Gerar chaves RSA
let keys = native_generate_rsa_keypair(2048);

// Assinar e verificar
let data = "Important message";
let signature = native_sign(data, keys.private_key);
let valid = native_verify(data, signature, keys.public_key);

// HMAC
let hmac = native_hmac_sha256("message", "secret");
```
