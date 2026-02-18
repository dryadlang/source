---
title: "Operadores e Precedência"
description: "Guia completo de operadores matemáticos, lógicos e bitwise no Dryad."
category: "Linguagem"
order: 16
---

# Operadores e Precedência

O sistema de operadores do Dryad equilibra a intuição de desenvolvedores C/JS com adições matemáticas rigorosas.

## 🚀 Leitura Rápida

- **Matemática**: Suporte nativo a exponenciação (`**`) e módulo euclidiano (`%%`).
- **Lógica**: Curto-circuito em `&&` e `||`.
- **Bitwise**: Operações em 64 bits (convertidas internamente de f64).
- **Atribuição**: Operadores compostos (`+=`, `*=`, etc) integrados.

---

## ⚙️ Visão Técnica

O parser do Dryad utiliza **Pratt Parsing** (Precedência de Ligação) para resolver expressões complexas com performance O(n).

### 1. Módulo Euclidiano (`%%`)

Diferente do `%` tradicional (resto da divisão), o `%%` garante que o resultado seja sempre positivo, seguindo a definição matemática de Euclides.

- **Implementação**: `((a % b) + b) % b`.

### 2. Conversões Bitwise e f64

Como todos os números no Dryad são `f64` (IEEE 754), as operações bitwise realizam um cast temporário para `i64` no nível da CPU antes de retornar o resultado para o formato flutuante.

| Nível | Operadores           | Descrição            |
| :---- | :------------------- | :------------------- |
| 1     | `()`, `[]`, `.`      | Agrupamento e Acesso |
| 2     | `!`, `-`, `~`, `...` | Unários, Spread      |
| 3     | `**`                 | Potência             |
| 4     | `*`, `/`, `%`, `%%`  | Multiplicativos      |
| 5     | `+`, `-`             | Aditivos             |

### 4. Operador Spread (`...`)

O operador spread permite expandir elementos de um Array ou Tupla em locais onde múltiplos argumentos ou elementos são esperados (como em literais de array ou chamadas de função).

```dryad
let part = [2, 3];
let full = [1, ...part, 4]; // [1, 2, 3, 4]

fn add(a, b, c) => a + b + c;
let nums = [1, 2, 3];
print(add(...nums)); // 6
```

---

## 📚 Referências e Paralelos

- **Pratt Parsing**: [Simple but Powerful Pratt Parsing](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html).
- **IEEE 754**: [Floating Point Standard](https://en.wikipedia.org/wiki/IEEE_754).
- **JS Operators**: [MDN Operators Reference](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators).
