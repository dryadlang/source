---
title: "Criptografia"
description: "Funções de hash, criptografia, assinaturas digitais e HMAC."
category: "Bibliotecas Padrão"
subcategory: "Segurança"
order: 37
---

# Cryptography and Security

Módulo para operações de segurança, hashing e proteção de dados críticos.

## Referência de Funções

### `sha256(data: string | array): string` (Alias: `native_hash_sha256`)

Gera o hash SHA-256 dos dados e retorna uma string hexadecimal.

### `native_hash_md5(data: string | array): string`

Gera o hash MD5 dos dados.

### `native_uuid(): string`

Gera um Identificador Único Universal (UUID) versão 4.

### `native_base64_encode(data: string | array): string`

Codifica dados para o formato Base64.

### `native_base64_decode(base64: string): string | array`

Decodifica uma string Base64. Retorna String se for UTF-8 válido, caso contrário retorna Array de bytes.

### `native_encrypt_aes(data: string | array, key: string): array`

Criptografa dados usando AES com a chave fornecida. Retorna o ciphertext como Array de bytes.

### `native_generate_rsa_keypair(bits: number): object`

Gera um par de chaves RSA (1024 a 4096 bits). Retorna um objeto com `public_key` e `private_key`.

### `native_sign(data: string | array, private_key: string): array`

Gera uma assinatura digital RSA para os dados.

---

## Exemplo de Uso

```dryad
#<crypto>

let senha = "minha_senha_secreta";
let hash = sha256(senha);
println("Hash: " + hash);

let token = native_uuid();
println("Sessão: " + token);
```
