---
title: "Orientação a Objetos"
description: "Classes, herança e OOP no Dryad."
category: "Linguagem"
order: 7
---

# Orientação a Objetos (Classes)

O Dryad utiliza um modelo de OOP baseado em classes com herança simples, visibilidade e suporte a métodos estáticos.

## 🚀 Leitura Rápida

- **Classes**: Plantas para criação de objetos via `class`.
- **Visibilidade**: `public` (padrão), `private`, `protected`.
- **Herança**: `extends` para herdar de uma classe pai.
- **Estáticos**: `static` para membros da classe (não da instância).
- **Contexto**: `this` refere-se à instância; `super` refere-se ao pai.

---

## ⚙️ Visão Técnica

O sistema de classes do Dryad é uma abstração sobre o motor de execução baseada em **Protótipos Dinâmicos** e **Ambientes Vinculados**.

### 1. Layout de Memória (Instance vs Class)

Para otimizar o uso de RAM, o Dryad separa dados mutáveis de métodos imutáveis:

- **`Instance`**: Contém apenas o estado único (propriedades).
- **`Class`**: Contém a tabela de métodos (vtable) compartilhada.

### 2. Visibilidade e Segurança

- **`public`**: Acessível de qualquer lugar (padrão).
- **`private`**: Membros privados são filtrados durante o look-up de propriedades se o contexto da chamada não for a própria classe.
- **`protected`**: Acessível na classe e em subclasses.
- **`static`**: Armazenados diretamente no objeto da Classe, não na instância.

### 3. Exemplo Completo (Tudo Funcional)

```dryad
class Conta {
    public titular;
    private saldo;
    static totalContas = 0;

    constructor(titular, depositoInicial) {
        this.titular = titular;
        this.saldo = depositoInicial;
        Conta.totalContas = Conta.totalContas + 1;
    }

    depositar(valor) {
        this.saldo = this.saldo + valor;
        return this.saldo;
    }

    static getTotalContas() {
        return Conta.totalContas;
    }
}

class ContaPremium extends Conta {
    private limite;

    constructor(titular, depositoInicial, limite) {
        super(titular, depositoInicial);
        this.limite = limite;
    }

    sacar(valor) {
        if (valor <= this.saldo + this.limite) {
            this.saldo = this.saldo - valor;
            return true;
        }
        return false;
    }
}

let conta = new ContaPremium("João", 1000, 500);
conta.depositar(500);
conta.sacar(1200);
println("Titular: " + conta.titular);
println("Total de contas: " + Conta.getTotalContas());
```

---

## ⛔ Funcionalidades Quebradas (NÃO usar)

As seguintes funcionalidades de OOP existem no AST mas **não funcionam** porque o lexer não as reconhece como keywords:

- **`interface` / `implements`** — Não implementados.
- **`get` / `set`** (getters/setters) — Não implementados.

---

## 📚 Referências e Paralelos

- **ES6 Classes**: Estética inspirada no [Modern JavaScript](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Classes).
- **Design Pattern**: [Prototype Pattern](https://refactoring.guru/design-patterns/prototype).
