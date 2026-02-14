# Database

Módulo para conexão e operações com bancos de dados SQLite e PostgreSQL.

## Ativação

```dryad
#<database>
```

## SQLite

### sqlite_open(path: string) -> connection

Abre ou cria um banco de dados SQLite.

```dryad
#<database>
let db = sqlite_open("myapp.db");
```

### sqlite_close(connection: object) -> boolean

Fecha a conexão com o banco de dados.

### sqlite_execute(connection: object, sql: string) -> result

Executa uma instrução SQL (INSERT, UPDATE, DELETE).

```dryad
#<database>
let db = sqlite_open("myapp.db");
let result = sqlite_execute(db, "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT)");
sqlite_close(db);
```

### sqlite_query(connection: object, sql: string) -> array

Executa uma consulta SELECT e retorna os resultados.

```dryad
#<database>
let db = sqlite_open("myapp.db");
let results = sqlite_query(db, "SELECT * FROM users WHERE age > 18");
sqlite_close(db);
```

### sqlite_prepare(connection: object, sql: string) -> statement

Prepara uma instrução SQL para execução.

```dryad
#<database>
let stmt = sqlite_prepare(db, "INSERT INTO users (name) VALUES (?)");
```

### sqlite_bind(statement: object, index: number, value: value) -> boolean

Associa um valor a um parâmetro na instrução preparada.

### sqlite_step(statement: object) -> boolean

Executa a próxima etapa da instrução.

### sqlite_columns(statement: object) -> array

Retorna os nomes das colunas do resultado.

## PostgreSQL

### pg_connect(connection_string: string) -> connection

Conecta a um banco de dados PostgreSQL.

```dryad
#<database>
let conn = pg_connect("host=localhost port=5432 dbname=mydb user=myuser password=mypass");
```

### pg_close(connection: object) -> boolean

Fecha a conexão PostgreSQL.

### pg_execute(connection: object, query: string, params: array) -> result

Executa uma instrução SQL com parâmetros.

```dryad
#<database>
let conn = pg_connect("host=localhost dbname=mydb");
let result = pg_execute(conn, "INSERT INTO users (name) VALUES ($1)", ["John"]);
pg_close(conn);
```

### pg_query(connection: object, sql: string) -> array

Executa uma consulta SELECT.

```dryad
#<database>
let results = pg_query(conn, "SELECT * FROM users");
```

### pg_prepare(connection: object, name: string, sql: string) -> statement

Prepara uma instrução SQL nomeada.

### pg_bind(connection: object, statement_name: string, params: array) -> boolean

Associa parâmetros a uma instrução preparada.

### pg_query_params(connection: object, query: string, params: array) -> array

Executa uma consulta com parâmetros.

```dryad
#<database>
let results = pg_query_params(conn, "SELECT * FROM users WHERE age > $1", [18]);
```

## Exemplo Completo

```dryad
#<database>

// SQLite
let db = sqlite_open("myapp.db");
sqlite_execute(db, "CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, name TEXT)");
sqlite_execute(db, "INSERT INTO users (name) VALUES ('Alice')");
let users = sqlite_query(db, "SELECT * FROM users");
sqlite_close(db);

// PostgreSQL
let conn = pg_connect("host=localhost dbname=mydb");
pg_execute(conn, "INSERT INTO users (name) VALUES ($1)", ["Bob"]);
let users = pg_query(conn, "SELECT * FROM users");
pg_close(conn);
```
