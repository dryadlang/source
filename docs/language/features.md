# Funcionalidades e Roadmap

Este documento acompanha o status de implementação de cada recurso da linguagem Dryad e planeja o futuro.

## Status Atual

Legenda: ✅ Implementado | ⚠️ Parcial | ❌ Não Implementado

### Core
- ✅ **Variáveis**: `let`, `const`.
- ✅ **Tipos Básicos**: Number, String, Bool, Null.
- ✅ **Tipos Compostos**: Array, Tuple, Object.
- ✅ **Controle de Fluxo**: If, While, Do-While, For, For-In.
- ✅ **Funções**: Declaração, Expressão, Lambda, Closure.

### Orientação a Objetos
- ✅ **Classes**: Declaração básica.
- ✅ **Herança**: `extends`.
- ✅ **Métodos**: Instância e Estáticos.
- ✅ **Propriedades**: Com inicialização.
- ⚠️ **Super**: Keyword reservada, mas runtime ainda não suporta chamadas `super.metodo()`.
- ⚠️ **Visibilidade**: `public`, `private`, `protected` são parseados mas ignorados no runtime (tudo é público).

### Concorrência
- ✅ **Threads**: Criação e execução real (`thread function`).
- ✅ **Async/Await**: Sintaxe suportada e execução básica.
- ✅ **Mutex**: Primitiva de sincronização implementada.

### Módulos
- ✅ **Sistema de Import/Export**: Estilo ES6.
- ✅ **Módulos Nativos**: Sistema de plugins via diretivas `#`.

## Roadmap Futuro

### Curto Prazo (v0.2)
1.  **Melhoria de Erros**: Mensagens de erro mais amigáveis no Parser.
2.  **Stdlib**: Expandir módulos nativos (`file_io`, `http`) para cobrir mais casos de uso.
3.  **Otimização**: Implementar cache de variáveis no interpretador para evitar lookups constantes em HashMap.

### Médio Prazo (v0.5)
1.  **Garbage Collection**: Implementar um GC real (atualmente usa contagem de referência do Rust/RC ou vazamento controlado).
2.  **Bytecode VM**: Migrar de Tree-Walking para Bytecode para performance.

### Longo Prazo (v1.0)
1.  **JIT Compiler**: Compilação Just-In-Time para código nativo.
2.  **LSP Server**: Servidor de linguagem completo para VS Code.
