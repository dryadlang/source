---
title: "Guia Detalhado de Sintaxe"
description: "Referência completa da gramática, identificadores e estruturas da linguagem Dryad."
category: "Linguagem"
order: 12
---

# Guia de Sintaxe da Linguagem Dryad

Este documento serve como referência definitiva para a sintaxe da linguagem Dryad, detalhando regras gramaticais, estruturas de controle e convenções. Abaixo estão as principais diretrizes para escrever código na linguagem Dryad.

## 1. Estrutura Léxica

### Comentários

- **Linha**: `// ...` até o fim da linha.
- **Bloco**: `/* ... */`, não aninháveis.

### Identificadores

- Devem começar com `a-z`, `A-Z` ou `_`.
- Podem conter números, mas não podem começar com eles.
- Sensíveis a maiúsculas e minúsculas.

## 2. Declaração de Variáveis

### `let` e `const`

- `let`: Declara uma variável mutável no escopo atual. Exemplo:

```dryad
let x = 10;
x = 20; // Permitido
```

- `const`: Declara uma constante imutável. Exemplo:

```dryad
const y = 30;
// y = 40; // Erro: Não é possível reatribuir uma constante
```

## 3. Controle de Fluxo

### Condicionais

- `if` / `else`: Avalia a expressão para `bool`.
- `match`: Pattern matching poderoso inspirado em Rust/Elixir.

```dryad
match (valor) {
    1 => print("Um"),
    [a, b] => print("Lista com " + a + " e " + b),
    { nome: n } => print("Olá " + n),
    _ => print("Outro")
}
```

### Loops

- `while`: Repete o bloco enquanto a condição for verdadeira. Verificado antes da execução.

```dryad
while (condicao) {
    // Código a ser repetido
}
```

- `for`: Itera sobre arrays ou intervalos.

```dryad
for (let i = 0; i < 10; i++) {
    // Código a ser executado
}
```

## 4. Operadores e Precedência

Da maior para a menor precedência:

1.  **Acesso/Chamada**: `.` (ponto), `[]`, `()`
2.  **Unário**: `!`, `-`, `++`, `--`
3.  **Potência**: `**` (direita para esquerda)
4.  **Multiplicativo**: `*`, `/`, `%`
5.  **Aditivo**: `+`, `-`
6.  **Shift**: `<<`, `>>`, `<<<`, `>>>`
7.  **Relacional**: `<`, `<=`, `>`, `>=`
8.  **Igualdade**: `==`, `!=`
9.  **Bitwise AND**: `&`
10. **Bitwise XOR**: `^`
11. **Bitwise OR**: `|`
12. **Lógico AND**: `&&`
13. **Lógico OR**: `||`
14. **Atribuição**: `=`, `+=`, `-=`, etc.

## 5. Funções

### Declaração

```javascript
function soma(a, b) {
  return a + b;
}
```

### Lambdas (Arrow Functions)

```javascript
let dobro = (x) => x * 2;
let soma = (a, b) => {
  return a + b;
};
```

_Nota: Lambdas capturam o `this` do contexto léxico (closure)._

### Async / Await

```javascript
async function buscar() {
  let dados = await fetch("url");
  return dados;
}
```

## 6. Classes e Objetos

```javascript
class Retangulo {
  largura = 0;
  altura = 0;

  area() {
    return this.largura * this.altura;
  }

  static criarQuadrado(lado) {
    let r = new Retangulo(); // 'new' opcional
    r.largura = lado;
    r.altura = lado;
    return r;
  }
}
```

## 7. Módulos

- **Import**: `import { x } from "mod";` ou `import * as m from "mod";`
- **Export**: `export function f() { ... }`

## 8. Diretivas Nativas

Uso especial para injetar módulos do runtime Rust:

```javascript
#console_io
#file_io
```
