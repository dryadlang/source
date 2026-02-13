# 📚 Índice de Exemplos - Linguagem Dryad

Este diretório contém exemplos práticos organizados por categoria, demonstrando as funcionalidades **realmente implementadas** na linguagem Dryad.

---

## 📁 **Estrutura dos Exemplos**

### 🎯 **basic/** - Funcionalidades Básicas
- [`operadores.dryad`](basic/operadores.dryad) - Todos os operadores implementados (aritméticos, lógicos, bitwise)
- [`controle_fluxo.dryad`](basic/controle_fluxo.dryad) - If/else, while, do-while, for (padrão C)
- [`funcoes.dryad`](basic/funcoes.dryad) - Declaração e uso de funções
- [`classes.dryad`](basic/classes.dryad) - Classes, construtores, métodos, herança
- [`constantes.dryad`](basic/constantes.dryad) - Uso de constantes em aplicações
- [`hashmaps.dryad`](basic/hashmaps.dryad) - Estruturas de dados chave-valor

### 🖥️ **console_io/** - Entrada/Saída do Console
- [`entrada_saida.dryad`](console_io/entrada_saida.dryad) - Input/output, timeouts, menus interativos

### 📁 **file_io/** - Manipulação de Arquivos
- [`manipulacao_arquivos.dryad`](file_io/manipulacao_arquivos.dryad) - CRUD de arquivos, diretórios, informações

### 🌐 **http/** - Cliente e Servidor HTTP
- [`cliente_http.dryad`](http/cliente_http.dryad) - GET, POST, headers, download, JSON
- [`servidor_http.dryad`](http/servidor_http.dryad) - Servidor web com rotas e API
- [`blog_server_completo.dryad`](http/blog_server_completo.dryad) - Sistema de blog completo
- [`teste_performance.dryad`](http/teste_performance.dryad) - Testes de performance HTTP

### 🔌 **networking/** - TCP/UDP
- [`cliente_tcp.dryad`](networking/cliente_tcp.dryad) - Cliente TCP, conexões, envio/recebimento
- [`servidor_tcp.dryad`](networking/servidor_tcp.dryad) - Servidor TCP Echo com múltiplos clientes
- [`cliente_servidor_tcp.dryad`](networking/cliente_servidor_tcp.dryad) - Exemplo completo TCP
- [`cliente_servidor_udp.dryad`](networking/cliente_servidor_udp.dryad) - Exemplo completo UDP

### ⚡ **async_threading/** - Programação Assíncrona e Threading
- [`basico_async.dryad`](async_threading/basico_async.dryad) - Introdução a async/await
- [`exemplo_simples.dryad`](async_threading/exemplo_simples.dryad) - Exemplo simples de threading
- [`async_threading_completo.dryad`](async_threading/async_threading_completo.dryad) - Exemplo completo
- [`classes_com_async.dryad`](async_threading/classes_com_async.dryad) - Async em classes
- [`servidor_com_async.dryad`](async_threading/servidor_com_async.dryad) - Servidor com async/threading

---

## 🚀 **Como Executar os Exemplos**

### Pré-requisitos
```bash
# Compilar o projeto Dryad
cargo build --release

# Ou usar o executável diretamente se já compilado
```

### Executar um exemplo
```bash
# Exemplo básico
cargo run --bin dryad run examples/basic/operadores.dryad

# Com output detalhado (tokens + AST)
cargo run --bin dryad run examples/basic/operadores.dryad --verbose

# Verificar sintaxe sem executar
cargo run --bin dryad check examples/basic/operadores.dryad
```

### Exemplos interativos
```bash
# Console I/O (requer input do usuário)
cargo run --bin dryad run examples/console_io/entrada_saida.dryad

# File I/O (criará/manipulará arquivos)
cargo run --bin dryad run examples/file_io/manipulacao_arquivos.dryad
```

### Exemplos de rede (requer conectividade)
```bash
# Cliente HTTP (testa APIs externas)
cargo run --bin dryad run examples/http/cliente_http.dryad

# Cliente TCP (conecta a servidores externos)
cargo run --bin dryad run examples/networking/cliente_tcp.dryad
```

---

## 📋 **Módulos Nativos Utilizados**

| Exemplo | Módulos Requeridos | Funcionalidades |
|---------|-------------------|-----------------|
| `operadores.dryad` | `console_io` | Saída básica |
| `controle_fluxo.dryad` | `console_io` | Loops e condicionais |
| `funcoes.dryad` | `console_io` | Funções e recursão |
| `classes.dryad` | `console_io` | OOP básica |
| `constantes.dryad` | `console_io` | Uso de constantes |
| `hashmaps.dryad` | `console_io` | Estruturas chave-valor |
| `entrada_saida.dryad` | `console_io` | I/O interativo |
| `manipulacao_arquivos.dryad` | `file_io`, `console_io` | Sistema de arquivos |
| `cliente_http.dryad` | `http_client`, `console_io` | Requisições HTTP |
| `servidor_http.dryad` | `http_server`, `console_io` | Servidor web |
| `blog_server_completo.dryad` | `http_server`, `console_io`, `file_io` | Sistema de blog |
| `teste_performance.dryad` | `http_client`, `http_server`, `time` | Performance HTTP |
| `cliente_tcp.dryad` | `tcp`, `console_io` | Networking TCP |
| `servidor_tcp.dryad` | `tcp`, `console_io` | Servidor TCP |
| `cliente_servidor_tcp.dryad` | `tcp`, `console_io`, `time` | TCP completo |
| `cliente_servidor_udp.dryad` | `udp`, `console_io`, `time` | UDP completo |
| `basico_async.dryad` | `console_io` | Async/await básico |
| `exemplo_simples.dryad` | `console_io` | Threading simples |
| `async_threading_completo.dryad` | `console_io` | Async/threading completo |
| `classes_com_async.dryad` | `console_io` | Async em classes |
| `servidor_com_async.dryad` | `http_server`, `console_io` | Servidor com async |

---

## ✅ **Funcionalidades Demonstradas**

### ✅ Implementado e Testado
- [x] Operadores aritméticos avançados (`**`, `%%`, `^^`, `##`)
- [x] Loops com sintaxe C obrigatória (`for (init; condition; update)`)
- [x] Sistema de classes completo
- [x] Módulos nativos (15+ categorias)
- [x] HTTP client/server completo
- [x] TCP/UDP networking
- [x] File I/O robusto
- [x] Console I/O avançado

### 🔄 Sintaxe Específica do Dryad
- [x] Diretivas de módulo: `#<console_io>`, `#<file_io>`, etc.
- [x] Parênteses obrigatórios em loops: `while (condition)`, `for (init; condition; update)`
- [x] Sintaxe de classes: `new ClassName(args)`
- [x] Sistema de erros integrado

---

## 🎓 **Dicas de Aprendizado**

1. **Comece pelos básicos**: Execute primeiro os exemplos em `basic/`
2. **Teste interativamente**: Use `console_io/` para entender input/output
3. **Explore I/O**: `file_io/` mostra manipulação de arquivos
4. **Networking**: `http/` e `networking/` para aplicações em rede
5. **Use --verbose**: Para entender como o parser funciona

---

## 🔧 **Troubleshooting**

### Erro de módulo não encontrado
```bash
# Certifique-se de que os módulos nativos estão carregados corretamente
# Verifique se as diretivas #<module> estão no início do arquivo
```

### Erro de sintaxe em loops
```bash
# Dryad exige parênteses em loops (padrão C):
# ✅ while (condition) { ... }
# ❌ while condition { ... }
```

### Problemas de rede
```bash
# Exemplos HTTP/TCP requerem conectividade à internet
# Verifique firewall e conexão de rede
```

---

**Versão dos exemplos**: v1.0  
**Compatível com**: Dryad v0.1+  
**Última atualização**: 2024