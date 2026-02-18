---
title: "Funções e Procedimentos"
description: "Declaração de funções, closures e modelos de execução concorrente."
category: "Linguagem"
order: 19
---

# Funções e Procedimentos

As funções são os blocos fundamentais de lógica no Dryad, suportando paradigmas funcionais, imperativos e concorrentes como cidadãos de primeira classe.

## 🚀 Leitura Rápida

- **Declarativas**: `function nome() { ... }`.
- **Anônimas**: `(a, b) => a + b` (Lambdas com closures).
- **Assíncronas**: `async` / `await` para I/O não-bloqueante.
- **Paralelas**: `thread function` para processamento em múltiplos núcleos.
- **Passagem**: Call-by-sharing (objetos por referência, primitivos por valor).

---

## ⚙️ Visão Técnica

O motor Dryad trata funções como **First-Class Citizens**, permitindo alta flexibilidade na composição de lógica.

### 1. Closures (Encapulamento de Ambiente)

Quando uma lambda é definida, o interpretador salva uma referência ao `Environment` atual. Graças ao uso de `Arc` no motor Rust, o ambiente capturado permanece vivo mesmo após a função pai ter terminado, permitindo padrões funcionais poderosos.

### 2. Funções de Thread (Paralelismo Real)

Diferente de Workers em JS, a `thread function` no Dryad realiza um fork lógico:

- **Isolamento**: Cada thread possui sua própria stack frame.
- **Compartilhamento**: Dados mutáveis devem ser protegidos por `Mutex` ou comunicados via `Channel` (Stdlib).

### 3. Modelo Async (Fibras)

Funções `async` são tratadas como máquinas de estados (Futures). O `await` suspende a execução da fibra atual sem bloquear a thread do sistema operacional, otimizando o throughput de I/O.

### 4. Funções Variádicas (Rest Parameters)

O Dryad suporta parâmetros rest usando a sintaxe `...`, permitindo que uma função aceite um número indefinido de argumentos, que são coletados em um Array.

```dryad
fn sumAll(prefix, ...numbers) {
    let total = 0;
    for n in numbers {
        total = total + n;
    }
    return prefix + ": " + total;
}

print(sumAll("Total", 1, 2, 3, 4, 5)); // Total: 15
```

---

## 📚 Referências e Paralelos

- **JS Functions**: Sintaxe familiar ao [MDN Functions](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Functions).
- **Computer Science**: "The Structure and Interpretation of Computer Programs" (SICP) - Seção sobre o Modelo de Ambiente.
- **Concurrency**: [M-N Threading Model](<https://en.wikipedia.org/wiki/Thread_(computing)#M:N_threading>).
