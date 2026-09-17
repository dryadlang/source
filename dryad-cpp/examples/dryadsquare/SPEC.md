# Square Language Specification (v0.1)

Square is a minimal imperative language used by **DryadSquare** to benchmark Turing-completeness and
self-hosting feasibility of Dryad. The entire compiler, virtual machine, and test harness are written
in Dryad only — no C++ extensions for this benchmark.

## Design goals

1. **Turing-complete** — sufficient to simulate any Turing machine (unbounded variables + `while`).
2. **Small surface** — easy to lex, parse, compile, and audit.
3. **Honest benchmark** — no host-language shortcuts; stdlib imports are allowed (they are Dryad code).

## Syntax

```text
program     := statement*
statement   := varDecl | assign | printStmt | whileStmt | ifStmt | block
varDecl     := 'var' IDENT '=' expression ';'
assign      := IDENT '=' expression ';'
printStmt   := 'print' expression ';'
whileStmt   := 'while' '(' expression ')' block
ifStmt      := 'if' '(' expression ')' block ('else' block)?
block       := '{' statement* '}'
expression  := comparison
comparison  := term (('<' | '<=' | '==' | '!=' | '>=' | '>') term)?
term        := factor (('+' | '-') factor)*
factor      := unary (('*' | '/' | '%') unary)*
unary       := '-' unary | primary
primary     := NUMBER | IDENT | '(' expression ')'
```

Comments: `//` to end of line.

Keywords: `var`, `while`, `if`, `else`, `print`.

## Semantics

- Variables live in an unbounded map (name → integer). Reading an undefined variable is an error.
- Integers are host integers (Dryad `number`). Loops and arithmetic are unbounded in principle.
- `print` writes the integer value followed by a newline to standard output.
- Comparisons return `1` (true) or `0` (false).
- Division/modulo by zero is a runtime error.

## Turing-completeness sketch

Square has:

- Unbounded storage (`var` bindings in a map that grows with the program).
- Conditional branching (`if`, comparison operators).
- Unbounded iteration (`while`).

This matches the usual imperative model used in textbooks: a `while` over counters with zero tests
can encode a two-counter Minsky machine, which is Turing-complete. Square is therefore
**Turing-complete** (not merely Turing-powerful for bounded memory).

## Bytecode (Square VM)

| Opcode | Name        | Operand(s)     | Effect                          |
|--------|-------------|----------------|---------------------------------|
| 0      | HALT        | —              | Stop execution                  |
| 1      | CONST       | value          | Push constant                   |
| 2      | LOAD        | name_index     | Push variable value             |
| 3      | STORE       | name_index     | Pop → variable                  |
| 4      | ADD         | —              | Pop b, pop a, push a+b          |
| 5      | SUB         | —              | Pop b, pop a, push a-b          |
| 6      | MUL         | —              | Pop b, pop a, push a*b          |
| 7      | DIV         | —              | Pop b, pop a, push a/b (int)    |
| 8      | MOD         | —              | Pop b, pop a, push a%b           |
| 9      | LT          | —              | Pop b, pop a, push a<b ? 1 : 0  |
| 10     | EQ          | —              | Pop b, pop a, push a==b ? 1 : 0 |
| 11     | JMP         | offset         | pc = offset                     |
| 12     | JMP_IF_FALSE| offset         | Pop; if zero, pc = offset       |
| 13     | PRINT       | —              | Pop and print value             |

Programs bundle `code` (instruction list) and `names` (string table for variables).

## Self-hosting roadmap

| Phase | Goal                                      | Status   |
|-------|-------------------------------------------|----------|
| 0     | Spec + Turing argument                    | Done     |
| 1     | Compiler + VM in Dryad                    | Done     |
| 2     | Golden tests (interpreter / VM / AOT)     | Done     |
| 3     | Square subset rewrites DryadSquare        | Future   |
| 4     | Full self-host (Square compiles Square)    | Future   |

Phase 3–4 require growing Square (functions, strings, modules) without cheating on the host compiler.
