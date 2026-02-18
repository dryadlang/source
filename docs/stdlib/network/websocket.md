---
title: "WebSocket"
description: "Conexões WebSocket para comunicação bidirecional em tempo real."
category: "Bibliotecas Padrão"
subcategory: "Rede"
order: 33
---

# WebSockets (Real-time Comms)

Interface para comunicação bidirecional de baixa latência através do protocolo WebSocket (RFC 6455).

> [!IMPORTANT]
> **Status Atual: Totalmente Operacional (Cliente & Servidor)**.
> Suporte completo para comunicação WebSocket em ambos os lados.

## Referência de Funções (Cliente)

### `ws_connect(url: string): object`

### `ws_connect(url: string): object`

Abre uma conexão WebSocket e retorna um objeto de conexão.

### `ws_send(id: string, message: string): bool`

Envia uma mensagem de texto para o servidor.

### `ws_receive(id: string): object | null`

Recebe uma mensagem. Retorna um objeto com `{ type: "text" | "binary", data: string | array }` ou `null` se não houver dados.

### `ws_close(id: string): bool`

Encerra a conexão WebSocket.

---

## Referência de Funções (Servidor)

### `ws_server_create(id: string, host: string, port: number): bool`

Cria uma nova instância de servidor WebSocket.

### `ws_server_start(id: string): bool`

Inicia o servidor e começa a escutar conexões.

### `ws_server_stop(id: string): bool`

Encerra o servidor.

### `ws_server_status(id: string): object`

Retorna o status do servidor `{ host, port, running }`.

---

## Exemplo de Uso

```dryad
#<websocket>

let conn = ws_connect("wss://echo.websocket.org");
if (conn) {
    ws_send(conn.id, "Olá Dryad!");
    let msg = ws_receive(conn.id);
    println("Echo: " + msg.data);
    ws_close(conn.id);
}
```
