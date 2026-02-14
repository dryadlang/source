---
title: "Funcionalidades Faltantes"
description: "Gap analysis e checklist do que ainda precisa ser portado para o interpretador."
category: "Projeto"
order: 4
---

# Funcionalidades Faltantes ou Parciais (Missing/Partial Features)

## 1. Linguagem e Sintaxe

### 1.1 Tipos e Estruturas

- [x] **Arrays Nativos**: Implementação completa de métodos como `.map()`, `.filter()`, `.reduce()`, `.push()`, `.pop()`, `.find()`, `.flat()`, `.zip()`, `.groupBy()`, etc.
- [ ] **Objetos (Maps)**: Sintaxe `{}` implementada, mas métodos como `.keys()`, `.values()`, `.entries()` faltam.
- [ ] **Destructuring**: `let { a, b } = obj` ou `let [x, y] = arr` (Ainda não parser/runtime support).
- [ ] **Spread/Rest Operator**: `...args` em funções e arrays (não implementado).
- [x] **Template Strings**: Interpolação `${expr}` dentro de backticks implementada via desaçucaramento para concatenação.

### 1.2 Controle de Fluxo

- [x] **Switch/Match**: Declaração `match` (pattern matching estilo Rust) implementada com suporte a guards e desestruturação básica.
- [ ] **Optional Chaining**: `obj?.prop` (não implementado).
- [ ] **Nullish Coalescing**: `??` (não implementado).

### 1.3 Funções

- [ ] **Parâmetros Default**: `function foo(x = 10)` (não implementado no parser).
- [ ] **Parâmetros Nomeados**: Chamada `foo(x: 10)` (não suportado).
- [ ] **Generators**: `function*` e `yield` (não implementado).

## 2. Orientação a Objetos

### 2.1 Classes

- [ ] **Modificadores de Acesso**: `private`, `protected`, `public` (Parser aceita keywords, mas Runtime ignora a verificação de visibilidade efetiva em tempo de execução).
- [ ] **Getters/Setters**: `get prop()`, `set prop(v)` (não implementado).
- [ ] **Interfaces/Traits**: Sem sistema de contratos ou tipos abstratos.
- [ ] **Propriedades Estáticas**: `static prop = val` (apenas métodos estáticos implementados).

## 3. Biblioteca Padrão (Stdlib)

### 3.1 I/O e Rede

- [ ] **TCP Server**: Apenas `tcp_client` implementado. Faltam `bind`, `listen`, `accept`.
- [ ] **HTTP Server**: Módulo `http_server.rs` existe, mas API pública não exposta completamente.
- [ ] **UDP Support**: Módulo `udp.rs` presente, mas bindings nativos incompletos.
- [ ] **File Streams**: Leitura/Escrita de arquivos grandes via stream (bufferizado) não implementada.

### 3.2 Sistema

- [ ] **Process Management**: `fork`, `kill`, sinais de processo.
- [x] **Memory Management**: Garbage Collector Mark-and-Sweep implementado (gerencia ciclos de memória no Heap).

## 4. Tooling

### 4.1 Debugger

- [ ] Suporte a breakpoints interativos.
- [ ] Inspeção de variáveis em tempo real (além de `print`).

### 4.2 Package Manager

- [ ] Sistema de dependências (`dryad.toml`) e resolução de pacotes externos.
