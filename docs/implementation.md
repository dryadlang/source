# Engenharia e Implementação (Core)

A implementação da Dryad é focada em modularidade e segurança, utilizando o ecossistema Rust para garantir um runtime robusto e de alta performance.

## 🚀 Leitura Rápida

- **Linguagem Core**: Escrita 100% em **Rust**.
- **Modelo**: Interpretador Tree-Walking (Execução direta de AST).
- **Módulos**: Organizados em crates independentes no workspace.
- **Extensível**: Sistema de funções nativas modularizado.

---

## ⚙️ Visão Técnica

### 1. Arquitetura de Crates

O projeto utiliza um **Workspace do Cargo**, distribuindo responsabilidades em unidades compiláveis de forma independente.

| Crate           | Responsabilidade               | Componentes Principais                  |
| :-------------- | :----------------------------- | :-------------------------------------- |
| `dryad_lexer`   | Análise Léxica e Tokenização   | `lexer.rs`, `token.rs`, `source.rs`     |
| `dryad_parser`  | Parsing de AST e Gramática     | `parser.rs`, `ast.rs`                   |
| `dryad_runtime` | Driver de Execução e Runtime   | `interpreter.rs`, `environment.rs`, etc |
| `dryad_errors`  | Gestão de Erros e Diagnósticos | `lib.rs`, `RuntimeError`                |
| `dryad_cli`     | Interface de Linha de Comando  | `main.rs`, `repl.rs`                    |
| `oak`           | Gerenciador de Pacotes         | `commands/`, `core/`                    |

### 2. Modularização do Interpretador

O interpretador central (`interpreter.rs`) delega a gestão de estado e recursos para sub-módulos especializados na crate `dryad_runtime`:

- **Environment**: Gerencia a pilha de escopos (variáveis locais e globais).
- **NativeRegistry**: Única fonte de verdade para descoberta e despacho de funções nativas.
- **Heap**: Gerencia o ciclo de vida de objetos complexos com suporte a Garbage Collection.

### 3. Fases de Implementação (Log)

O desenvolvimento segue um cronograma de estabilização e refatoração:

- **Fase 1 (Segurança)**: Implementação de Proteção de Recursão, Sandbox de FS e Ativação Estrita de Módulos.
- **Fase 2 (Estrutura)**: Modularização do Interpretador, extração do `Environment` e implementação do GC Mark-and-Sweep.
- **Fase 3 (Expansão)**: Unificação de módulos nativos e otimização de performance (em progresso).

---

## 📚 Referências de Engenharia

- **Pattern Design**: [Delegation Pattern](https://en.wikipedia.org/wiki/Delegation_pattern) - Utilizado para separar `Environment` do `Interpreter`.
- **Memory Safety**: [Rust Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html) - Base de toda a segurança do runtime.

---

## Roadmap Técnico Atualizado

- [x] Refatoração Modular do Interpretador.
- [x] Implementação de Garbage Collection Automático.
- [ ] Migração para Bytecode VM (Planned).
- [ ] JIT experimental utilizando Cranelift.
