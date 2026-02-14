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

## 2.1 Classes

### 2.1.1 Declaração de Classes

```dryad
class NomeDaClasse {
    // Propriedades
    let propriedade = valor;
    
    // Métodos
    function metodo() {
        // corpo
    }
}
```

### 2.1.2 Modificadores de Acesso

O Dryad suporta modificadores de visibilidade para controlar o acesso a membros da classe.

| Modificador | Descrição | Implementado |
|-------------|-----------|--------------|
| `public` | Acessível de qualquer lugar | ✅ |
| `private` | Acessível apenas na classe | ✅ |
| `protected` | Acessível na classe e subclasses | ❌ |

```dryad
class Exemplo {
    public let valorPublico = 1;
    private let valorPrivado = 2;
    protected let valorProtegido = 3;
    
    public function metodoPublico() { }
    private function metodoPrivado() { }
}
```

**Status atual**: 
- **Propriedades**: Verificação completa para `public` e `private`.
- **Métodos**: Verificação completa para `public` e `private`.
- **Protected**: Aceito pelo parser, mas tratado como `public` em runtime (precisa implementar verificação de herança).

### 2.1.3 Getters e Setters

Permitem controlar o acesso a propriedades com lógica personalizada.

```dryad
class Pessoa {
    private let _nome = "";
    private let _idade = 0;
    
    get nome() {
        return this._nome;
    }
    
    set nome(novoNome) {
        this._nome = novoNome;
    }
    
    get idade() {
        return this._idade;
    }
    
    set idade(novaIdade) {
        if (novaIdade >= 0) {
            this._idade = novaIdade;
        }
    }
}

let p = new Pessoa();
p.nome = "João";      // chama set nome("João")
print(p.nome);        // chama get nome()
p.idade = 25;
```

**Status atual**: ✅ Implementado.

### 2.1.4 Propriedades Estáticas

Propriedades que pertencem à classe, não às instâncias.

```dryad
class Contador {
    static let quantidade = 0;
    
    constructor() {
        Contador.quantidade = Contador.quantidade + 1;
    }
}

print(Contador.quantidade);  // 0
let c1 = new Contador();
let c2 = new Contador();
print(Contador.quantidade);  // 2
```

**Status atual**: ✅ Implementado para propriedades e métodos. Verificação de visibilidade também funciona.

### 2.1.5 Interfaces (Traits)

Contratos que definem um conjunto de métodos que uma classe deve implementar.

```dryad
interface Printable {
    function print();
    function toString();
}

interface Serializable {
    function toJson();
}

class Relatorio implements Printable, Serializable {
    function print() {
        // implementação
    }
    
    function toString() {
        return "Relatório";
    }
    
    function toJson() {
        return "{}";
    }
}
```

**Status atual**: ❌ Não implementado. Sistema de contratos ou tipos abstratos não existe.

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
