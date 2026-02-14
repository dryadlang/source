# JSON Stream

Módulo para parsing e codificação de JSON incremental e em streaming.

## Ativação

```dryad
#<json_stream>
```

## Funções

### json_parse_incremental(data: string) -> value

Parseia uma string JSON de forma incremental.

```dryad
#<json_stream>
let data = json_parse_incremental('{"key": "value"}');
```

### json_parse_stream(chunks: array) -> value

Parseia JSON de um array de chunks (streaming).

```dryad
#<json_stream>
let chunks = ['{"key": ', '"value"}'];
let data = json_parse_stream(chunks);
```

### json_create_parser() -> parser

Cria um objeto parser de JSON para parsing incremental.

```dryad
#<json_stream>
let parser = json_create_parser();
```

### json_parser_feed(parser: object, chunk: string) -> value

alimenta o parser com um chunk de dados JSON.

```dryad
#<json_stream>
let result = json_parser_feed(parser, '{"name": "');
```

### json_parser_done(parser: object) -> boolean

Finaliza o parsing e retorna true se bem-sucedido.

### json_encoder_create() -> encoder

Cria um encoder de JSON.

### json_encoder_encode(encoder: object, value: value) -> string

Codifica um valor Dryad para JSON.

```dryad
#<json_stream>
let encoder = json_encoder_create();
let json = json_encoder_encode(encoder, my_object);
```
