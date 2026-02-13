# Controle de Fluxo

O controle de fluxo no Dryad segue a sintaxe imperativa clássica (C-Style), permitindo ramificações e iterações complexas.

## 🚀 Leitura Rápida

- **Condicionais**: `if`, `else if`, `else`.
- **Loops**: `while` (enquanto), `for` (clássico), `for-in` (objetos).
- **Lógica**: Baseada em valores Truthy e Falsy.
- **Sintaxe**: Parênteses obrigatórios e chaves recomendadas.

---

## ⚙️ Visão Técnica

O interpretador gerencia o fluxo de controle através da avaliação condicional de nós da AST.

### 1. Lógica Truthy/Falsy

No Dryad, qualquer valor pode ser convertido para booleano em contextos de controle.

| Valor                      | Avaliação  | Nota Técnica                                |
| :------------------------- | :--------- | :------------------------------------------ |
| `false`, `null`, `0`, `""` | **Falsy**  | Mapeado para `false` no runtime.            |
| Todos os outros            | **Truthy** | Incluindo arrays/objetos vazios `[]`, `{}`. |

Internamente, isso é implementado pelo método `is_truthy(Value) -> bool`, crucial para o nó `Stmt::If`.

### 2. Implementação de Loops

Os loops no Dryad são implementados através de recursão controlada ou loops nativos do Rust no interpretador.

- **`while`**: Avalia a condição; se true, executa o corpo e reinicia o processo.
- **`for-in`**: Otimizado para iterar sobre as chaves de um `HashMap` (Object) de forma segura, garantindo que mutações no objeto durante a iteração não causem crashes no interpretador (usando snapshots das chaves).

### 3. Early Returns e Breaks

O interpretador utiliza um mecanismo de "Short-circuit" baseado em resultados especiais (como `Signal::Return` ou `Signal::Break`) para interromper a execução de blocos e propagar sinais para os controladores de loop ou funções.

---

## 📚 Referências e Paralelos

- **C-Style Syntax**: Inspirado em [ANSI C](https://en.wikipedia.org/wiki/ANSI_C).
- **Control Flow Analysis**: [Compilers: Principles, Techniques, and Tools](https://en.wikipedia.org/wiki/Compilers:_Principles,_Techniques,_and_Tools) - Capítulo sobre Controle de Fluxo.
- **Rust If-Let**: O Dryad planeja suporte a padrões de matching similares ao `if let` do Rust no futuro.
