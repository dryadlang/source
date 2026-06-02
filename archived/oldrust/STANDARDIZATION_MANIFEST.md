# 📋 MANIFESTO DE PADRONIZAÇÃO — Dryad Compiler & Project

> **Este documento define os padrões obrigatórios de código para TODOS os crates do projeto Dryad.**
>
> **Este manifesto é vinculante e não negociável.**  
> **Exceções requerem aprovação explícita do Tech Lead.**

---

**Versão**: 1.0  
**Data**: 2026-03-22  
**Status**: Ativo  
**Escopo**: Aplicável a todos os crates (dryad_*, oak, benchmarks)

---

## 🎯 Propósito

Define standards de desenvolvimento, patterns de implementação, e workflows obrigatórios para:
- Desenvolvimento do compilador Dryad (bytecode → IR → objeto)
- Manutenção da linguagem e runtime
- Integração de novos backends e geradores
- Refatoração segura com zero regressions

---

## 📋 PRINCÍPIOS FUNDAMENTAIS

### 1. Test-Driven Development (TDD)
```
OBRIGATÓRIO: SEMPRE escrever testes ANTES de implementação
Ordem: Test → Implementation → Refactor → Commit
```

**Regra**: Nenhum código em produção sem testes correspondentes.

### 2. Zero Regressions
```
OBRIGATÓRIO: Todos os testes baseline DEVEM continuar passando
Aceitação: 100% de testes passando, NENHUMA exceção
```

**Regra**: Se um commit quebra testes, ele é rejeitado IMEDIATAMENTE.

### 3. Código em Inglês
```
OBRIGATÓRIO: Todo código deve estar em English
Exceção: Comentários/docs podem ser em Português (quando necessário)
```

**Regra**: Variáveis, funções, tipos, módulos - SEMPRE English.

### 4. Commits Atômicos e Descritivos
```
OBRIGATÓRIO: Um commit = uma feature/fix logicamente completo
Formato: "type: description (problem-solving focus)"

Exemplos:
✓ "feat: add bitwise and arithmetic opcodes to bytecode converter"
✓ "fix: correct SetLocal opcode handler to load local address"
✓ "docs: update AOT compiler status with bytecode converter completion"
✗ "update" / "fix stuff" / "changes"
```

**Regra**: Commit message deve ser compreensível SEM ler o código.

### 5. Código Auto-Documentado
```
OBRIGATÓRIO: Código deve ser legível sem comentários
- Nomes descritivos de variáveis/funções
- Estrutura clara e lógica
- Tipos explícitos (sem `as any`, `@ts-ignore`)

Comentários APENAS para:
- Algoritmos complexos
- Fórmulas matemáticas
- Decisões não-óbvias
- Referências a specs
```

**Regra**: Se precisa de comentário para entender, refatore o código.

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

### 1.2 Estrutura de Diretórios

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

### 2.1 Tipo de Erro Único

O projeto DEVE usar **um único tipo de erro**: `DryadError` do crate `dryad_errors`.

**Ação:** Eliminar `RuntimeError` de `dryad_runtime/src/errors.rs`. Todas as funções que hoje retornam `RuntimeError` devem ser migradas para retornar `DryadError`.

### 2.2 Construtores de Erro — Usar Catálogo Centralizado

O projeto usa um **catálogo centralizado** (`error_catalog.rs`) com todas as definições de erro. Use SEMPRE `from_catalog` ou `from_catalog_fmt`:

#### 2.2.1 Uso Básico — `from_catalog()`

Use quando a mensagem padrão do catálogo é apropriada:

```rust
use dryad_errors::{error_catalog, DryadError, SourceLocation};

// CORRETO — usa mensagem do catálogo
return Err(DryadError::from_catalog(
    error_catalog::e2003(),  // Error code from catalog
    self.current_location()  // Location info
));
```

#### 2.2.2 Com Mensagem Customizada — `from_catalog_fmt()`

Use quando precisa interpolar variáveis ou contexto específico:

```rust
// CORRETO — preserva informação específica do runtime
return Err(DryadError::from_catalog_fmt(
    error_catalog::e3005(),                           // Error code
    &format!("Undefined variable: '{}'", var_name),  // Custom message
    self.current_location()                          // Location
));
```

#### 2.2.3 Quando Localização Não Está Disponível

Use `SourceLocation::unknown()` como fallback:

```rust
// Aceitável quando não há informação de localização
return Err(DryadError::from_catalog(
    error_catalog::e3100(),
    SourceLocation::unknown()
));
```

#### 2.2.4 Validação de Erro — Checklist

Antes de usar um código de erro, verifique:

- [ ] O código existe em `error_catalog.rs` (ex: `e3005` existe?)
- [ ] O código está na faixa certa:
   - 1000-1999 para Lexer
   - 2000-2999 para Parser
   - 3000-3999 para Runtime
   - 5000-5999 para I/O
- [ ] Se precisa de `SourceLocation`, use `self.current_location()` ou `self.location`
- [ ] Se precisa de contexto dinâmico, use `from_catalog_fmt()`
- [ ] A mensagem está em English (catálogo) ou Portuguese (output para usuário)

#### 2.2.5 PROIBIDO

```rust
// ❌ PROIBIDO — construtor legado (deprecated)
DryadError::new(3001, "Variavel nao definida")

// ❌ PROIBIDO — sem localização
return Err(DryadError::from_catalog(error_catalog::e3005(), ???));

// ❌ PROIBIDO — sem estruturar erro
panic!("Undefined variable");
valor.unwrap();  // Can panic in production

// ❌ PROIBIDO — concatenação de strings
let msg = "Error: ".to_string() + &var_name;
return Err(DryadError::from_catalog_fmt(
    error_catalog::e3005(),
    &msg,  // Use format!() instead
    location
));
```

**Regras:**
- `DryadError::new()` é deprecated — use `from_catalog` ou `from_catalog_fmt`
- Todo erro DEVE incluir `SourceLocation` (mínimo: `SourceLocation::unknown()`)
- Erros dinâmicos usam `from_catalog_fmt()`, nunca concatenação de strings
- Código de erro DEVE estar definido em error_catalog.rs (validação em compile time)
- Não criar novos tipos de erro — sempre usar `DryadError` com código apropriado

### 2.3 Propagação de Erros

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

`Box<dyn Error>` apaga o tipo do erro e impede pattern matching. Toda função pública DEVE retornar `Result<T, DryadError>`.

---

## 3. Nomenclatura e Estilo

### 3.1 Convenções de Nomes

| Elemento | Convenção | Exemplo |
|----------|-----------|---------|
| Tipos (struct, enum, trait) | PascalCase | `TokenWithLocation`, `DryadError` |
| Funções e métodos | snake_case | `next_token()`, `check_expr()` |
| Constantes | SCREAMING_SNAKE_CASE | `MAX_RECURSION_DEPTH` |
| Módulos | snake_case | `native_modules`, `file_io` |
| Tipo alias | PascalCase | `type HeapId = usize` |
| Lifetimes | minúsculas curtas | `'a`, `'src` |
| Generics | maiúsculas curtas ou descritivas | `T`, `E`, `Value` |

### 3.2 Idioma

**Todo o código DEVE estar em inglês.** Isso inclui:

- Nomes de variáveis, funções, tipos, módulos
- Comentários de código (`//`, `///`)
- Mensagens de erro internas (formato de DryadError)
- Doc comments

**Exceção:** Mensagens de erro voltadas ao usuário final (output do CLI) podem estar em português.

```rust
// CORRETO — código em inglês
/// Returns true if the token is a specific keyword
pub fn is_keyword(&self, keyword: &str) -> bool { ... }

// CORRETO — mensagem para o usuário em português
eprintln!("Erro: arquivo '{}' não encontrado", filename);

// PROIBIDO — misturar idiomas no código
/// Retorna true se o token é uma palavra-chave específica
pub fn is_keyword(&self, keyword: &str) -> bool { ... }
```

### 3.3 Imports

Ordem obrigatória, separada por linha em branco:

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

// 4. Imports do próprio crate
use crate::interpreter::Value;
use crate::heap::Heap;
```

**Regras:**
- NUNCA usar `use crate::*` ou `use modulo::*` (wildcard imports)
- Imports dentro de cada grupo devem estar em ordem alfabética
- Cada grupo separado por uma linha em branco

---

## 4. Documentação

### 4.1 Doc Comments Obrigatórios

Todo item público (`pub`) DEVE ter doc comment:

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
- `///` para itens públicos (structs, enums, funções, traits, métodos)
- `//!` para module-level docs em lib.rs e mod.rs
- `//` para comentários de implementação interna
- Seções `# Arguments`, `# Returns`, `# Errors` obrigatórias para funções públicas com assinaturas não-triviais
- `# Examples` recomendado (mas não obrigatório)

### 4.2 Comentários de Implementação

```rust
// CORRETO — explica o "porquê"
// Skip whitespace before checking for keywords to avoid
// false positives on identifiers like "letter"
self.skip_whitespace();

// PROIBIDO — explica o "o que" (o código já diz isso)
// Skip whitespace
self.skip_whitespace();
```

### 4.3 TODO/FIXME

TODO e FIXME são permitidos, mas DEVEM seguir o formato:

```rust
// TODO(#123): Implement type inference for lambda return types
// FIXME(#456): This clone is unnecessary, use a reference instead
```

Onde `#123` é o número da issue no GitHub. TODOs sem issue associada NÃO devem existir no main branch.

---

## 5. Padrões de Código

### 5.1 Tamanho de Funções

- **Máximo:** 80 linhas por função (excluindo doc comments e closures simples)
- Funções maiores DEVEM ser divididas em funções auxiliares privadas
- `match` com mais de 10 arms deve ser extraído para uma função dedicada

### 5.2 Clone e Performance

```rust
// PROIBIDO — clone desnecessário
let name = self.name.clone();
do_something(&name);

// CORRETO — usar referência
do_something(&self.name);

// ACEITÁVEL — clone necessário por ownership
let name = self.name.clone();
self.map.insert(key, name); // precisa de ownership
```

**Regra:** Todo `.clone()` deve ser justificável. Se você pode usar uma referência, use.

### 5.3 Magic Numbers

```rust
// PROIBIDO
if self.call_depth > 1000 { ... }
chunk.add_constant(Value::Number(42.0));

// CORRETO
const MAX_RECURSION_DEPTH: usize = 1000;
if self.call_depth > MAX_RECURSION_DEPTH { ... }
```

Números literais DEVEM ser constantes nomeadas, exceto:
- `0`, `1`, `-1` em contextos óbvios (inicialização, incremento)
- Índices de array em loops

### 5.4 Pattern Matching

```rust
// CORRETO — exhaustive, com caso explicitamente vazio se necessário
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

O catch-all `_ => {}` em enums DEVE ter um comentário explicando porque os outros casos são ignorados.

### 5.5 Unsafe

```rust
// OBRIGATÓRIO — todo bloco unsafe DEVE ter um safety comment
// SAFETY: The pointer is guaranteed to be valid because [razao].
// The lifetime is bounded by [escopo].
unsafe {
    // ...
}
```

Blocos `unsafe` DEVEM:
1. Ter comentário `// SAFETY:` explicando porque é seguro
2. Ser o menor possível (uma operação por bloco)
3. Ser encapsulados em funções safe com validação nos argumentos

---

## 6. Testes

### 6.1 Organização

```
crates/dryad_<nome>/
  src/
    modulo.rs           # Código de produção
  tests/
    modulo_test.rs      # Testes de integração
```

**Regras:**
- Testes unitários: `#[cfg(test)] mod tests { ... }` no FINAL do arquivo de implementação
- Testes de integração: `tests/` na raiz do crate
- NUNCA colocar testes em `lib.rs` — mover para arquivo próprio ou para `tests/`
- Nomes de teste: `test_<funcionalidade>_<cenario>` (ex: `test_lexer_handles_unicode_escape`)

### 6.2 Padrão de Teste

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
- Seguir padrão Arrange/Act/Assert
- Um assert principal por teste (asserts auxiliares são aceitáveis)
- Helpers reutilizáveis no topo do módulo de testes
- `unwrap()` e `expect()` são PERMITIDOS em testes

### 6.3 Test Coverage

```
Mínimo obrigatório:
- Happy path (sucesso esperado)
- Error cases (erro esperado)
- Edge cases (limites, valores especiais)
- Regressions (comparar com comportamento anterior)
```

---

## 7. Arquitetura de Módulos Nativos

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
// Em cada módulo nativo:
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

## 8. Divisão de Arquivos Grandes

### 8.1 interpreter.rs (5099 linhas) → módulo interpreter/

```
interpreter/
  mod.rs              # Struct Interpreter, new(), execute(), métodos públicos
  statements.rs       # execute_statement() e todos os handlers de Stmt::*
  expressions.rs      # evaluate_expression() e todos os handlers de Expr::*
  classes.rs          # Lógica de classes, herança, visibilidade
  modules.rs          # Import/export, resolução de módulos
  concurrency.rs      # Threads, mutexes, async/await
  bytecode_bridge.rs  # Conversão para bytecode VM
  builtins.rs         # Funções built-in (print, typeof, etc)
```

### 8.2 parser.rs (3415 linhas) → módulo parser/

```
parser/
  mod.rs              # Struct Parser, new(), parse(), métodos públicos
  statements.rs       # statement(), var_declaration(), if_statement(), etc.
  expressions.rs      # expression(), binary(), unary(), call(), etc.
  classes.rs          # class_declaration(), interface_declaration()
  patterns.rs         # Pattern parsing (destructuring, match arms)
  modules.rs          # import_statement(), export_statement(), use_statement()
  helpers.rs          # peek(), advance(), expect(), consume_semicolon()
```

---

## 9. Padrões de Implementação (Compilador)

### 1. Novos Opcodes Bytecode

**Estrutura Obrigatória**:
```rust
// 1. Definir no OpCode enum
pub enum OpCode {
    MyNewOp(u8),
    ...
}

// 2. Implementar no Compiler
OpCode::MyNewOp(idx) => {
    // Generate bytecode
}

// 3. Implementar no VM (se aplicável)
OpCode::MyNewOp(idx) => {
    // Execute bytecode
}

// 4. Implementar no Converter
OpCode::MyNewOp(idx) => {
    // Convert to IR
    let ir_instr = self.build_ir(idx);
    self.add_instruction(ir_instr);
}

// 5. Testes para cada estágio
#[test]
fn test_opcode_mynewhop() { ... }

#[test]
fn test_convert_mynewhop() { ... }
```

**Regra**: Não implementar parcialmente. Ir do Opcode até IR completo.

### 2. Novos Backends

**Estrutura Obrigatória**:
```
crates/dryad_aot/src/backend/
├── <architecture>.rs
├── <architecture>/
│   ├── register_allocator.rs
│   ├── codegen.rs
│   └── tests.rs
└── mod.rs (export)
```

**Regra**: Arquitetura nova = módulo separado + testes separados.

### 3. Novos Geradores

**Estrutura Obrigatória**:
```rust
pub struct MyGenerator { ... }

impl Generator for MyGenerator {
    fn generate_object(&self, module: &IrModule, code: &[u8]) -> Result<Vec<u8>, String> {
        // Validar entrada
        // Gerar headers
        // Gerar sections
        // Testar saída
        // Retornar
    }
    
    fn format_name(&self) -> &'static str { "FORMAT" }
    fn file_extension(&self) -> &'static str { ".ext" }
}

#[cfg(test)]
mod tests {
    // Header validity tests
    // Section structure tests
    // Magic bytes tests
    // Size tests
}
```

**Regra**: Cada gerador tem testes específicos de formato.

---

## 10. Checklist de Refatoração

### Fase 1 — Fundação (sem mudar comportamento)
- [ ] Corrigir `dryad_checker/Cargo.toml`: edition "2021", workspace deps, adicionar description
- [ ] Padronizar todos os Cargo.toml com workspace dependencies
- [ ] Adicionar `//!` doc comments em todos os lib.rs
- [ ] Traduzir doc comments e comentários de código para inglês

### Fase 2 — Sistema de Erros
- [ ] Deprecar `DryadError::new()`, adicionar `#[deprecated]`
- [ ] Implementar `From<RuntimeError> for DryadError`
- [ ] Migrar native modules de `RuntimeError` para `DryadError`
- [ ] Eliminar `dryad_runtime/src/errors.rs` após migração completa
- [ ] Eliminar todos os `Box<dyn Error>` em funções públicas
- [ ] Remover `unwrap()`/`expect()`/`panic!()` de código de produção

### Fase 3 — Divisão de Arquivos
- [ ] Dividir `interpreter.rs` em sub-módulos (seção 8.1)
- [ ] Dividir `parser.rs` em sub-módulos (seção 8.2)
- [ ] Implementar trait `NativeModule` (seção 7.1)
- [ ] Refatorar registro de módulos nativos

### Fase 4 — Qualidade
- [ ] Padronizar imports (ordem e agrupamento) em todos os arquivos
- [ ] Eliminar wildcard imports (`use modulo::*`)
- [ ] Adicionar doc comments em todos os itens públicos
- [ ] Resolver ou converter TODOs em issues do GitHub
- [ ] Extrair magic numbers para constantes nomeadas
- [ ] Adicionar `// SAFETY:` em todos os blocos unsafe

### Fase 5 — Testes
- [ ] Mover testes de `dryad_bytecode/src/lib.rs` para `tests/`
- [ ] Padronizar nomes de teste (`test_<funcionalidade>_<cenario>`)
- [ ] Garantir que cada crate tenha pelo menos testes unitários básicos

---

## 11. Restrições Hard (NUNCA VIOLE)

### ❌ Proibido
```rust
// NUNCA: Type suppression
as any
@ts-ignore
@ts-expect-error
#[allow(unused)]  // sem justificativa

// NUNCA: Empty catch/error handling
catch(_) {}
Err(_) => {}

// NUNCA: Delete/skip tests
// - Testes com falha = Bug para corrigir, não para esconder

// NUNCA: Commit sem testes passando
// - Force push para main/master
// - Destructive git operations sem revisão

// NUNCA: Hardcoded values
const SIZE = 512;  // ❌
const SECTOR_SIZE: usize = 512;  // ✓

// NUNCA: Variáveis globais mutáveis
static mut COUNTER: i32 = 0;  // ❌ (use Arc<Mutex<>>)
```

---

## 12. Anti-Patterns (NUNCA FAZER)

### ❌ Code Smell #1: Função Muito Grande
```rust
// NUNCA:
fn process_bytecode(data: &[u8]) -> Result<Vec<u8>, String> {
    // 500 linhas de código
}

// SEMPRE:
fn process_bytecode(data: &[u8]) -> Result<Vec<u8>, String> {
    let tokens = self.tokenize(data)?;
    let ast = self.parse(tokens)?;
    let bytecode = self.compile(ast)?;
    Ok(bytecode)
}
```

### ❌ Code Smell #2: Deeply Nested
```rust
// NUNCA:
if a {
    if b {
        if c {
            if d {
                // 4+ níveis de indentação
            }
        }
    }
}

// SEMPRE:
if !a { return Err("..."); }
if !b { return Err("..."); }
if !c { return Err("..."); }
if !d { return Err("..."); }
// Código principal
```

### ❌ Code Smell #3: Copiar-Colar
```rust
// NUNCA:
fn convert_op1() { /* 50 linhas */ }
fn convert_op2() { /* 48 linhas idênticas */ }

// SEMPRE:
fn convert_binary_op(left: Reg, right: Reg, op: BinOp) { ... }
fn convert_op1() { convert_binary_op(...) }
fn convert_op2() { convert_binary_op(...) }
```

### ❌ Code Smell #4: Sem Testes
```rust
// NUNCA fazer commit sem testes

// SEMPRE:
#[test]
fn test_new_feature() {
    let result = new_feature();
    assert_eq!(result, expected);
}
```

---

## 13. Integração com Ecossistema (Compiler-Specific)

### Ao Modificar Bytecode VM
```
DEVE atualizar:
1. OpCode enum em dryad_bytecode/src/opcode.rs
2. VM implementation em dryad_bytecode/src/vm.rs
3. Compiler em dryad_bytecode/src/compiler.rs
4. Testes em dryad_bytecode/tests/
5. Converter em dryad_aot/src/compiler/converter.rs
6. AOT testes em dryad_aot/tests/
7. Documentação correspondente
```

### Ao Modificar IR
```
DEVE atualizar:
1. IrInstruction enum
2. IrModule struct
3. Generator impls (elf, pe)
4. Backend impls
5. Converter
6. Testes
7. Documentação
```

### Ao Adicionar Backend
```
DEVE:
1. Criar módulo separado
2. Implementar trait Backend
3. Adicionar código generation
4. Adicionar register allocator
5. Adicionar testes específicos
6. Documentar em manuals/aot/
7. Adicionar ao README principal
```

---

## 14. Workflow Recomendado

### Para uma Feature Nova

```bash
# 1. Criar branch
git checkout -b feature/feature-name

# 2. Escrever testes (PRIMEIRO!)
# Editar tests/integration_feature.rs

# 3. Verificar que os testes FALHAM
cargo test -p dryad_aot --test integration_feature
# EXPECTED: FAIL

# 4. Implementar feature
# Editar src/...

# 5. Verificar que os testes PASSAM
cargo test -p dryad_aot --test integration_feature
# EXPECTED: PASS

# 6. Verificar que NÃO quebrou nada
cargo test -p dryad_aot --lib
# EXPECTED: PASS (incluindo todos os baseline tests)

# 7. Code quality
cargo clippy
cargo fmt

# 8. Commit
git add .
git commit -m "feat: implement feature-name"

# 9. Fazer push
git push origin feature/feature-name

# 10. Criar Pull Request
# No GitHub: feature/feature-name → main
```

---

## 15. Métricas de Qualidade

### Obrigatórias
```
✓ Test Pass Rate: 100%
✓ Test Coverage: >= 80%
✓ Clippy Warnings: 0 (novos)
✓ Compilation: clean
✓ Commits Atômicos: 1 feature = 1+ commits (não 1 mega-commit)
```

### Recomendadas
```
→ Cobertura de edge cases
→ Documentação de APIs
→ Exemplos de uso
→ Performance baseline
```

---

## 16. Ferramentas de Verificação

Antes de cada PR, executar:

```bash
# Compilação limpa
cargo build --workspace 2>&1 | grep -c "warning"  # Deve ser 0

# Linter
cargo clippy --workspace -- -D warnings

# Formatação
cargo fmt --all -- --check

# Testes
cargo test --workspace
```

**Meta:** Zero warnings no clippy, zero diferenças no fmt, todos os testes passando.

---

## 📝 CHANGELOG

### v1.0 (2026-03-22)
- Merged root STANDARDIZATION_MANIFEST with develop/ version
- Combined general project standards + compiler-specific patterns
- Added integration ecosystem section for compiler work
- Removed duplication while preserving all unique content
- Scope now covers entire project (dryad_*, oak, benchmarks)

### Anteriormente (root version - 2026-03-21)
- Initial project-wide standardization
- General crate structure and error handling

### Anteriormente (develop version - 2026-03-22)
- Compiler-specific patterns and opcodes
- AOT compilation pipeline standards
- Backend and generator patterns

---

**Este manifesto é um documento vivo. Atualize-o conforme o projeto evolui.**

**Última atualização**: 2026-03-22  
**Status**: Vinculante e obrigatório para TODO código produzido
