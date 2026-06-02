# 📡 Especificações de Native Functions - Networking Modules

**Data:** 26 de setembro de 2025  
**Versão:** 1.0  

---

## 📡 WebSocket (Cliente/Servidor) `#<websocket>`

### 🔌 Cliente WebSocket

```dryad
native_ws_connect(url);                 // Conecta a servidor WebSocket
/*
Conecta a um servidor WebSocket na URL especificada.
Entrada: uma string representando a URL do WebSocket (ws:// ou wss://).
Retorna: um número inteiro representando o ID da conexão WebSocket.
*/

native_ws_send(socket_id, message);     // Envia mensagem
/*
Envia uma mensagem através da conexão WebSocket especificada.
Entrada: um número inteiro (ID da conexão) e uma string com a mensagem.
Retorna: null
*/

native_ws_recv(socket_id);              // Recebe mensagem (não-bloqueante)
/*
Recebe uma mensagem da conexão WebSocket especificada (operação não-bloqueante).
Entrada: um número inteiro representando o ID da conexão.
Retorna: uma string com a mensagem recebida ou null se não houver mensagens.
*/

native_ws_recv_blocking(socket_id, timeout_ms); // Recebe mensagem (bloqueante)
/*
Recebe uma mensagem da conexão WebSocket com timeout (operação bloqueante).
Entrada: um número inteiro (ID da conexão) e um número inteiro (timeout em ms).
Retorna: uma string com a mensagem recebida ou null em caso de timeout.
*/

native_ws_close(socket_id);             // Fecha conexão
/*
Fecha uma conexão WebSocket específica.
Entrada: um número inteiro representando o ID da conexão.
Retorna: null
*/

native_ws_is_connected(socket_id);      // Verifica status da conexão
/*
Verifica se uma conexão WebSocket está ativa.
Entrada: um número inteiro representando o ID da conexão.
Retorna: um booleano (true se conectado, false caso contrário).
*/

native_ws_get_state(socket_id);         // Obtém estado da conexão
/*
Obtém o estado atual da conexão WebSocket.
Entrada: um número inteiro representando o ID da conexão.
Retorna: uma string com o estado ("connecting", "open", "closing", "closed").
*/

native_ws_ping(socket_id);              // Envia ping
/*
Envia um frame de ping através da conexão WebSocket.
Entrada: um número inteiro representando o ID da conexão.
Retorna: null
*/

native_ws_pong(socket_id);              // Envia pong
/*
Envia um frame de pong através da conexão WebSocket.
Entrada: um número inteiro representando o ID da conexão.
Retorna: null
*/

native_ws_send_binary(socket_id, data); // Envia dados binários
/*
Envia dados binários através da conexão WebSocket.
Entrada: um número inteiro (ID da conexão) e um array de bytes.
Retorna: null
*/

native_ws_recv_binary(socket_id);       // Recebe dados binários
/*
Recebe dados binários da conexão WebSocket.
Entrada: um número inteiro representando o ID da conexão.
Retorna: um array de bytes ou null se não houver dados.
*/
```

### 🖥️ Servidor WebSocket

```dryad
native_ws_listen(port);                 // Inicia servidor WebSocket
/*
Inicia um servidor WebSocket na porta especificada.
Entrada: um número inteiro representando a porta.
Retorna: um número inteiro representando o ID do servidor.
*/

native_ws_accept(server_id);            // Aceita conexão de cliente
/*
Aceita uma nova conexão de cliente no servidor WebSocket.
Entrada: um número inteiro representando o ID do servidor.
Retorna: um número inteiro representando o ID da conexão do cliente ou null se não houver conexões pendentes.
*/

native_ws_broadcast(server_id, message); // Envia mensagem para todos os clientes
/*
Envia uma mensagem para todos os clientes conectados ao servidor.
Entrada: um número inteiro (ID do servidor) e uma string com a mensagem.
Retorna: um número inteiro representando quantos clientes receberam a mensagem.
*/

native_ws_broadcast_except(server_id, except_id, message); // Broadcast exceto um cliente
/*
Envia uma mensagem para todos os clientes exceto um específico.
Entrada: um número inteiro (ID do servidor), um número inteiro (ID do cliente a excluir) e uma string com a mensagem.
Retorna: um número inteiro representando quantos clientes receberam a mensagem.
*/

native_ws_broadcast_binary(server_id, data); // Broadcast de dados binários
/*
Envia dados binários para todos os clientes conectados.
Entrada: um número inteiro (ID do servidor) e um array de bytes.
Retorna: um número inteiro representando quantos clientes receberam os dados.
*/

native_ws_get_clients(server_id);       // Lista clientes conectados
/*
Obtém uma lista de todos os clientes conectados ao servidor.
Entrada: um número inteiro representando o ID do servidor.
Retorna: um array de números inteiros representando os IDs dos clientes conectados.
*/

native_ws_client_info(client_id);       // Informações do cliente
/*
Obtém informações sobre um cliente específico.
Entrada: um número inteiro representando o ID do cliente.
Retorna: um objeto com informações do cliente (IP, porta, etc.).
*/

native_ws_kick_client(client_id);       // Desconecta cliente específico
/*
Força a desconexão de um cliente específico.
Entrada: um número inteiro representando o ID do cliente.
Retorna: null
*/

native_ws_stop_server(server_id);       // Para o servidor
/*
Para o servidor WebSocket e desconecta todos os clientes.
Entrada: um número inteiro representando o ID do servidor.
Retorna: null
*/
```

### ⚙️ Configurações WebSocket

```dryad
native_ws_set_timeout(socket_id, ms);   // Define timeout
/*
Define o timeout para operações WebSocket.
Entrada: um número inteiro (ID da conexão/servidor) e um número inteiro (timeout em ms).
Retorna: null
*/

native_ws_set_keepalive(socket_id, enable); // Ativa/desativa keepalive
/*
Ativa ou desativa o keepalive da conexão WebSocket.
Entrada: um número inteiro (ID da conexão) e um booleano.
Retorna: null
*/

native_ws_set_max_frame_size(socket_id, size); // Define tamanho máximo do frame
/*
Define o tamanho máximo de frame WebSocket.
Entrada: um número inteiro (ID da conexão) e um número inteiro (tamanho em bytes).
Retorna: null
*/

native_ws_set_compression(socket_id, enable); // Ativa/desativa compressão
/*
Ativa ou desativa a compressão de mensagens WebSocket.
Entrada: um número inteiro (ID da conexão) e um booleano.
Retorna: null
*/

native_ws_set_headers(socket_id, headers); // Define headers customizados
/*
Define headers HTTP personalizados para o handshake WebSocket.
Entrada: um número inteiro (ID da conexão) e um objeto com os headers.
Retorna: null
*/

native_ws_set_subprotocol(socket_id, protocol); // Define subprotocolo
/*
Define o subprotocolo WebSocket a ser usado.
Entrada: um número inteiro (ID da conexão) e uma string com o nome do protocolo.
Retorna: null
*/
```

---

## 🌍 TCP (Cliente e Servidor) `#<tcp>`

### 🔌 Cliente TCP

```dryad
native_tcp_connect(host, port);         // Conecta a servidor TCP
/*
Conecta a um servidor TCP no host e porta especificados.
Entrada: uma string representando o host e um número inteiro representando a porta.
Retorna: um número inteiro representando o ID da conexão TCP.
*/

native_tcp_send(socket_id, data);       // Envia dados
/*
Envia dados através da conexão TCP especificada.
Entrada: um número inteiro (ID da conexão) e uma string ou array de bytes.
Retorna: um número inteiro representando quantos bytes foram enviados.
*/

native_tcp_recv(socket_id, size);       // Recebe dados (não-bloqueante)
/*
Recebe dados da conexão TCP (operação não-bloqueante).
Entrada: um número inteiro (ID da conexão) e um número inteiro (máximo de bytes a receber).
Retorna: uma string com os dados recebidos ou null se não houver dados.
*/

native_tcp_recv_blocking(socket_id, size, timeout_ms); // Recebe dados (bloqueante)
/*
Recebe dados da conexão TCP com timeout (operação bloqueante).
Entrada: um número inteiro (ID da conexão), um número inteiro (bytes a receber) e um número inteiro (timeout em ms).
Retorna: uma string com os dados recebidos ou null em caso de timeout.
*/

native_tcp_recv_all(socket_id);         // Recebe todos os dados disponíveis
/*
Recebe todos os dados disponíveis na conexão TCP.
Entrada: um número inteiro representando o ID da conexão.
Retorna: uma string com todos os dados disponíveis.
*/

native_tcp_recv_until(socket_id, delimiter); // Recebe até delimitador
/*
Recebe dados até encontrar um delimitador específico.
Entrada: um número inteiro (ID da conexão) e uma string com o delimitador.
Retorna: uma string com os dados recebidos incluindo o delimitador.
*/

native_tcp_close(socket_id);            // Fecha conexão
/*
Fecha uma conexão TCP específica.
Entrada: um número inteiro representando o ID da conexão.
Retorna: null
*/

native_tcp_is_connected(socket_id);     // Verifica status da conexão
/*
Verifica se uma conexão TCP está ativa.
Entrada: um número inteiro representando o ID da conexão.
Retorna: um booleano (true se conectado, false caso contrário).
*/

native_tcp_get_peer_addr(socket_id);    // Obtém endereço do peer
/*
Obtém o endereço IP e porta do peer conectado.
Entrada: um número inteiro representando o ID da conexão.
Retorna: um objeto com propriedades "ip" e "port".
*/

native_tcp_get_local_addr(socket_id);   // Obtém endereço local
/*
Obtém o endereço IP e porta local da conexão.
Entrada: um número inteiro representando o ID da conexão.
Retorna: um objeto com propriedades "ip" e "port".
*/
```

### 🖥️ Servidor TCP

```dryad
native_tcp_listen(port);                // Inicia servidor TCP
/*
Inicia um servidor TCP na porta especificada.
Entrada: um número inteiro representando a porta.
Retorna: um número inteiro representando o ID do servidor.
*/

native_tcp_bind(ip, port);              // Bind em IP específico
/*
Faz bind do servidor TCP em um IP específico e porta.
Entrada: uma string representando o IP e um número inteiro representando a porta.
Retorna: um número inteiro representando o ID do servidor.
*/

native_tcp_accept(server_id);           // Aceita conexão de cliente
/*
Aceita uma nova conexão de cliente no servidor TCP.
Entrada: um número inteiro representando o ID do servidor.
Retorna: um número inteiro representando o ID da conexão do cliente ou null se não houver conexões pendentes.
*/

native_tcp_accept_blocking(server_id, timeout_ms); // Aceita conexão (bloqueante)
/*
Aceita uma nova conexão com timeout (operação bloqueante).
Entrada: um número inteiro (ID do servidor) e um número inteiro (timeout em ms).
Retorna: um número inteiro representando o ID da conexão ou null em caso de timeout.
*/

native_tcp_get_clients(server_id);      // Lista clientes conectados
/*
Obtém uma lista de todos os clientes conectados ao servidor.
Entrada: um número inteiro representando o ID do servidor.
Retorna: um array de números inteiros representando os IDs dos clientes conectados.
*/

native_tcp_broadcast(server_id, data);  // Envia dados para todos os clientes
/*
Envia dados para todos os clientes conectados ao servidor.
Entrada: um número inteiro (ID do servidor) e uma string ou array de bytes.
Retorna: um número inteiro representando quantos clientes receberam os dados.
*/

native_tcp_stop_server(server_id);      // Para o servidor
/*
Para o servidor TCP e fecha todas as conexões.
Entrada: um número inteiro representando o ID do servidor.
Retorna: null
*/

native_tcp_kick_client(client_id);      // Desconecta cliente específico
/*
Força a desconexão de um cliente específico.
Entrada: um número inteiro representando o ID do cliente.
Retorna: null
*/

native_tcp_set_backlog(server_id, backlog); // Define tamanho da fila de conexões
/*
Define o tamanho máximo da fila de conexões pendentes.
Entrada: um número inteiro (ID do servidor) e um número inteiro (tamanho da fila).
Retorna: null
*/
```

### ⚙️ Configurações TCP

```dryad
native_tcp_set_timeout(socket_id, ms);  // Define timeout
/*
Define o timeout para operações TCP.
Entrada: um número inteiro (ID da conexão/servidor) e um número inteiro (timeout em ms).
Retorna: null
*/

native_tcp_set_nodelay(socket_id, enable); // Ativa/desativa Nagle's algorithm
/*
Ativa ou desativa o algoritmo de Nagle (TCP_NODELAY).
Entrada: um número inteiro (ID da conexão) e um booleano.
Retorna: null
*/

native_tcp_set_keepalive(socket_id, enable); // Ativa/desativa keepalive
/*
Ativa ou desativa o keepalive da conexão TCP.
Entrada: um número inteiro (ID da conexão) e um booleano.
Retorna: null
*/

native_tcp_set_reuseaddr(socket_id, enable); // Ativa/desativa reuseaddr
/*
Ativa ou desativa a reutilização de endereço (SO_REUSEADDR).
Entrada: um número inteiro (ID da conexão/servidor) e um booleano.
Retorna: null
*/

native_tcp_set_reuseport(socket_id, enable); // Ativa/desativa reuseport
/*
Ativa ou desativa a reutilização de porta (SO_REUSEPORT).
Entrada: um número inteiro (ID da conexão/servidor) e um booleano.
Retorna: null
*/

native_tcp_set_linger(socket_id, enable, timeout); // Configura linger
/*
Configura o comportamento de linger para a conexão TCP.
Entrada: um número inteiro (ID da conexão), um booleano (ativar) e um número inteiro (timeout em segundos).
Retorna: null
*/

native_tcp_set_recv_buffer_size(socket_id, size); // Define tamanho do buffer de recepção
/*
Define o tamanho do buffer de recepção TCP.
Entrada: um número inteiro (ID da conexão) e um número inteiro (tamanho em bytes).
Retorna: null
*/

native_tcp_set_send_buffer_size(socket_id, size); // Define tamanho do buffer de envio
/*
Define o tamanho do buffer de envio TCP.
Entrada: um número inteiro (ID da conexão) e um número inteiro (tamanho em bytes).
Retorna: null
*/

native_tcp_get_recv_buffer_size(socket_id); // Obtém tamanho do buffer de recepção
/*
Obtém o tamanho atual do buffer de recepção TCP.
Entrada: um número inteiro representando o ID da conexão.
Retorna: um número inteiro representando o tamanho do buffer em bytes.
*/

native_tcp_get_send_buffer_size(socket_id); // Obtém tamanho do buffer de envio
/*
Obtém o tamanho atual do buffer de envio TCP.
Entrada: um número inteiro representando o ID da conexão.
Retorna: um número inteiro representando o tamanho do buffer em bytes.
*/
```

---

## 🌐 UDP (Datagramas) `#<udp>`

### 📡 Cliente/Servidor UDP

```dryad
native_udp_socket();                    // Cria socket UDP
/*
Cria um novo socket UDP.
Entrada: nenhuma.
Retorna: um número inteiro representando o ID do socket UDP.
*/

native_udp_bind(socket_id, port);       // Faz bind em porta específica
/*
Faz bind do socket UDP em uma porta específica.
Entrada: um número inteiro (ID do socket) e um número inteiro (porta).
Retorna: null
*/

native_udp_bind_addr(socket_id, ip, port); // Faz bind em IP e porta específicos
/*
Faz bind do socket UDP em um IP e porta específicos.
Entrada: um número inteiro (ID do socket), uma string (IP) e um número inteiro (porta).
Retorna: null
*/

native_udp_send(socket_id, data, host, port); // Envia datagrama
/*
Envia um datagrama UDP para o host e porta especificados.
Entrada: um número inteiro (ID do socket), uma string ou array de bytes (dados), uma string (host) e um número inteiro (porta).
Retorna: um número inteiro representando quantos bytes foram enviados.
*/

native_udp_recv(socket_id);             // Recebe datagrama (não-bloqueante)
/*
Recebe um datagrama UDP (operação não-bloqueante).
Entrada: um número inteiro representando o ID do socket.
Retorna: um objeto com propriedades "data" (string), "ip" (string) e "port" (número) ou null se não houver dados.
*/

native_udp_recv_blocking(socket_id, timeout_ms); // Recebe datagrama (bloqueante)
/*
Recebe um datagrama UDP com timeout (operação bloqueante).
Entrada: um número inteiro (ID do socket) e um número inteiro (timeout em ms).
Retorna: um objeto com propriedades "data", "ip" e "port" ou null em caso de timeout.
*/

native_udp_recv_from(socket_id, max_size); // Recebe com tamanho máximo
/*
Recebe um datagrama UDP com tamanho máximo especificado.
Entrada: um número inteiro (ID do socket) e um número inteiro (tamanho máximo em bytes).
Retorna: um objeto com propriedades "data", "ip" e "port".
*/

native_udp_close(socket_id);            // Fecha socket
/*
Fecha um socket UDP específico.
Entrada: um número inteiro representando o ID do socket.
Retorna: null
*/

native_udp_connect(socket_id, host, port); // Conecta socket (para envio sem especificar destino)
/*
"Conecta" o socket UDP a um host e porta específicos para simplificar envios subsequentes.
Entrada: um número inteiro (ID do socket), uma string (host) e um número inteiro (porta).
Retorna: null
*/

native_udp_send_connected(socket_id, data); // Envia usando conexão estabelecida
/*
Envia dados através de um socket UDP "conectado".
Entrada: um número inteiro (ID do socket) e uma string ou array de bytes.
Retorna: um número inteiro representando quantos bytes foram enviados.
*/
```

### 📢 Broadcast e Multicast UDP

```dryad
native_udp_broadcast(socket_id, data, port); // Envia broadcast
/*
Envia um datagrama UDP em broadcast para a rede local.
Entrada: um número inteiro (ID do socket), uma string ou array de bytes (dados) e um número inteiro (porta).
Retorna: um número inteiro representando quantos bytes foram enviados.
*/

native_udp_set_broadcast(socket_id, enable); // Ativa/desativa broadcast
/*
Ativa ou desativa a capacidade de broadcast do socket UDP.
Entrada: um número inteiro (ID do socket) e um booleano.
Retorna: null
*/

native_udp_join_multicast(socket_id, group_ip); // Entra em grupo multicast
/*
Faz o socket UDP entrar em um grupo multicast.
Entrada: um número inteiro (ID do socket) e uma string (IP do grupo multicast).
Retorna: null
*/

native_udp_leave_multicast(socket_id, group_ip); // Sai do grupo multicast
/*
Faz o socket UDP sair de um grupo multicast.
Entrada: um número inteiro (ID do socket) e uma string (IP do grupo multicast).
Retorna: null
*/

native_udp_multicast_send(socket_id, data, group_ip, port); // Envia multicast
/*
Envia um datagrama UDP para um grupo multicast.
Entrada: um número inteiro (ID do socket), uma string ou array de bytes (dados), uma string (IP do grupo) e um número inteiro (porta).
Retorna: um número inteiro representando quantos bytes foram enviados.
*/

native_udp_set_multicast_ttl(socket_id, ttl); // Define TTL multicast
/*
Define o Time To Live (TTL) para pacotes multicast.
Entrada: um número inteiro (ID do socket) e um número inteiro (TTL).
Retorna: null
*/

native_udp_set_multicast_loop(socket_id, enable); // Ativa/desativa loop multicast
/*
Ativa ou desativa o loopback de pacotes multicast.
Entrada: um número inteiro (ID do socket) e um booleano.
Retorna: null
*/
```

### ⚙️ Configurações UDP

```dryad
native_udp_set_timeout(socket_id, ms);  // Define timeout
/*
Define o timeout para operações UDP.
Entrada: um número inteiro (ID do socket) e um número inteiro (timeout em ms).
Retorna: null
*/

native_udp_set_recv_buffer_size(socket_id, size); // Define tamanho do buffer de recepção
/*
Define o tamanho do buffer de recepção UDP.
Entrada: um número inteiro (ID do socket) e um número inteiro (tamanho em bytes).
Retorna: null
*/

native_udp_set_send_buffer_size(socket_id, size); // Define tamanho do buffer de envio
/*
Define o tamanho do buffer de envio UDP.
Entrada: um número inteiro (ID do socket) e um número inteiro (tamanho em bytes).
Retorna: null
*/

native_udp_get_recv_buffer_size(socket_id); // Obtém tamanho do buffer de recepção
/*
Obtém o tamanho atual do buffer de recepção UDP.
Entrada: um número inteiro representando o ID do socket.
Retorna: um número inteiro representando o tamanho do buffer em bytes.
*/

native_udp_get_send_buffer_size(socket_id); // Obtém tamanho do buffer de envio
/*
Obtém o tamanho atual do buffer de envio UDP.
Entrada: um número inteiro representando o ID do socket.
Retorna: um número inteiro representando o tamanho do buffer em bytes.
*/

native_udp_set_reuseaddr(socket_id, enable); // Ativa/desativa reuseaddr
/*
Ativa ou desativa a reutilização de endereço (SO_REUSEADDR).
Entrada: um número inteiro (ID do socket) e um booleano.
Retorna: null
*/

native_udp_set_reuseport(socket_id, enable); // Ativa/desativa reuseport
/*
Ativa ou desativa a reutilização de porta (SO_REUSEPORT).
Entrada: um número inteiro (ID do socket) e um booleano.
Retorna: null
*/

native_udp_get_local_addr(socket_id);   // Obtém endereço local
/*
Obtém o endereço IP e porta local do socket.
Entrada: um número inteiro representando o ID do socket.
Retorna: um objeto com propriedades "ip" e "port".
*/

native_udp_get_stats(socket_id);        // Obtém estatísticas do socket
/*
Obtém estatísticas de uso do socket UDP.
Entrada: um número inteiro representando o ID do socket.
Retorna: um objeto com estatísticas (bytes enviados, recebidos, pacotes perdidos, etc.).
*/
```

---

## 📋 Resumo das Native Functions

| Módulo | Functions Cliente | Functions Servidor | Functions Configuração | Total |
|--------|------------------|-------------------|----------------------|-------|
| **WebSocket** | 11 | 9 | 6 | **26** |
| **TCP** | 10 | 9 | 10 | **29** |
| **UDP** | 11 | 0 | 9 | **20** |
| | | | **TOTAL** | **75** |

---

## 🎯 Prioridade de Implementação

### **Fase 1 - Básico** (Core functionality)
1. **TCP**: `native_tcp_connect`, `native_tcp_listen`, `native_tcp_send`, `native_tcp_recv`, `native_tcp_close`, `native_tcp_accept`
2. **UDP**: `native_udp_socket`, `native_udp_bind`, `native_udp_send`, `native_udp_recv`, `native_udp_close`
3. **WebSocket**: `native_ws_connect`, `native_ws_send`, `native_ws_recv`, `native_ws_close`, `native_ws_listen`, `native_ws_accept`

### **Fase 2 - Intermediário** (Enhanced functionality)
1. Operações bloqueantes com timeout
2. Configurações básicas (timeout, keepalive, nodelay)
3. Informações de conexão (peer address, local address)

### **Fase 3 - Avançado** (Advanced features)
1. Broadcast e multicast UDP
2. Configurações avançadas de socket
3. Estatísticas e monitoramento
4. Funcionalidades específicas de WebSocket (ping/pong, binary data)