# Dryad AOT Compiler

Compilador Ahead-of-Time (AOT) que converte código Dryad para executáveis nativos.

## Arquitetura

```
Código Dryad (.dryad)
    ↓
Bytecode (dryad_bytecode)
    ↓
IR (Intermediate Representation)
    ↓
Backend (x86_64, ARM64)
    ↓
Arquivo Objeto (.o)
    ↓
Linker (gcc/clang)
    ↓
Executável Nativo (ELF/PE)
```

## Uso

### Como Biblioteca

```rust
use dryad_aot::{AotCompiler, Target};

let compiler = AotCompiler::new(Target::X86_64Linux);
compiler.compile_file("script.dryad", "output")?;
```

### CLI (Futuro)

```bash
dryad build script.dryad -o programa
dryad build script.dryad --target=x86_64-windows -o programa.exe
```

## Estrutura

- `ir/` - Intermediate Representation
  - Instruções de baixo nível
  - Tipos
  - Módulos e funções
  
- `backend/` - Backends de arquitetura
  - `x86_64.rs` - Backend x86_64
  - `arm64.rs` - Backend ARM64
  
- `generator/` - Geradores de formato
  - `elf.rs` - Gerador ELF (Linux)
  - `pe.rs` - Gerador PE (Windows)
  
- `compiler/` - Orquestração
  - `converter.rs` - Bytecode → IR
  - `options.rs` - Opções de compilação

## Status

- [x] Estrutura básica
- [x] IR completa  
- [x] Conversor Bytecode → IR (completo para 60+ opcodes)
- [x] Backend x86_64 (partial)
- [x] Backend ARM64 (completo)
- [x] Gerador ELF básico
- [x] Gerador PE (PE32+ completo)
- [x] Otimizações (DCE + constant folding)
- [x] Local variables (stack allocation)
- [ ] Debug info (DWARF)
- [ ] Runtime library linking

## Alvos Suportados

| Alvo | Status |
|------|--------|
| x86_64 Linux | 🚧 Em desenvolvimento |
| x86_64 Windows | ⏳ Planejado |
| ARM64 Linux | ⏳ Planejado |
| ARM64 macOS | ⏳ Planejado |

## Exemplos

Ver diretório `examples/`.

## Licença

MIT
