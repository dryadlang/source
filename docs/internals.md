---
title: "Funcionamento Interno"
description: "Pipeline do compilador, M-N threads e gestão de memória avançada."
category: "Desenvolvimento"
order: 7
---

# Funcionamento Interno do Dryad

Este documento mergulha nos detalhes técnicos da arquitetura da linguagem Dryad, focando em como ela gerencia recursos, executa instruções e orquestra a concorrência.

## 🚀 Leitura Rápida

- **Pipeline**: Lexer (Tokens) → Parser (AST) → Analyzer → Runtime.
- **Memória**: Híbrida (Stack para primitivos, Arc/RwLock para heap).
- **Paralelismo**: M-N Scheduling (Milhares de fibras em poucas threads de sistema).
- **Extensível**: Sistema de módulos nativos via FFI com Rust.

---

## ⚙️ Visão Técnica

### 1. Pipeline de Execução

O Dryad evita a compilação JIT (Just-In-Time) complexa em favor de um interpretador de AST resiliente e otimizado, facilitando a portabilidade.

1.  **Lexer**: Máquina de estados DFA para scan de tokens.
2.  **Parser**: Recursive Descent com Pratt Parsing para precedência.
3.  **Static Analysis**: Verificação de escopo e mutabilidade antes da execução.
4.  **Runtime**: Executor Tree-Walking que utiliza o modelo de Visitor.

### 2. Gerenciamento de Memória Híbrido

Diferente de linguagens com GC "Stop-the-World" (como Java), o Dryad utiliza contagem de referências atômica.

- **Ownership de Rust**: O interpretador herda a segurança do Rust. Quando um `Value` sai de escopo, as referências são decrementadas e a memória é liberada imediatamente.
- **Mutexes e Interior Mutability**: Estruturas globais são protegidas por `RwLock`, permitindo múltiplas leituras simultâneas mas escrita exclusiva.

### 3. Concorrência M-N (Green Threads)

Utilizamos a crate **Crossbeam** e **Tokio** para gerenciar o balanceamento de carga entre núcleos da CPU.

- **Fibras**: São corrotinas leves que pausão em IO, cedendo o núcleo para outra fibra.
- **Threads Nativa**: Criadas via `std::thread`, ideais para processamento pesado que não deve bloquear o loop de eventos das fibras.

---

## 📚 Referências e Paralelos

- **Concordância**: [Crossbeam Documentation](https://docs.rs/crossbeam/latest/crossbeam/).
- **Gerenciamento de Memória**: [Automatic Reference Counting (ARC)](https://en.wikipedia.org/wiki/Automatic_Reference_Counting).
- **Arquitetura VM**: "Virtual Machine Design and Implementation in Rust" (Artigo de referência para o design do interpretador).

---

## 4. Segurança e Isolamento

Cada thread gerada pelo Dryad possui seu próprio contexto de variáveis locais, mas compartilha o acesso a módulos globais de forma imutável (Read-Only), eliminando a maioria das condições de corrida por design.

### 4.1 Runtime Hardening

- **Limite de Recursão**: O interpretador impõe um limite de recursão de 1000 chamadas (`MAX_RECURSION_DEPTH`) para evitar stack overflows. Quando excedido, um erro `E3040` é disparado.
- **Sandbox Security**: Funções nativas potencialmente perigosas (como `native_exec`) agora requerem a flag `--allow-unsafe` no runtime. Sem esta flag, o `NativeModuleManager` bloqueia a execução por segurança.

---

## 5. Ecossistema Oak (Package Manager)

O Oak foi refatorado para seguir uma arquitetura modular:

- **Core**: Contém a lógica de configuração (`core/config.rs`) e definições de CLI (`core/cli.rs`).
- **Commands**: Cada funcionalidade (init, install, run, etc.) reside em seu próprio módulo em `commands/`.
- **Registry**: Sistema de resolução de pacotes multi-registry com suporte a resolução de conflitos.
