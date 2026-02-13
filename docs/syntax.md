---
title: "Referência de Sintaxe"
description: "Visão geral da sintaxe e gramática da linguagem Dryad."
category: "Linguagem"
order: 1
---

# Referência de Sintaxe (Gramática)

A sintaxe do Dryad é uma evolução da estética C-Style, focada em legibilidade e suporte nativo a paradigmas modernos como assincronia e paralelismo.

## 🚀 Leitura Rápida

- **C-Style**: Chaves `{}` e ponto e vírgula `;` são fundamentais.
- **Variáveis**: `let` (mutável) e `const` (imutável).
- **Controle**: `if/else`, `while`, `for`, `for-in`, `try/catch`.
- **Funções**: `function`, `=>` (lambdas), `async`, `thread`.
- **OO**: `class`, `extends`, `constructor`, `this`, `super`.

---

## ⚙️ Visão Técnica

A gramática do Dryad é processada por um parser LL(k), que prioriza a detecção de erros clara e a construção de uma AST rica em metadados.

### 1. Sistema de Blocos e Escopo

O Dryad utiliza escopo léxico puro. Diferente do JavaScript (pré-ES6), **não existe hoisting** de variáveis declaradas com `let` ou `const`. Isso evita bugs silenciosos onde variáveis são acessadas antes de serem definidas.

### 2. Funções e Closures

As funções são objetos de primeira classe. Lambdas capturam o ambiente via ARC (Atomic Reference Counting), permitindo que persistam com segurança em contextos assíncronos ou multithread.

### 3. Concorrência Nativa

A sintaxe `thread function` é um açúcar sintático para o spawn de threads do sistema operacional, garantindo isolamento total de memória (Shared-Nothing by default).

---

## 📚 Referências e Paralelos

- **Estética**: Inspirada no [C++](https://isocpp.org/) e [JavaScript modernizado](https://tc39.es/ecma262/).
- **Teoria de Linguagens**: "Compilers: Principles, Techniques, and Tools" (Aho, Lam, Sethi, Ullman).
- **Semântica**: "Types and Programming Languages" (Pierce).

---

## Detalhamento da Gramática

### 1. Comentários

- **Linha única**: `// comentário`
- **Bloco**: `/* comentário */`

### 2. Declaração de Variáveis

O parser suporta `let` e `const`. O uso de `var` é permitido por retrocompatibilidade, mas tratado internamente como `let`.

```dryad
let x = 10;
const PI = 3.14;
```

### 3. Estruturas de Controle (Exemplos)

#### Condicionais

```dryad
if (x > 0) {
    print("Positivo");
} else {
    print("Negativo ou Zero");
}
```

#### Loops

```dryad
for (let i = 0; i < 10; i++) {
    print(i);
}
```

### 4. Classes e Orientação a Objetos

```dryad
class Motor {
    constructor(potencia) {
        this.potencia = potencia;
    }
}

class Carro extends Motor {
    ligar() {
        print("Vrummm!");
    }
}
```
