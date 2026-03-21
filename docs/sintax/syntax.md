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

> ⚠️ **NÃO IMPLEMENTADO**: O bloco `namespace` existe no AST (`Stmt::Namespace`) mas **não está implementado** no lexer (não é keyword) nem no parser (sem handler). O operador `::` funciona como acesso a propriedade.

```dryad
// BROKEN — namespace como bloco NÃO funciona
namespace MathUtils {
    let PI = 3.14159;
    fn circleArea(r) => PI * (r ** 2);
}

// O operador :: funciona como acesso a propriedade:
// MathUtils::circleArea(10) — equivalente a MathUtils.circleArea(10)
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

> Para a tabela completa de precedência, consulte `SYNTAX_MANIFEST.md` (seção 7.1) na raiz do projeto.

| Nível | Operadores                          | Descrição                 |
| :---- | :---------------------------------- | :------------------------ |
| 1 (menor) | `=`                            | Atribuição em expressão   |
| 2     | `\|\|`                              | OR lógico                 |
| 3     | `&&`                                | AND lógico                |
| 4     | `\|`                                | OR bitwise                |
| 5     | `^`                                 | XOR bitwise               |
| 6     | `&`                                 | AND bitwise               |
| 7     | `==`, `!=`                          | Igualdade                 |
| 8     | `<`, `<=`, `>`, `>=`               | Comparação                |
| 9     | `<<`, `>>`, `<<<`, `>>>`           | Shift bitwise             |
| 10    | `+`, `-`                            | Aditivos                  |
| 11    | `*`, `/`, `%`, `%%`                | Multiplicativos           |
| 12    | `**`, `^^`, `##`                   | Potência (direita-assoc.) |
| 13    | `!`, `-`, `++`, `--` (pré)         | Unários                   |
| 14 (maior) | `++`, `--` (pós), `[]`, `.`, `()`, `::` | Postfix e Acesso |

---

## 📚 Referências e Paralelos

- **Gramática de Referência**: Consulte a crate `dryad_parser` para a EBNF completa.
- **Padrões de Design**: Inspirado no [Rust Book](https://doc.rust-lang.org/book/).
- **Performance**: O modo bytecode (`--compile`) oferece 2-3x mais velocidade que o modo AST.
