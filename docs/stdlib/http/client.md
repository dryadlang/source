---
title: "Cliente HTTP"
description: "Realização de requisições web síncronas e assíncronas."
category: "Bibliotecas Padrão"
subcategory: "HTTP"
order: 1
---

# HTTP Client

A biblioteca HTTP do Dryad permite a integração com APIs web e serviços remotos utilizando protocolos modernos e seguros.

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
