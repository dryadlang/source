# Language Diagrams (PlantUML)

Visual reference for the Dryad language core and standard library.

## Location

```
docs/diagrams/dryad-language/
├── dryad-language.puml      # Master index (includes all parts)
└── parts/
    ├── 01-architecture.puml
    ├── 02-lexical.puml
    ├── 03-ast.puml
    ├── 04-types.puml
    ├── 05-compiler.puml
    ├── 06-execution.puml
    ├── 07-bytecode-microir.puml
    ├── 08-modules.puml
    ├── 09-runtime.puml
    ├── 10-intrinsics.puml
    └── 11-stdlib.puml
```

## Render

Install (Arch/CachyOS):

```bash
sudo pacman -S plantuml graphviz   # graphviz optional; diagrams use smetana layout
```

```bash
# All diagrams
plantuml docs/diagrams/dryad-language/parts/*.puml

# Single diagram
plantuml docs/diagrams/dryad-language/parts/03-ast.puml
```

In Cursor/VS Code, install the **PlantUML** extension (`jebbs.plantuml`) and open a file under `parts/` for preview. The master file `dryad-language.puml` shows an index card; individual diagrams live in `parts/*.puml`.

## Diagram index

| # | Title | Contents |
|---|-------|----------|
| 01 | Architecture | Layers, artifacts, triple execution overview |
| 02 | Lexical & Syntax | Tokens, 37 keywords, operators, decorators |
| 03 | AST | 33 node types, class hierarchy |
| 04 | Type System | ValueType, profiler, annotations, IrType |
| 05 | Compiler Pipeline | Lexer → parser → bytecode → AOT |
| 06 | Execution Model | Interpreter / VM / AOT dispatch |
| 07 | Bytecode & Micro-IR | All VM opcodes + AOT IR |
| 08 | Modules & Libraries | .dyd/.dyl, exports, packages |
| 09 | Runtime | Environment, OOP, VM, async |
| 10 | Intrinsics | Syscall registry by category |
| 11 | Standard Library | @std/* module tree + dependencies |

## Source of truth

Diagrams are derived from `dryad-cpp/include/dryad/` and `dryad-cpp/stdlib/@std/`. When code changes, update the corresponding `.puml` part.
