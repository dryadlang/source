---
title: "Guia de Instalação"
description: "Como baixar e instalar o toolchain da linguagem Dryad em seu sistema."
category: "Introdução"
order: 2
---

# Instalação do Dryad

O Dryad é distribuído como um conjunto de ferramentas (toolchain) que inclui o compilador, o interpretador e o gerenciador de pacotes Oak.

## Métodos de Instalação

### 1. Script de Instalação (Recomendado)

O método mais rápido para instalar o Dryad em sistemas Unix (Linux e macOS) é através do nosso script de shell.

```bash
curl -fsSL https://get.dryadlang.org | sh
```

### 2. Binários Pré-compilados (Windows)

Para usuários Windows, oferecemos um instalador MSI ou um arquivo ZIP contendo os binários necessários.

1.  Aceda à nossa [Página de Downloads](/downloads).
2.  Baixe a versão mais recente para `Windows (x64)`.
3.  Execute o instalador ou extraia o ZIP e adicione a pasta `bin` ao seu `PATH`.

### 3. Usando Docker

Se preferir não instalar nada localmente, pode usar a nossa imagem oficial do Docker.

```bash
docker pull dryadlang/dryad:latest
docker run -it dryadlang/dryad dryad repl
```

## Verificando a Instalação

Após a instalação, abra um novo terminal e verifique se o comando `dryad` está disponível:

```bash
dryad --version
```

Você também deve ter acesso ao gerenciador de pacotes `oak`:

```bash
oak --help
```

## Próximos Passos

Agora que você tem o Dryad instalado, confira o nosso [Guia de Sintaxe](/docs/syntax) para escrever o seu primeiro programa!
