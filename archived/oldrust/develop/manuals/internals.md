---
title: "Funcionamento Interno"
description: "Pipeline do compilador, M-N threads e gestão de memória avançada."
category: "Desenvolvimento"
order: 67
---

# Funcionamento Interno do Dryad

Este documento mergulha nos detalhes técnicos da arquitetura da linguagem Dryad, focando em como ela gerencia recursos, executa instruções e orquestra a concorrência.

## 🚀 Leitura Rápida

- **Pipeline**: Lexer (Tokens) → Parser (AST) → Interpreter (Tree-Walk).
- **Memória**: Gestão de Heap via Mark-and-Sweep Garbage Collector (GC).
- **Paralelismo**: M-N Scheduling (Milhares de fibras em poucas threads de sistema).
- **Extensível**: Sistema de módulos nativos via FFI com Rust e ativação estrita.
- **Arquitetura**: Interpretador modularizado com separação de Ambiente e Registro Nativo.

---

## ⚙️ Visão Técnica

### 1. Pipeline de Execução

O Dryad utiliza um interpretador de AST (Abstract Syntax Tree) otimizado, focado em portabilidade e facilidade de depuração.

1.  **Lexer**: Máquina de estados DFA para scan de tokens. Implementa proteção contra indexação insegura (out-of-bounds).
2.  **Parser**: Recursive Descent com Pratt Parsing para precedência de operadores.
3.  **Runtime**: Executor Tree-Walking que alterna entre métodos `execute` (para `Stmt`) e `evaluate` (para `Expr`).

### 2. Gestão de Memória e Garbage Collection

Diferente das versões iniciais que usavam apenas `Rc/Arc`, o Dryad implementa um **Garbage Collector Mark-and-Sweep** para gerenciar o Heap, permitindo ciclos de referência e controle fino de memória.

#### 2.1 Estrutura do Heap

O `Heap` centraliza todos os objetos gerenciados (`Array`, `Object`, `Tuple`, `Instance`, `Closure`). Cada objeto é identificado por um `HeapId` (usize).

#### 2.2 Ciclo do GC

O GC é acionado automaticamente baseado em um limite de alocações (`gc_threshold`).

- **Trigger**: Por padrão, o GC é disparado a cada 1000 alocações.
- **Fase de Mark**: O interpretador identifica os "Roots" (variáveis globais, stack de chamadas, constantes, classes). O GC percorre recursivamente todos os `HeapId` alcançáveis a partir destes roots, marcando-os.
- **Fase de Sweep**: Todos os objetos não marcados são removidos do `HashMap` interno do Heap, liberando memória.

### 3. Arquitetura Modular do Interpretador

O `Interpreter` foi refatorado para reduzir o acoplamento, delegando responsabilidades para dois sub-módulos principais:

#### 3.1 Environment (`environment.rs`)

Gerencia todo o estado mutável do programa:

- **Scopes**: Pilha de escopos para variáveis locais (`call_stack_vars`).
- **Store**: Armazenamento de `variables`, `constants`, `classes` e `imported_modules`.
- **Contexto**: Mantém o `current_instance` para suporte ao `this`.

#### 3.2 NativeRegistry (`native_registry.rs`)

Encapsula o `NativeModuleManager` e simplifica a interface de chamadas nativas:

- **Despacho**: Resolve e executa funções nativas síncronas e assíncronas.
- **Ativação**: Gerencia a ativação estrita de categorias de módulos (ex: `#console_io`).

---

## 🛡️ Segurança e Hardening

### 4.1 Sandbox Security (Fase 1)

O runtime implementa um modelo de "Least Privilege" para funções nativas:

- **Sandbox Root**: Restringe o acesso ao sistema de arquivos a um diretório específico. Tentativas de acesso fora da raiz resultam em erro.
- **Flags de Permissão**: Funções críticas (ex: `exec`) só são habilitadas se a flag `allow_unsafe` for passada explicitamente.
- **Código de Erro 6001**: Erro padrão para diretiva de ativação de módulo nativo inválida ou não encontrada.

### 4.2 Limite de Recursão

O interpretador impõe um limite de recursão de 1000 chamadas (`MAX_RECURSION_DEPTH`) para evitar Stack Overflows. Quando excedido, o erro `E3040` é lançado.

---

## 📦 Ecossistema Oak (Package Manager)

O Oak segue uma arquitetura modular focada em extensibilidade:

- **Core**: Lógica de configuração (`core/config.rs`) e CLI (`core/cli.rs`).
- **Commands**: Divisão de subcomandos em arquivos independentes.
- **Registry**: Sistema de resolução de pacotes com suporte a integridade via hashes SHA-256 no futuro.

---

## 📚 Referências de Implementação

- **GC Implementation**: Localizado em `crates/dryad_runtime/src/heap.rs`.
- **Structural Refactor Docs**: Localizado em `docs/implementation/done/t3/`.
