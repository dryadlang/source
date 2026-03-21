# SYNTAX MANIFEST — Dryad Language v0.1.0

> **Propósito**: Especificação sintática completa e definitiva da linguagem Dryad baseada **exclusivamente no que está implementado** no código-fonte. Este documento é a autoridade final para toda decisão sintática.  
> **Uso**: Refatoração da sintaxe, validação de features, guia de crescimento futuro.  
> **Data**: 2026-03-21  
> **Status**: Fonte da verdade (source of truth)

---

## Índice

1. [Princípios Fundamentais](#1-princípios-fundamentais)
2. [Palavras-chave (Keywords)](#2-palavras-chave-keywords)
3. [Tokens e Símbolos](#3-tokens-e-símbolos)
4. [Literais e Tipos](#4-literais-e-tipos)
5. [Declaração de Variáveis](#5-declaração-de-variáveis)
6. [Sistema de Tipos (Anotações)](#6-sistema-de-tipos-anotações)
7. [Operadores e Precedência](#7-operadores-e-precedência)
8. [Controle de Fluxo](#8-controle-de-fluxo)
9. [Funções](#9-funções)
10. [Classes e OOP](#10-classes-e-oop)
11. [Pattern Matching](#11-pattern-matching)
12. [Tratamento de Erros](#12-tratamento-de-erros)
13. [Sistema de Módulos](#13-sistema-de-módulos)
14. [Módulos Nativos (Standard Library)](#14-módulos-nativos-standard-library)
15. [Concorrência](#15-concorrência)
16. [Template Strings](#16-template-strings)
17. [Features Quebradas / Não Implementadas](#17-features-quebradas--não-implementadas)
18. [Regras de Crescimento Futuro](#18-regras-de-crescimento-futuro)

---

## 1. Princípios Fundamentais

1. **Tipagem dinâmica** com anotações de tipo opcionais.
2. **Sintaxe inspirada em JavaScript/TypeScript** — familiar para desenvolvedores web.
3. **Semicolons obrigatórios** após statements (exceto antes de `}` ou EOF, onde são opcionais).
4. **Case-sensitive** — `let` ≠ `Let`.
5. **Comentários**: `//` (linha) e `/* ... */` (bloco).
6. **Extensão de arquivo**: `.dryad`.
7. **Encoding**: UTF-8.

---

## 2. Palavras-chave (Keywords)

### 2.1. Lista Completa — 37 keywords

Fonte: `crates/dryad_lexer/src/lexer.rs` linhas 550-554.

Estas são as **únicas** palavras que o lexer reconhece como `Token::Keyword`. Qualquer outra palavra é tokenizada como `Token::Identifier`.

| Categoria | Keywords |
|-----------|----------|
| **Declaração** | `let`, `const` |
| **Controle de fluxo** | `if`, `else`, `for`, `while`, `do`, `break`, `continue`, `match`, `return` |
| **Funções** | `function`, `fn`, `async`, `await` |
| **Classes/OOP** | `class`, `extends`, `new`, `this`, `super`, `static`, `public`, `private`, `protected` |
| **Módulos** | `import`, `export`, `use`, `from`, `as` |
| **Erros** | `try`, `catch`, `finally`, `throw` |
| **Concorrência** | `thread`, `mutex` |
| **Iteração** | `in` |

### 2.2. Valores Especiais (NÃO são keywords)

| Token | Tipo no Lexer |
|-------|---------------|
| `true` | `Token::Boolean(true)` |
| `false` | `Token::Boolean(false)` |
| `null` | `Token::Literal("null")` |

### 2.3. Keywords NÃO implementadas

> ⚠️ As seguintes palavras **NÃO** estão na lista de keywords do lexer e serão tokenizadas como `Token::Identifier`:

| Palavra | Status |
|---------|--------|
| `var` | **NÃO EXISTE** — Docs mencionam mas o lexer não reconhece. Usar `let`. |
| `interface` | **QUEBRADO** — Parser espera `Token::Keyword("interface")` mas lexer gera `Token::Identifier("interface")`. Nunca será alcançado. |
| `implements` | **QUEBRADO** — Mesmo problema que `interface`. |
| `namespace` | **QUEBRADO** — Não está no lexer E não tem suporte no parser (AST existe: `Stmt::Namespace`). |
| `switch` | **NÃO EXISTE** — Mencionado em docs como "em desenvolvimento". Zero código. |

---

## 3. Tokens e Símbolos

### 3.1. Categorias de Token

Fonte: `crates/dryad_lexer/src/token.rs`.

| Token | Descrição |
|-------|-----------|
| `Identifier(String)` | Nomes de variáveis, funções, classes |
| `Number(f64)` | Literais numéricos (sempre f64) |
| `String(String)` | Literais de string (`"..."` ou `'...'`) |
| `Boolean(bool)` | `true` / `false` |
| `Literal(String)` | Literais especiais (`null`) |
| `Keyword(String)` | Palavras-chave reservadas (seção 2.1) |
| `Operator(String)` | Operadores (seção 7) |
| `Symbol(char)` | Símbolos de pontuação |
| `Arrow` | `=>` (para lambdas) |
| `NativeDirective(String)` | `#<module_name>` |
| `TemplateStart` / `TemplateEnd` | `` ` `` (backticks) |
| `TemplateContent(String)` | Conteúdo dentro de template strings |
| `InterpolationStart` / `InterpolationEnd` | `${` e `}` em templates |
| `Eof` | Fim do arquivo |

### 3.2. Símbolos de Pontuação

```
( ) [ ] { } . , ; : =
```

> Nota: `::` NÃO é um token único. São dois `Symbol(':')` consecutivos. O parser interpreta `::` como acesso a namespace/propriedade.

### 3.3. Formato de Números

| Formato | Prefixo | Exemplo |
|---------|---------|---------|
| Decimal | (nenhum) | `42`, `3.14` |
| Binário | `0b` | `0b1010` |
| Octal | `0o` | `0o777` |
| Hexadecimal | `0x` | `0xFF` |

Todos os números são armazenados como `f64`.

### 3.4. Escape Sequences em Strings

```
\n  — newline
\t  — tab
\r  — carriage return
\\  — backslash
\"  — aspas duplas
\'  — aspas simples
\uXXXX — Unicode (4 dígitos hex)
```

---

## 4. Literais e Tipos

### 4.1. Tipos de Valor em Runtime

Fonte: `crates/dryad_runtime/src/value.rs`.

| Tipo | Representação Interna | Exemplo |
|------|----------------------|---------|
| **Number** | `f64` | `42`, `3.14`, `0xFF` |
| **String** | `String` | `"hello"`, `'world'` |
| **Bool** | `bool` | `true`, `false` |
| **Null** | — | `null` |
| **Array** | `HeapId` (heap-allocated) | `[1, 2, 3]` |
| **Tuple** | `HeapId` (heap-allocated) | `(1, "a", true)` |
| **Object** | `HeapId` (heap-allocated) | `{ key: value }` |
| **Function** | Nome, params, body, closure | `function f() {}` |
| **AsyncFunction** | Nome, params, body, closure | `async function f() {}` |
| **ThreadFunction** | Nome, params, body | `thread function f() {}` |
| **Lambda** | `HeapId` | `(x) => x * 2` |
| **Class** | `HeapId` | `class Foo {}` |
| **Instance** | `HeapId` | `new Foo()` |
| **Thread** | `JoinHandle` | Thread em execução |
| **Mutex** | `Arc<Mutex<Value>>` | `mutex()` |
| **Promise** | `Arc<Mutex<PromiseState>>` | Resultado de `async` |
| **Exception** | `String` | Erro lançado com `throw` |
| **Result** | `(bool, Box<Value>)` | Ok/Err do operador `?` |

### 4.2. Literais Sintáticos

```dryad
// Números
42
3.14
0b1010
0o777
0xFF

// Strings
"string com aspas duplas"
'string com aspas simples'
`template string com ${interpolação}`

// Booleanos
true
false

// Null
null

// Array
[1, 2, 3]
[]

// Tuple
(1, "hello", true)

// Object literal
{ nome: "João", idade: 30 }
{
    chave: "valor",
    metodo() { return 42; }
}
```

---

## 5. Declaração de Variáveis

### 5.1. `let` — Variável Mutável

```dryad
let nome;                    // Sem inicialização (valor: null)
let nome = "João";           // Com inicialização
let idade: number = 30;     // Com anotação de tipo
```

- Pode ser reatribuída após declaração.
- Inicialização é opcional.
- Anotação de tipo é opcional.

### 5.2. `const` — Constante

```dryad
const PI = 3.14159;          // Obrigatória inicialização
const NOME: string = "App";  // Com anotação de tipo
```

- **DEVE** ter valor inicial (erro `2013` caso contrário).
- Não pode ser reatribuída.
- Anotação de tipo é opcional.

### 5.3. `var` — NÃO EXISTE

> ⛔ `var` NÃO é uma keyword reconhecida pelo lexer. O README menciona `var` mas é **incorreto**. Use `let` ou `const`.

### 5.4. Atribuição

```dryad
x = 10;              // Atribuição simples
x += 5;              // Atribuição composta
obj.prop = valor;    // Atribuição de propriedade
arr[0] = valor;      // Atribuição por índice
this.prop = valor;   // Atribuição em this
```

Operadores de atribuição composta: `+=`, `-=`, `*=`, `/=`.

---

## 6. Sistema de Tipos (Anotações)

### 6.1. Tipos Disponíveis

Fonte: `crates/dryad_parser/src/ast.rs`, enum `Type`.

| Sintaxe | Tipo AST | Descrição |
|---------|----------|-----------|
| `number` | `Type::Number` | Numérico (f64) |
| `string` | `Type::String` | Cadeia de caracteres |
| `bool` | `Type::Bool` | Booleano |
| `null` | `Type::Null` | Nulo |
| `any` | `Type::Any` | Qualquer tipo |
| `number[]` | `Type::Array(Box<Number>)` | Array tipado |
| `(number, string)` | `Type::Tuple(vec)` | Tupla tipada |
| `fn(number, string) -> bool` | `Type::Function(params, ret)` | Tipo função |
| `NomeClasse` | `Type::Class(String)` | Instância de classe |

### 6.2. Onde Anotações São Aceitas

```dryad
// Variáveis
let x: number = 42;
const y: string = "hello";

// Parâmetros de função
function soma(a: number, b: number): number {
    return a + b;
}

// Lambdas
const fn: fn(number) -> number = (x: number): number => x * 2;
```

### 6.3. Natureza das Anotações

As anotações de tipo são **opcionais** e atualmente servem para documentação. Dryad é uma linguagem de **tipagem dinâmica** — os tipos são verificados em runtime, não em tempo de compilação.

---

## 7. Operadores e Precedência

### 7.1. Tabela de Precedência (menor para maior)

Fonte: `crates/dryad_parser/src/parser.rs`, cadeia de funções de parsing.

| Prec. | Operadores | Associatividade | Função Parser |
|-------|-----------|-----------------|---------------|
| 1 | `=` (expressão) | Direita | `assignment()` |
| 2 | `\|\|` | Esquerda | `logical_or()` |
| 3 | `&&` | Esquerda | `logical_and()` |
| 4 | `\|` (bitwise) | Esquerda | `bitwise_or()` |
| 5 | `^` (bitwise xor) | Esquerda | `bitwise_xor()` |
| 6 | `&` (bitwise and) | Esquerda | `bitwise_and()` |
| 7 | `==`, `!=` | Esquerda | `equality()` |
| 8 | `<`, `<=`, `>`, `>=` | Esquerda | `comparison()` |
| 9 | `<<`, `>>`, `<<<`, `>>>` | Esquerda | `shift()` |
| 10 | `+`, `-` | Esquerda | `term()` |
| 11 | `*`, `/`, `%`, `%%` | Esquerda | `factor()` |
| 12 | `**`, `^^`, `##` | **Direita** | `power()` |
| 13 | `!`, `-` (unário), `++`, `--` (pré) | Direita | `unary()` |
| 14 | `++`, `--` (pós), `[]`, `.`, `()`, `::` | Esquerda | `postfix()` |

### 7.2. Operadores Aritméticos

| Operador | Descrição |
|----------|-----------|
| `+` | Soma / Concatenação de strings |
| `-` | Subtração |
| `*` | Multiplicação |
| `/` | Divisão |
| `%` | Módulo |
| `%%` | Módulo (alternativo) |
| `**` | Potência |

### 7.3. Operadores de Comparação

| Operador | Descrição |
|----------|-----------|
| `==` | Igualdade |
| `!=` | Desigualdade |
| `<` | Menor que |
| `<=` | Menor ou igual |
| `>` | Maior que |
| `>=` | Maior ou igual |

> Nota: NÃO existem `===` ou `!==` (strict equality). Dryad usa `==` e `!=`.

### 7.4. Operadores Lógicos

| Operador | Descrição |
|----------|-----------|
| `&&` | AND lógico |
| `\|\|` | OR lógico |
| `!` | NOT lógico (unário) |

### 7.5. Operadores Bitwise

| Operador | Descrição |
|----------|-----------|
| `&` | AND bitwise |
| `\|` | OR bitwise |
| `^` | XOR bitwise |
| `<<` | Shift esquerda |
| `>>` | Shift direita |
| `<<<` | Shift esquerda (unsigned) |
| `>>>` | Shift direita (unsigned) |

### 7.6. Operadores de Incremento/Decremento

| Operador | Tipo | Descrição |
|----------|------|-----------|
| `++x` | Pré-incremento | Incrementa antes de usar |
| `x++` | Pós-incremento | Usa e depois incrementa |
| `--x` | Pré-decremento | Decrementa antes de usar |
| `x--` | Pós-decremento | Usa e depois decrementa |

### 7.7. Operadores de Atribuição Composta

| Operador | Equivalente |
|----------|-------------|
| `+=` | `x = x + valor` |
| `-=` | `x = x - valor` |
| `*=` | `x = x * valor` |
| `/=` | `x = x / valor` |

### 7.8. Operadores Especiais

| Operador | Descrição |
|----------|-----------|
| `**` | Potência |
| `^^` | Potência (alternativo) |
| `##` | Potência (alternativo) |
| `%%` | Módulo (alternativo) |
| `?` | Try operator (propaga erro em Result) |
| `...` | Spread/Rest |

### 7.9. Operadores de Acesso

| Operador | Descrição | Exemplo |
|----------|-----------|---------|
| `.` | Acesso a propriedade/método | `obj.prop`, `obj.method()` |
| `[]` | Acesso por índice | `arr[0]`, `obj["key"]` |
| `.N` | Acesso a tupla por índice | `tuple.0`, `tuple.1` |
| `::` | Acesso a namespace | `Modulo::funcao` |

---

## 8. Controle de Fluxo

### 8.1. `if` / `else`

```dryad
if (condição) {
    // bloco
}

if (condição) {
    // bloco if
} else {
    // bloco else
}

if (a > b) {
    // ...
} else if (a == b) {
    // ...
} else {
    // ...
}
```

- Parênteses ao redor da condição são **obrigatórios**.
- Chaves são **obrigatórias** para o bloco.

### 8.2. `while`

```dryad
while (condição) {
    // corpo
}
```

### 8.3. `do-while`

```dryad
do {
    // corpo (executa pelo menos uma vez)
} while (condição);
```

### 8.4. `for` (C-style)

```dryad
for (let i = 0; i < 10; i++) {
    // corpo
}
```

- Inicialização, condição e atualização são opcionais.
- Parênteses obrigatórios.

### 8.5. `for-in` (ForEach)

```dryad
for (item in colecao) {
    // corpo
}

for (let item in array) {
    // com declaração
}
```

- Itera sobre elementos de arrays e objetos iteráveis.
- `item` pode ser um pattern (destructuring).

### 8.6. `break` e `continue`

```dryad
while (true) {
    if (condição) break;
    if (outra) continue;
}
```

- Válidos dentro de `while`, `do-while`, `for`, `for-in`.
- Não suportam labels.

### 8.7. `match` (Pattern Matching)

```dryad
match (valor) {
    1 => println("um"),
    2 => println("dois"),
    "hello" => println("saudação"),
    [a, b] => println("array de dois"),
    _ => println("outro"),
}
```

Veja seção 11 para detalhes completos sobre patterns.

---

## 9. Funções

### 9.1. Declaração com `function`

```dryad
function nome(param1, param2) {
    return param1 + param2;
}

// Com tipos
function soma(a: number, b: number): number {
    return a + b;
}

// Com valor padrão
function greet(nome = "Mundo") {
    println("Olá, " + nome);
}

// Com rest params
function soma(...numeros) {
    // numeros é um array
}
```

### 9.2. Declaração com `fn`

```dryad
fn nome(param1, param2) {
    return param1 + param2;
}
```

> `fn` e `function` são **sinônimos exatos**. Ambos produzem o mesmo AST (`Stmt::FunctionDeclaration`). `fn` existe como forma abreviada.

### 9.3. Async Functions

```dryad
async function buscarDados() {
    let resultado = await operacaoAsync();
    return resultado;
}
```

- Declaradas com `async function`.
- Retornam uma `Promise`.
- Dentro delas, `await` pode ser usado.

### 9.4. Lambda (Arrow Functions)

```dryad
// Expressão simples
const dobro = (x) => x * 2;

// Com tipos
const soma = (a: number, b: number): number => a + b;

// Com rest params
const todos = (...args) => args;
```

- **Sintaxe**: `(params) => expressão`
- O corpo é **uma expressão** (não um bloco).
- Parênteses ao redor dos parâmetros são obrigatórios.
- Captura o escopo léxico (closure).

### 9.5. Parâmetros

| Feature | Sintaxe | Exemplo |
|---------|---------|---------|
| Simples | `nome` | `function f(x) {}` |
| Com tipo | `nome: tipo` | `function f(x: number) {}` |
| Com default | `nome = valor` | `function f(x = 10) {}` |
| Com tipo e default | `nome: tipo = valor` | `function f(x: number = 10) {}` |
| Rest | `...nome` | `function f(...args) {}` |

- Rest parameter deve ser o último.
- Apenas um rest parameter por função.

---

## 10. Classes e OOP

### 10.1. Declaração de Classe

```dryad
class Animal {
    nome = "Sem nome";

    constructor(nome) {
        this.nome = nome;
    }

    falar() {
        println("Som genérico");
    }
}
```

### 10.2. Herança

```dryad
class Cachorro extends Animal {
    latir() {
        println("Au au!");
    }

    falar() {
        super.falar();
        this.latir();
    }
}
```

- Herança simples via `extends`.
- `super` acessa métodos e propriedades da classe pai.
- `super.metodo()` chama método do pai.

### 10.3. Instanciação

```dryad
let dog = new Cachorro("Rex");
dog.falar();
```

- `new NomeClasse(args)` cria uma instância.
- O constructor é chamado automaticamente.

### 10.4. Membros de Classe

#### 10.4.1. Propriedades

```dryad
class Exemplo {
    publica = "visível";
    private secreta = "escondida";
    protected meio = "herdável";
    static contador = 0;
}
```

#### 10.4.2. Métodos

```dryad
class Exemplo {
    metodo() { }
    static metodoEstatico() { }
    async metodoAsync() { }
    private metodoPrivado() { }
}
```

#### 10.4.3. Getters e Setters

```dryad
class Pessoa {
    private _nome = "";

    get nome() {
        return this._nome;
    }

    set nome(valor) {
        this._nome = valor;
    }
}
```

### 10.5. Visibilidade

| Modificador | Acesso |
|------------|--------|
| `public` | Acessível de qualquer lugar (padrão) |
| `private` | Acessível apenas dentro da classe |
| `protected` | Acessível na classe e subclasses |

- O padrão (sem modificador) é `public`.

### 10.6. Membros Estáticos

```dryad
class Counter {
    static count = 0;

    static increment() {
        Counter.count++;
    }
}

Counter.increment();
println(Counter.count); // 1
```

### 10.7. `this` e `super`

| Keyword | Uso |
|---------|-----|
| `this` | Referência à instância atual |
| `this.prop` | Acesso a propriedade da instância |
| `this.metodo()` | Chamada de método da instância |
| `super` | Referência à classe pai |
| `super.metodo()` | Chamada de método do pai |
| `super.prop` | Acesso a propriedade do pai |

---

## 11. Pattern Matching

### 11.1. Expressão `match`

```dryad
let resultado = match (valor) {
    pattern1 => corpo1,
    pattern2 => corpo2,
    _ => corpo_default,
};
```

### 11.2. Tipos de Pattern

Fonte: `crates/dryad_parser/src/ast.rs`, enum `Pattern`.

| Pattern | Sintaxe | Descrição |
|---------|---------|-----------|
| **Identifier** | `nome` | Vincula o valor a uma variável |
| **Literal** | `42`, `"texto"`, `true`, `null` | Compara com valor literal |
| **Wildcard** | `_` | Aceita qualquer valor (descarta) |
| **Array** | `[a, b, c]` | Destructuring de array |
| **Tuple** | `(a, b)` | Destructuring de tupla |
| **Object** | `{ chave: padrao }` | Destructuring de objeto |
| **Rest** | `...rest` | Captura restante |

### 11.3. Guards

```dryad
match (x) {
    n if n > 0 => println("positivo"),
    n if n < 0 => println("negativo"),
    _ => println("zero"),
}
```

- Guards adicionam uma condição extra ao pattern.
- Sintaxe: `pattern if condição => corpo`.

### 11.4. Destructuring em `for-in`

```dryad
let pares = [(1, "um"), (2, "dois")];
for ((num, nome) in pares) {
    println(num + ": " + nome);
}
```

### 11.5. Spread/Rest Operator

```dryad
// Spread em arrays
let a = [1, 2, 3];
let b = [...a, 4, 5];

// Rest em patterns
let [primeiro, ...resto] = array;

// Spread em argumentos
funcao(...args);
```

---

## 12. Tratamento de Erros

### 12.1. `try` / `catch` / `finally`

```dryad
try {
    // código que pode falhar
} catch (erro) {
    // tratamento do erro
} finally {
    // sempre executa
}
```

- `catch` é opcional (mas pelo menos `catch` ou `finally` deve existir).
- `finally` é opcional.
- O parâmetro de `catch` é um nome de variável simples.

### 12.2. `throw`

```dryad
throw "Erro genérico";
throw excecao;
```

- `throw` aceita qualquer expressão.
- Cria um valor `Exception(String)` em runtime.

### 12.3. Operador `?` (Try Expression)

```dryad
let valor = operacaoQueRetornaResult()?;
```

- `?` é um operador postfix (Expr::Try na AST).
- Propaga erro em valores `Result`.
- `Result` é representado como `Value::Result(bool, Box<Value>)`:
  - `true` = Ok(valor)
  - `false` = Err(valor)

---

## 13. Sistema de Módulos

### 13.1. `import` (ES6-style)

```dryad
// Named import
import { func1, func2 } from "modulo";

// Namespace import
import * as utils from "modulo";

// Side-effect import
import "modulo";
```

Fonte AST: `Stmt::Import(ImportKind, String, SourceLocation)`.

| ImportKind | Sintaxe |
|------------|---------|
| `Named(Vec<String>)` | `import { a, b } from "mod"` |
| `Namespace(String)` | `import * as nome from "mod"` |
| `SideEffect` | `import "mod"` |

### 13.2. `export`

```dryad
export function minhaFunc() { }
export class MinhaClasse { }
export const VALOR = 42;
```

- `export` aceita qualquer statement válido.
- Wraps o statement em `Stmt::Export(Box<Stmt>)`.

### 13.3. `use` (Import Simplificado)

```dryad
use "./utils/helper.dryad";
```

- Import legado/simplificado.
- Carrega e executa o módulo referenciado.
- Path é relativo ao arquivo atual.

### 13.4. Módulos Nativos — `#<module>`

```dryad
#<file_io>
#<http_client>
#<crypto>
```

- Diretiva especial no topo do arquivo.
- Registra funções nativas no escopo global.
- `#` seguido de `<nome_modulo>`.
- Tokenizado como `Token::NativeDirective(String)`.

---

## 14. Módulos Nativos (Standard Library)

### 14.1. Lista Completa — 18 módulos

Fonte: `crates/dryad_runtime/src/native_modules/`.

---

#### `#<console_io>` — Entrada/Saída no Console

```
print(value)               — Imprime sem newline
println(value)             — Imprime com newline
input(prompt)              — Lê linha do stdin
native_input()             — Input nativo
native_input_char()        — Lê um caractere
native_input_bytes()       — Lê bytes
native_input_timeout(ms)   — Input com timeout
native_print(value)        — Print nativo
native_println(value)      — Println nativo
native_write_stdout(value) — Escreve em stdout
native_flush()             — Flush de stdout
```

---

#### `#<terminal_ansi>` — Terminal ANSI

```
native_clear_screen()           — Limpa tela
native_move_cursor(x, y)        — Move cursor
native_set_color(color)         — Define cor
native_set_style(style)         — Define estilo
native_reset_style()            — Reseta estilo
native_hide_cursor()            — Esconde cursor
native_show_cursor()            — Mostra cursor
native_terminal_size()          — Tamanho do terminal
ansi_red(text)                  — Texto em vermelho
```

---

#### `#<binary_io>` — I/O Binário

```
native_write_bytes(path, bytes)      — Escreve bytes
native_append_bytes(path, bytes)     — Anexa bytes
native_overwrite_chunk(path, offset, bytes) — Sobrescreve chunk
native_read_bytes(path)              — Lê bytes
native_read_chunk(path, offset, len) — Lê chunk
native_file_size(path)               — Tamanho do arquivo
to_hex(bytes)                        — Converte para hex
```

---

#### `#<file_io>` — Sistema de Arquivos

```
// Síncronos
native_read_file(path)     — Lê arquivo
native_write_file(path, content) — Escreve arquivo
native_append_file(path, content) — Anexa ao arquivo
native_file_exists(path)   — Verifica existência
native_is_dir(path)        — Verifica se é diretório
native_list_dir(path)      — Lista diretório
native_mkdir(path)         — Cria diretório
native_remove_file(path)   — Remove arquivo
native_remove_dir(path)    — Remove diretório

// Aliases simplificados
read_file(path)            — Alias de native_read_file
write_file(path, content)  — Alias de native_write_file
file_exists(path)          — Alias de native_file_exists
is_dir(path)               — Alias de native_is_dir
list_dir(path)             — Alias de native_list_dir
mkdir(path)                — Alias de native_mkdir
remove_file(path)          — Alias de native_remove_file
remove_dir(path)           — Alias de native_remove_dir

// Assíncronos
async_read_file(path)      — Leitura assíncrona
async_write_file(path, content) — Escrita assíncrona
async_append_file(path, content) — Append assíncrono
```

---

#### `#<time>` (alias: `#<date_time>`) — Tempo

```
native_now()           — Timestamp atual (ms)
native_sleep(ms)       — Pausa execução
native_timestamp()     — Timestamp Unix
native_date()          — Data formatada
native_time()          — Hora formatada
native_format_date(fmt) — Formata data
native_uptime()        — Uptime do processo
current_timestamp()    — Alias de native_timestamp
```

---

#### `#<system_env>` — Sistema/Ambiente

```
native_platform()      — Plataforma (linux, macos, windows)
native_arch()          — Arquitetura (x86_64, aarch64)
native_env(key)        — Lê variável de ambiente
native_set_env(k, v)   — Define variável de ambiente
native_exec(cmd)       — Executa comando do sistema
native_exec_output(cmd) — Executa e retorna output
native_pid()           — PID do processo
native_exit(code)      — Encerra processo
get_current_dir()      — Diretório atual
```

---

#### `#<encode_decode>` — Codificação/Decodificação

```
native_json_encode(value)  — Objeto → JSON string
native_json_decode(str)    — JSON string → Objeto
native_csv_encode(data)    — Dados → CSV
native_csv_decode(str)     — CSV → Dados
native_xml_encode(data)    — Dados → XML
native_xml_decode(str)     — XML → Dados
```

---

#### `#<crypto>` — Criptografia

```
sha256(data)                        — Hash SHA-256
native_hash_md5(data)               — Hash MD5
native_uuid()                       — Gera UUID v4
native_base64_encode(data)          — Codifica Base64
native_base64_decode(data)          — Decodifica Base64
native_hex_encode(data)             — Codifica Hex
native_hex_decode(data)             — Decodifica Hex
native_random_bytes(n)              — N bytes aleatórios
native_random_string(n)             — String aleatória
native_random_seed(seed)            — Define seed
native_bytes_to_string(bytes)       — Bytes → String
native_encrypt_aes(data, key)       — Criptografa AES
native_decrypt_aes(data, key)       — Decriptografa AES
native_encrypt_rsa(data, pub_key)   — Criptografa RSA
native_decrypt_rsa(data, priv_key)  — Decriptografa RSA
native_sign(data, key)              — Assinatura digital
native_verify(data, sig, key)       — Verifica assinatura
native_generate_rsa_keypair(bits)   — Gera par RSA
native_hmac_sha256(data, key)       — HMAC SHA-256
native_hmac_sha512(data, key)       — HMAC SHA-512
```

---

#### `#<debug>` — Debug e Testes

```
debug(value)                         — Debug output
log(value)                           — Log output
native_typeof(value)                 — Tipo do valor
native_memory_usage()                — Uso de memória
native_stack_trace()                 — Stack trace
native_perf_start(label)             — Inicia timer
native_perf_end(label)               — Finaliza timer
native_assert(condition)             — Assert booleano
native_assert_equal(a, b)            — Assert igualdade
native_assert_not_equal(a, b)        — Assert desigualdade
native_assert_true(value)            — Assert verdadeiro
native_assert_false(value)           — Assert falso
native_assert_type(value, type_str)  — Assert tipo
native_test_regex(pattern, input)    — Testa regex
```

---

#### `#<utils>` — Utilitários

```
native_eval(code)              — Avalia código Dryad
native_clone(value)            — Clona valor
native_watch_file(path, cb)    — Observa arquivo
native_random_int(min, max)    — Inteiro aleatório
native_random_float(min, max)  — Float aleatório
native_random_string(len)      — String aleatória
native_random_bytes(n)         — Bytes aleatórios
native_random_seed(seed)       — Define seed
native_regex_match(pat, str)   — Match regex
native_regex_replace(pat, repl, str) — Replace regex
native_regex_split(pat, str)   — Split por regex
native_regex_test(pat, str)    — Testa regex
```

---

#### `#<http_client>` — Cliente HTTP

```
native_http_get(url)                  — GET request
native_http_post(url, body)           — POST request
native_http_headers(url, headers)     — Request com headers
native_http_download(url, path)       — Download arquivo
native_http_status(url)               — Status code
native_http_json(url)                 — GET → parse JSON
// + ~20 funções de configuração (timeout, proxy, auth, etc.)
```

---

#### `#<http_server>` — Servidor HTTP

```
native_http_server_create(config)     — Cria servidor
native_http_server_start(server)      — Inicia servidor
native_http_server_stop(server)       — Para servidor
native_http_server_status(server)     — Status do servidor
native_http_server_route(s, method, path, handler) — Define rota
native_http_server_get(s, path, handler)   — Rota GET
native_http_server_post(s, path, handler)  — Rota POST
native_http_server_put(s, path, handler)   — Rota PUT
native_http_server_delete(s, path, handler) — Rota DELETE
native_http_server_static(s, path, dir)    — Arquivos estáticos
native_http_server_file(s, path, file)     — Serve arquivo
native_http_server_html(s, path, html)     — Serve HTML
native_http_server_json(s, path, data)     — Serve JSON
native_http_server_cors(s, config)         — Configura CORS
native_http_server_middleware(s, fn)        — Adiciona middleware
```

---

#### `#<tcp>` — TCP

```
tcp_server_create(config)    — Cria servidor TCP
tcp_server_start(server)     — Inicia servidor
tcp_server_stop(server)      — Para servidor
tcp_server_status(server)    — Status
tcp_client_create(config)    — Cria cliente TCP
tcp_client_connect(client)   — Conecta
tcp_client_disconnect(client) — Desconecta
tcp_client_send(client, data) — Envia dados
tcp_client_receive(client)   — Recebe dados
tcp_client_status(client)    — Status
tcp_resolve_hostname(host)   — Resolve hostname
tcp_get_local_ip()           — IP local
```

---

#### `#<udp>` — UDP

```
udp_server_create(config)         — Cria servidor UDP
udp_server_start(server)          — Inicia servidor
udp_server_stop(server)           — Para servidor
udp_server_status(server)         — Status
udp_client_create(config)         — Cria cliente UDP
udp_client_bind(client, addr)     — Bind a endereço
udp_client_send(client, data)     — Envia dados
udp_client_receive(client)        — Recebe dados
udp_client_send_to(client, addr, data) — Envia para endereço
udp_client_receive_from(client)   — Recebe com endereço
udp_client_status(client)         — Status
udp_client_set_timeout(client, ms) — Define timeout
udp_client_close(client)          — Fecha cliente
udp_resolve_hostname(host)        — Resolve hostname
udp_get_local_ip()                — IP local
udp_port_available(port)          — Verifica porta disponível
```

---

#### `#<ffi>` — Foreign Function Interface

```
ffi_load_library(path)           — Carrega biblioteca nativa
ffi_unload_library(handle)       — Descarrega biblioteca
ffi_call(handle, func, args)     — Chama função nativa
ffi_get_symbol(handle, name)     — Obtém símbolo
ffi_list_libraries()             — Lista bibliotecas carregadas
```

---

#### `#<json_stream>` — JSON Streaming

```
json_parse_incremental(chunk)    — Parse incremental
json_parse_stream(stream)        — Parse de stream
json_create_parser()             — Cria parser
json_parser_feed(parser, data)   — Alimenta parser
json_parser_done(parser)         — Finaliza parser
json_encoder_create(config)      — Cria encoder
json_encoder_encode(encoder, data) — Codifica dados
```

---

#### `#<websocket>` — WebSocket

```
ws_connect(url)             — Conecta a WebSocket
ws_send(conn, data)         — Envia dados
ws_receive(conn)            — Recebe dados
ws_close(conn)              — Fecha conexão
ws_server_create(config)    — Cria servidor WS
ws_server_start(server)     — Inicia servidor
ws_server_stop(server)      — Para servidor
ws_server_status(server)    — Status do servidor
```

---

#### `#<database>` — Banco de Dados

```
// SQLite
sqlite_open(path)                    — Abre database
sqlite_close(db)                     — Fecha database
sqlite_execute(db, sql, params)      — Executa SQL
sqlite_query(db, sql, params)        — Query SQL

// PostgreSQL
pg_connect(connection_string)        — Conecta ao Postgres
pg_execute(conn, sql, params)        — Executa SQL
pg_query(conn, sql, params)          — Query SQL
pg_close(conn)                       — Fecha conexão
```

---

## 15. Concorrência

### 15.1. `async` / `await`

```dryad
async function buscar() {
    let dados = await fetch("url");
    return dados;
}

let promessa = buscar();
let resultado = await promessa;
```

- `async function` retorna uma `Promise`.
- `await` pausa a execução até a Promise resolver.
- `await` só pode ser usado dentro de funções `async` ou no top-level.

### 15.2. `thread function`

```dryad
thread function tarefa(dados) {
    // Executa em thread nativa do SO
    return resultado;
}

let handle = tarefa("input");
```

- Cria uma thread real do sistema operacional.
- Declarada com `thread function nome(params) { ... }`.
- Retorna um `Thread` handle.

### 15.3. `thread()` (chamada inline)

```dryad
let handle = thread(funcao, arg1, arg2);
```

- Executa uma função existente em nova thread.
- AST: `Expr::ThreadCall(func, args)`.

### 15.4. `mutex()`

```dryad
let lock = mutex();
```

- Cria um mutex para sincronização entre threads.
- AST: `Expr::MutexCreation`.
- Runtime: `Value::Mutex(Arc<Mutex<Value>>)`.

---

## 16. Template Strings

```dryad
let nome = "João";
let msg = `Olá, ${nome}! Hoje é ${native_date()}.`;
```

- Delimitadas por backticks (`` ` ``).
- Interpolação com `${ expressão }`.
- Podem conter múltiplas linhas.
- Expressões arbitrárias dentro de `${}`.

---

## 17. Features Quebradas / Não Implementadas

### 17.1. ⛔ QUEBRADO — `interface` / `implements`

**Problema**: O parser (linha 123-124) verifica `Token::Keyword("interface")`, mas o lexer (linhas 550-554) **não inclui** `interface` na lista de keywords. O lexer gera `Token::Identifier("interface")`, que nunca match no parser.

**AST existe**: `Stmt::InterfaceDeclaration`, `InterfaceMember`, `InterfaceMethod` — todos definidos em ast.rs.

**Solução necessária**: Adicionar `"interface"` e `"implements"` à lista de keywords no lexer.

### 17.2. ⛔ QUEBRADO — `namespace`

**Problema**: 
1. `namespace` não está na lista de keywords do lexer.
2. O parser **não tem** handler para `namespace` (grep retorna zero resultados).
3. AST existe: `Stmt::Namespace(String, Vec<Stmt>, SourceLocation)`.

**Solução necessária**: Adicionar `"namespace"` ao lexer + implementar parsing.

### 17.3. ⛔ NÃO EXISTE — `var`

**Problema**: Documentação e README mencionam `var` como declaração de variável, mas `var` **nunca existiu** como keyword. O lexer só reconhece `let` e `const`.

**Solução**: Remover todas as referências a `var` da documentação. Usar `let`.

### 17.4. ⛔ NÃO EXISTE — `switch`

**Problema**: Documentação menciona `switch` como "em desenvolvimento". Zero evidência no código. Use `match` como alternativa.

### 17.5. ⚠️ Limitações Conhecidas

- **`fn` vs `function`**: Ambos funcionam identicamente. Considerar padronizar para um.
- **Operadores duplicados**: `**`, `^^`, `##` fazem a mesma coisa (potência). `%%` e `%` fazem a mesma coisa (módulo). Considerar remover redundâncias.
- **`::` como acesso a namespace**: Funciona mas é implementado como dois `Symbol(':')` — frágil.
- **Anotações de tipo**: Parsed e armazenadas no AST, mas não validadas em runtime (pura documentação).

---

## 18. Regras de Crescimento Futuro

### 18.1. Regras para Adicionar Keywords

1. **SEMPRE** adicionar à lista de keywords no lexer (`lexer.rs` linhas 550-554).
2. **SEMPRE** adicionar handler no parser (`parser.rs` função `statement()`).
3. **SEMPRE** adicionar variante ao AST (`ast.rs`).
4. **SEMPRE** implementar no runtime/interpreter.
5. **SEMPRE** adicionar testes.
6. **NUNCA** adicionar ao AST sem lexer + parser (como `namespace`).

### 18.2. Regras para Adicionar Operadores

1. Adicionar tokenização no lexer (`next_token()` match).
2. Definir precedência — inserir na cadeia de funções do parser no nível correto.
3. Implementar avaliação no runtime.
4. Documentar na tabela de precedência (seção 7.1).

### 18.3. Regras para Adicionar Módulos Nativos

1. Criar arquivo em `crates/dryad_runtime/src/native_modules/`.
2. Registrar no `mod.rs` do diretório.
3. Funções seguem o padrão `native_nome(args: Vec<Value>) -> Result<Value, DryadError>`.
4. Documentar neste manifesto (seção 14).

### 18.4. Regras de Sintaxe

1. **Consistência**: Toda nova feature deve seguir os padrões sintáticos existentes.
2. **Sem ambiguidade**: Toda construção deve ter exatamente uma interpretação possível.
3. **Semicolons**: Obrigatórios após statements (exceto antes de `}` e EOF).
4. **Chaves**: Obrigatórias para blocos de código (sem blocos single-line sem chaves).
5. **Parênteses**: Obrigatórios em condições de `if`, `while`, `for`.
6. **Naming**: Keywords em minúsculas, sem underscores.
7. **Strings**: Aspas duplas (`"`) ou simples (`'`), sem diferença semântica. Templates com backtick (`` ` ``).
8. **Comentários**: `//` e `/* */` apenas. Sem `#` para comentários.

### 18.5. Checklist para Novas Features

- [ ] Keyword adicionada ao lexer (se aplicável)
- [ ] Token/Operador adicionado ao lexer (se aplicável)
- [ ] AST node definido
- [ ] Parser handler implementado
- [ ] Runtime evaluation implementada
- [ ] Testes unitários escritos
- [ ] Documentação atualizada neste manifesto
- [ ] Sem ambiguidade com sintaxe existente
- [ ] Sem conflito com keywords existentes

---

## Apêndice A — Gramática Resumida (EBNF simplificado)

```ebnf
program         = { statement } ;

statement       = var_decl | const_decl | assignment | if_stmt | while_stmt
                | do_while_stmt | for_stmt | foreach_stmt | break_stmt
                | continue_stmt | try_stmt | throw_stmt | return_stmt
                | func_decl | async_func_decl | thread_func_decl
                | class_decl | export_stmt | import_stmt | use_stmt
                | native_directive | block | expr_stmt ;

var_decl        = "let" IDENTIFIER [ ":" type ] [ "=" expression ] ";" ;
const_decl      = "const" IDENTIFIER [ ":" type ] "=" expression ";" ;
assignment      = IDENTIFIER "=" expression ";"
                | IDENTIFIER ("+=" | "-=" | "*=" | "/=") expression ";" ;

if_stmt         = "if" "(" expression ")" block [ "else" (if_stmt | block) ] ;
while_stmt      = "while" "(" expression ")" block ;
do_while_stmt   = "do" block "while" "(" expression ")" ";" ;
for_stmt        = "for" "(" [var_decl | assignment] ";" [expression] ";" [statement] ")" block ;
foreach_stmt    = "for" "(" pattern "in" expression ")" block ;

func_decl       = ("function" | "fn") IDENTIFIER "(" params ")" [ ":" type ] block ;
async_func_decl = "async" "function" IDENTIFIER "(" params ")" block ;
thread_func_decl = "thread" "function" IDENTIFIER "(" params ")" block ;

class_decl      = "class" IDENTIFIER [ "extends" IDENTIFIER ] "{" { class_member } "}" ;
class_member    = [visibility] ["static"] ["async"] method | property | getter | setter ;
visibility      = "public" | "private" | "protected" ;

import_stmt     = "import" "{" IDENTIFIER { "," IDENTIFIER } "}" "from" STRING ";"
                | "import" "*" "as" IDENTIFIER "from" STRING ";"
                | "import" STRING ";" ;
export_stmt     = "export" statement ;
use_stmt        = "use" STRING ";" ;

native_directive = "#<" IDENTIFIER ">" ;

expression      = assignment_expr ;
assignment_expr = logical_or [ "=" assignment_expr ] ;
logical_or      = logical_and { "||" logical_and } ;
logical_and     = bitwise_or { "&&" bitwise_or } ;
bitwise_or      = bitwise_xor { "|" bitwise_xor } ;
bitwise_xor     = bitwise_and { "^" bitwise_and } ;
bitwise_and     = equality { "&" equality } ;
equality        = comparison { ("==" | "!=") comparison } ;
comparison      = shift { ("<" | "<=" | ">" | ">=") shift } ;
shift           = term { ("<<" | ">>" | "<<<" | ">>>") term } ;
term            = factor { ("+" | "-") factor } ;
factor          = power { ("*" | "/" | "%" | "%%") power } ;
power           = unary { ("**" | "^^" | "##") unary } ;
unary           = ("!" | "-" | "++" | "--") unary | postfix ;
postfix         = primary { "++" | "--" | "[" expression "]" | "." member | "(" args ")" | "::" IDENTIFIER } ;

primary         = NUMBER | STRING | BOOLEAN | "null" | IDENTIFIER | "this" | "super"
                | "await" unary | "mutex" "(" ")" | "new" IDENTIFIER "(" args ")"
                | "match" "(" expression ")" "{" { match_arm } "}"
                | "(" expression ")" | "[" expressions "]" | "{" object_props "}"
                | "(" params ")" "=>" expression
                | template_string ;

pattern         = IDENTIFIER | LITERAL | "_" | "[" patterns "]" | "(" patterns ")" | "{" obj_patterns "}" | "..." IDENTIFIER ;
match_arm       = pattern [ "if" expression ] "=>" statement ;
type            = "number" | "string" | "bool" | "null" | "any" | type "[]" | "(" types ")" | "fn" "(" types ")" "->" type | IDENTIFIER ;
```

---

## Apêndice B — Códigos de Erro Relevantes

| Código | Categoria | Descrição |
|--------|-----------|-----------|
| 1001 | Léxico | Caractere inesperado |
| 1004 | Léxico | Formato de número inválido |
| 1005 | Léxico | Escape sequence inválida |
| 2003 | Parser | Esperado `;` |
| 2008 | Parser | Atribuição inválida em expressão |
| 2011 | Parser | Esperado nome após `let` |
| 2012 | Parser | Esperado nome após `const` |
| 2013 | Parser | Constante sem valor inicial |
| 2029-2030 | Parser | Erro em `mutex()` |
| 2033 | Parser | Esperado statement |
| 2071-2076 | Parser | Erros de acesso (array, propriedade, chamada) |
| 2080-2083 | Parser | Erros de `super` e `::` |
| 2090-2091 | Parser | Erros de `new` |

---

*Documento gerado a partir da análise direta do código-fonte das crates `dryad_lexer`, `dryad_parser`, e `dryad_runtime`. Toda informação foi verificada contra a implementação real.*
