# 📋 MANIFESTO DE PADRONIZAÇÃO - Compilador Dryad

**Versão**: 1.0  
**Data**: 2026-03-22  
**Status**: Ativo  
**Proprietário**: Equipe de Desenvolvimento Dryad

---

## 🎯 Propósito

Este manifesto define os padrões, princípios e procedimentos que **DEVEM** ser seguidos em TODA e QUALQUER implementação, modificação ou expansão do compilador Dryad.

**Este documento é vinculante e não negociável.**

---

## 📏 PRINCÍPIOS FUNDAMENTAIS

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

## 🏗️ ESTRUTURA DE ARQUIVOS

### Padrão de Organização
```
crates/
├── dryad_<component>/
│   ├── src/
│   │   ├── lib.rs
│   │   ├── module1.rs
│   │   ├── module2.rs
│   │   └── tests.rs (unit tests inline com #[cfg(test)])
│   ├── tests/
│   │   └── integration_<feature>.rs
│   ├── Cargo.toml
│   └── README.md
```

### Convenção de Nomes
```
Módulos:       snake_case (lexer, parser, converter)
Tipos:         PascalCase (Lexer, Parser, IrModule)
Traits:        PascalCase (Generator, Optimizer)
Funções:       snake_case (tokenize, parse, convert)
Constants:     SCREAMING_SNAKE_CASE (MAX_OPCODES, PE_MAGIC)
Variables:     snake_case (token_count, bytecode_chunk)
```

---

## ✅ CHECKLIST DE IMPLEMENTAÇÃO

Antes de começar QUALQUER trabalho, responda:

### [ ] Planejamento
- [ ] Objetivo é claro e específico?
- [ ] Tamanho é manejável (~80 linhas max por função)?
- [ ] Dependencies já existem ou precisam ser criadas?
- [ ] Há testes baseline que podem quebrar?

### [ ] Implementação
- [ ] Testes escritos PRIMEIRO?
- [ ] Código compila SEM erros?
- [ ] Código compila SEM warnings (novos)?
- [ ] 100% dos testes passam (incluindo baseline)?
- [ ] Código segue style guide?
- [ ] Código é auto-documentado?

### [ ] Verificação
- [ ] `cargo test -p <crate> --lib` ✅
- [ ] `cargo build --release` ✅
- [ ] `cargo clippy` ✅ (sem warnings novos)
- [ ] Git history é limpo e atômico?

### [ ] Documentação
- [ ] README.md atualizado?
- [ ] Docstring em tipos/traits públicos?
- [ ] Exemplos de uso no README?
- [ ] CHANGELOG.md atualizado?

### [ ] Commit
- [ ] Mensagem descreve o "por quê"?
- [ ] Uma feature por commit?
- [ ] Commit é reversível com `git revert`?

---

## 🔐 RESTRIÇÕES HARD (NUNCA VIOLE)

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

## 📦 PADRÕES DE IMPLEMENTAÇÃO

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

## 🧪 PADRÕES DE TESTE

### Unit Tests (inline com `#[cfg(test)]`)
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_<functionality>() {
        // Arrange
        let input = ...;
        
        // Act
        let result = function(input);
        
        // Assert
        assert_eq!(result, expected);
        assert!(condition, "descriptive message");
    }
}
```

**Regra**: Nome do teste DESCREVE o comportamento esperado.

### Integration Tests (arquivo separado)
```
tests/integration_<feature>.rs

#[test]
fn test_<feature>_end_to_end() {
    // Full pipeline test
    // Test multiple components together
}
```

**Regra**: Integration tests testam pipeline completo.

### Test Coverage
```
Mínimo obrigatório:
- Happy path (sucesso esperado)
- Error cases (erro esperado)
- Edge cases (limites, valores especiais)
- Regressions (comparar com comportamento anterior)
```

---

## 📚 PADRÕES DE DOCUMENTAÇÃO

### README.md Obrigatório
```markdown
# Component Name

Brief description

## Architecture
- Components
- Data flow
- Dependencies

## API
- Public types
- Key functions
- Examples

## Status
- [x] Completed features
- [ ] Planned features

## Testing
How to run tests
```

### Docstrings Obrigatórias (tipos públicos)
```rust
/// Breve descrição de uma linha
///
/// Descrição detalhada, se necessário
///
/// # Example
/// ```
/// let x = Struct::new();
/// ```
pub struct MyType { ... }
```

### Comments (apenas quando necessário)
```rust
// Simples operação: evite comentário
let result = a + b;

// Algoritmo complexo: inclua comentário
// LRU cache eviction: remove oldest unused entry
let victim = cache.iter()
    .min_by_key(|entry| entry.last_accessed())
    .map(|e| e.key())
    .unwrap();
```

---

## 🔄 PADRÕES DE EVOLUÇÃO

### Expandir um Componente Existente

**Passo 1**: Entender Estado Atual
```bash
git log --oneline <component>  # Ver histórico
cargo test -p <crate> --lib   # Verificar testes baseline
```

**Passo 2**: Criar Branches Temáticas
```bash
git checkout -b feature/new-opcodes
git checkout -b fix/bug-in-converter
```

**Passo 3**: Implementar Incrementalmente
- Feature pequena por commit
- Teste + Implementação por commit
- Verificar antes de cada commit

**Passo 4**: Manter Histórico Limpo
```bash
git log --oneline feature/new-opcodes
# Resultado: série clara de commits atômicos
```

**Passo 5**: Merge com Main
```bash
git checkout main
git pull origin main
git merge --no-ff feature/new-opcodes
```

---

## ⚠️ ANTI-PATTERNS (NUNCA FAZER)

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

fn tokenize(&self, ...) -> Result<Vec<Token>, String> { ... }
fn parse(&self, ...) -> Result<Ast, String> { ... }
fn compile(&self, ...) -> Result<Vec<u8>, String> { ... }
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

## 🚀 WORKFLOW RECOMENDADO

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
# EXPECTED: PASS (incluindo 33 baseline tests)

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

## 📊 MÉTRICAS DE QUALIDADE

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

## 🎓 EXEMPLOS DE CONFORMIDADE

### ✅ Exemplo CORRETO - Task 5

```
Commit: 741e66b1
Mensagem: "fix: correct SetLocal opcode handler to load local address before storing"

Estrutura:
1. Identifica problema específico
2. Descreve solução
3. One commit = one fix
4. Todos os testes passam
5. Sem warnings novos
6. Código auto-documentado

Resultado: Pronto para produção ✅
```

### ❌ Exemplo ERRADO (hipotético)

```
Commit: abc123def
Mensagem: "update stuff"

Problemas:
- Mensagem vaga
- Múltiplas features em um commit
- Alguns testes falhando
- Warnings não tratados
- Refactoring + feature + bugfix tudo junto

Resultado: Rejeitado ❌
```

---

## 🔗 INTEGRAÇÃO COM ECOSSISTEMA

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

## 📞 CONTATO E REVISÃO

### Code Review Obrigatório Para
- ✅ Mudanças em opcode definitions
- ✅ Novos backends/generators
- ✅ Mudanças em IR core
- ✅ Refactorings maiores

### Aprovadores
- Tech Lead (decisões de arquitetura)
- Code Reviewer (qualidade de código)
- Test Reviewer (cobertura de testes)

---

## 📝 CHANGELOG

### v1.0 (2026-03-22)
- Initial manifest creation
- Baseline rules for compiler development
- 7 core principles established
- Workflow documentation

---

**Este manifesto é vinculante a partir da data acima.**  
**Toda implementação DEVE estar em conformidade.**  
**Exceções requerem aprovação explícita do Tech Lead.**

**Última atualização**: 2026-03-22
