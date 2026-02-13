# Ignored Code / Mockups

Este documento lista funcionalidades que existem no código fonte mas não estão completamente implementadas ou são apenas stubs/mockups.

## Runtime

### `crates/dryad_runtime/src/native_functions_legacy.rs.bak`
- **Arquivo Completo**: Arquivo de backup com implementações antigas de funções nativas.
    - *Ação*: Deve ser removido do repositório, pois o novo sistema de módulos (`native_modules/`) já o substituiu.

### `crates/dryad_runtime/src/native_modules/udp.rs`
- **Stub**: O arquivo existe mas contém apenas a estrutura básica sem implementação real de sockets UDP.
    - *Status*: `Not Implemented`.

### `crates/dryad_runtime/src/native_modules/http_server.rs`
- **Parcial**: Servidor HTTP básico implementado mas não expõe API completa para o usuário Dryad (middleware, routing).
    - *Status*: `Experimental`.

## Oak Package Manager

### `crates/oak/src/main.rs`
- **Linha 336 (Publish Command)**: O comando existe no enum e CLI, mas a implementação é um `eprintln!("Erro ao publicar pacote...")`.
    - *Status*: `Mocked`.
- **Linha 768 (Install Simulated)**: O sistema de instalação cai para um modo "simulado" se falhar o registry remoto.
    - *Status*: `Mockup` para testes locais.

## Parser

### `crates/dryad_parser/src/ast.rs`
- **Variant `ImportKind::Namespace`**: O parser reconhece `import * as`, mas o runtime pode não suportar corretamente a importação de namespace completo dependendo do módulo.
    - *Status*: `Parsing Only`.

## Errors

### `crates/dryad_errors/src/error_urls.rs`
- **Linhas 41-64**: Códigos de erro 4000-9000 (Tipos, I/O, Sistema) são apenas reservados e geram URLs, mas não são disparados pelo interpretador atual.
    - *Status*: `Planned`.
