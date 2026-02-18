---
title: "Orientação a Objetos"
description: "Classes, herança e o modelo de protótipos do Dryad."
category: "Linguagem"
order: 7
---

# Orientação a Objetos (Classes e Interfaces)

O Dryad utiliza um modelo de Orientação a Objetos baseado em classes, foca em clareza sintática e eficiência de memória via Protótipos Dinâmicos.

## 🚀 Leitura Rápida

- **Classes**: Plantas para criação de objetos via `class`.
- **Visibilidade**: Suporte a `public` e `private` (validado em runtime).
- **Herança**: Suporte a extensão de classe via `extends`.
- **Interfaces**: Contratos múltiplos definidos via `interface` e `implements`.
- **Contexto**: `this` refere-se à instância; `super` refere-se ao pai.

---

## ⚙️ Visão Técnica

O sistema de classes do Dryad é uma abstração sobre o motor de execução baseada em **Protótipos Dinâmicos** e **Ambientes Vinculados**.

### 1. Layout de Memória (Instance vs Class)

Para otimizar o uso de RAM, o Dryad separa dados mutáveis de métodos imutáveis:

- **`Instance`**: Contém apenas o estado único (propriedades).
- **`Class`**: Contém a tabela de métodos (vtable) compartilhada.

### 2. Visibilidade e Segurança

Diferente de JS onde `#` é usado para privado, o Dryad usa keywords clássicas:

- **`private`**: Membros privados são filtrados durante o look-up de propriedades se o contexto da chamada não for a própria classe.
- **`static`**: Armazenados diretamente no objeto da Classe, não na instância.

### 3. Exemplo de Implementação

```dryad
interface Printable {
    function print();
}

class Pessoa implements Printable {
    private let nome;

    constructor(n) {
        this.nome = n;
    }

    function print() {
        println("Pessoa: " + this.nome);
    }
}
```

---

## 📚 Referências e Paralelos

- **ES6 Classes**: Estética inspirada no [Modern JavaScript](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Classes).
- **Design Pattern**: [Prototype Pattern](https://refactoring.guru/design-patterns/prototype).
- **Segurança**: Verificações de acesso em runtime implementadas na crate `dryad_runtime`.
- **Static Analysis**: O `dryad_checker` agora valida a conformidade de interfaces e a hierarquia de herança antes da execução.
