---
title: "Guia Detalhado de Sintaxe"
description: "Referência completa da gramática, identificadores e estruturas da linguagem Dryad."
category: "Linguagem"
order: 12
---

# Guia de Sintaxe da Linguagem Dryad

Este documento serve como referência definitiva para a sintaxe da linguagem Dryad, detalhando regras gramaticais, estruturas de controle e convenções.

## 🚀 Leitura Rápida

- **Sintaxe C-Style**: Familiar para quem vem de JavaScript, C# ou Rust.
- **Variáveis**: Declaração obrigatória com `let` ou `const`.
- **Tipos**: Dinâmicos por padrão, mas com verificação estática opcional via `dryad_checker`.
- **Modos**: Interpretado (AST) ou Compilado (Bytecode VM).

---

## ⚙️ Visão Técnica

O Dryad utiliza um parser descendente recursivo (Pratt Parser para expressões) que gera uma Árvore de Sintaxe Abstrata (AST). Esta AST pode ser executada diretamente ou compilada para bytecode para maior performance.

### 1. Estrutura Léxica

- **Identificadores**: Devem começar com letras ou `_`. Sensíveis a maiúsculas/minúsculas.
- **Ponto e Vírgula**: Obrigatório após declarações e expressões (E2003).

### 2. Controle de Fluxo Avançado

Além dos tradicionais `if` e `for`, o Dryad foca em expressividade:

```javascript
match (valor) {
    1 => print("Um"),
    [head, ...tail] => print("Lista: " + head + ", cauda: " + tail),
    _ => print("Outro")
}
```

### 3. Organização via Namespaces

O Dryad permite agrupar declarações em blocos nomeados para evitar colisões no escopo global:

```dryad
namespace MathUtils {
    let PI = 3.14159;
    fn circleArea(r) => PI * (r ** 2);
}

println(MathUtils.circleArea(10));
```

### 4. Sistema de Erros (Result & ?)

Para handling robusto de erros, o Dryad utiliza o tipo `Result`:

```dryad
fn divide(a, b) {
    if (b == 0) return Result(false, "Divisão por zero");
    return Result(true, a / b);
}

let result = divide(10, 0)?; // O operador '?' propaga o erro se for Result(false, ...)
```

### 5. Sistema de Módulos e Diretivas

O Dryad integra-se ao host nativo via diretivas nativas (sistema `#`):

```javascript
#<console_io>
#<file_io>

// Uso imediato das funções nativas
println("Hello World");
```

---

## 4. Detalhes de Implementação

### Operadores e Precedência

| Nível | Operadores           | Descrição                 |
| :---- | :------------------- | :------------------------ |
| 1     | `()`, `[]`, `.`, `?` | Agrupamento e Acesso, Try |
| 2     | `!`, `-`, `~`, `...` | Unários, Spread           |
| 3     | `**`                 | Exponenciação             |
| 4     | `*`, `/`, `%`, `%%`  | Multiplicativos           |
| 5     | `+`, `-`             | Aditivos                  |

---

## 📚 Referências e Paralelos

- **Gramática de Referência**: Consulte a crate `dryad_parser` para a EBNF completa.
- **Padrões de Design**: Inspirado no [Rust Book](https://doc.rust-lang.org/book/).
- **Performance**: O modo bytecode (`--compile`) oferece 2-3x mais velocidade que o modo AST.
