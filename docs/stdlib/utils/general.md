---
title: "Utilidades Gerais"
description: "JSON, Random, Base64 e outras ferramentas utilitárias."
category: "Bibliotecas Padrão"
subcategory: "Utilitários"
order: 1
---

# Utilidades Gerais (Utils)

O módulo `utils` contém funções transversais para manipulação de dados, tempo e segurança.

## 🚀 Leitura Rápida

- **JSON**: Serialização e Deserialização nativa.
- **Tempo**: Pausa de execução controlada.
- **Cripto**: Hashing SHA256 e UUID v4.
- **Encoding**: Suporte total a Base64.

---

## ⚙️ Visão Técnica

Este módulo integra algumas das crates mais estáveis do ecossistema Rust no runtime do Dryad.

### 1. Serialização JSON (`serde`)

A função `json_stringify` converte a árvore de `Value` do Dryad para o formato JSON.

- **Tipos Mapeados**: `Value::Number` → JSON Number, `Value::Object` → JSON Object, etc.
- **Performance**: Utiliza a crate **Serde JSON**, garantindo uma das implementações mais rápidas e seguras do mercado.

### 2. Funções de Tempo e Bloqueio

`sleep(ms)` suspende a execução. No modelo de interpretador do Dryad:

- **Blocking**: Na thread nativa, utiliza `std::thread::sleep`.
- **Efeito Fibra**: Diferente de `async sleep` em JS, o `sleep` do Dryad é síncrono para a unidade de execução atual, ideal para scripts e automações simples.

### 3. Criptografia e Identificadores

- **SHA256**: Utiliza a crate `sha2` para hashing determinístico de strings.
- **UUID**: Gera identificadores únicos universais seguindo a RFC 4122 via crate `uuid`.

---

## 📚 Referências e Paralelos

- **JSON Engine**: [Serde JSON Crate](https://docs.rs/serde_json/latest/serde_json/).
- **Crypto**: [RustCrypto Project](https://github.com/RustCrypto).
- **Standards**: [RFC 4122 (UUID)](https://www.rfc-editor.org/rfc/rfc4122.html).

---

## Funções em Destaque

### `json_parse(json_string: string): any`

Transforma uma string JSON em uma estrutura viva no Dryad (Arrays, Objetos, etc).

### `random(): number`

Gera um valor de ponto flutuante pseudo-aleatório utilizando o gerador interno do interpretador (baseado na crate `rand`).
