---
title: "Operadores e Precedência"
description: "Guia completo de operadores matemáticos, lógicos e bitwise no Dryad."
category: "Linguagem"
order: 16
---

# Operadores e Precedência

O sistema de operadores do Dryad é projetado para ser intuitivo para desenvolvedores C/JS, mas com adições poderosas para cálculos matemáticos e lógicos.

## 🚀 Leitura Rápida

- **Matemática**: Suporte nativo a exponenciando (`**`) e módulo positivo (`%%`).
- **Lógica**: Curto-circuito em `&&` e `||`.
- **Bitwise**: Operações em 64 bits para manipulação de baixo nível.
- **Precedência**: Segue o padrão matemático rigoroso.

---

## ⚙️ Visão Técnica

O motor de parsing do Dryad utiliza **Pratt Parsing** (também conhecido como Precedência de Ligação) para resolver expressões sem ambiguidade.

### 1. Módulo Positivo (`%%`) vs Módulo (`%`)

Diferente da maioria das linguagens onde `-10 % 3` resulta em `-1`, o Dryad introduz o operador matemático "verdadeiro":

- **`%`**: Resto da divisão (comportamento padrão de C/Rust).
- **`%%`**: Modulo Euclidiano. Garante que o resultado esteja sempre no intervalo `[0, divisor)`.
- **Implementação**: `((a % b) + b) % b`.

### 2. Conversões Bitwise

Como todos os números no Dryad são `f64`, operações bitwise (como `&`, `|`, `<<`) realizam uma conversão temporária:

1. O valor `f64` é truncado para um inteiro `i64`.
2. A operação bitwise é executada no nível da CPU.
3. O resultado é convertido de volta para `f64` para armazenamento no `Value`.

### 3. Tabelas de Precedência (Top-Down)

| Nível | Operadores          | Descrição            |
| :---- | :------------------ | :------------------- |
| 1     | `()`, `[]`, `.`     | Agrupamento e Acesso |
| 2     | `!`, `-`, `~`       | Unários              |
| 3     | `**`, `^^`          | Potência e Raiz      |
| 4     | `*`, `/`, `%`, `%%` | Multiplicativos      |
| 5     | `+`, `-`            | Aditivos             |
| 6     | `<<`, `>>`, `>>>`   | Shifts               |

---

## 📚 Referências e Paralelos

- **Pratt Parsing**: [Simple but Powerful Pratt Parsing](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html).
- **IEEE 754**: [Floating Point Arithmetic](https://en.wikipedia.org/wiki/IEEE_754).
- **Modulo Euclidiano**: [Euclidean Division](https://en.wikipedia.org/wiki/Euclidean_division).

---

## Exemplos de Lógica

Os operadores lógicos `&&` e `||` são **curto-circuito**. Isso significa que a segunda expressão só é avaliada se necessário.

```dryad
function teste() {
    println("Fui chamado!");
    return true;
}

let x = false && teste(); // teste() NUNCA será chamado
```
