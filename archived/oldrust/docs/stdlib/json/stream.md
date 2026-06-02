---
title: "JSON Stream"
description: "Parsing e codificação de JSON incremental e em streaming."
category: "Bibliotecas Padrão"
subcategory: "JSON"
order: 36
---

# JSON Stream (Incremental Parsing)

Módulo especializado para processamento de JSON de alta performance e baixa latência através de processamento incremental.

## 🚀 Leitura Rápida

- **Incemental**: Não precisa de todo o JSON na memória; processe por pedaços (chunks).
- **Encoder**: Serialização rápida de tipos Dryad para Strings JSON.
- **Streaming**: Ideal para integração com Sockets e Fluxos de I/O.
- **Ativação**: Chamado via diretiva `#json_stream`.

---

## ⚙️ Visão Técnica

Diferente do `json_parse` global (que carrega tudo na RAM), o `json_stream` utiliza uma **State Machine** interna para manter o estado do parsing entre chamadas.

## Referência de Funções

### `json_create_parser(): object`

Cria um novo objeto de estado para o parser incremental.

### `json_parser_feed(parser: object, chunk: string): any`

Alimenta o parser com um pedaço (chunk) de JSON. Se o chunk completar um objeto válido, ele é retornado. Caso contrário, retorna `null`.

### `json_parse_incremental(json_string: string): any`

Versão simplificada que faz o parse de uma string JSON completa usando o motor incremental.

### `json_parse_stream(chunks: [string]): any`

Recebe um array de pedaços de string e os combina para realizar o parse final.

### `json_encoder_create(): object`

Cria um objeto configurável para codificação JSON. Suporta a propriedade `.pretty` (bool).

### `json_encoder_encode(encoder: object, data: any): string`

Codifica os dados para uma string JSON usando as configurações do encoder.

---

## Exemplo de Uso

```dryad
#<json_stream>

let parser = json_create_parser();

// Simulando chunks de rede
json_parser_feed(parser, '{"id": 123,');
json_parser_feed(parser, '"status": "ok"}');

let resultado = json_parser_feed(parser, ""); // Completa o parse
println(resultado.status); // "ok"
```
