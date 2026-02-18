---
title: "Utilidades Gerais"
description: "JSON, Random, Base64 e outras ferramentas utilitárias."
category: "Bibliotecas Padrão"
subcategory: "Utilitários"
order: 39
---

# Utilidades Gerais (Utils)

Este módulo centraliza funções transversais essenciais para manipulação de dados, tempo e utilitários de sistema.

## 🚀 Leitura Rápida

- **JSON**: Suporte nativo a parsing e stringify de alta performance.
- **Tempo**: Suspensão de execução via `sleep`.
- **Random**: Geração de números pseudo-aleatórios.
- **Ativação**: Disponível globalmente ou via `#<utils>`.

---

## ⚙️ Visão Técnica

O módulo `utils` integra algumas das crates mais estáveis do ecossistema Rust no runtime do Dryad.

### 1. Serialização JSON

A função `json_stringify` converte a árvore de `Value` para o formato de intercâmbio JSON. Utiliza a crate **Serde JSON**, garantindo segurança contra ataques de profundidade de aninhamento e performance de ponta.

### 2. Funções de Tempo

`sleep(ms)` suspende a execução da fibra atual de forma síncrona. No motor Dryad, isso é implementado mapeando o tempo de interrupção diretamente para as primitivas de thread do sistema operacional (ou o timer da Event Loop no futuro).

---

## 📚 Referências e Paralelos

- **JSON Engine**: [Serde JSON Crate](https://docs.rs/serde_json/latest/serde_json/).
- **Crypto**: [RustCrypto Project](https://github.com/RustCrypto).
- **JS Comparison**: Similar ao objeto `JSON` e às funções globais `setTimeout` (comportamento síncrono).

---

## Principais Funções

### `native_eval(code: string): any`

Executa dinamicamente uma expressão Dryad passada como string. Útil para cálculos simples ou parsing de tipos básicos.

### `native_clone(obj: any): any`

Cria uma **cópia profunda** (deep clone) de qualquer valor, incluindo Arrays e Objetos aninhados, garantindo que não haja referências compartilhadas.

### `native_random_int(min: number, max: number): number`

Gera um número inteiro aleatório entre `min` e `max` (inclusive).

### `native_random_float(min: number, max: number): number`

Gera um número de ponto flutuante aleatório no intervalo especificado.

### `native_random_string(length: number, charset: string): string`

Gera uma string aleatória com o comprimento e conjunto de caracteres fornecidos.

### `native_regex_match(pattern: string, text: string): [string] | null`

Busca todas as capturas de uma regex no texto. Retorna um array com os grupos encontrados ou `null`.

### `native_regex_replace(pattern: string, replacement: string, text: string): string`

Substitui todas as ocorrências do padrão pelo texto de substituição.

### `native_watch_file(path: string): number`

Inicia um observador (watcher) no caminho especificado. O ID retornado pode ser usado para gerenciar a observação.

---

## Exemplo de Uso

```dryad
#utils

let id = native_random_string(8, "0123456789ABCDEF");
println("ID Gerado: " + id);

let dados = { valor: 10 };
let copia = native_clone(dados);
copia.valor = 20;

println(dados.valor); // Continua sendo 10
```
