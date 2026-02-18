---
title: "CLI do Oak"
description: "Comandos e uso do gerenciador de pacotes da linha de comando."
category: "Ferramentas"
order: 51
---

# Oak CLI Reference

Oak é o gerenciador de pacotes oficial da linguagem Dryad. Ele gerencia dependências, scripts e a estrutura de projetos. Este documento descreve os principais comandos e funcionalidades do Oak CLI.

## Instalação

O Oak é distribuído junto com o toolchain da Dryad. Certifique-se de que ele está instalado corretamente.

```bash
# Verificar instalação
oak --help
```

## Comandos

### `init`

Inicializa um novo projeto Dryad.

```bash
oak init <nome_do_projeto> [--type=project|library]
```

- **`--type`**: Define o tipo do projeto. Pode ser `project` (aplicação) ou `library` (biblioteca).

### `install`

Instala as dependências do projeto.

```bash
oak install
```

- **Checksum Validation**: O Oak agora verifica automaticamente o hash SHA-256 de cada pacote baixado para garantir a integridade. Se o hash calculado não coincidir com o fornecido pelo registry, a instalação será abortada.
- **Semver Support**: O comando `install` agora suporta ranges de versões semânticas (ex: `^1.0.0`, `~2.1.0`).
- **Resolução de Conflitos**: Se um pacote for encontrado em múltiplos registries, o Oak agora solicita interativamente qual fonte deve ser utilizada (ou permite que você configure uma prioridade).

### `publish`

Publica um pacote no registro do Oak.

```bash
oak publish
```

- **Nota Técnica**: O comando `publish` requer autenticação com o registro configurado.

### `run`

Executa scripts definidos no `oaklibs.json`.

```bash
oak run start
oak run test
```

### `lock`

Regera o arquivo de travamento `oaklock.json` baseado nos pacotes presentes em `oak_modules/`.

```bash
oak lock
```

Útil se você modificar manualmente a pasta de módulos ou se o arquivo de travamento estiver dessincronizado.

### `registry`

Gerencia os registries configurados.

```bash
# Listar registries
oak registry list

# Adicionar um novo registry
oak registry add private http://meu-registry.com/api

# Remover um registry
oak registry remove private
```

## Arquivos de Configuração

### `oaklibs.json`

Arquivo principal de manifesto do projeto. Define metadados e dependências.

```json
{
  "name": "meu-projeto",
  "version": "0.1.0",
  "type": "project",
  "dependencies": {
    "dryad-utils": "1.0.0"
  },
  "scripts": {
    "start": "oak exec main.dryad"
  }
}
```

### `oaklock.json`

Arquivo gerado automaticamente que mapeia os módulos para seus caminhos físicos. **Não deve ser editado manualmente.**

- **Determinismo**: O lockfile agora armazena a versão exata instalada e o hash de integridade de cada módulo, garantindo que instalações futuras em diferentes máquinas sejam idênticas.

## Registry

O Oak suporta múltiplos registries simultaneamente. A configuração é global (armazenada em `~/.oak/config.json`).

Se um pacote for encontrado em múltiplos registries, o Oak solicitará interativamente qual fonte deve ser utilizada.
