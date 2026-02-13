# Engenharia e Implementação (Core)

A implementação da Dryad é focada em modularidade e segurança, utilizando o ecossistema Rust para garantir um runtime robusto e de alta performance.

## 🚀 Leitura Rápida

- **Linguagem Core**: Escrita 100% em **Rust**.
- **Modelo**: Interpretador Tree-Walking (Execução direta de AST).
- **Módulos**: Organizados em crates independentes (`dryad_lexer`, `dryad_parser`, etc).
- **Extensível**: Sistema de funções nativas via FFI.

---

## ⚙️ Visão Técnica

### 1. Arquitetura Baseada em Crates

O projeto utiliza um **Workspace do Cargo**, o que permite compilar componentes isoladamente, facilitando testes unitários e linting.

| Crate           | Responsabilidade       | Tecnologia Chave      |
| :-------------- | :--------------------- | :-------------------- |
| `dryad_lexer`   | Análise Léxica         | Logos / State Machine |
| `dryad_parser`  | Gramática e AST        | Recursive Descent     |
| `dryad_runtime` | Interpretador e Scopes | Environment Stacks    |
| `dryad_errors`  | Diagnósticos           | Miette / Diagnostics  |

### 2. O Ciclo de Vida da Execução

Diferente de sistemas baseados em Bytecode (como Python ou Node), o Dryad atualmente percorre a árvore sintática:

1.  **Frontend**: O `dryad_cli` recebe o arquivo e instancia o `Lexer`.
2.  **Middle**: O `Parser` transforma os tokens em nós `Stmt` e `Expr`.
3.  **Backend**: O `Interpreter` (Runtime) visita cada nó, alternando entre `execute` e `evaluate`.

### 3. Sistema de Funções Nativas (FFI)

As bibliotecas padrão (`std_io`, `std_http`) são conectadas ao runtime através de um mapeamento de nomes de funções Dryad para closures do Rust, que possuem acesso ao estado do interpretador.

---

## 📚 Referências e Paralelos

- **Rust Architecture**: [The Cargo Book - Workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html).
- **Design Pattern**: [Visitor Pattern](https://refactoring.guru/design-patterns/visitor) - Base do motor de execução.
- **Parsing Theory**: [Recursive Descent Parsers](https://en.wikipedia.org/wiki/Recursive_descent_parser).

---

## Próximos Passos (Roadmap Técnico)

- [ ] Implementação de **Bytecode VM** para performance 10x superior.
- [ ] JIT experimental utilizando **Cranelift** ou **LLVM**.
- [ ] Otimização de Garbage Collection para ciclos complexos.
