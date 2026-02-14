# Bytecode VM - Documentação de Implementação

## Visão Geral

A Máquina Virtual Bytecode do Dryad foi implementada seguindo a abordagem híbrida recomendada, focando inicialmente em uma VM baseada em pilha robusta e eficiente.

## Estrutura do Projeto

```
crates/dryad_bytecode/
├── Cargo.toml              # Configuração da crate
└── src/
    ├── lib.rs              # API pública e re-exportações
    ├── opcode.rs           # Definição dos opcodes
    ├── value.rs            # Sistema de tipos dinâmicos
    ├── chunk.rs            # Armazenamento de bytecode
    ├── vm.rs               # Máquina Virtual principal
    ├── compiler.rs         # Compilador AST -> Bytecode
    └── debug.rs            # Disassembler e utilitários
```

## Arquitetura

### 1. Opcodes (opcode.rs)

Os opcodes são organizados por categoria para facilitar manutenção e otimizações futuras:

```rust
pub enum OpCode {
    // Constantes
    Constant(u8), ConstantLong(u16), Nil, True, False,
    
    // Aritmética
    Add, Subtract, Multiply, Divide, Modulo, Negate,
    
    // Comparações
    Equal, Greater, Less, GreaterEqual, LessEqual,
    
    // Lógicas
    Not, And, Or,
    
    // Bitwise
    BitAnd, BitOr, BitXor, BitNot, ShiftLeft, ShiftRight,
    
    // Variáveis
    DefineGlobal(u8), GetGlobal(u8), SetGlobal(u8),
    GetLocal(u8), SetLocal(u8),
    
    // Controle de Fluxo
    Jump(u16), JumpIfFalse(u16), JumpIfTrue(u16), Loop(u16),
    Break, Continue,
    
    // Funções
    Call(u8), Return, Closure(u8),
    GetUpvalue(u8), SetUpvalue(u8), CloseUpvalue,
    
    // Objetos
    Class(u8), Method(u8), Invoke(u8),
    GetProperty(u8), SetProperty(u8), This, Super(u8),
    
    // Coleções
    Array(u16), Index, SetIndex, Tuple(u8), TupleAccess(u8),
    
    // Pilha
    Pop, PopN(u8), Dup, DupN(u8), Swap,
    
    // Misc
    Print, PrintLn, Nop, Halt,
}
```

### 2. Sistema de Valores (value.rs)

Implementa tipagem dinâmica com suporte a objetos gerenciados:

```rust
pub enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
    Object(HeapId),  // Referência para objetos no heap
}

pub enum Object {
    Instance { class_name, fields },
    Class { name, methods, superclass },
    Array(Vec<Value>),
    Tuple(Vec<Value>),
    Closure(Rc<Function>, Vec<Value>),
    Map(HashMap<String, Value>),
}
```

**Operações suportadas:**
- Aritméticas: add, subtract, multiply, divide, modulo, negate
- Comparações: greater, less, greater_equal, less_equal
- Lógicas: is_truthy, not
- Bitwise: bit_and, bit_or, bit_xor, bit_not, shift_left, shift_right

### 3. Chunks (chunk.rs)

Unidade básica de armazenamento de bytecode:

```rust
pub struct Chunk {
    pub code: Vec<OpCode>,      // Vetor de opcodes
    pub constants: Vec<Value>,  // Tabela de constantes
    pub lines: Vec<usize>,      // Mapeamento para linhas do código fonte
    pub name: String,           // Nome do chunk (para debug)
}
```

**Capacidades:**
- Tabela de constantes até 65536 valores (u16)
- Mapeamento de linhas para stack traces
- Builder pattern para construção facilitada

### 4. VM (vm.rs)

Máquina Virtual baseada em pilha com:

```rust
pub struct VM {
    stack: Vec<Value>,                  // Pilha de valores
    frames: Vec<CallFrame>,             // Frames de chamada
    globals: HashMap<String, Value>,    // Variáveis globais
    heap: Heap,                         // Gerenciamento de objetos
    debug_mode: bool,                   // Flag de debug
    max_frames: usize,                  // Limite de recursão
}
```

**Features:**
- Execução de opcodes com verificação de tipos
- Gerenciamento de escopos (locais e globais)
- Chamadas de função com frames
- Modo de debug com trace de execução

### 5. Compilador (compiler.rs)

Traduz AST do Dryad para bytecode:

```rust
pub struct Compiler {
    current_chunk: Chunk,
    locals: Vec<Local>,
    scope_depth: usize,
    chunks: Vec<Chunk>,
}
```

**Suporte atual:**
- Expressões literais, variáveis, binárias, unárias
- Declarações de variáveis (var, const)
- Atribuições simples
- Blocos e escopos
- Controle de fluxo: if/else, while, do-while, for
- **Funções completas**: declaração, chamada, return, parâmetros
- Arrays e tuples (básico)

**Em desenvolvimento:**
- Closures (upvalues - parcial)
- Classes e objetos
- Exceções (try/catch)
- ForEach
- Incremento/decremento

### 6. Debug (debug.rs)

Disassembler para visualização de bytecode:

```rust
pub struct Disassembler;

impl Disassembler {
    pub fn disassemble(chunk: &Chunk, name: &str);
    pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize;
}

// Trait para facilitar uso
pub trait DebugChunk {
    fn disassemble(&self, name: &str);
    fn disassemble_instruction(&self, offset: usize) -> usize;
}
```

## Exemplo de Uso

```rust
use dryad_bytecode::{Compiler, VM, InterpretResult};
use dryad_parser::Parser;

fn main() {
    // Código fonte
    let source = r#"
        var x = 10;
        var y = 20;
        print x + y;
    "#;
    
    // Parse
    let program = Parser::new(Lexer::new(source).tokenize()).parse();
    
    // Compilação
    let mut compiler = Compiler::new();
    let chunk = compiler.compile(program).expect("Erro de compilação");
    
    // Debug: mostra bytecode
    chunk.disassemble("script");
    
    // Execução
    let mut vm = VM::new();
    vm.set_debug_mode(true);
    
    match vm.interpret(chunk) {
        InterpretResult::Ok => println!("Execução bem-sucedida!"),
        InterpretResult::CompileError => eprintln!("Erro de compilação"),
        InterpretResult::RuntimeError => eprintln!("Erro em tempo de execução"),
    }
}
```

## Checklist de Implementação

### Fase 1 - Base ✅
- [x] Sistema de opcodes organizado por categoria
- [x] Tipos de valores dinâmicos (Nil, Boolean, Number, String, Object)
- [x] Heap para gerenciamento de objetos
- [x] Chunks de bytecode com constantes e linhas
- [x] VM baseada em pilha com loop principal
- [x] Operações aritméticas básicas
- [x] Constantes e literais
- [x] Disassembler para debug

### Fase 2 - Variáveis ✅
- [x] Variáveis locais com escopo
- [x] Variáveis globais
- [x] Pop para limpeza de pilha
- [x] Sistema de Locals com depth

### Fase 3 - Controle de Fluxo ✅
- [x] Jump e JumpIfFalse/JumpIfTrue
- [x] Loop para while
- [x] If/else com patching de offsets
- [x] Break e Continue (opcodes definidos)

### Fase 4 - Coleções (Parcial)
- [x] Arrays (opcodes definidos)
- [x] Tuples (opcodes definidos)
- [x] Indexação (opcodes definidos)
- [ ] Implementação completa na VM

### Fase 5 - Funções ✅
- [x] Definição de funções
- [x] Chamadas de função
- [x] Return de valores
- [x] Parâmetros e argumentos
- [x] Variáveis locais em funções
- [x] Recursão
- [ ] Closures e upvalues (parcial - opcodes existem)

### Fase 6 - Objetos (Planejado)
- [ ] Classes
- [ ] Métodos
- [ ] Propriedades
- [ ] Herança (super)

### Fase 7 - Otimizações (Futuro)
- [ ] Constant folding
- [ ] Dead code elimination
- [ ] Peephole optimizations
- [ ] Cache de bytecode

## Integração com CLI

Para usar o modo bytecode na CLI:

```bash
# Executa com bytecode
dryad run script.dryad --compile

# Debug de bytecode
DRYAD_DEBUG_BYTECODE=1 dryad run script.dryad --compile
```

## Métricas de Performance (Metas)

| Métrica | Interpretador AST | Bytecode VM | Ganho |
|---------|-------------------|-------------|-------|
| Inicialização | 100% | 80% | 1.25x |
| Execução loops | 100% | 40% | 2.5x |
| Execução geral | 100% | 33% | 3x |
| Uso de memória | 100% | 70% | 1.4x |

**Meta geral:** 2-3x mais rápido que o interpretador AST atual.

## Próximos Passos

1. **✅ Funções implementadas** - Suporte completo a declaração, chamada, return
2. **Implementar classes** - Suporte completo a OOP
3. **Implementar closures** - Upvalues funcionais
4. **Testes** - Criar suite de testes abrangente
4. **Benchmarks** - Comparar performance com interpretador AST
5. **Otimizações** - Aplicar técnicas de otimização de bytecode
6. **Serialização** - Permitir salvar e carregar bytecode compilado

## Referências

- Crafting Interpreters (Robert Nystrom)
- Lua 5.1 VM Documentation
- Python Bytecode Documentation
- JVM Specification
