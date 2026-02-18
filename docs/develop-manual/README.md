# Documentação de Desenvolvimento Dryad

Bem-vindo à documentação técnica completa do projeto Dryad. Esta seção contém toda a informação necessária para entender, contribuir e estender a linguagem.

**Legenda de status:** ✅ concluído · 🚧 em andamento · 📋 planejado · 🟡 em desenvolvimento

---

## 📚 Índice Geral

### Gestão do Projeto

- **[Roadmap](roadmap.md)** - Visão estratégica e planejamento de longo prazo
- **[TODO](todo.md)** - Tarefas pendentes e próximos passos
- **[DONE](done.md)** - Tarefas concluídas e histórico

### Manuais Principais

- **[Visão Geral da Arquitetura](architecture_overview.md)** - Arquitetura geral do projeto
- **[Manual do Lexer](lexer_manual.md)** - Análise léxica
- **[Manual do Parser](parser_manual.md)** - Análise sintática
- **[Manual do Runtime](runtime_manual.md)** - Sistema de execução
- **[Manual de Erros](errors_manual.md)** - Sistema de erros
- **[Gerenciamento de Pacotes](package_management.md)** - Oak Package Manager

### Documentação Técnica

- **[Internals](../internals.md)** - Detalhes internos da implementação
- **[Validator Logic](../validator_logic.md)** - Lógica de validação

---

## 🔧 Sistema Bytecode

O bytecode VM é uma máquina virtual baseada em pilha que oferece performance 2-3x melhor que o interpretador AST.

**Status:** ✅ Completo (~95% da linguagem)

### Documentação

- **[Overview](bytecode/overview.md)** - Introdução ao sistema bytecode
- **[Implementação](bytecode/implementation.md)** - Detalhes técnicos
- **[Integração](bytecode/integration.md)** - Como usar
- **[Funções](bytecode/functions.md)** - Sistema de funções
- **[Portabilidade](bytecode/portability.md)** - Garantias de portabilidade
- **[JIT](bytecode/jit.md)** - Plano de JIT compilation
- **[Status](bytecode/status.md)** - Histórico e status atual

### Recursos

- **Código:** `crates/dryad_bytecode/`
- **Testes:** `crates/dryad_bytecode/tests/`
- **Exemplos:** `test_*.dryad`

---

## 🚀 Compilação AOT

Sistema de compilação Ahead-of-Time para gerar executáveis nativos de alta performance.

**Status:** � Em desenvolvimento (55% — núcleo implementado; consulte `aot/status.md` para detalhes)

### Documentação

- **[Overview](aot/overview.md)** - Introdução ao sistema AOT
- **[Plano de Compilação](aot/compilation-plan.md)** - Arquitetura completa
- **[Guia ELF](aot/elf-format-guide.md)** - Formato ELF (Linux)
- **[Guia PE](aot/pe-format-guide.md)** - Formato PE (Windows)
- **[Roadmap AOT](aot/roadmap.md)** - Timeline de 12 meses
- **[Status](aot/status.md)** - Status da implementação

---

## 🔨 Refactoring

Documentação de refatorações estruturais e guidelines.

### Documentação

- **[Guidelines](refactoring/guidelines.md)** - Diretrizes de refatoração
- **[Structural Refactor](refactoring/structural-refactor.md)** - Refatorações estruturais
- **[Danger Zones](refactoring/danger-zones.md)** - Áreas de risco

---

## 🎯 Como Usar Esta Documentação

### Para Novos Contribuidores

1. Comece com [Visão Geral da Arquitetura](architecture_overview.md)
2. Leia os manuais principais (Lexer, Parser, Runtime)
3. Explore o [Bytecode Overview](bytecode/overview.md)
4. Consulte o [Roadmap](roadmap.md) para visão de longo prazo
5. Verifique o [TODO](todo.md) para tarefas disponíveis

### Para Desenvolvedores Ativos

1. Consulte o [TODO](todo.md) para próximas tarefas
2. Atualize o [DONE](done.md) ao concluir implementações
3. Siga as [Guidelines de Refatoração](refactoring/guidelines.md)
4. Verifique [Danger Zones](refactoring/danger-zones.md) antes de mudanças críticas

---

**Última atualização:** 16 de fevereiro de 2026  
**Versão:** 1.1 (Bytecode completo, AOT em desenvolvimento)
