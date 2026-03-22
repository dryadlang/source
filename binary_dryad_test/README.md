# 🚀 Teste de Compilação Dryad → PE Binary

Este diretório contém um teste end-to-end do pipeline de compilação AOT do Dryad.

## 📁 Estrutura

```
binary_dryad_test/
├── Cargo.toml          # Configuração do projeto Rust
├── main.dryad          # Código Dryad de entrada
├── src/main.rs         # Program a que executa o pipeline
├── test_program.exe    # Executável PE gerado (saída)
└── RESULTADO.md        # Resultado da compilação
```

## 🏃 Como Executar

### Primeira execução (compilação necessária)

```bash
cd binary_dryad_test
cargo run
```

### Execuções posteriores (sem recompilação)

```bash
cargo run --release
```

## 📊 Pipeline Demonstrado

```
main.dryad (código fonte)
    ↓
[Lexer] → tokens
    ↓
[Parser] → AST
    ↓
[Bytecode Compiler] → opcodes
    ↓
[BytecodeToIrConverter] → IR
    ↓
[PeGenerator] → binário PE32+
    ↓
test_program.exe (executável Windows)
```

## 📝 Código de Exemplo

O arquivo `main.dryad` contém:
```dryad
5 + 3
```

Simples, mas demonstra:
- Parsing de expressões
- Compilação para bytecode
- Conversão para IR
- Geração de PE32+ válido

## ✅ Validação

O executável gerado é validado:
- Magic bytes "MZ" (DOS header)
- Assinatura PE válida
- Tamanho mínimo 512 bytes
- Estrutura correta do PE32+

```bash
file test_program.exe
# PE32+ executable for MS Windows 6.00 (console), x86-64
```

## 🔧 Modificar o Código

Para testar com diferentes expressões, edite `main.dryad`:

```dryad
# Expressões suportadas (ver constraints abaixo)
5 + 3       # adição
10 - 2      # subtração
3 * 4       # multiplicação
8 / 2       # divisão
2 << 1      # shift left
8 >> 1      # shift right
# etc...
```

## ⚠️ Constraints Conhecidos

Atualmente o conversor `BytecodeToIrConverter` suporta:
- ✅ Constantes: Constant, Nil, True, False
- ✅ Aritmética: Add, Sub, Mul, Div, Mod, Negate
- ✅ Comparação: Equal, Greater, Less, GreaterEqual, LessEqual
- ✅ Lógica: Not, And, Or
- ✅ Bitwise: BitAnd, BitOr, BitXor, BitNot, ShiftLeft, ShiftRight
- ✅ Variáveis locais: GetLocal, SetLocal
- ❌ Variáveis globais: DefineGlobal, GetGlobal, SetGlobal
- ❌ Controle de fluxo: Jump, JumpIfFalse, JumpIfTrue, Loop
- ❌ Funções: Call, Closure, etc.

## 📚 Estrutura do Código

### `src/main.rs`

1. **Lexer**: Tokenização do código Dryad
2. **Parser**: Construção da árvore sintática (AST)
3. **Compiler**: Compilação de AST para bytecode
4. **BytecodeToIrConverter**: Conversão bytecode → IR
5. **PeGenerator**: Geração do binário PE32+

Cada fase é exibida com informações de debug.

## 🎯 Próximas Melhorias

- [ ] Suporte a mais opcodes do bytecode
- [ ] Geração real de código x86_64 (atualmente usa NOPs)
- [ ] Otimizações de IR
- [ ] Linking com runtime libraries
- [ ] Debug info (DWARF)

## 📋 Saída Esperada

```
📄 Código Dryad lido (7 bytes)
─────────────────────────────────────
5 + 3
─────────────────────────────────────

🔤 Fase 1: Tokenização (Lexer)...
✓ 4 tokens gerados

🌳 Fase 2: Parsing (Parser)...
✓ AST construída (1 statements)

📦 Fase 3: Compilação para Bytecode...
✓ Bytecode gerado (6 opcodes)

Primeiros opcodes:
  [0] Constant(0)
  [1] Constant(1)
  [2] Add
  [3] Pop
  [4] Nil
  [5] Return
  ...

🔵 Fase 4: Conversão para IR...
✓ IR gerada com 1 funcções
  Função 0: main

🪟 Fase 5: Geração de Binário PE (Windows)...
✓ PE binary gerado (1376 bytes)

📋 Verificação do PE Binary:
  Magic: [77, 90]
  PE Signature: "PE\0\0"
  Tamanho mínimo (512 bytes): true

💾 Fase 6: Salvando PE binary...
✓ Binário salvo em: test_program.exe
  Tamanho final: 1376 bytes (1.34 KB)

✅ Compilação completa!
═══════════════════════════════════
Pipeline: Dryad → Bytecode → IR → PE
```

## 🤝 Licença

Mesmo do projeto Dryad (MIT)
