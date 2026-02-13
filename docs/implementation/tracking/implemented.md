---
title: "Funcionalidades Atuais"
description: "Checklist de tudo que já está implementado e estável na linguagem."
category: "Projeto"
order: 3
---

# Funcionalidades Implementadas (Implemented Features)

**Versão Base**: Dryad v1.0
**Status**: ✅ Production Ready

## 1. Core Language (Linguagem Base)

### 1.1 Variáveis e Tipos

- [x] Declaração de variáveis mutáveis (`let`)
- [x] Declaração de constantes imutáveis (`const`)
- [x] Tipos Primitivos: `Number` (f64), `String` (UTF-8), `Boolean`, `Null`
- [x] Tipos Compostos: `Array` (Listas dinâmicas), `Object` (Mapas chave-valor), `Tuple` (Sequências fixas)
- [x] Interpolação de Strings (Concatenação com `+`)

### 1.2 Operadores

- [x] Aritméticos: `+`, `-`, `*`, `/`, `%`
- [x] Aritméticos Estendidos: `**` (Potência), `%%` (Módulo Positivo), `^^` (Raiz), `##` (Base 10)
- [x] Comparação: `==`, `!=`, `<`, `>`, `<=`, `>=`
- [x] Lógicos: `&&`, `||`, `!`
- [x] Bitwise: `&`, `|`, `^`, `~`, `<<`, `>>`, `>>>`, `<<<`
- [x] Atribuição: `=`, `+=`, `-=`, `*=`, `/=`, `%=`
- [x] Unários: `++` (Pre/Post), `--` (Pre/Post), `-` (Negativo)

### 1.3 Controle de Fluxo

- [x] Condicional: `if`, `else if`, `else`
- [x] Loops: `while`, `do-while`
- [x] Loops: `for` (estilo C: init; cond; step)
- [x] Loops: `for ... in` (Iteração sobre arrays/objetos)
- [x] Controle: `break`, `continue`
- [x] Exceções: `try`, `catch`, `finally`, `throw`

## 2. Funções e Modularidade

### 2.1 Funções

- [x] Declaração nomeada (`function foo() {}`)
- [x] Expressões de função (`let foo = function() {}`)
- [x] Arrow Functions / Lambdas (`(x) => x * 2`)
- [x] Retorno implícito (null) e explícito (`return`)
- [x] Recursão suporte
- [x] Closures (Captura de escopo)

### 2.2 Orientação a Objetos

- [x] Declaração de Classes (`class Foo {}`)
- [x] Construtores (`constructor() {}`)
- [x] Métodos de Instância
- [x] Métodos Estáticos (`static foo() {}`)
- [x] Propriedades com inicialização padrão
- [x] Herança Simples (`extends Parent`)
- [x] Instanciação (`new Foo()`)
- [x] Acesso a membros (`this.prop`)

### 2.3 Módulos

- [x] Importação (`import { foo } from "bar"`)
- [x] Exportação (`export function foo() {}`)
- [x] Namespaces (`import * as sys from "sys"`)
- [x] Uso de módulos nativos via diretiva (`#<module>`)

## 3. Runtime e Concorrência

### 3.1 Concorrência

- [x] Threads Nativas (`thread function`)
- [x] Async/Await (`async function`, `await promise`)
- [x] Mutex (`mutex()`)

### 3.2 Biblioteca Padrão (Native Modules)

- [x] `console_io`: print, println, input
- [x] `file_io`: read, write, append, delete, exists, mkdir, list_dir
- [x] `system_env`: platform, arch, env vars, exec, pid
- [x] `http_client`: get, post, download (via reqwest)
- [x] `tcp`: client sockets
- [x] `utils`: random, sleep, base64, json, sha256, uuid
- [x] `time`: timestamp, delay
- [x] `crypto`: hash functions

## 4. Tooling (Ferramentas)

- [x] CLI (`dryad`)
- [x] Subcomandos: `run`, `check`, `tokens`, `repl`
- [x] Mensagens de erro com contexto visual (Source snippets)
- [x] Sistema de URLs de erro (e.g., `error_urls.rs`)
