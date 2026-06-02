---
title: "Visão Geral da Arquitetura"
description: "Visão de alto nível da implementação do interpretador Dryad."
category: "Desenvolvimento"
order: 1
---

# Visão Geral da Arquitetura

Este documento fornece uma visão de alto nível da implementação do interpretador Dryad.

## 🚀 Leitura Rápida

- **Modular**: Dividido em crates Rust independentes.
- **Pipeline**: Lexer → Parser → Interpreter (Tree-Walking).
- **Runtime**: Baseado em HashMaps para escopo e suporte nativo a threads.
- **Segurança**: Herda garantias de memória do Rust (Memory Safety).

---

## ⚙️ Visão Técnica

O Dryad é projetado seguindo os princípios de sistemas distribuídos e extensibilidade modular. A arquitetura é inspirada na clareza do [Crafting Interpreters](https://craftinginterpreters.com/) e na robustez do compilador do Rust (`rustc`).

### 1. Sistema de Crates (Rust Modularity)

Seguindo o padrão de projetos Rust modernos, cada componente é uma crate separada. Isso permite:

- **Testes Isolados**: `cargo test -p dryad_lexer` sem carregar o runtime.
- **Reutilização**: O `dryad_lexer` pode ser usado por ferramentas de linter ou VS Code sem depender do executor principal.

### 2. Componentes Principais

#### Dryad Lexer (`dryad_lexer`)

Responsável por transformar o código fonte em tokens.

- **Internals**: Implementado como um iterador Unicode-aware. Diferente de lexers baseados em Regex, ele utiliza uma máquina de estados finitos manual para maior performance.
- **Link**: [Rust `logos` crate pattern](https://docs.rs/logos/latest/logos/)

#### Dryad Parser (`dryad_parser`)

Consome tokens e produz a Árvore Sintática Abstrata (AST).

- **Algoritmo**: Recursive Descent (Descida Recursiva).
- **Precedência**: Implementa o algoritmo de Pratt para expressões matemáticas complexas, similar ao que é visto no parser da linguagem [Go](https://go.dev/).

#### Dryad Runtime (`dryad_runtime`)

Executa a AST.

- **Modelo**: Atualmente um _Tree-Walking Interpreter_.
- **Concorrência**: Utiliza o modelo de _M-N Scheduling_ através da crate `crossbeam` para gerenciar balanceamento de carga de threads.

---

## 📚 Referências e Paralelos

- **Base de Implementação**: [Rust Programming Language](https://www.rust-lang.org/)
- **Gestão de Cargas**: Inspirado por [Tokio.rs](https://tokio.rs/) (runtime assíncrono).
- **Teoria de Compiladores**: [The Dragon Book](https://en.wikipedia.org/wiki/Compilers:_Principles,_Techniques,_and_Tools).

---

## Estrutura de Diretórios

```bash
crates/
├── dryad_cli/        # Interface de Linha de Comando (entry point)
├── dryad_lexer/      # Scanner / Tokenizer
├── dryad_parser/     # Syntax Analysis (AST)
├── dryad_runtime/    # Evaluation Machine (Value, Scope, Stdlib)
├── dryad_errors/     # Diagnostic System (miette-like)
└── dryad_benchmark/  # Performance tracking
```
