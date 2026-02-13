# Danger Zone / Technical Debt

Este documento lista trechos de código que representam riscos de segurança, performance ou estabilidade.

## Runtime

### `crates/dryad_runtime/src/interpreter.rs`
- **Recursão Infinita (Stack Overflow)**: O interpretador é recursivo (AST Walk). Scripts com recursão profunda ou loops muito longos podem estourar a pilha do host (Rust).
    - *Risco*: Crash da aplicação host.
    - *Solução*: Implementar verificação de profundidade ou mudar para interação.

### `crates/dryad_runtime/src/native_modules/system_env.rs`
- **Linha 118 (native_exec)**: Permite execução arbitrária de comandos do sistema (`sh -c`).
    - *Risco*: **RCE (Remote Code Execution)** se o script Dryad vier de fonte não confiável.
    - *Solução*: Adicionar flag de permissão `--allow-exec` no CLI ou sandbox.

### `crates/dryad_runtime/src/native_modules/file_io.rs`
- **Acesso ao Filesystem**: Não há restrição de diretório (chroot/jail). Scripts podem ler/escrever em `/etc/passwd` ou outros arquivos sensíveis se o usuário tiver permissão.
    - *Risco*: Vazamento de dados / Corrupção de sistema.
    - *Solução*: Restringir acesso apenas ao diretório do projeto.

## Oak Package Manager

### `crates/oak/src/main.rs`
- **Linha 769 (install_simulated_package)**: Instala pacotes sem verificação de integridade (hash/assinatura) no modo fallback.
    - *Risco*: Ataque de Supply Chain (Man-in-the-Middle).
    - *Solução*: Exigir HTTPS e checksum sempre.
- **Linha 252 (env var)**: Lê `OAK_REGISTRY_URL` sem validação.
    - *Risco*: Redirecionamento para registry malicioso.

## Concorrência

### `crates/dryad_runtime/src/interpreter.rs`
- **Threads e Rc<RefCell>**: O uso de `Rc` não é *thread-safe* (`Send`/`Sync`). Se o interpretador compartilhar objetos entre threads nativas sem lock global (GIL) ou clonagem profunda, haverá *Data Race* e *Undefined Behavior*.
    - *Risco*: **Crítico**. Corrupção de memória.
    - *Solução*: Usar `Arc<Mutex<T>>` para estado compartilhado ou isolar memória por thread (Actor Model).

## Lexer

### `crates/dryad_lexer/src/lexer.rs`
- **Unsafe Indexing**: Acesso direto ao slice de bytes da string sem verificação de fronteira de caracteres UTF-8 em alguns loops manuais.
    - *Risco*: Panic em runtime com strings UTF-8 malformadas.
