---
title: "Tarefas Pendentes"
description: "Lista de tarefas e TODOs para implementação"
category: "Desenvolvimento"
order: 2
---

# Tarefas Pendentes (TODO)

Este documento centraliza todas as tarefas pendentes, oportunidades de refatoração e dívidas técnicas do projeto Dryad.

## Prioridade Alta 🔴

### Oak Package Manager

- [x] **T2.1** Validação de Checksum/Integridade
  - Calcular SHA-256 do arquivo baixado
  - Comparar com hash do registry
  - Abortar se falhar
- [x] **T2.2** Semantic Versioning Real
  - Implementar semver na resolução de dependências
- [x] **T2.3** Lockfile Determinístico
  - Corrigir e melhorar arquivo de lock

### Linguagem e Sintaxe

- [x] **T3.2** Pattern Matching Avançado
  - Guards mais complexos
  - Pattern matching em arrays/tuplas aninhadas
- [x] **T3.3** Destructuring e Spread
  - `let {a, b} = obj`
  - `let [x, y] = arr`
  - `...args` em funções e arrays

### Standard Library (Real Implementation)

- [x] **Database**: Implementar bindings reais para `rusqlite` e `tokio-postgres` (atualmente mocks).
- [x] **WebSockets**: Implementar suporte a protocolos reais (client/server) em `websocket.rs`.
- [x] **Rede (TCP/UDP)**: Finalizar implementações nativas reais em `tcp.rs` e `udp.rs`.
- [x] **HTTP Server**: Completar API pública e suporte a middlewares.

### Dryad Checker (Static Analysis)

- [x] **OOP**: Adicionar verificação de tipos para classes, herança e interfaces.
- [x] **Functions**: Validar assinaturas de funções em chamadas recursivas e lambdas.
- [x] **Generics**: Estudo de viabilidade para tipos genéricos básicos.

### Funcionalidades Faltantes (Missing Features)

- [x] **Sistema de Erros**: Implementar `Result<T, E>` nativo e propagation operator `?`.
- [x] **Módulos**: Melhorar `import` circular e cache de módulos.
- [x] **Namespaces**: Implementar agrupamento de funções/classes em namespaces.

---



---

## Prioridade Média 🟡

### OOP e Classes

- [ ] Modificadores de Acesso (`private`, `protected`, `public`)
  - Parser aceita keywords, mas falta verificação em runtime
- [ ] Getters/Setters (`get prop()`, `set prop(v)`)
- [ ] Propriedades Estáticas (`static prop = val`)

### Funções

- [x] Parâmetros Default (`function foo(x = 10)`)
- [ ] Parâmetros Nomeados (`foo(x: 10)`)
- [ ] Generators (`function*` e `yield`)

### Objetos (Maps)

- [ ] Métodos `.keys()`, `.values()`, `.entries()`

### Sintaxe Adicional

- [ ] **Switch/Match**: Completar `eval_match` e guards no Interpreter.
- [ ] Optional Chaining (`obj?.prop`)
- [ ] Nullish Coalescing (`??`)

### Herança e Composição

- [ ] Validar a implementação atual de herança.
- [ ] Implementar um sistema de composição.

### Refatorações Técnicas (Technical Debt)

- [ ] **Native Modules**: Unificar `http_client.rs` e `http.rs` em `native_modules/http/mod.rs`.
- [ ] **Parser & AST**:
  - Interning de strings (`SmolStr`) para reduzir clones.
  - Boxear variantes grandes do enum `Expr`.
  - Implementar Pratt Parser para expressões.
- [ ] **Lexer**: Migrar para `Chars` iterator ou biblioteca `logos`.
- [ ] **CLI**: Criar `Runner` struct para evitar duplicação de setup.

---

## Prioridade Baixa 🟢

### I/O e Rede

- [ ] TCP Server (`bind`, `listen`, `accept`)
- [ ] UDP Support: Implementar sockets reais em `udp.rs`.
- [ ] File Streams (leitura/escrita bufferizada)

### Sistema

- [ ] Process Management (`fork`, `kill`, sinais)

### Tooling

- [ ] Oak Publish Command
  - Autenticação com Token
  - Empacotar em `.tar.gz`
  - Upload via HTTP POST
- [ ] Language Server Protocol (LSP)
- [ ] Debugger Interativo (breakpoints)
- [ ] Profiler

### AOT (Compilação Nativa)

**Status:** 🚧 Em desenvolvimento (≈55%) — consulte `develop-manual/aot/` para especificações técnicas.

#### Conversor e IR

- [ ] **Instruções**: Implementar suporte a todas as instruções do bytecode no conversor IR.
- [ ] **Funções**: Conversão de múltiplas funções e tratamento de variáveis locais.
- [ ] **Otimizações**: Implementar SSA (Static Single Assignment) e Phi nodes completos.

#### Backends e Formatos

- [ ] **Backend x86_64**: Resolução de labels, suporte a mais instruções e calling conventions.
- [ ] **Gerador ELF (Linux)**: Section headers, tabela de símbolos e relocações.
- [ ] **Gerador PE (Windows)**: Implementar headers (DOS, COFF, Optional), seção de imports e section table.
- [ ] **Backend ARM64**: Iniciar implementação do stub para suporte a Apple Silicon e Raspberry Pi.

#### Runtime e Tooling

- [ ] **Runtime em C**: Desenvolver biblioteca mínima com funções de I/O e alocação de memória.
- [ ] **Milestone 1**: Atingir o "Hello World" nativo totalmente funcional (ELF).

---

## Áreas de Risco e Segurança (Danger Zones)

Trechos de código que representam riscos de segurança, performance ou estabilidade.

### Runtime (`crates/dryad_runtime/src/interpreter.rs`)

- [ ] **Stack Overflow**: O interpretador é recursivo (AST Walk).
  - _Risco_: Crash da aplicação host em recursões profundas.
  - _Solução_: Implementar verificação de profundidade ou mudar para execução iterativa.

### Segurança (`crates/dryad_runtime/src/native_modules/file_io.rs`)

- [ ] **Acesso ao Filesystem**: O script pode acessar qualquer diretório do host.
  - _Risco_: Leitura/escrita de arquivos sensíveis.
  - _Solução_: Implementar sistema de permissões ou chroot/jail.

### Concorrência (`crates/dryad_runtime/src/environment.rs`)

- [ ] **Thread-Safety**: Uso massivo de `Rc<RefCell<T>>`.
  - _Risco_: Incompatibilidade com sistemas multi-thread.
  - _Solução_: Migrar para `Arc<Mutex<T>>` ou implementar Actor Model.

### Lexer (`crates/dryad_lexer/src/lexer.rs`)

- [ ] **Unsafe Indexing**: Indexação direta de bytes em strings UTF-8.
  - _Risco_: Panic ao processar caracteres multi-byte.
  - _Solução_: Usar `.chars()` ou iteradores seguros.

---

## Código Ignorado / Mockups (Stubs)

- **UDP/TCP**: `crates/dryad_runtime/src/native_modules/udp.rs` / `tcp.rs` (Parciais).
- **WebSockets**: `crates/dryad_runtime/src/native_modules/websocket.rs` (Mock).
- **Databases**: `crates/dryad_runtime/src/native_modules/database.rs` (Mock).
- **Legacy**: Remover `crates/dryad_runtime/src/native_functions_legacy.rs.bak`.

---

## Guidelines de Refatoração

1. **Nunca quebre o build**: Sempre mantenha o código compilando.
2. **Testes primeiro**: Garanta cobertura antes de refatorar.
3. **Commits pequenos**: Mudanças incrementais e documentadas.
4. **Code review**: Obrigatório para todas as refatorações.
5. **Padrões**: Use `cargo fmt` e `cargo clippy`.

---

## Notas

- Siga a ordem de prioridade para evitar bloqueios.
- Marque como concluído movendo para `done.md`.
- Atualize este documento conforme novas tarefas surgirem.
