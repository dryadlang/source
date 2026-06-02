# 🧪 Testes das Funcionalidades Web - Linguagem Dryad

Este diretório contém uma suíte completa de testes para validar todas as funcionalidades web implementadas na linguagem Dryad.

## 📁 Arquivos de Teste

### 1. `test_web_features.dryad` - Teste Básico Completo
**Objetivo**: Demonstração básica de todas as funcionalidades web implementadas.

**Cobre**:
- ✅ **DataStructures**: HashMap, Stack, Queue, Set
- ✅ **HTTP**: GET, POST, PUT, DELETE com headers e JSON
- ✅ **WebSocket**: Conexão, envio, recebimento, ping/pong
- ✅ **TCP**: Cliente/servidor, envio/recebimento de dados
- ✅ **UDP**: Socket, broadcast, multicast
- ✅ **SystemWeb**: Verificação de portas, DNS, ping, interfaces
- ✅ **WebServer**: Criação, rotas, middleware, request/response

**Como executar**:
```bash
dryad run tests/test_web_features.dryad
```

---

### 2. `test_webserver_api.dryad` - API REST Completa
**Objetivo**: Demonstração detalhada de uma API REST real usando o WebServer.

**Características**:
- 🌐 **API de Usuários**: CRUD completo
- 🔐 **Autenticação**: Login/logout
- 📁 **Arquivos Estáticos**: CSS, HTML, assets
- ⚡ **WebSocket**: Notificações em tempo real
- 🛡️ **Middlewares**: CORS, JSON parser, autenticação
- ❌ **Tratamento de Erros**: Responses 404, 500, etc.

**Rotas Testadas**:
- `GET /api/users` - Listar usuários
- `POST /api/users` - Criar usuário
- `PUT /api/users/:id` - Atualizar usuário
- `DELETE /api/users/:id` - Deletar usuário
- `POST /api/auth/login` - Login
- `GET /ws/notifications` - WebSocket upgrade

**Como executar**:
```bash
dryad run tests/test_webserver_api.dryad
```

---

### 3. `test_chat_integration.dryad` - Sistema Integrado de Chat
**Objetivo**: Teste de integração completo simulando um sistema de chat em tempo real.

**Arquitetura Testada**:
1. **🔍 Verificação de Sistema**: Conectividade, portas, interfaces
2. **📡 Descoberta UDP**: Broadcast para encontrar usuários
3. **💬 Chat TCP**: Servidor dedicado para mensagens privadas
4. **🔄 API REST**: Gestão de salas e usuários
5. **⚡ WebSocket**: Chat em tempo real
6. **🔔 Notificações**: Sistema multi-canal
7. **📊 Monitoramento**: Estatísticas e health checks
8. **🧹 Cleanup**: Fechamento limpo de recursos

**Fluxo de Teste**:
```
Verificação → Descoberta → TCP Server → API REST → WebSocket → Notificações → Monitoramento → Cleanup
```

**Como executar**:
```bash
dryad run tests/test_chat_integration.dryad
```

---

### 4. `test_performance_stress.dryad` - Teste de Performance
**Objetivo**: Validação de performance e limites das implementações.

**Testes de Carga**:
- 🏗️ **DataStructures**: 100 HashMap + 200 Stack + 150 Queue operações
- 🌐 **HTTP**: 50 requisições simultâneas (GET/POST/PUT/DELETE)
- ⚡ **WebSocket**: 20 conexões + 100 mensagens
- 🔌 **TCP/UDP**: 15+30 conexões + 135 mensagens
- 🖥️ **WebServer**: 100 rotas + 200 requisições
- 🌍 **SystemWeb**: 50 operações de sistema
- 💾 **Memória**: 1500 itens grandes

**Métricas Coletadas**:
- ⏱️ Tempo de execução por módulo
- 💾 Uso de memória
- 🚀 Throughput de operações
- 📊 Relatório de performance detalhado

**Como executar**:
```bash
dryad run tests/test_performance_stress.dryad
```

---

## 🎯 Casos de Uso Cobertos

### 1. **Aplicações Web Modernas**
- SPA (Single Page Applications)
- APIs RESTful
- Microservices
- WebSocket para real-time

### 2. **Sistemas de Rede**
- Chat applications
- IoT device discovery
- P2P communication
- Network monitoring

### 3. **Serviços Backend**
- HTTP servers
- TCP/UDP services
- File serving
- API gateways

---

## 📊 Estatísticas dos Testes

| Arquivo | Linhas | Funções Testadas | Complexidade |
|---------|--------|------------------|--------------|
| `test_web_features.dryad` | 200+ | 80+ | Básica |
| `test_webserver_api.dryad` | 180+ | 20+ | Alta |
| `test_chat_integration.dryad` | 300+ | 60+ | Muito Alta |
| `test_performance_stress.dryad` | 400+ | 80+ | Extrema |

**Total**: 1000+ linhas cobrindo todas as 80+ funções implementadas.

---

## 🚀 Como Executar os Testes

### Execução Individual
```bash
# Teste básico
dryad run tests/test_web_features.dryad

# API REST
dryad run tests/test_webserver_api.dryad

# Sistema integrado
dryad run tests/test_chat_integration.dryad

# Performance
dryad run tests/test_performance_stress.dryad
```

### Execução em Lote
```bash
# Executar todos os testes
for test in tests/test_*.dryad; do
    echo "Executando $test..."
    dryad run "$test"
    echo "---"
done
```

### Com Output Detalhado
```bash
# Com debug
dryad run --debug tests/test_web_features.dryad

# Com profiling
dryad run --profile tests/test_performance_stress.dryad
```

---

## 📋 Checklist de Validação

Após executar os testes, verifique:

### ✅ **Funcionalidades Básicas**
- [ ] DataStructures funcionam corretamente
- [ ] HTTP requests retornam responses válidas
- [ ] WebSocket conecta e envia mensagens
- [ ] TCP/UDP estabelecem conexões
- [ ] SystemWeb retorna dados de rede
- [ ] WebServer responde a todas as rotas

### ✅ **Integração**
- [ ] Múltiplos protocolos funcionam juntos
- [ ] Upgrade HTTP → WebSocket funciona
- [ ] Discovery UDP → Chat TCP integra
- [ ] API REST + WebSocket cooperam

### ✅ **Performance**
- [ ] Tempos de execução aceitáveis
- [ ] Memória não explode com carga
- [ ] Múltiplas conexões simultâneas
- [ ] Throughput adequado

### ✅ **Robustez**
- [ ] Cleanup de recursos funciona
- [ ] Tratamento de erros adequado
- [ ] Não há vazamentos de memória
- [ ] Sistema permanece estável

---

## 🔧 Configuração de Ambiente

### Dependências
- Dryad runtime com módulos nativos
- Portas 3000-9999 disponíveis para testes
- Conectividade de rede local

### Variáveis de Ambiente
```bash
export DRYAD_DEBUG=1          # Habilitar debug
export DRYAD_PROFILE=1        # Habilitar profiling
export DRYAD_NET_TIMEOUT=5000 # Timeout de rede em ms
```

---

## 📈 Interpretando Resultados

### ✅ **Sucesso**
- Todos os prints aparecem na ordem correta
- Não há mensagens de erro
- Tempos de performance razoáveis
- Cleanup bem-sucedido

### ❌ **Falha**
- Erros de tipo ou argumentos
- Timeouts de rede
- Vazamentos de memória
- Crashes do runtime

### ⚠️ **Warnings**
- Performance abaixo do esperado
- Recursos não liberados
- Conexões perdidas

---

## 🎉 Conclusão

Esta suíte de testes comprova que a linguagem Dryad possui um conjunto robusto e completo de funcionalidades web, pronto para desenvolvimento de aplicações modernas de rede e web.

**Características Validadas**:
- 🌐 **Protocolos**: HTTP, WebSocket, TCP, UDP
- 🗂️ **Estruturas**: HashMap, Stack, Queue, Set
- 🖥️ **Servidor**: WebServer completo com routing
- 🌍 **Sistema**: Networking e conectividade
- ⚡ **Performance**: Adequada para produção
- 🔧 **Integração**: Todos os módulos cooperam

**A linguagem Dryad está pronta para construir o futuro da web!** 🚀
