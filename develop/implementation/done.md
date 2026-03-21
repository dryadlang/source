---
title: "Tarefas Concluídas"
description: "Histórico de implementações concluídas e milestones atingidos"
category: "Desenvolvimento"
order: 3
---

# Tarefas Concluídas (DONE)

Este documento registra todas as funcionalidades implementadas, tarefas técnicas finalizadas e marcos atingidos no projeto Dryad.

## Changelog Recente

### [v1.3.0] - 2026-02-18

**Language Syntax & Oak Enhancements (Fase 4)**

- [x] **Default Parameters**: Suporte a valores padrões em funções e métodos (`fn foo(x = 10)`).
- [x] **Variadic Functions**: Suporte a argumentos variáveis com `...args`.
- [x] **Spread Operator**: Expansão de arrays e argumentos em chamadas (`...arr`).
- [x] **Pattern Matching V2**: Suporte a `...rest` em destructuring de arrays.
- [x] **Oak Package Manager**:
  - Validação rigorosa de **Checksums/Integridade**.
  - **Lockfile Determinístico** e reprodutível.
  - Resolução de dependências com **Semantic Versioning**.

### [v1.2.0] - 2026-02-18

**Standard Library & Language Features (Fase 3)**

- [x] **PostgreSQL**: Bindings reais usando `tokio-postgres` (substituindo mocks).
- [x] **WebSocket Server**: Suporte nativo para criação de servidores WS.
- [x] **HTTP Server**: Handlers dinâmicos (lambdas) e execução segura via polling.
- [x] **Static Analysis**: `TypeChecker` com suporte a Classes e Interfaces.
- [x] **Error System**: Implementação do tipo `Result` e operador `?`.
- [x] **Namespaces**: Organização de código com a palavra-chave `namespace`.

### [v1.1.0] - 2026-02-16

**Bytecode VM & Estabilidade**

- [x] **Bytecode VM**: Máquina virtual baseada em pilha completa (69+ opcodes).
- [x] **Performance**: Ganho de 2-3x em relação ao interpretador AST.
- [x] **Portabilidade**: 100% Rust, compatível com x86 e ARM.
- [x] **Segurança**: Sandbox de execução e proteção contra Stack Overflow (Recursion Limit).
- [x] **FFI (Foreign Function Interface)**: Suporte básico para carregamento de bibliotecas dinâmicas (`.so`/`.dll`) via `libloading`.
- [x] **Módulos Nativos**: Implementação estável de `console_io`, `file_io` (async/sync), `time`, `crypto`, `json_stream` e `system_env`.
- [x] **Static Checker (v0.1)**: Motor de verificação de tipos básico para variáveis, funções e expressões.

### [v1.0.0] - 2026-01-15

**Core da Linguagem Estável**

- [x] **Sintaxe**: Variáveis, loops, funções, classes, closures, template strings.
- [x] **Estruturas**: Arrays (33+ métodos), Mapas (Objetos), Tuplas.
- [x] **Patterns**: Destructuring básico e Pattern Matching (v1).
- [x] **Oak PM**: Sistema de pacotes básico com suporte a dependências.

---

## Funcionalidades Implementadas (Checklist)

### 1. Core Language & Parser

- [x] **Lexer/Parser**: Suporte a tokens UTF-8, Pratt Parser para expressões.
- [x] **Optimizer**: Constant folding e simplificação de expressões no nível de AST.
- [x] **Patterns**: Destructuring em `let/const` e patterns em `match`.
- [x] **Erro System**: 60+ códigos de erro documentados (1xxx-9xxx).

### 2. Runtime & VM

- [x] **Bytecode VM**: Executor eficiente com frames de chamada e proteção de aridade.
- [x] **Heap & GC**: Gerenciamento automático de memória (Mark-and-Sweep).
- [x] **Exception Handling**: Sistema `try/catch/finally` integrado ao bytecode.
- [x] **Async/Await**: Suporte básico para operações não bloqueantes.

### 3. Biblioteca Padrão (Native Modules)

- [x] **I/O**: `console_io`, `file_io`, `terminal_ansi`, `binary_io`.
- [x] **Data**: `json_stream` (incremental), `encode_decode` (base64, hex).
- [x] **Utils**: `crypto` (sha256), `time` (timestamp, formatting), `uuid`.
- [x] **FFI**: Chamadas nativas em C (`i32`, `f64`, `string`, `pointer`).

### 4. Ferramentas

- [x] **Oak Package Manager**: Init, install, run, registry access.
- [x] **Dryad Checker**: Verificação estática de tipos para variáveis e funções.
- [x] **Benchmark Suite**: Ferramentas de medição de performance comparativa.

---

## Projetos Técnicos Concluídos

- [x] **Modularização do Interpretador**: Separação de `Environment` e `NativeRegistry`.
- [x] **Sandbox de Segurança**: Proteção de acesso ao host configurável.
- [x] **Sistema de Erros v1**: Centralização de mensagens em `dryad_errors`.

---

## Métricas e Qualidade

- **Status**: Production Ready (v1.1)
- **Testes**: 100% de passagem nos testes de regressão (Bytecode & AST).
- **Cobertura**: Foco em segurança e core do runtime.

---

## Lições Aprendidas

- **Borrow Checker**: O uso de `Rc<RefCell<V>>` foi essencial para o heap flexível, mas exige cuidado com ciclos.
- **Bytecode**: Simplificou drasticamente a implementação de closures e recursão em comparação ao AST walk.
- **FFI**: A segurança é o maior desafio ao integrar código externo.

_Última atualização: 18 de fevereiro de 2026_
