# Develop — Documentação Interna de Desenvolvimento

Documentação interna para desenvolvedores do projeto Dryad. **Não faz parte da documentação pública da linguagem.**

---

## Estrutura

### [manuals/](manuals/) — Manuais Técnicos
Documentação detalhada da implementação do interpretador:

| Documento | Descrição |
|-----------|-----------|
| [Arquitetura](manuals/architecture_overview.md) | Visão geral da arquitetura |
| [Internos](manuals/internals.md) | Detalhes internos da implementação |
| [Lexer](manuals/lexer_manual.md) | Análise léxica |
| [Parser](manuals/parser_manual.md) | Análise sintática |
| [Runtime](manuals/runtime_manual.md) | Sistema de execução |
| [Erros](manuals/errors_manual.md) | Sistema de erros |
| [Otimizador](manuals/optimizer.md) | Otimizações |
| [Validador](manuals/validator_logic.md) | Lógica de validação |
| [Pacotes](manuals/package_management.md) | Gerenciamento de pacotes |
| [Desenvolvedor](manuals/DEVELOPER_MANUAL.md) | Manual completo do desenvolvedor |
| [Catálogo de Erros](manuals/ERROR_CATALOG.md) | Catálogo detalhado de erros |
| [Módulos Nativos](manuals/NATIVE_MODULES.md) | Especificações de módulos nativos |
| [Funções Nativas](manuals/NATIVE_FUNCTIONS_SPECS.md) | Specs de funções nativas |
| [Guia de Erros](manuals/DRYAD_ERROR_GUIDE.md) | Guia detalhado de erros |
| [Runtime Técnico](manuals/RUNTIME_TECHNICAL_MANUAL.md) | Manual técnico do runtime |
| [Scripts de Build](manuals/BUILD_SCRIPTS_README.md) | Documentação de build |

### [manuals/bytecode/](manuals/bytecode/) — Sistema Bytecode
| Documento | Descrição |
|-----------|-----------|
| [Implementação](manuals/bytecode/implementation.md) | Detalhes técnicos da VM |
| [Integração](manuals/bytecode/integration.md) | Como usar o bytecode |
| [Funções](manuals/bytecode/functions.md) | Sistema de funções no bytecode |
| [Portabilidade](manuals/bytecode/portability.md) | Garantias de portabilidade |
| [JIT](manuals/bytecode/jit.md) | Plano de JIT compilation |
| [Session 5-6 Completion](manuals/bytecode/SESSION_COMPLETION_NOTES.md) | Notas da sessão (Phase 5-6) |

### [manuals/aot/](manuals/aot/) — Compilação AOT
| Documento | Descrição |
|-----------|-----------|
| [Plano de Compilação](manuals/aot/compilation-plan.md) | Arquitetura completa |
| [Guia ELF](manuals/aot/elf-format.md) | Formato ELF (Linux) |
| [Guia PE](manuals/aot/pe-format.md) | Formato PE (Windows) |

### [implementation/](implementation/) — Status do Projeto
| Documento | Descrição |
|-----------|-----------|
| [Concluído](implementation/done.md) | Tarefas concluídas |
| [Pendente](implementation/todo.md) | Tarefas pendentes |
| [Roadmap](implementation/roadmap.md) | Planejamento estratégico |

### [plans/](plans/) — Planos Futuros
- [Controles UI Ipê](plans/2026-03-09-ipe-ui-controls-system.md)
- [Melhorias de Linguagem](plans/2026-03-16-dryad-language-improvements.md)
- [Bytecode: Fix Parameter Scope Bug](plans/2026-03-21-parameter-scope-fix.md)
- [Bytecode: Integration Tests Completion](plans/2026-03-21-integration-test-completion.md)
