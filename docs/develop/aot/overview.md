---
title: "Visão Geral da Compilação AOT"
description: "Introdução ao sistema de compilação Ahead-of-Time para binários nativos"
category: "Desenvolvimento"
subcategory: "AOT"
order: 80
---

# Compilação AOT (Ahead-of-Time) Dryad

## Introdução

O sistema de compilação AOT do Dryad permite compilar código Dryad diretamente para executáveis nativos de alta performance, eliminando a necessidade de uma VM em runtime.

**Status:** � Em desenvolvimento (≈55% — núcleo AOT implementado; consulte `aot/status.md` para progresso)

---

## Objetivos

### Performance

- Executáveis **10-50x mais rápidos** que bytecode
- Startup **instantâneo** (< 1ms)
- **Sem overhead** de VM
- Otimizações em tempo de compilação

### Distribuição

- **Executáveis standalone** (sem dependências)
- **Tamanho reduzido** com tree shaking
- **Multiplataforma** (Linux, Windows, macOS)
- **Fácil deployment**

---

## Arquitetura

### Pipeline de Compilação

```
Código Fonte (.dryad)
    ↓
Parser → AST
    ↓
Bytecode Compiler
    ↓
IR (Intermediate Representation)
    ↓
Backend (x86_64/ARM64)
    ↓
Assembly (.s)
    ↓
Linker
    ↓
Executável Nativo (ELF/PE)
```

### Componentes Planejados

1. **IR Generator** - Converte bytecode para representação intermediária
2. **Backend x86_64** - Gera código assembly para x86_64
3. **Backend ARM64** - Gera código assembly para ARM64 (futuro)
4. **ELF Generator** - Cria executáveis Linux
5. **PE Generator** - Cria executáveis Windows
6. **Runtime Library** - Biblioteca C mínima para suporte
7. **Linker Integration** - Integração com ld/lld

---

## Plataformas Suportadas

### Linux (Prioridade 1)

- **Formato:** ELF64
- **Arquitetura:** x86_64
- **Syscalls:** Linux syscalls diretas
- **Libc:** Opcional (linkagem estática ou dinâmica)
- **Timeline:** Meses 2-4

### Windows (Prioridade 2)

- **Formato:** PE/COFF
- **Arquitetura:** x86_64
- **APIs:** kernel32.dll, msvcrt.dll
- **Convenção:** Windows x64 calling convention
- **Timeline:** Meses 4-6

### macOS (Futuro)

- **Formato:** Mach-O
- **Arquitetura:** x86_64, ARM64 (Apple Silicon)
- **Timeline:** Após v1.0

---

## Roadmap de Implementação

### Fase 1: Fundações (Meses 1-2)

- [ ] Criar crate `dryad_aot`
- [ ] Definir IR (Intermediate Representation)
- [ ] Implementar conversor Bytecode → IR
- [ ] Backend x86_64 básico
- [ ] Runtime mínimo em C

**Entregável:** "Hello World" compilado para Linux

### Fase 2: Linux ELF (Meses 2-4)

- [ ] Gerador ELF completo
- [ ] Linkagem estática e dinâmica
- [ ] Suporte a funções
- [ ] Suporte a arrays e strings
- [ ] Otimizações básicas

**Entregável:** Executáveis ELF funcionais

### Fase 3: Windows PE (Meses 4-6)

- [ ] Gerador PE/COFF
- [ ] Import Table (kernel32, msvcrt)
- [ ] Adaptação do runtime para Windows
- [ ] Cross-compilation Linux → Windows

**Entregável:** Executáveis Windows funcionais

### Fase 4: Features Avançadas (Meses 6-8)

- [ ] Classes e OOP (vtables)
- [ ] Garbage Collector
- [ ] Exceções nativas (unwind tables)
- [ ] Herança e polimorfismo

**Entregável:** Suporte completo a OOP

### Fase 5: Otimizações (Meses 9-10)

- [ ] SSA (Static Single Assignment)
- [ ] Constant folding/propagation
- [ ] Dead code elimination
- [ ] Loop optimizations
- [ ] Function inlining
- [ ] SIMD (SSE, AVX)

**Entregável:** Performance competitiva com C

### Fase 6: Debug e Ferramentas (Meses 11-12)

- [ ] DWARF debug info (Linux)
- [ ] PDB debug info (Windows)
- [ ] Comando `dryad build`
- [ ] Flags de otimização (-O0, -O1, -O2, -O3)
- [ ] Cross-compilation

**Entregável:** v1.0 estável com ferramentas completas

---

## Documentação Detalhada

### Guias Técnicos

- **[Plano de Compilação](compilation-plan.md)** - Arquitetura completa do sistema AOT
- **[Guia ELF](elf-format-guide.md)** - Especificação técnica do formato ELF
- **[Guia PE](pe-format-guide.md)** - Especificação técnica do formato PE/COFF
- **[Roadmap](roadmap.md)** - Timeline detalhado de 12 meses
- **[Status](status.md)** - Status atual da implementação

---

## Métricas de Sucesso

### Performance

- Programas simples: < 2x mais lentos que C
- Programas complexos: < 5x mais lentos que C
- Startup: < 10ms

### Tamanho

- "Hello World": < 100KB (estático), < 20KB (dinâmico)
- Programa médio: < 5MB (estático)

### Compatibilidade

- Linux: glibc 2.17+, kernel 3.2+
- Windows: Windows 7+, Windows Server 2012+

---

## Exemplo de Uso (Futuro)

```bash
# Compilar para Linux
dryad build app.dryad -o app --target linux-x86_64

# Compilar para Windows (cross-compilation)
dryad build app.dryad -o app.exe --target windows-x86_64

# Com otimizações
dryad build app.dryad -o app -O3

# Com debug info
dryad build app.dryad -o app --debug

# Executar
./app
```

---

## Benefícios

### Para Desenvolvedores

- ✅ Performance máxima
- ✅ Deploy simplificado
- ✅ Sem dependências de runtime
- ✅ Debugging nativo (GDB, LLDB)

### Para Usuários Finais

- ✅ Executáveis rápidos
- ✅ Startup instantâneo
- ✅ Sem instalação de VM
- ✅ Tamanho reduzido

---

## Desafios

1. **Complexidade de Implementação** - Geração de código nativo é complexa
2. **Multiplataforma** - Diferentes formatos e convenções
3. **Garbage Collection** - GC nativo é desafiador
4. **Tamanho dos Executáveis** - Precisa de tree shaking eficiente
5. **Debug Info** - DWARF/PDB são formatos complexos

---

## Recursos Necessários

### Pessoal

- 1-2 desenvolvedores experientes em Rust/C
- 1 desenvolvedor com experiência em LLVM/Compilers
- 1 desenvolvedor com experiência em Windows/PE

### Ferramentas

- LLVM (opcional, para backend)
- NASM ou GAS (assembler)
- GCC/Clang (linker)
- Windows SDK (para testes)
- VMs Windows/Linux

### Tempo

- **12 meses** para versão completa
- **6 meses** para versão usável
- **3 meses** para protótipo

---

## Próximos Passos

1. **Finalizar planejamento** - Revisar e aprovar documentação
2. **Criar crate `dryad_aot`** - Estrutura inicial
3. **Implementar IR** - Representação intermediária
4. **Protótipo x86_64** - Backend básico
5. **Runtime C** - Biblioteca mínima

---

**Timeline Estimado:** 12 meses para v1.0 completo

Para detalhes técnicos completos, consulte os documentos específicos listados acima.
