---
title: "Rede TCP"
description: "Comunicação via sockets TCP para cliente e servidor."
category: "Bibliotecas Padrão"
subcategory: "Rede"
order: 32
---

# Networking (TCP Sockets)

Interface para comunicação em rede utilizando o protocolo TCP.

> [!NOTE]
> **Status Atual: Funcional e Otimizado**.
> Tanto o cliente quanto o servidor TCP estão operacionais. O cliente agora suporta conexões persistentes para maior eficiência.

## Referência de Funções

### Servidor TCP

- `tcp_server_create(id, host?, port?, max_clients?)`: Registra um novo servidor.
- `tcp_server_start(id)`: Inicia a escuta por conexões.
- `tcp_server_stop(id)`: Encerra o servidor.
- `tcp_server_status(id)`: Retorna objeto com status e porta.

### Cliente TCP

- `tcp_client_create(id, host, port)`: Instancia um cliente.
- `tcp_client_connect(id)`: Abre a conexão física.
- `tcp_client_send(id, data: string)`: Envia dados.
- `tcp_client_receive(id)`: Lê dados recebidos (até 4KB).
- `tcp_client_disconnect(id)`: Fecha a conexão.

---

## Exemplo de Uso (Cliente)

```dryad
#<tcp>

tcp_client_create("meu_cliente", "google.com", 80);
if (tcp_client_connect("meu_cliente")) {
    tcp_client_send("meu_cliente", "GET / HTTP/1.1\r\nHost: google.com\r\n\r\n");
    let resp = tcp_client_receive("meu_cliente");
    println(resp);
    tcp_client_disconnect("meu_cliente");
}
```
