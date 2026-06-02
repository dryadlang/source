---
title: "Rede UDP"
description: "Comunicação via datagramas UDP para cliente e servidor."
category: "Bibliotecas Padrão"
subcategory: "Rede"
order: 34
---

# Networking (UDP Datagrams)

Interface para comunicação em rede utilizando o protocolo UDP (User Datagram Protocol), ideal para aplicações que requerem baixa latência e podem tolerar perda eventual de pacotes.

## 🚀 Leitura Rápida

- **Server**: Crie servidores que escutam datagramas em uma porta específica.
- **Client**: Envie e receba dados sem a necessidade de manter uma conexão persistente.
- **Utilitários**: Resolução de nomes e detecção de IP local.
- **Ativação**: Requer `#<udp>`.

---

## Referência de Funções

### Servidor UDP

- `udp_server_create(id, host?, port?)`: Configura um novo servidor UDP.
- `udp_server_start(id)`: Inicia a thread do servidor para processar pacotes.
- `udp_server_stop(id)`: Interrompe o servidor.
- `udp_server_status(id)`: Retorna o status e endereço de bind.

### Cliente UDP

- `udp_client_create(id, host, port)`: Configura um cliente para um destino padrão.
- `udp_client_bind(id, local_port?)`: Vincula o cliente a uma porta local (necessário para receber).
- `udp_client_send(id, message)`: Envia uma mensagem para o host/porta configurado.
- `udp_client_receive(id)`: Recebe dados do socket.
- `udp_client_send_to(id, message, host, port)`: Envia dados para um destino específico.
- `udp_client_receive_from(id)`: Recebe dados e o endereço do remetente.

---

## Exemplo de Uso

```dryad
#<udp>

// Cliente simples
udp_client_create("meu_udp", "127.0.0.1", 9000);
udp_client_bind("meu_udp");

udp_client_send("meu_udp", "Ping!");
let resp = udp_client_receive("meu_udp");
println("Resposta: " + resp);
```
