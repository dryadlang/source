# WebSocket

Módulo para conexões WebSocket cliente e servidor.

## Ativação

```dryad
#<websocket>
```

## Funções de Cliente

### ws_connect(url: string) -> connection

Conecta a um servidor WebSocket.

```dryad
#<websocket>
let conn = ws_connect("ws://localhost:8080/ws");
```

### ws_send(connection: object, message: string) -> boolean

Envia uma mensagem através da conexão WebSocket.

```dryad
#<websocket>
ws_send(conn, "Hello, server!");
```

### ws_receive(connection: object) -> message

Recebe uma mensagem da conexão WebSocket.

```dryad
#<websocket>
let msg = ws_receive(conn);
```

### ws_close(connection: object) -> boolean

Fecha a conexão WebSocket.

```dryad
#<websocket>
ws_close(conn);
```

## Funções de Servidor

### ws_create_server(port: number) -> server

Cria um servidor WebSocket.

```dryad
#<websocket>
let server = ws_create_server(8080);
```

### ws_server_accept(server: object) -> connection

Aceita uma conexão de cliente.

```dryad
#<websocket>
let client = ws_server_accept(server);
```

### ws_server_send(connection: object, message: string) -> boolean

Envia uma mensagem para o cliente.

```dryad
#<websocket>
ws_server_send(client, "Welcome!");
```

### ws_server_receive(connection: object) -> message

Recebe uma mensagem do cliente.

```dryad
#<websocket>
let msg = ws_server_receive(client);
```

## Exemplo Completo

```dryad
#<websocket>

// Servidor WebSocket
let server = ws_create_server(8080);
let client = ws_server_accept(server);

// Enviar e receber mensagens
ws_server_send(client, "Connected!");
let msg = ws_server_receive(client);

ws_close(client);
```
