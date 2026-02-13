---
title: "API do Registro"
description: "Documentação da API REST do registro de pacotes Oak."
category: "Guia de Uso"
order: 3
---

# Oak Registry API

O Oak Registry é um serviço simples que mapeia nomes de pacotes para metadados (principalmente URLs Git). Ele permite que desenvolvedores gerenciem dependências de forma eficiente, delegando o armazenamento para repositórios Git (GitHub, GitLab, etc).

## Especificação da API

### `GET /packages/:name/:version?`

Retorna metadados de um pacote.

#### Parâmetros

- `name`: Nome do pacote (ex: `dryad-utils`).
- `version`: (Opcional) Versão específica (ex: `1.0.0`). Se omitido, retorna a `latest`.

#### Resposta (JSON)

```json
{
  "version": "1.0.0",
  "gitUrl": "https://github.com/Dryad-lang/utils.git",
  "tag": "v1.0.0",
  "dependencies": {
    "dryad-stdlib": "^0.1.0"
  }
}
```

- `version`: Versão resolvida.
- `gitUrl`: URL do repositório Git para clonar.
- `tag`: Tag ou branch do Git correspondente à versão.
- `dependencies`: Dependências deste pacote.

### `GET /search?q=:query`

Pesquisa pacotes por nome.

#### Parâmetros

- `q`: Termo de pesquisa.

#### Resposta (JSON)

Array de strings com nomes dos pacotes encontrados.

```json
["dryad-utils", "dryad-stdlib"]
```

## Implementação de Referência

Uma implementação mock do registry está disponível em `dryad-registry-mock/`.

### Exemplo de Uso

```dryad
let response = http_get("https://registry.dryadlang.org/packages/dryad-utils");
let package = json_parse(response);
println("Pacote: " + package.name);
println("Versão: " + package.version);
```
