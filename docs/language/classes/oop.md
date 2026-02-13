---
title: "Orientação a Objetos"
description: "Classes, herança e o modelo de protótipos do Dryad."
category: "Linguagem"
order: 7
---

# Orientação a Objetos

O Dryad utiliza um modelo de Orientação a Objetos baseado em classes, focado em clareza sintática e eficiência de memória.

## 🚀 Leitura Rápida

- **Classes**: Plantas para criação de objetos.
- **Herança**: Reutilize lógica via `extends`.
- **Construtor**: Método especial `constructor` para inicialização.
- **Contexto**: `this` refere-se à instância atual; `super` refere-se ao pai.

---

## ⚙️ Visão Técnica

O sistema de classes do Dryad é uma abstração sobre o motor de execução baseada em **Protótipos Dinâmicos** e **Ambientes Vinculados**.

### 1. Layout de Memória (Instance vs Class)

Para otimizar o uso de RAM, o Dryad separa dados mutáveis de métodos imutáveis:

- **`Instance`**: Contém apenas o estado único (propriedades) em um `HashMap<String, Value>`. Possui um ponteiro para sua classe de origem.
- **`Class`**: Contém a tabela de métodos (vtable) e referências à superclasse. Métodos são compartilhados por todas as instâncias.

### 2. Vinculação do `this` (Binding)

Quando um método é chamado (`instancia.falar()`), o interpretador realiza os seguintes passos:

1.  Busca o método na classe da instância.
2.  Cria um novo ambiente para a execução do método.
3.  Define uma variável especial `this` dentro desse ambiente aponta para a instância.

### 3. Cadeia de Herança

A busca por métodos e propriedades segue a cadeia de protótipos em tempo de execução:
`Instância` → `Classe` → `SuperClasse` → `...` → `Null`.

---

## 📚 Referências e Paralelos

- **ES6 Classes**: O Dryad adota a estética do [ECMAScript 2015](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Classes).
- **Design Pattern**: [Prototype Pattern](https://refactoring.guru/design-patterns/prototype).
- **Rust Implementation**: Utiliza `Arc<RwLock<ClassInner>>` para permitir que múltiplas instâncias em threads diferentes acessem os mesmos métodos com segurança.

---

## Exemplo Avançado

```dryad
class Contador {
    valor = 0;
    incrementar() {
        this.valor++;
    }
}

let c = new Contador();
c.incrementar();
```

> [!NOTE]
> Diferente de linguagens estáticas, as propriedades podem ser adicionadas ou removidas dinamicamente da instância se desejado, embora o uso de `class` recomende uma estrutura fixa.
