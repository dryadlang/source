# Manual Técnico: Sistema de Erros

**Localização**: `crates/dryad_errors/`
**Responsável**: Centralizar a definição, formatação e reporte de erros para todas as fases.

## 1. Arquitetura Orientada a DX

A filosofia do sistema de erros é focada na **Experiência do Desenvolvedor (DX)**. Um erro deve responder:
1.  **Onde?** (Arquivo, Linha, Coluna, Snippet visual).
2.  **O quê?** (Código, Mensagem técnica).
3.  **Por quê?** (Contexto, Variáveis, Stack Trace).
4.  **Como corrigir?** (Sugestões, Links).

## 2. A Estrutura `DryadError`

O enum `DryadError` encapsula todos os tipos possíveis de falha.

```rust
pub enum DryadError {
    Lexer { 
        code: u16, 
        message: String, 
        location: SourceLocation 
    },
    Parser { 
        expected: Vec<String>, 
        found: String, 
        location: SourceLocation 
    },
    Runtime { 
        code: u16, 
        message: String, 
        stack_trace: StackTrace 
    },
    // ... Type, Io, Module, System
}
```

Cada variante carrega dados específicos úteis para aquela fase. Ex: `Parser` carrega "tokens esperados" para sugerir correções.

## 3. Riqueza de Contexto

### `SourceLocation`
Armazena não apenas coordenadas, mas opcionalmente a *linha de código fonte original* (`source_line`). Isso permite que o formatador de erro imprima o trecho de código sem precisar reler o arquivo do disco (IO-free error reporting).

### `DebugContext`
Metadata auxiliar anexável a qualquer erro:
```rust
pub struct DebugContext {
    pub suggestions: Vec<String>, // Dicas ("Você quis dizer 'length'?")
    pub help_url: Option<String>, // Link para docs
}
```

### `StackTrace`
Para erros de runtime, mantém um vetor de `StackFrame`, permitindo impressão similar a Python/Java:
```
Erro 3001: Variável não definida 'x'
  at calcular_soma (main.dryad:10)
  at main (main.dryad:22)
```

## 4. Sistema de Ajuda Automática (`error_urls.rs`)

O módulo `error_urls` atua como uma base de conhecimento embutida. Ele mapeia códigos numéricos para URLs e dicas.
- **Automação**: O método `with_auto_context()` consulta essa base. Se um erro `3001` é gerado, ele injeta automaticamente a URL `https://dryadlang.org/errors#e3001`.

## 5. Faixas de Códigos Reservadas

Dividimos o espaço de erros em faixas lógicas para facilitar a identificação rápida:

| Faixa | Categoria | Descrição |
| :--- | :--- | :--- |
| **1000-1999** | **Léxico** | Falha na tokenização (chars inválidos). |
| **2000-2999** | **Sintático** | Estrutura gramatical inválida. |
| **3000-3999** | **Runtime** | Erros de execução lógica (divisão por zero, null pointer). |
| **4000-4999** | **Tipagem** | (Reservado para futuro Type Checker). |
| **5000-5999** | **I/O** | Falhas de sistema de arquivos ou rede. |
| **6000-6999** | **Módulos** | Falhas de import/export/DL. |
| **8000-8999** | **Warnings** | Avisos não fatais (variáveis não usadas). |
| **9000+** | **Sistema** | Pânicos internos do compilador (bugs). |
