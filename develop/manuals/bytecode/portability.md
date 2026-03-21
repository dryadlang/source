# Portabilidade x86/ARM do Bytecode

## Visão Geral

O Bytecode VM do Dryad foi projetado desde o início para ser **100% portável** entre arquiteturas x86 e ARM. O bytecode é uma representação intermediaria independente de arquitetura.

## Por que Bytecode é Portável?

### 1. Abstração de Hardware

O bytecode opera em um nível de abstração alto:
- Não usa registros específicos da CPU
- Não depende de endianness
- Não usa instruções de assembly
- Stack-based (não depende de arquitetura de registradores)

### 2. Representação dos Dados

```rust
// Valores são representados de forma uniforme
pub enum Value {
    Nil,
    Boolean(bool),      // 1 byte
    Number(f64),        // IEEE 754 - padrão em todas as arquiteturas
    String(String),     // UTF-8 - independente de arquitetura
    Object(HeapId),     // Referência opaca
    Function(Rc<Function>),  // Ponteiro gerenciado
}
```

### 3. Opcodes

Todos os opcodes são representados como enum Rust:
```rust
pub enum OpCode {
    Constant(u8),       // Índice na tabela
    Add,                // Operação abstrata
    Call(u8),           // Número de argumentos
    // ...
}
```

**Não há:**
- Endianness específico
- Tamanho de palavra hardcoded
- Dependências de alignment
- Código assembly inline

## Compatibilidade Garantida

### ✅ x86 (32-bit e 64-bit)
- Intel/AMD processors
- Windows, Linux, macOS

### ✅ ARM
- ARMv7, ARMv8 (AArch64)
- Linux, macOS (Apple Silicon), Android
- Raspberry Pi, dispositivos embarcados

### ✅ Outras Arquiteturas
- WebAssembly (via compilação)
- RISC-V (futuro)

## Implementação Portável

### Heap Management
```rust
pub struct Heap {
    objects: HashMap<HeapId, Rc<RefCell<Object>>>,
    next_id: u64,  // u64 é consistente em todas as arquiteturas
}
```

### Stack VM
```rust
pub struct VM {
    stack: Vec<Value>,     // Vec gerencia memória automaticamente
    frames: Vec<CallFrame>, // Sem pointers crus
    // ...
}
```

### Sem Unsafe Code
O bytecode não usa `unsafe` do Rust, garantindo:
- Memory safety
- Thread safety
- Portabilidade

## Future: JIT Backends

Quando implementarmos JIT no futuro, teremos backends específicos:

```
Bytecode VM (portável)
       │
       ├── JIT x86_64 Backend
       │   └── Gera código x86-64 nativo
       │
       ├── JIT ARM64 Backend  
       │   └── Gera código ARM64 nativo
       │
       └── JIT WASM Backend (futuro)
           └── Gera WebAssembly
```

### Arquitetura do JIT (Futuro)

```rust
// Trait para backends JIT
trait JitBackend {
    fn compile_function(&self, bytecode: &Chunk) -> Result<NativeCode, Error>;
}

struct X86_64Backend;
struct ARM64Backend;

impl JitBackend for X86_64Backend {
    fn compile_function(&self, bytecode: &Chunk) -> Result<NativeCode, Error> {
        // Gera código x86-64
    }
}

impl JitBackend for ARM64Backend {
    fn compile_function(&self, bytecode: &Chunk) -> Result<NativeCode, Error> {
        // Gera código ARM64
    }
}
```

## Testes de Portabilidade

### CI/CD Pipeline
```yaml
# .github/workflows/ci.yml
strategy:
  matrix:
    os: [ubuntu-latest, windows-latest, macos-latest]
    arch: [x86_64, aarch64]
    
jobs:
  test:
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v3
      - name: Test Bytecode
        run: cargo test --package dryad_bytecode
```

### Targets Suportados
```bash
# Compilar para x86_64 Linux
cargo build --target x86_64-unknown-linux-gnu

# Compilar para ARM64 Linux  
cargo build --target aarch64-unknown-linux-gnu

# Compilar para ARM64 macOS (Apple Silicon)
cargo build --target aarch64-apple-darwin

# Compilar para ARM Android
cargo build --target armv7-linux-androideabi
```

## Compatibilidade de Dados

### Serialização de Bytecode
Quando implementarmos cache de bytecode:

```rust
// Bytecode salvo em formato binário portável
struct BytecodeFile {
    magic: [u8; 4],      // 'DRY\0'
    version: u16,        // Versão do formato
    arch: Architecture,  // Arquitetura alvo (para JIT)
    constants: Vec<Constant>,
    code: Vec<OpCode>,
}

enum Architecture {
    Bytecode,    // Código portável (padrão)
    X86_64,      // Código nativo x86-64
    ARM64,       // Código nativo ARM64
    // ...
}
```

**Formato binário será:**
- Little-endian (padrão)
- Alignment natural
- Versionado
- Checksum para integridade

## Performance por Arquitetura

### x86_64
- Melhor performance em computação pesada
- Excelente para benchmarks matemáticos
- JIT mais maduro

### ARM64  
- Melhor eficiência energética
- Ótimo para dispositivos móveis
- Apple Silicon muito rápido

### Bytecode Interpretado
- Mesma performance em todas as arquiteturas
- Overhead de interpretação
- Compatibilidade máxima

## Recomendações

### Desenvolvimento
1. **Use bytecode** durante desenvolvimento (portável)
2. **Teste em múltiplas arquiteturas** via CI
3. **Evite unsafe code** que possa quebrar portabilidade

### Produção
1. **Bytecode** para máxima compatibilidade
2. **JIT** quando necessário (por arquitetura)
3. **Cache de bytecode** compilado (futuro)

### Deployment
```bash
# Deploy universal (bytecode)
./dryad run script.dryad --compile

# Deploy com JIT x86_64 (futuro)
./dryad run script.dryad --compile --jit=x86_64

# Deploy com JIT ARM64 (futuro)  
./dryad run script.dryad --compile --jit=arm64
```

## Checklist de Portabilidade

- [x] Código 100% Rust (portável)
- [x] Sem dependências de arquitetura
- [x] Sem código assembly inline
- [x] Sem unsafe code crítico
- [x] IEEE 754 para floats
- [x] UTF-8 para strings
- [x] Endianness independente
- [ ] Testes em ARM64 (pendente hardware)
- [ ] CI multi-arquitetura
- [ ] Benchmarks comparativos

## Referências

- [Rust Platform Support](https://doc.rust-lang.org/nightly/rustc/platform-support.html)
- [IEEE 754 Standard](https://ieeexplore.ieee.org/document/8766229)
- [ARM Architecture](https://developer.arm.com/architectures)
- [x86-64 Architecture](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html)
