# Documentação da Linguagem Dryad

Documentação oficial da linguagem Dryad. Tudo aqui reflete o que está **efetivamente implementado** no código-fonte.

Para documentação interna de desenvolvimento (manuais técnicos, planos futuros, roadmap), veja a pasta [`develop/`](../develop/).

---

## Referência da Linguagem

- [Visão Geral da Sintaxe](syntax.md) — Keywords, estrutura básica, comentários
- [Guia Detalhado de Sintaxe](syntax/syntax.md) — Gramática completa, organização de código
- [Variáveis e Escopo](syntax/variables/declarations.md) — `let`, `const`, escopo, shadowing
- [Tipos de Dados](syntax/types/data_types.md) — Number, String, Bool, Null, Array, Tuple, Object
- [Funções](syntax/functions/definitions.md) — `function`, `fn`, `async function`, `thread function`, lambdas
- [Classes e OOP](syntax/classes/oop.md) — Herança, visibilidade, static, getters/setters
- [Controle de Fluxo](syntax/control_flow/statements.md) — if, while, do-while, for, for-in
- [Pattern Matching](syntax/control_flow/match.md) — `match`, destructuring, guards
- [Operadores](syntax/operators/operators.md) — Aritméticos, lógicos, bitwise, precedência

## Erros

- [Catálogo de Erros](errors/error_codes.md) — Todos os códigos de erro por categoria
- [Guia de Erros](errors/errors.md) — Erros comuns e como resolvê-los

## Biblioteca Padrão (Módulos Nativos)

18 módulos nativos carregados via diretivas `#<module>`:

| Módulo | Descrição |
|--------|-----------|
| [console_io](stdlib/console_io.md) | Entrada/saída no console (print, input) |
| [terminal_ansi](stdlib/terminal_ansi.md) | Controle de terminal ANSI (cores, cursor) |
| [file_io](stdlib/file_io/overview.md) | Sistema de arquivos (leitura, escrita, diretórios) |
| [binary_io](stdlib/binary_io.md) | Operações binárias |
| [time](stdlib/time.md) | Tempo e data (now, sleep, timestamp) |
| [system_env](stdlib/system/env.md) | Sistema operacional e ambiente |
| [crypto](stdlib/security/crypto.md) | Criptografia (SHA-256, AES, UUID, Base64) |
| [encode_decode](stdlib/encode_decode.md) | Codificação (JSON, CSV, XML) |
| [debug](stdlib/debug.md) | Debug (typeof, perf, assert, regex test) |
| [utils](stdlib/utils/general.md) | Utilitários (eval, clone, regex, random) |
| [http_client](stdlib/http/client.md) | Cliente HTTP (GET, POST, download) |
| [http_server](stdlib/http/server.md) | Servidor HTTP (rotas, middleware, CORS) |
| [tcp](stdlib/network/tcp.md) | TCP (servidor, cliente, resolução DNS) |
| [udp](stdlib/network/udp.md) | UDP (servidor, cliente, datagramas) |
| [ffi](stdlib/system/ffi.md) | Foreign Function Interface |
| [json_stream](stdlib/json/stream.md) | JSON streaming (parse incremental) |
| [websocket](stdlib/network/websocket.md) | WebSocket (cliente, servidor) |
| [database](stdlib/database/overview.md) | Bancos de dados (SQLite, PostgreSQL) |

## Ferramentas

- [Oak Package Manager](oak_package_manager/cli.md) — CLI do gerenciador de pacotes
- [API do Registro](oak_package_manager/registry_api.md) — API do registry

## Instalação

- [Guia de Instalação](install.md)
