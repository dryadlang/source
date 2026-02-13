---
title: "Funções e Procedimentos"
description: "Declaração de funções, closures e modelos de execução concorrente."
category: "Linguagem"
order: 5
---

# Funções e Procedimentos

As funções são os blocos fundamentais de lógica no Dryad, suportando paradigmas funcionais, imperativos e concorrentes.

## 🚀 Leitura Rápida

- **Declarativas**: `function nome() { ... }`.
- **Anônimas**: `(a, b) => a + b` (Lambdas com closures).
- **Assíncronas**: `async` / `await` para I/O não-bloqueante.
- **Paralelas**: `thread function` para processamento em múltiplos núcleos.

---

## ⚙️ Visão Técnica

O motor Dryad trata funções como **Cidadãos de Primeira Classe** (First-Class Citizens), permitindo que sejam passadas como argumentos e retornadas de outras funções.

### 1. Closures (Encapulamento de Ambiente)

As Lambdas no Dryad não são apenas ponteiros de função; elas capturam o escopo onde foram criadas.

- **Mecanismo**: Quando uma lambda é definida, o interpretador salva uma referência ao `Environment` atual.
- **Persistência**: Graças ao uso de `Arc` no Rust, o ambiente capturado permanece vivo mesmo após a função pai ter terminado sua execução.

### 2. Funções de Thread (Paralelismo Real)

Ao declarar uma `thread function`, o Dryad realiza um fork lógico do interpretador:

- **Isolamento**: A nova thread recebe uma cópia superficial (shallow copy) do ambiente global, mas possui sua própria stack frame de variáveis locais.
- **Sincronização**: Para compartilhar dados mutáveis entre threads, deve-se usar os tipos `Mutex` ou `Channel` da stdlib.

### 3. Modelo Async (Máquina de Estados)

Funções `async` são transformadas internamente em **Futures**. O `await` suspende a execução da fibra atual sem bloquear a thread do sistema operacional subjacente.

---

## 📚 Referências e Paralelos

- **Rust Closures**: [Fn, FnMut, and FnOnce Traits](https://doc.rust-lang.org/book/ch13-01-closures.html).
- **Computer Science**: "The Structure and Interpretation of Computer Programs" (SICP) - Seção sobre o Modelo de Ambiente.
- **Concurrency**: [M-N Threading Model (Wikipedia)](<https://en.wikipedia.org/wiki/Thread_(computing)#M:N_threading>).

---

## Passagem de Parâmetros

O Dryad utiliza o modelo **Call-by-Sharing**:

- **Primitivos**: Copiados (similar a Deep Copy por serem pequenos).
- **Objetos/Arrays**: Referência compartilhada. Mudanças dentro da função refletem no objeto original.

```dryad
function mudar(arr) {
    arr[0] = "mudou";
}
let lista = [1];
mudar(lista); // lista agora é ["mudou"]
```
