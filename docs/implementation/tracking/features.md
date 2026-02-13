---
title: "Funcionalidades Futuras"
description: "Roadmap de novas features e melhorias planejadas para o Dryad."
category: "Projeto"
order: 2
---

# Features e Roadmap (Future Work)

## 1. Melhorias na Linguagem (Language Improvements)

### 1.1 Pattern Matching (Match Expression)

**Descrição**: Implementar uma expressão `match` poderosa inspirada em Rust/Elixir.
**Requisitos**:

- [ ] Sintaxe: `match expr { pat1 => block, pat2 => block }`.
- [ ] Patterns: Literais, Variáveis, Arrays, Objetos, Tuplas.
- [ ] Guards: `pat if condition`.
      **DoD (Definition of Done)**:
- Parser suporta palavra-chave `match` e estrutura de blocos.
- Runtime avalia padrões e executa o braço correto.
- Testes unitários cobrindo todos os tipos de patterns.

### 1.2 Sistema de Tipos Opcional (Gradual Typing)

**Descrição**: Permitir anotações de tipo opcionais para análise estática e otimização.
**Requisitos**:

- [ ] Sintaxe: `function add(a: number, b: number): number`.
- [ ] Checker: Ferramenta CLI `dryad check` valida tipos.
- [ ] Runtime: Ignora tipos em execução (type erasure) ou valida em debug.
      **DoD**:
- Parser aceita anotações `: Type`.
- AST armazena informações de tipo.
- Passagem de verificação de tipos implementada.

### 1.3 Garbage Collection (GC)

**Descrição**: Substituir ou complementar `Rc<RefCell>` com um GC real (Mark-and-Sweep) para lidar com ciclos de referência.
**Requisitos**:

- [ ] Heap gerenciado centralizado.
- [ ] Tracing de raízes (stack, globais).
- [ ] Coletor incremental para evitar pausas longas.
      **DoD**:
- Implementação de algoritmo Mark-and-Sweep.
- Testes de stress criando ciclos de objetos e verificando liberação de memória.

## 2. Ecossistema e Ferramentas

### 2.1 Package Manager (Seed)

**Descrição**: Criar gerenciador de pacotes oficial `seed`.
**Requisitos**:

- [ ] Manifesto `dryad.toml`.
- [ ] Registro central de pacotes.
- [ ] Resolução de dependências (instalação em `dryad_modules/`).
      **DoD**:
- Comando `seed install <pkg>`.
- Comando `seed publish`.
- Versionamento semântico suportado.

### 2.2 Standard Library 2.0

**Descrição**: Expandir a stdlib para cobrir casos de uso comuns de backend.
**Requisitos**:

- [ ] **JSON Stream**: Parser incremental.
- [ ] **Crypto**: Suporte a RSA, AES, HMAC.
- [ ] **Database**: Drivers nativos para SQLite, PostgreSQL (via FFI).
- [ ] **WebSockets**: Cliente e Servidor.
      **DoD**:
- Módulos implementados em Rust e expostos via `NativeModule`.
- Documentação completa em `technical_docs/stdlib`.

### 2.3 FFI (Foreign Function Interface)

**Descrição**: Permitir carregar bibliotecas dinâmicas (`.so`, `.dll`) escritas em C/Rust.
**Requisitos**:

- [ ] API `dlopen` / `dlsym`.
- [ ] Conversão de tipos Dryad <-> C ABI.
      **DoD**:
- Exemplo funcional chamando `libc` (ex: `printf`).

### 2.4 Interface de Debug para IDEs

**Descrição**: Adicionar uma interface para execução de compilação em modo debug, permitindo que IDEs tenham acesso ao sistema e recolham informações internas como valores de variáveis, tokens, heap do GC, além de suportar criação de breakpoints e re-execução a partir de um ponto específico.
**Requisitos**:

- [ ] Protocolo de comunicação (ex: via socket ou API REST) para integração com IDEs.
- [ ] Acesso em tempo real a valores de variáveis durante execução.
- [ ] Inspeção do heap do Garbage Collector (GC) para análise de memória.
- [ ] Suporte a criação e gerenciamento de breakpoints.
- [ ] Capacidade de re-execução (re-run) a partir de um ponto específico no código.
      **DoD**:
- IDEs podem conectar-se ao runtime Dryad em modo debug.
- Ferramentas de debug (como VS Code extension) podem inspecionar estado interno.
- Testes de integração com pelo menos uma IDE popular.

## 3. Performance

### 3.1 JIT Compilation (Just-In-Time)

**Descrição**: Compilar bytecode ou AST quente para código de máquina nativo.
**Requisitos**:

- [ ] Profiling de funções quentes.
- [ ] Backend LLVM ou Cranelift.
      **DoD**:
- Benchmark `end_to_end` 10x mais rápido que o interpretador atual.

### 3.2 Otimizações de AST

**Descrição**: Passagem de otimização antes da execução.
**Requisitos**:

- [ ] Constant Folding (`2 + 2` -> `4`).
- [ ] Dead Code Elimination.
      **DoD**:
- Árvore AST reduzida após otimização.
