# Manifesto de Padronizacao — Dryad Compiler

> Este documento define os padroes obrigatorios de codigo para todos os crates do projeto Dryad.
> Toda refatoracao deve seguir estas regras. Nenhum PR deve ser aceito se violar este manifesto.

---

## Estado Atual (Diagnostico)

O codigo atual apresenta os seguintes problemas sistematicos:

1. **Dois sistemas de erro paralelos** — `DryadError` (dryad_errors) e `RuntimeError` (dryad_runtime/errors.rs) coexistem sem interoperabilidade clara
2. **Arquivos monstro** — `interpreter.rs` (5099 linhas), `parser.rs` (3415 linhas)
3. **Cargo.toml inconsistente** — `dryad_checker` usa `edition = "2024"` e paths relativos em vez de workspace; falta `description`
4. **Uso de `unwrap()`/`panic!()` em codigo de producao** — presente em interpreter.rs, vm.rs, debug_server.rs
5. **`Box<dyn Error>` misturado com tipos de erro tipados** — cli/main.rs e native modules
6. **Duplicacao de tipos** — `Value` existe em dryad_runtime e dryad_bytecode separadamente
7. **Documentacao inconsistente** — dryad_bytecode/dryad_aot tem doc comments extensos, dryad_checker/dryad_runtime quase zero
8. **Organizacao de testes fragmentada** — testes inline em lib.rs, arquivos separados, sem padrao
9. **Comentarios em portugues e ingles misturados** no mesmo arquivo
10. **Modulos nativos com registro manual repetitivo** — 20 blocos identicos em `register_all_categories()`

---

## 1. Estrutura de Crates

### 1.1 Cargo.toml

Todos os crates DEVEM seguir este template:

```toml
[package]
name = "dryad_<nome>"
version = "0.1.0"
edition = "2021"
description = "<descricao em portugues>"

[dependencies]
# Dependencias internas SEMPRE via workspace
dryad_errors = { workspace = true }

# Dependencias externas com versao explicita
serde = { version = "1.0", features = ["derive"] }
```

**Regras:**
- `edition = "2021"` em todos os crates (corrigir dryad_checker que usa "2024")
- Campo `description` obrigatorio em todos os crates
- Dependencias internas SEMPRE via `{ workspace = true }`, NUNCA via `{ path = "../..." }`
- Workspace Cargo.toml deve declarar todas as dependencias compartilhadas em `[workspace.dependencies]`

### 1.2 Estrutura de Diretorios

```
crates/dryad_<nome>/
  Cargo.toml
  src/
    lib.rs          # Apenas declaracoes de modulos e re-exports
    mod1.rs         # Modulos planos para arquivos < 500 linhas
    mod2/           # Subdiretorio para modulos > 500 linhas ou com sub-modulos
      mod.rs
      sub1.rs
      sub2.rs
  tests/            # Testes de integracao (quando necessarios)
    test_feature.rs
```

**Regras:**
- Nenhum arquivo `.rs` pode exceder **800 linhas** (incluindo testes inline)
- Arquivos acima de 500 linhas devem ser avaliados para divisao
- `interpreter.rs` (5099 linhas) e `parser.rs` (3415 linhas) DEVEM ser divididos em sub-modulos

### 1.3 lib.rs

Todos os `lib.rs` DEVEM seguir este template:

```rust
//! # dryad_<nome>
//!
//! <Descricao curta do crate em uma linha.>

// Modulos internos (privados por padrao)
mod modulo_interno;

// Modulos publicos
pub mod modulo_publico;

// Re-exports da API publica
pub use modulo_publico::{TipoPrincipal, funcao_principal};
```

**Regras:**
- Module-level doc comment (`//!`) obrigatorio em todos os lib.rs
- Re-exports explicitos — nao expor modulos inteiros desnecessariamente
- Constantes como `VERSION` NAO devem estar em lib.rs (usar `env!("CARGO_PKG_VERSION")` onde necessario)

---

## 2. Sistema de Erros

### 2.1 Tipo de Erro Unico

O projeto DEVE usar **um unico tipo de erro**: `DryadError` do crate `dryad_errors`.

**Acao:** Eliminar `RuntimeError` de `dryad_runtime/src/errors.rs`. Todas as funcoes que hoje retornam `RuntimeError` devem ser migradas para retornar `DryadError`.

### 2.2 Construtores de Erro

Usar SEMPRE os construtores tipados do `DryadError`:

```rust
// CORRETO — construtor tipado com localizacao
DryadError::runtime(3001, "Variavel nao definida", location, stack_trace)
DryadError::lexer(1001, "Caracter inesperado", location)
DryadError::io_error(5001, "Arquivo nao encontrado", location, "read".into(), Some(path))

// PROIBIDO — construtor generico sem contexto
DryadError::new(3001, "Variavel nao definida")  // Perde localizacao!
```

**Regras:**
- `DryadError::new()` deve ser removido ou marcado como `#[deprecated]`
- Todo erro DEVE incluir `SourceLocation` valido
- Erros de modulos nativos devem usar os construtores tipados (nao strings)
- Implementar `From<RuntimeError> for DryadError` como ponte temporaria durante a migracao

### 2.3 Propagacao de Erros

```rust
// CORRETO — operador ?
let tokens = lexer.next_token()?;

// CORRETO — match quando precisa tratar casos especificos
match resultado {
    Ok(valor) => processar(valor),
    Err(e) => return Err(enriquecer_erro(e)),
}

// PROIBIDO em codigo de producao
valor.unwrap()          // Usa .unwrap() — pode panic
valor.expect("msg")     // Usa .expect() — pode panic
panic!("mensagem")      // Panic explicito

// PERMITIDO apenas em:
// - Testes (#[cfg(test)])
// - Inicializacao de statics (lazy_static!)
// - Condicoes verdadeiramente irrecuperaveis (corrupcao de memoria)
```

### 2.4 Sem Box<dyn Error>

```rust
// PROIBIDO
fn minha_funcao() -> Result<(), Box<dyn std::error::Error>>

// CORRETO
fn minha_funcao() -> Result<(), DryadError>
```

`Box<dyn Error>` apaga o tipo do erro e impede pattern matching. Toda funcao publica DEVE retornar `Result<T, DryadError>`.

---

## 3. Nomenclatura e Estilo

### 3.1 Convencoes de Nomes

| Elemento | Convencao | Exemplo |
|----------|-----------|---------|
| Tipos (struct, enum, trait) | PascalCase | `TokenWithLocation`, `DryadError` |
| Funcoes e metodos | snake_case | `next_token()`, `check_expr()` |
| Constantes | SCREAMING_SNAKE_CASE | `MAX_RECURSION_DEPTH` |
| Modulos | snake_case | `native_modules`, `file_io` |
| Tipo alias | PascalCase | `type HeapId = usize` |
| Lifetimes | minusculas curtas | `'a`, `'src` |
| Generics | maiusculas curtas ou descritivas | `T`, `E`, `Value` |

### 3.2 Idioma

**Todo o codigo DEVE estar em ingles.** Isso inclui:

- Nomes de variaveis, funcoes, tipos, modulos
- Comentarios de codigo (`//`, `///`)
- Mensagens de erro internas (formato de DryadError)
- Doc comments

**Excecao:** Mensagens de erro voltadas ao usuario final (output do CLI) podem estar em portugues, pois e a interface com o usuario Dryad.

```rust
// CORRETO — codigo em ingles
/// Returns true if the token is a specific keyword
pub fn is_keyword(&self, keyword: &str) -> bool { ... }

// CORRETO — mensagem para o usuario em portugues
eprintln!("Erro: arquivo '{}' nao encontrado", filename);

// PROIBIDO — misturar idiomas no codigo
/// Retorna true se o token e uma palavra-chave especifica  // <- portugues em doc comment
pub fn is_keyword(&self, keyword: &str) -> bool { ... }
```

### 3.3 Imports

Ordem obrigatoria, separada por linha em branco:

```rust
// 1. Imports da standard library
use std::collections::HashMap;
use std::path::PathBuf;

// 2. Imports de crates externos
use serde::{Deserialize, Serialize};
use tokio::fs;

// 3. Imports de crates internos do workspace
use dryad_errors::{DryadError, SourceLocation};
use dryad_parser::ast::Expr;

// 4. Imports do proprio crate
use crate::interpreter::Value;
use crate::heap::Heap;
```

**Regras:**
- NUNCA usar `use crate::*` ou `use modulo::*` (wildcard imports)
- Imports dentro de cada grupo devem estar em ordem alfabetica
- Cada grupo separado por uma linha em branco

---

## 4. Documentacao

### 4.1 Doc Comments Obrigatorios

Todo item publico (`pub`) DEVE ter doc comment:

```rust
/// Tokenizes the source code into a sequence of tokens.
///
/// # Arguments
/// * `source` - The source code string to tokenize
///
/// # Returns
/// A vector of tokens with their source locations
///
/// # Errors
/// Returns `DryadError::Lexer` if the source contains invalid characters
pub fn tokenize(source: &str) -> Result<Vec<TokenWithLocation>, DryadError> { ... }
```

**Regras:**
- `///` para itens publicos (structs, enums, funcoes, traits, metodos)
- `//!` para module-level docs em lib.rs e mod.rs
- `//` para comentarios de implementacao interna
- Secoes `# Arguments`, `# Returns`, `# Errors` obrigatorias para funcoes publicas com assinaturas nao-triviais
- `# Examples` recomendado (mas nao obrigatorio)

### 4.2 Comentarios de Implementacao

```rust
// CORRETO — explica o "porqe"
// Skip whitespace before checking for keywords to avoid
// false positives on identifiers like "letter"
self.skip_whitespace();

// PROIBIDO — explica o "o que" (o codigo ja diz isso)
// Skip whitespace
self.skip_whitespace();
```

### 4.3 TODO/FIXME

TODO e FIXME sao permitidos, mas DEVEM seguir o formato:

```rust
// TODO(#123): Implement type inference for lambda return types
// FIXME(#456): This clone is unnecessary, use a reference instead
```

Onde `#123` e o numero da issue no GitHub. TODOs sem issue associada NAO devem existir no main branch.

---

## 5. Padroes de Codigo

### 5.1 Tamanho de Funcoes

- **Maximo:** 80 linhas por funcao (excluindo doc comments e closures simples)
- Funcoes maiores DEVEM ser divididas em funcoes auxiliares privadas
- `match` com mais de 10 arms deve ser extraido para uma funcao dedicada

### 5.2 Clone e Performance

```rust
// PROIBIDO — clone desnecessario
let name = self.name.clone();
do_something(&name);

// CORRETO — usar referencia
do_something(&self.name);

// ACEITAVEL — clone necessario por ownership
let name = self.name.clone();
self.map.insert(key, name); // precisa de ownership
```

**Regra:** Todo `.clone()` deve ser justificavel. Se voce pode usar uma referencia, use.

### 5.3 Magic Numbers

```rust
// PROIBIDO
if self.call_depth > 1000 { ... }
chunk.add_constant(Value::Number(42.0));

// CORRETO
const MAX_RECURSION_DEPTH: usize = 1000;
if self.call_depth > MAX_RECURSION_DEPTH { ... }
```

Numeros literais DEVEM ser constantes nomeadas, exceto:
- `0`, `1`, `-1` em contextos obvios (inicializacao, incremento)
- Indices de array em loops

### 5.4 Pattern Matching

```rust
// CORRETO — exhaustive, com caso explicitamente vazio se necessario
match token {
    Token::Number(n) => handle_number(n),
    Token::String(s) => handle_string(s),
    Token::Eof => break,
    // Other token types are handled as expression statements
    _ => handle_default(token),
}

// PROIBIDO — catch-all silencioso que esconde bugs
match stmt {
    Stmt::VarDeclaration(..) => { ... },
    Stmt::ConstDeclaration(..) => { ... },
    _ => {} // <- O que acontece com os outros 20+ variantes?
}
```

O catch-all `_ => {}` em enums DEVE ter um comentario explicando porque os outros casos sao ignorados.

### 5.5 Unsafe

```rust
// OBRIGATORIO — todo bloco unsafe DEVE ter um safety comment
// SAFETY: The pointer is guaranteed to be valid because [razao].
// The lifetime is bounded by [escopo].
unsafe {
    // ...
}
```

Blocos `unsafe` DEVEM:
1. Ter comentario `// SAFETY:` explicando porque e seguro
2. Ser o menor possivel (uma operacao por bloco)
3. Ser encapsulados em funcoes safe com validacao nos argumentos

---

## 6. Testes

### 6.1 Organizacao

```
crates/dryad_<nome>/
  src/
    modulo.rs           # Codigo de producao
  tests/
    modulo_test.rs      # Testes de integracao
```

**Regras:**
- Testes unitarios: `#[cfg(test)] mod tests { ... }` no FINAL do arquivo de implementacao
- Testes de integracao: `tests/` na raiz do crate
- NUNCA colocar testes em `lib.rs` — mover para arquivo proprio ou para `tests/`
- Nomes de teste: `test_<funcionalidade>_<cenario>` (ex: `test_lexer_handles_unicode_escape`)

### 6.2 Padrao de Teste

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Helper functions at the top
    fn dummy_location() -> SourceLocation {
        SourceLocation::unknown()
    }

    #[test]
    fn test_binary_addition_produces_number() {
        // Arrange
        let left = Expr::Literal(Literal::Number(1.0), dummy_location());
        let right = Expr::Literal(Literal::Number(2.0), dummy_location());

        // Act
        let result = evaluate_binary("+", left, right);

        // Assert
        assert_eq!(result, Value::Number(3.0));
    }
}
```

**Regras:**
- Seguir padrao Arrange/Act/Assert
- Um assert principal por teste (asserts auxiliares sao aceitaveis)
- Helpers reutilizaveis no topo do modulo de testes
- `unwrap()` e `expect()` sao PERMITIDOS em testes

---

## 7. Arquitetura de Modulos Nativos

O sistema atual de registro manual em `NativeModuleManager::register_all_categories()` tem 20 blocos repetitivos. Deve ser refatorado para:

### 7.1 Trait de Registro

```rust
/// Every native module MUST implement this trait
pub trait NativeModule {
    /// Module category name (e.g., "file_io", "crypto")
    fn category(&self) -> &str;

    /// Register synchronous functions
    fn register_sync(&self, registry: &mut HashMap<String, NativeFunction>);

    /// Register async functions (optional, default empty)
    fn register_async(&self, _registry: &mut HashMap<String, AsyncNativeFunction>) {}
}
```

### 7.2 Auto-registro via Inventory ou Macro

```rust
// Em cada modulo nativo:
pub struct FileIoModule;

impl NativeModule for FileIoModule {
    fn category(&self) -> &str { "file_io" }
    fn register_sync(&self, registry: &mut HashMap<String, NativeFunction>) {
        registry.insert("read_file".into(), native_read_file);
        registry.insert("write_file".into(), native_write_file);
    }
}

// No NativeModuleManager:
fn register_all_categories(&mut self) {
    let modules: Vec<Box<dyn NativeModule>> = vec![
        Box::new(file_io::FileIoModule),
        Box::new(crypto::CryptoModule),
        // ...
    ];
    for module in modules {
        let mut sync_fns = HashMap::new();
        let mut async_fns = HashMap::new();
        module.register_sync(&mut sync_fns);
        module.register_async(&mut async_fns);
        self.categories.insert(module.category().to_string(), sync_fns);
        if !async_fns.is_empty() {
            self.async_categories.insert(module.category().to_string(), async_fns);
        }
    }
}
```

---

## 8. Divisao de Arquivos Grandes

### 8.1 interpreter.rs (5099 linhas) -> modulo interpreter/

```
interpreter/
  mod.rs              # Struct Interpreter, new(), execute(), metodos publicos
  statements.rs       # execute_statement() e todos os handlers de Stmt::*
  expressions.rs      # evaluate_expression() e todos os handlers de Expr::*
  classes.rs          # Logica de classes, heranca, visibilidade
  modules.rs          # Import/export, resolucao de modulos
  concurrency.rs      # Threads, mutexes, async/await
  bytecode_bridge.rs  # Conversao para bytecode VM
  builtins.rs         # Funcoes built-in (print, typeof, etc)
```

### 8.2 parser.rs (3415 linhas) -> modulo parser/

```
parser/
  mod.rs              # Struct Parser, new(), parse(), metodos publicos
  statements.rs       # statement(), var_declaration(), if_statement(), etc.
  expressions.rs      # expression(), binary(), unary(), call(), etc.
  classes.rs          # class_declaration(), interface_declaration()
  patterns.rs         # Pattern parsing (destructuring, match arms)
  modules.rs          # import_statement(), export_statement(), use_statement()
  helpers.rs          # peek(), advance(), expect(), consume_semicolon()
```

---

## 9. Checklist de Refatoracao

### Fase 1 — Fundacao (sem mudar comportamento)
- [ ] Corrigir `dryad_checker/Cargo.toml`: edition "2021", workspace deps, adicionar description
- [ ] Padronizar todos os Cargo.toml com workspace dependencies
- [ ] Adicionar `//!` doc comments em todos os lib.rs
- [ ] Traduzir doc comments e comentarios de codigo para ingles

### Fase 2 — Sistema de Erros
- [ ] Deprecar `DryadError::new()`, adicionar `#[deprecated]`
- [ ] Implementar `From<RuntimeError> for DryadError`
- [ ] Migrar native modules de `RuntimeError` para `DryadError`
- [ ] Eliminar `dryad_runtime/src/errors.rs` apos migracao completa
- [ ] Eliminar todos os `Box<dyn Error>` em funcoes publicas
- [ ] Remover `unwrap()`/`expect()`/`panic!()` de codigo de producao

### Fase 3 — Divisao de Arquivos
- [ ] Dividir `interpreter.rs` em sub-modulos (secao 8.1)
- [ ] Dividir `parser.rs` em sub-modulos (secao 8.2)
- [ ] Implementar trait `NativeModule` (secao 7.1)
- [ ] Refatorar registro de modulos nativos

### Fase 4 — Qualidade
- [ ] Padronizar imports (ordem e agrupamento) em todos os arquivos
- [ ] Eliminar wildcard imports (`use modulo::*`)
- [ ] Adicionar doc comments em todos os itens publicos
- [ ] Resolver ou converter TODOs em issues do GitHub
- [ ] Extrair magic numbers para constantes nomeadas
- [ ] Adicionar `// SAFETY:` em todos os blocos unsafe

### Fase 5 — Testes
- [ ] Mover testes de `dryad_bytecode/src/lib.rs` para `tests/`
- [ ] Padronizar nomes de teste (`test_<funcionalidade>_<cenario>`)
- [ ] Garantir que cada crate tenha pelo menos testes unitarios basicos

---

## 10. Ferramentas de Verificacao

Antes de cada PR, executar:

```bash
# Compilacao limpa
cargo build --workspace 2>&1 | grep -c "warning"  # Deve ser 0

# Linter
cargo clippy --workspace -- -D warnings

# Formatacao
cargo fmt --all -- --check

# Testes
cargo test --workspace
```

**Meta:** Zero warnings no clippy, zero diferencas no fmt, todos os testes passando.

---

*Este manifesto e um documento vivo. Atualize-o conforme o projeto evolui.*
*Ultima atualizacao: 2026-03-21*
