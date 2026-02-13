---
title: "Oportunidades de Refactor"
description: "Pontos do codebase que necessitam de limpeza ou melhorias estruturais."
category: "Projeto"
order: 5
---

# Refactor Opportunities

Este documento lista melhorias estruturais, limpezas de código e otimizações identificadas no codebase atual.

## Índice

1. [Runtime & Interpreter](#runtime--interpreter)
2. [Native Modules](#native-modules)
3. [Parser & AST](#parser--ast)
4. [Lexer](#lexer)
5. [Oak Package Manager](#oak-package-manager)
6. [CLI](#cli)

---

## Runtime & Interpreter

### `crates/dryad_runtime/src/interpreter.rs`

- **Linha 132-150**: A struct `Interpreter` acumula muitas responsabilidades (estado, ambiente, execução, funções nativas).
  - _Melhoria_: Extrair `Environment` para um módulo próprio `environment.rs`.
  - _Melhoria_: Extrair gerenciamento de funções nativas para `native_registry.rs`.
- **Linha 450+**: O método `evaluate` usa recursão profunda para avaliar expressões.
  - _Melhoria_: Implementar _trampolining_ ou migrar para uma máquina virtual baseada em pilha (Bytecode VM) para evitar Stack Overflow em scripts complexos.
- **Linha 800+**: Tratamento de `Result<Value, DryadError>` em cada passo é verboso.
  - _Melhoria_: Usar macros ou o operador `?` de forma mais ergonômica, talvez criando um alias `RuntimeResult<T>`.

### `crates/dryad_runtime/src/native_functions.rs`

- **Arquivo Completo**: Contém muitas funções soltas que são apenas wrappers.
  - _Melhoria_: Agrupar funções por módulo (Math, String, etc) em sub-structs ou traits para melhor organização.

## Native Modules

### `crates/dryad_runtime/src/native_modules/http_client.rs` e `http.rs`

- **Redundância**: Existem dois arquivos relacionados a HTTP.
  - _Melhoria_: Unificar em `crates/dryad_runtime/src/native_modules/http/mod.rs` e sub-módulos.

### `crates/dryad_runtime/src/native_modules/file_io.rs`

- **Sync I/O**: Operações de arquivo são síncronas e bloqueantes.
  - _Melhoria_: Migrar para `tokio::fs` para permitir I/O assíncrono real quando usado com `async/await` do Dryad.

## Parser & AST

### `crates/dryad_parser/src/ast.rs`

- **Clonagem Excessiva**: A AST usa `String` e `Vec` pesadamente, exigindo muitos `.clone()` durante a execução no interpretador atual.
  - _Melhoria_: Introduzir interning de strings (`SmolStr` ou similar) e `Rc<stmt>` na AST para baratear cópias.
- **Enum `Expr` Gigante**: O enum `Expr` tem muitas variantes, o que aumenta o tamanho da struct.
  - _Melhoria_: Boxear variantes grandes e raras para reduzir o `sizeof(Expr)`.

### `crates/dryad_parser/src/parser.rs`

- **Linha 1200+**: O método `statement` é um grande `match` que cresce indefinidamente.
  - _Melhoria_: Usar padrão _Pratt Parser_ ou dividir em métodos menores (`parse_decl`, `parse_control_flow`) em traits separadas.

## Lexer

### `crates/dryad_lexer/src/lexer.rs`

- **Iteração de Caracteres**: O uso de `self.source[self.position]` com verificações de limite manuais.
  - _Melhoria_: Usar `Chars` iterator ou uma biblioteca como `logos` para gerar um lexer mais performático e seguro.

## Oak Package Manager

### `crates/oak/src/main.rs`

- **Monólito**: O arquivo `main.rs` contém 2000 linhas com toda a lógica (CLI, Config, Instalação, Registry).
  - _Melhoria_: Refatorar urgentemente.
    - `commands/`: Subcomandos (`init.rs`, `install.rs`).
    - `core/config.rs`: Structs `OakConfig`, `OakLock`.
    - `core/dependency.rs`: Lógica de resolução.
    - `registry/`: Cliente de API.

## CLI

### `crates/dryad_cli/src/main.rs`

- **Duplicação de Setup**: A criação do `Interpreter` e `Lexer` se repete em `run`, `repl`, `check`.
  - _Melhoria_: Criar um `Runner` struct que encapsula o pipeline de execução.
