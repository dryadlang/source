# Roadmap Técnico: Compilação AOT para Binários Nativos

## Fase 1: Fundações (Meses 1-2)

### Semana 1-2: Arquitetura e IR
- [ ] Criar crate `dryad_aot`
- [ ] Definir IR (Intermediate Representation)
  - Registradores virtuais
  - Instruções de baixo nível
  - Representação de tipos
- [ ] Implementar conversor Bytecode → IR
  - Mapear cada opcode para IR
  - Otimizações básicas (constant folding)

**Entregável:** Sistema IR funcional que converte bytecode simples

### Semana 3-4: Backend x86_64
- [ ] Implementar gerador de assembly x86_64
  - Seleção de instruções
  - Alocação de registradores (básica)
  - Emissão de assembly
- [ ] Convenção de chamada System V (Linux)
- [ ] Convenção de chamada Windows x64
- [ ] Suporte a syscalls Linux

**Entregável:** Geração de código assembly para programas simples

### Semana 5-6: Runtime Mínimo
- [ ] Implementar runtime em C
  - Alocação de memória (malloc/free)
  - I/O básico (printf, read)
  - Gerenciamento de strings
  - Arrays dinâmicos
- [ ] Compilar runtime como biblioteca estática
- [ ] Testar runtime independentemente

**Entregável:** Runtime funcional que pode ser linkado

### Semana 7-8: Integração
- [ ] Integrar gerador de código + runtime
- [ ] Pipeline completo: .dryad → .s → .o → binário
- [ ] Testar com programas simples
- [ ] Documentar processo

**Entregável:** Compilador AOT funcional para programas básicos

## Fase 2: Linux ELF (Meses 2-3)

### Semana 9-10: Gerador ELF
- [ ] Implementar estrutura ELF64
- [ ] Gerar ELF headers (EHDR, PHDR)
- [ ] Criar seções (.text, .rodata, .data, .bss)
- [ ] Calcular offsets e endereços virtuais
- [ ] Alinhamento e padding

**Entregável:** Gerador ELF que cria executáveis válidos

### Semana 11-12: Linkagem
- [ ] Integrar com linker (ld)
- [ ] Linkagem estática com runtime
- [ ] Linkagem dinâmica com libc (opcional)
- [ ] Resolver símbolos
- [ ] Tratar relocações

**Entregável:** Executáveis ELF funcionais no Linux

### Semana 13-14: Otimizações Básicas
- [ ] Register allocation (graph coloring simples)
- [ ] Eliminação de código morto
- [ ] Inlining de funções pequenas
- [ ] Constant propagation

**Entregável:** Código gerado mais eficiente

### Semana 15-16: Features da Linguagem
- [ ] Suporte a funções (calls, returns)
- [ ] Suporte a variáveis locais (stack frame)
- [ ] Suporte a arrays (alocação dinâmica)
- [ ] Suporte a strings
- [ ] If/else, loops

**Entregável:** Suporte a maioria das construções da linguagem

## Fase 3: Windows PE (Meses 4-5)

### Semana 17-18: Gerador PE
- [ ] Implementar estrutura PE/COFF
- [ ] DOS Header e PE Signature
- [ ] COFF Header e Optional Header
- [ ] Section Table
- [ ] Criar seções (.text, .rdata, .data)

**Entregável:** Gerador PE que cria executáveis válidos

### Semana 19-20: Imports e APIs Windows
- [ ] Implementar Import Table
- [ ] Importar kernel32.dll (ExitProcess)
- [ ] Importar msvcrt.dll (printf, scanf)
- [ ] Gerar IAT e ILT
- [ ] Testar imports

**Entregável:** Executáveis Windows que usam APIs do sistema

### Semana 21-22: Adaptações Windows
- [ ] Adaptar runtime para Windows
- [ ] Convenção de chamada Windows x64
- [ ] Shadow space
- [ ] Tratamento de erros Windows
- [ ] Console vs GUI subsystem

**Entregável:** Runtime Windows funcional

### Semana 23-24: Integração e Testes
- [ ] Pipeline completo Windows
- [ ] Cross-compilation (Linux → Windows)
- [ ] Testar em Windows 10/11
- [ ] Resolver bugs de compatibilidade

**Entregável:** Executáveis Windows estáveis

## Fase 4: Features Avançadas (Meses 6-8)

### Semana 25-28: Classes e Objetos
- [ ] Implementar vtable
- [ ] Suporte a métodos virtuais
- [ ] This pointer
- [ ] Construtores/destrutores
- [ ] Herança (single inheritance)
- [ ] Polimorfismo

**Entregável:** Suporte completo a OOP

### Semana 29-32: Garbage Collection
- [ ] Implementar GC simples (mark-and-sweep)
- [ ] Alocador de memória eficiente
- [ ] Root finding
- [ ] Finalizers
- [ ] Otimizações (generational GC opcional)

**Entregável:** Gerenciamento automático de memória

### Semana 33-36: Exceções
- [ ] Implementar unwind tables
- [ ] Try/catch/finally nativo
- [ ] Stack unwinding
- [ ] Destructors em exceções (RAII)
- [ ] Compatibilidade com sistema de exceções nativo

**Entregável:** Tratamento de exceções completo

## Fase 5: Otimizações (Meses 9-10)

### Semana 37-40: Otimizações de Código
- [ ] SSA (Static Single Assignment)
- [ ] Constant folding/propagation
- [ ] Dead code elimination
- [ ] Common subexpression elimination
- [ ] Loop optimizations (unrolling, invariant code motion)
- [ ] Function inlining agressivo

**Entregável:** Código altamente otimizado

### Semana 41-44: Otimizações de Arquitetura
- [ ] SIMD (SSE, AVX) para arrays
- [ ] Branch prediction hints
- [ ] Cache-friendly data structures
- [ ] Tail call optimization
- [ ] Profile-guided optimization (PGO)

**Entregável:** Performance máxima

## Fase 6: Ferramentas e Debug (Meses 11-12)

### Semana 45-48: Debug Info
- [ ] Gerar DWARF debug info (Linux)
- [ ] Gerar PDB debug info (Windows)
- [ ] Mapeamento bytecode → código nativo
- [ ] Stack traces simbólicos
- [ ] Suporte a GDB/LLDB

**Entregável:** Debugging completo

### Semana 49-52: Ferramentas
- [ ] Comando `dryad build`
- [ ] Flags de otimização (-O0, -O1, -O2, -O3, -Os)
- [ ] Cross-compilation
- [ ] Stripping de símbolos
- [ ] Análise de tamanho (bloaty-like)

**Entregável:** CLI completa para compilação AOT

## Checkpoints de Milestone

### Milestone 1 (Mês 2)
- ✅ Programa "Hello World" compilado para Linux x86_64
- ✅ Runtime básico funcional
- ✅ Pipeline completo funcionando

### Milestone 2 (Mês 4)
- ✅ Executáveis ELF completos com todas as features
- ✅ Otimizações básicas implementadas
- ✅ Performance 5-10x melhor que bytecode

### Milestone 3 (Mês 6)
- ✅ Executáveis Windows PE funcionais
- ✅ Cross-compilation Linux → Windows
- ✅ Suporte a OOP completo

### Milestone 4 (Mês 8)
- ✅ Garbage Collector funcional
- ✅ Exceções nativas
- ✅ Features avançadas da linguagem

### Milestone 5 (Mês 10)
- ✅ Otimizações avançadas
- ✅ Performance competitiva com C/C++
- ✅ Benchmarks positivos

### Milestone 6 (Mês 12)
- ✅ Debug info completa
- ✅ CLI madura
- ✅ Documentação completa
- ✅ Primeira versão estável (v1.0)

## Recursos Necessários

### Pessoal
- 1-2 desenvolvedores experientes em Rust/C
- 1 desenvolvedor com experiência em LLVM/Compilers
- 1 desenvolvedor com experiência em Windows/PE

### Ferramentas
- LLVM (biblioteca)
- NASM ou GAS (assembler)
- GCC/Clang (linker)
- Windows SDK (para testes)
- VMs Windows/Linux

### Tempo
- 12 meses para versão completa
- 6 meses para versão usável
- 3 meses para protótipo

## Riscos e Mitigações

### Risco 1: Complexidade do LLVM
**Mitigação:** Começar com backend próprio simples, migrar para LLVM depois

### Risco 2: Compatibilidade Windows
**Mitigação:** Testes contínuos em VMs Windows, CI multi-plataforma

### Risco 3: Performance Insuficiente
**Mitigação:** Benchmarks frequentes, perfilamento, otimizações incrementais

### Risco 4: Tamanho dos Executáveis
**Mitigação:** Tree shaking, linkagem dinâmica, runtime minimalista

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

## Conclusão

Este roadmap de 12 meses resultará em um compilador AOT completo capaz de gerar executáveis nativos de alta performance para Linux e Windows a partir de código Dryad.

A fase inicial foca em estabelecer as fundações (IR, backend x86_64, runtime), seguida pela implementação dos formatos ELF e PE, features avançadas da linguagem, otimizações e finalmente ferramentas de debugging.

**Estimativa total: 12 meses para versão 1.0**
