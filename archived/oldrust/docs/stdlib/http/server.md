---
title: "Servidor HTTP"
description: "Criação e gerenciamento de servidores web nativos no Dryad."
category: "Bibliotecas Padrão"
subcategory: "HTTP"
order: 31
---

# HTTP Server

O módulo `http_server` permite criar servidores web robustos, configurar rotas, servir conteúdo estático e gerenciar requisições de forma nativa.

## 🚀 Leitura Rápida

- **Gerenciamento**: Crie, inicie e pare servidores com facilidade.
- **Roteamento**: Suporte a métodos GET, POST, PUT e DELETE.
- **Estáticos**: Sirva arquivos HTML, CSS e JS diretamente do disco.
- **Ativação**: Requer `#<http_server>`.

---

## Referência de Funções

### Gerenciamento de Servidor

- `native_http_server_create(id, host?, port?)`: Registra uma nova instância de servidor.
- `native_http_server_start(id)`: Inicia o servidor em uma thread separada.
- `native_http_server_stop(id)`: Encerra o servidor.
- `native_http_server_status(id)`: Retorna um objeto com `host`, `port` e `running`.

### Roteamento e Respostas (Dinâmico)

- **`native_http_server_handle(id, method, path, lambda)`**: Registra um handler dinâmico (lambda) que recebe `(request)` e retorna uma resposta.

Exemplo de handler dinâmico:

```dryad
native_http_server_handle("meu_app", "GET", "/hello", fn(req) {
    return "Olá " + req.query.name;
});
```

### Roteamento e Respostas (Estático/Fixas)

- `native_http_server_get(id, path, body)`: Define uma rota GET que retorna o corpo fixo.
- `native_http_server_post(id, path, body)`: Define uma rota POST.
- `native_http_server_put(id, path, body)`: Define uma rota PUT.
- `native_http_server_delete(id, path, body)`: Define uma rota DELETE.
- `native_http_server_route(id, method, path, body, status?)`: Define uma rota genérica.

### Conteúdo Especializado

- `native_http_server_html(id, path, html_content)`: Atalho para servir conteúdo HTML.
- `native_http_server_json(id, path, json_content)`: Atalho para servir respostas JSON.
- `native_http_server_static(id, web_path, file_path)`: Mapeia um caminho web para um arquivo físico.
- `native_http_server_file(id, web_path, file_path)`: Alias para `native_http_server_static`.

---

## Exemplo de Uso

```dryad
#<http_server>

native_http_server_create("meu_app", "0.0.0.0", 3000);

// Define rotas
native_http_server_get("meu_app", "/", "<h1>Bem-vindo ao Dryad</h1>");
native_http_server_json("meu_app", "/api/status", "{ \"status\": \"online\" }");

// Inicia o servidor
native_http_server_start("meu_app");

println("Servidor rodando em http://localhost:3000");
```
