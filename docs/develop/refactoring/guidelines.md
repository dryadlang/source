---
title: "Guidelines de Refatoração"
description: "Diretrizes e padrões para refatoração segura do código Dryad."
category: "Desenvolvimento"
order: 97
subcategory: "Refactoring"
---

# Guidelines de Refatoração

## Princípios Gerais

1. **Nunca quebre o build**: Sempre mantenha o código compilando
2. **Testes primeiro**: Garanta cobertura de testes antes de refatorar
3. **Commits pequenos**: Faça mudanças incrementais e bem documentadas
4. **Code review**: Todas as refatorações devem passar por review

## Processo de Refatoração

### 1. Análise
- Identifique o problema ou código a ser melhorado
- Documente o comportamento atual esperado
- Liste dependências e impactos potenciais

### 2. Preparação
- Crie testes que verifiquem o comportamento atual
- Faça backup ou use branches Git
- Documente em `structural-refactor.md` se for uma mudança grande

### 3. Execução
- Faça mudanças pequenas e testáveis
- Execute a suite de testes frequentemente
- Atualize documentação conforme necessário

### 4. Validação
- Todos os testes devem passar
- Verifique performance (benchmarks se aplicável)
- Code review obrigatório

## Padrões de Código

### Rust
- Use `cargo fmt` e `cargo clippy` antes de commitar
- Documente funções públicas com docstrings
- Prefira composição over herança
- Use tipos do sistema de tipos ao invés de `String` genérico

### Dryad (Linguagem)
- Módulos devem ter responsabilidade única
- Funções devem ser pequenas e focadas
- Evite código duplicado (DRY)
- Documente APIs públicas

## Checklist de Refatoração

- [ ] Código compila sem warnings
- [ ] Todos os testes passam
- [ ] Documentação atualizada
- [ ] CHANGELOG.md atualizado (se necessário)
- [ ] Code review aprovado
- [ ] Performance não degradou (ou melhorou)

## O que Evitar

- **Refatorações gigantes**: Divida em passos menores
- **Mudanças sem testes**: Sempre tenha testes de segurança
- **Refatorar e adicionar features**: Separe as preocupações
- **Ignorar warnings do compilador**: Trate todos os warnings

## Recursos

- [Clean Code - Robert C. Martin](https://www.amazon.com/Clean-Code-Handbook-Software-Craftsmanship/dp/0132350882)
- [Refactoring - Martin Fowler](https://refactoring.com/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
