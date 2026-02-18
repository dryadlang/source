---
title: "Cliente HTTP"
description: "Realização de requisições web síncronas e assíncronas."
category: "Bibliotecas Padrão"
subcategory: "HTTP"
order: 1
---

# HTTP Client

A biblioteca HTTP do Dryad permite## Referência de Funções

### `native_http_get(url: string, headers?: object): string`

Executa uma requisição GET e retorna o corpo da resposta como string.

### `native_http_post(url: string, body: string, headers?: object): string`

Envia uma requisição POST com o corpo fornecido.

### `native_http_json(url: string): any`

Realiza um GET e decodifica automaticamente o JSON da resposta para uma estrutura Dryad.

### `native_http_headers(url: string): object`

Retorna um objeto contendo todos os cabeçalhos (headers) da resposta.

### `native_http_download(url: string, path: string): void`

Baixa o conteúdo da URL diretamente para o arquivo especificado no `path`.

### `native_http_status(url: string): number`

Retorna apenas o código de status HTTP (ex: 200, 404).

---

## Configuração Global

O Dryad permite configurar o comportamento do cliente HTTP por URL usando funções `native_http_set_*`.

- **Timeout**: `native_http_set_timeout(url, ms)`
- **Headers**: `native_http_set_headers(url, { "Auth": "..." })`
- **Proxy**: `native_http_set_proxy(url, "http://proxy:8080")`
- **Auth**: `native_http_set_auth(url, "user", "pass")`
- **SSL**: `native_http_set_ssl_verify(url, bool)`

---

## Exemplo de Uso

```dryad
#<http_client>

// Configura um timeout de 5 segundos
native_http_set_timeout("https://api.github.com", 5000);

let perfil = native_http_json("https://api.github.com/users/dryadlang");
println("Seguidores: " + perfil.followers);
```

a integração com APIs web e serviços remotos utilizando protocolos modernos e seguros.

## 🚀 Leitura Rápida

- **Simples**: Funções `get`, `post` e `download` prontas para uso.
- **Seguro**: Suporte a HTTPS (TLS 1.2/1.3) via Rustls nativo.
- **Padronizado**: Segue as especificações HTTP/1.1 e HTTP/2.

---

## ⚙️ Visão Técnica

O cliente HTTP é uma camada fina de abstração sobre a crate **Reqwest** do ecossistema Rust, conhecida por sua segurança e velocidade.

### 1. Motor de Requisição

Internamente, o runtime mantém um `reqwest::blocking::Client` reutilizável para aproveitar o **Connection Pooling** (reuso de conexões TCP abertas para o mesmo host).

### 2. Segurança de Conexão (TLS)

O Dryad prefere o **Rustls** em vez de OpenSSL por ser uma implementação 100% Rust, eliminando vulnerabilidades de C e facilitando o deploy cross-platform (sem dependência de DLLs de sistema).

### 3. Sincronia e Threads

As chamadas HTTP são bloqueantes para a fibra/thread atual. Se você precisa fazer múltiplas chamadas simultâneas, deve utilizar o sistema de concorrência nativo da linguagem:

```dryad
// Exemplo de requisições paralelas
thread function buscarDados() {
    let res = http_get("https://api.exemplo.com/dados");
    println(res);
}

buscarDados(); // Executa em paralelo
```

---

## 📚 Referências e Paralelos

- **Crate Base**: [Reqwest Documentation](https://docs.rs/reqwest/latest/reqwest/).
- **Network Stack**: [Hyper (HTTP implementation for Rust)](https://hyper.rs/).
- **Standards**: [RFC 9110 (HTTP Semantics)](https://www.rfc-editor.org/rfc/rfc9110.html).

---

## Exemplo de Uso

### `http_get(url: string): string`

```dryad
let perfil = http_get("https://api.github.com/users/dryadlang");
println(perfil);
```

### `http_download(url: string, path: string)`

Ideal para downloads de arquivos binários. O runtime gerencia o stream de bytes para o disco de forma eficiente.
