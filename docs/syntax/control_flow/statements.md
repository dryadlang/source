---
title: "Controle de Fluxo"
description: "Estruturas condicionais e loops: if, while, for e for-in."
category: "Linguagem"
order: 17
---

# Controle de Fluxo (Statements)

O controle de fluxo no Dryad segue a sintaxe imperativa clássica, permitindo ramificações e iterações com lógica baseada em verdade.

## 🚀 Leitura Rápida

- **Condicionais**: `if`, `else if`, `else`.
- **Loops**: `while`, `for` (clássico), `for-in` (iteração de objetos).
- **Truthiness**: Lógica flexível (não apenas booleanos).
- **Blocos**: Delimitados por `{}` com escopo léxico próprio.

---

## ⚙️ Visão Técnica

O interpretador gerencia o fluxo de controle através da avaliação condicional de nós da AST ou saltos no bytecode.

### 1. Sistema Truthy/Falsy

No Dryad, qualquer valor pode ser convertido para booleano:

- **Falsy**: `false`, `null`, `0`, `""`.
- **Truthy**: Todo o resto (incluindo objetos e arrays vazios).

### 2. Snapshots em `for-in`

Para evitar problemas de concorrência ou crashes ao modificar um objeto durante a iteração, o Dryad tira um "snapshot" das chaves no início do loop. Isso garante que o loop termine mesmo se chaves forem deletadas.

### 3. Sinais de Controle (Short-circuit)

O runtime utiliza sinais internos (`Signal::Return`, `Signal::Break`) para interromper a execução de blocos de forma limpa, garantindo que o unwinding da stack de execução ocorra corretamente.

---

## 📚 Referências e Paralelos

- **C-Style Syntax**: Inspirado no fluxo de controle do [ANSI C](https://en.wikipedia.org/wiki/ANSI_C).
- **CS**: [Control Flow Analysis](https://en.wikipedia.org/wiki/Control-flow_analysis).
- **Rust If-Let**: O Dryad planeja suporte a padrões similares ao `if let` futuramente.
