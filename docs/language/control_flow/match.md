# Pattern Matching (match)

O Dryad oferece um sistema de pattern matching poderoso e expressivo através da palavra-chave `match`. Ele permite comparar um valor contra uma série de padrões e executar o código correspondente ao primeiro padrão que coincidir.

## Sintaxe Básica

```dryad
match (expressao) {
    padrao1 => expressao_ou_bloco,
    padrao2 if guarda => expressao_ou_bloco,
    _ => padrao_default
}
```

## Tipos de Padrões

### 1. Literais

Compara o valor diretamente com um literal (número, string, booleano ou null).

```dryad
match (status) {
    200 => "OK",
    404 => "Não Encontrado",
    500 => "Erro Interno",
    _ => "Desconhecido"
}
```

### 2. Identificadores (Bindings)

Um identificador num padrão captura o valor e o torna disponível dentro do escopo do braço do match.

```dryad
match (get_user()) {
    "admin" => print("Acesso total"),
    user => print("Bem-vindo, " + user)
}
```

### 3. Wildcard (`_`)

O caractere sublinhado corresponde a qualquer valor mas não o captura. É útil como um caso "catch-all" ao final de um match.

### 4. Desestruturação de Arrays

Permite extrair elementos de listas.

```dryad
match ([1, 2, 3]) {
    [1, x, 3] => print("O meio é " + x),
    [first, ..] => print("Começa com " + first),
    _ => print("Outra lista")
}
```

### 5. Desestruturação de Objetos

Permite extrair valores de chaves específicas de um objeto.

```dryad
match (pessoa) {
    { nome: "Pedro", idade: i } => print("Pedro tem " + i + " anos"),
    { nome: n } => print("Nome: " + n),
    _ => print("Pessoa desconhecida")
}
```

### 6. Desestruturação de Tuplas

Semelhante a arrays, mas para tipos `Tuple`.

```dryad
match (coordenadas) {
    (0, 0) => "Origem",
    (x, y) => "Ponto em " + x + ", " + y
}
```

## Guardas (if guards)

Você pode adicionar uma condição extra a um padrão usando a palavra-chave `if`. O padrão só coincidirá se a condição do guarda também for verdadeira.

```dryad
match (numero) {
    n if n > 0 => "Positivo",
    n if n < 0 => "Negativo",
    _ => "Zero"
}
```

## Comportamento

1. As expressões são avaliadas de cima para baixo.
2. O primeiro padrão que coincidir (e cujo guarda, se houver, for verdadeiro) é executado.
3. Se o braço for um bloco `{ ... }`, o valor retornado é o resultado da última expressão ou o valor do `return` explícito.
4. Se nenhum padrão coincidir, um runtime error é lançado. Por isso, recomenda-se sempre usar um wildcard `_` ao final se não houver cobertura total.
