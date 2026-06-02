---
title: "Banco de Dados"
description: "Conexão e operações com SQLite e PostgreSQL."
category: "Bibliotecas Padrão"
subcategory: "Database"
order: 35
---

# Database (SQL Connectors)

Este módulo fornece uma interface unificada para interação com bancos de dados relacionais.

> [!IMPORTANT]
> **Status Atual: Totalmente Funcional (SQLite & PostgreSQL)**.
> Ambos os motores de banco de dados estão implementados. PostgreSQL utiliza `tokio-postgres` para conexões reais.

## 🚀 Leitura Rápida

- **SQLite**: Banco local baseado em arquivo. Ativado via `#<database>`.
- **PostgreSQL**: Suporte completo para produção via `pg_connect`.
- **Interface**: Baseada em handles de conexão.
- **Segurança**: Suporte a parâmetros para evitar SQL Injection.

---

## ⚙️ Visão Técnica

O módulo `database` utiliza o padrão de **Handles Opacos**. O script não acessa a estrutura do driver Rust diretamente, mas recebe um identificador que o runtime usa para localizar a conexão em seu registro interno.

### 1. Fluxo de Trabalho Recomendado

1.  **Open/Connect**: Abrir conexão (`sqlite_open` ou `pg_connect`).
2.  **Exec/Query**: Executar (`sqlite_execute` / `pg_execute`) para comandos sem retorno ou consultar (`sqlite_query` / `pg_query`) para obter dados.
3.  **Close**: Fechar explicitamente (`sqlite_close` / `pg_close`) para evitar vazamento de memória.

### 2. Funções Disponíveis (Referência)

| Categoria      | Funções                                                         |
| :------------- | :-------------------------------------------------------------- |
| **SQLite**     | `sqlite_open`, `sqlite_execute`, `sqlite_query`, `sqlite_close` |
| **PostgreSQL** | `pg_connect`, `pg_execute`, `pg_query`, `pg_close`              |

---

## 📚 Referências e Paralelos

- **Rust Drivers**: Suporte via [rusqlite](https://docs.rs/rusqlite/).
- **API Design**: Focada em simplicidade e performance para scripts rápidos.
- **Standards**: [SQL-92 Standard](https://en.wikipedia.org/wiki/SQL-92).

---

## Exemplo de Uso

```dryad
#<database>

// 1. Abrir (ou criar) o banco de dados
let db = sqlite_open("app.db");

// 2. Criar uma tabela
sqlite_execute(db.id, "CREATE TABLE IF NOT EXISTS logs (msg TEXT)");

// 3. Inserir dados
sqlite_execute(db.id, "INSERT INTO logs (msg) VALUES ('Acesso detectado')");

// 4. Consultar dados
let rows = sqlite_query(db.id, "SELECT * FROM logs");
for (let row in rows) {
    println("Log: " + row.msg);
}

// 5. Fechar conexão
sqlite_close(db.id);
```
