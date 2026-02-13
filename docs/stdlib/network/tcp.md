# Networking: TCP

A biblioteca de rede TCP da Dryad permite comunicação eficiente via sockets TCP. Este documento descreve as funções disponíveis e fornece exemplos de uso.

## Cliente

### `tcp_client_connect(host: string, port: number): connection`
Estabelece conexão com um servidor.
- **Retorno**: Um objeto/handle de conexão opaco.

### `tcp_client_send(conn: connection, data: string)`
Envia dados pela conexão estabelecida.

### `tcp_client_receive(conn: connection): string`
Lê dados da conexão (bloqueante).

### `tcp_client_disconnect(conn: connection)`
Fecha a conexão.

## Exemplo de Uso

```dryad
let conn = tcp_client_connect("google.com", 80);
tcp_client_send(conn, "GET / HTTP/1.0\r\n\r\n");
let resp = tcp_client_receive(conn);
println(resp);
tcp_client_disconnect(conn);
```
