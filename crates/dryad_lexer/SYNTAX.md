# � Sintaxe Completa da Linguagem Dryad

**Target:** Usuários e Desenvolvedores  
**Versão:** 0.1.1  
**Data:** Janeiro 2025  
**Status:** Documentação Completa da Sintaxe

---

## 📋 Índice

1. [Introdução](#introdução)
2. [Tipos de Dados](#tipos-de-dados)
3. [Variáveis e Declarações](#variáveis-e-declarações)
4. [Operadores](#operadores)
5. [Estruturas de Controle](#estruturas-de-controle)
6. [Funções](#funções)
7. [Classes e Objetos](#classes-e-objetos)
8. [Módulos e Imports](#módulos-e-imports)
9. [Funções Nativas](#funções-nativas)
10. [Comentários](#comentários)
11. [Palavras Reservadas](#palavras-reservadas)
12. [Exemplos Práticos](#exemplos-práticos)
13. [Funcionalidades Futuras](#funcionalidades-futuras)

---

## 🚀 Introdução

Dryad é uma linguagem de programação moderna e expressiva, projetada para ser simples de aprender mas poderosa o suficiente para projetos complexos. Esta documentação cobre toda a sintaxe disponível na versão atual.

### Características Principais
- **Tipagem dinâmica**: Tipos são inferidos automaticamente
- **Orientação a objetos**: Suporte completo a classes e herança
- **Modular**: Sistema robusto de módulos e imports
- **Interativa**: REPL integrado para desenvolvimento rápido
- **Extensível**: Funções nativas e integração com outras linguagens

---

## 🏷️ Tipos de Dados

### Tipos Primitivos

#### Number (Número)
Todos os números em Dryad são de ponto flutuante (64-bit).

```dryad
let idade = 25;
let altura = 1.75;
let pi = 3.14159;
let negativo = -42;
```

#### String (Cadeia de caracteres)
Strings são delimitadas por aspas duplas.

```dryad
let nome = "João";
let sobrenome = "Silva";
let frase = "Olá, mundo!";
let vazio = "";
```

#### Boolean (Booleano)
Valores verdadeiro ou falso.

```dryad
let verdadeiro = true;
let falso = false;
let maior = 10 > 5;  // true
let menor = 3 > 8;   // false
```

#### Null (Nulo)
Representa ausência de valor.

```dryad
let indefinido = null;
let nada = null;
```

### Tipos Compostos

#### Arrays
```dryad
// Planejado para versões futuras
let numeros = [1, 2, 3, 4, 5];
let nomes = ["Ana", "Bruno", "Carlos"];
```

#### Objects
```dryad
// Planejado para versões futuras
let pessoa = {
    nome: "Maria",
    idade: 30,
    ativo: true
};
```

---

## 📝 Variáveis e Declarações

### Declaração de Variáveis

Use a palavra-chave `let` para declarar variáveis:

```dryad
let nome = "Dryad";
let versao = 0.1;
let ativo = true;
```

### Reatribuição de Variáveis

```dryad
let contador = 0;
contador = contador + 1;
contador = 10;
```

### Inicialização

Variáveis podem ser declaradas sem valor inicial:

```dryad
let x;  // x é null
x = 42; // agora x é 42
```

---

## ⚡ Operadores

### Operadores Aritméticos

```dryad
let a = 10;
let b = 3;

let soma = a + b;          // 13
let subtracao = a - b;     // 7
let multiplicacao = a * b; // 30
let divisao = a / b;       // 3.333...
```

### Operadores de Comparação

```dryad
let x = 5;
let y = 10;

let igual = x == y;        // false
let diferente = x != y;    // true
let menor = x < y;         // true
let maior = x > y;         // false
let menorIgual = x <= y;   // true
let maiorIgual = x >= y;   // false
```

### Operadores Lógicos

```dryad
let a = true;
let b = false;

let e = a && b;            // false (AND)
let ou = a || b;           // true (OR)
let nao = !a;              // false (NOT)
```

### Operadores de Atribuição

```dryad
let x = 5;
x += 2;  // x = x + 2 (agora x é 7)
x -= 3;  // x = x - 3 (agora x é 4)
x *= 2;  // x = x * 2 (agora x é 8)
x /= 4;  // x = x / 4 (agora x é 2)
```

### Operadores de Incremento/Decremento

```dryad
let contador = 0;
contador++;  // Incrementa 1 (agora contador é 1)
contador--;  // Decrementa 1 (agora contador é 0)
```

### Operadores de calculo avançado
```dryad
let modulo = 10 % 3; // Resto da divisão (1)
let exponenciacao = 2 ** 3; // 2 elevado a 3 (8)
let raizEnésima = 27 ^^ (1/3); // Raiz cúbica de 27 (3)
let moduloSeguro = 10 %% 3; // Sempre positivo (1)
let potenciaBase10 = 10 ## 3; // 1000
```

### Operadores de byte
```dryad
let byte1 = 0b1010; // 10 em binário
let byte2 = 0o12;   // 10 em octal
let byte3 = 0xA;    // 10 em hexadecimal
```

### Operadores byteshift e bitwise
```dryad
let deslocamentoEsquerda = 1 << 2; // 4 (1 * 2^2)
let deslocamentoDireita = 4 >> 2; // 1 (4 / 2^2)
let bitwiseAnd = 0b1100 & 0b1010; // 0b1000 (8)
let bitwiseOr = 0b1100 | 0b1010;  // 0b1110 (14)
let bitwiseXor = 0b1100 !^ 0b1010; // 0b0110 (6)
let deslocamentoSimétricoDireita = 0b1010 >>> 1; // 0b0101 (5)
let deslocamentoSimétricoEsquerda = 0b0101 <<< 1; // 0b1010 (10)
```

### Operadores de Concatenção de Strings

```dryad
let saudacao = "Olá, " + "Dryad!"; // "Olá, Dryad!"
let nomeCompleto = "João" + " " + "Silva"; // "João Silva"
```

### Precedência de Operadores

De maior para menor precedência:

1. `!` (NOT)
2. `*`, `/` (Multiplicação, Divisão)
3. `+`, `-` (Adição, Subtração)
4. `<`, `>`, `<=`, `>=` (Comparação)
5. `==`, `!=` (Igualdade)
6. `&&` (AND)
7. `||` (OR)

```dryad
// Exemplos de precedência
let resultado1 = 2 + 3 * 4;     // 14 (não 20)
let resultado2 = (2 + 3) * 4;   // 20
let resultado3 = !false && true; // true
```

---

## 🔀 Estruturas de Controle

### Condicionais (if/else)

#### If simples
```dryad
let idade = 18;
if idade >= 18 {
    print("Maior de idade");
}
```

#### If-else
```dryad
let nota = 7.5;
if nota >= 7.0 {
    print("Aprovado");
} else {
    print("Reprovado");
}
```

#### If-else encadeado
```dryad
let pontuacao = 85;
if pontuacao >= 90 {
    print("Excelente");
} else if pontuacao >= 80 {
    print("Bom");
} else if pontuacao >= 70 {
    print("Regular");
} else {
    print("Insuficiente");
}
```

### Loops

#### While
```dryad
let i = 0;
while i < 5 {
    print(i);
    i = i + 1;
}
```

#### For
```dryad
for i = 0; i < 5; i = i + 1 {
    print(i);
}
```

#### Do-While (Planejado)
```dryad
// Planejado para versões futuras
let i = 0;
do {
    print(i);
    i = i + 1;
} while i < 5;
```

#### Arrays, Matrizes e Tuplos
```dryad
let numeros = [1, 2, 3, 4, 5];
let matriz = [[1, 2], [3, 4]];
let vazio = []; // Array vazio
let tupla = (1, "dois", 3.0);
let tuplaVazia = (); // Tupla vazia
let valortupla = tupla.1; // Acessa o segundo elemento da tupla
let valorarray = numeros[2]; // Acessa o terceiro elemento do array
let valormatriz = matriz[1][0]; // Acessa o primeiro elemento da segunda linha da matriz
```

#### Try Catch Finally, Exceptions, Throw
```dryad
try {
    // Código que pode gerar erro
    let resultado = operacaoRiscosa();
    throw "Erro customizado"; // Lança uma exceção
} catch (erro) {
    // Tratamento do erro
    print("Erro capturado: " + erro);
} finally {
    // Código que sempre será executado
    print("Limpeza sempre executada");
}

// Try com apenas finally
try {
    let dados = processarDados();
} finally {
    liberarRecursos();
}

// Throw statements
throw "Mensagem de erro";
throw variavelErro;
```

#### Foreach (depende da implementação de arrays/matrizes/tuplos)
```dryad
for item in lista {
    // bloco de código para cada item
}

// Itera sobre arrays
for num in [1, 2, 3, 4, 5] {
    print(num);
}

// Itera sobre tuplas
for element in (1, "text", true) {
    print(element);
}

// Itera sobre strings (caractere por caractere)
for char in "Dryad" {
    print(char);
}
```
---

## 🔧 Funções

### Declaração de Funções

```dryad
function saudacao(nome) {
    return "Olá, " + nome + "!";
}

let mensagem = saudacao("Maria");
print(mensagem); // "Olá, Maria!"
```



### Funções sem Retorno

```dryad
function cumprimentar(nome) {
    print("Oi, " + nome + "!");
}

cumprimentar("João"); // "Oi, João!"
```

### Funções com Múltiplos Parâmetros

```dryad
function somar(a, b) {
    return a + b;
}

function calcular(x, y, z) {
    let resultado = x + y * z;
    return resultado;
}

let soma = somar(5, 3);          // 8
let calculo = calcular(2, 3, 4); // 14
```

### Funções Recursivas

```dryad
function fatorial(n) {
    if n <= 1 {
        return 1;
    }
    return n * fatorial(n - 1);
}

let resultado = fatorial(5); // 120
```

### Funções como Valores

```dryad
function quadrado(x) {
    return x * x;
}

let funcao = quadrado;
let resultado = funcao(4); // 16
```

#### Funções nativas 
// São funções que buscam funções basicas do sistema e outras derivadas do rust.
// Estas são pré definidas e não precisam ser declaradas pelo usuário.
// Para evitar sobrecarga de funções o codigo deve ter uma diretiva para definir quais funções nativas serão carregadas no código.

exemplo de diretiva:

#<console_io>
#<file_io>
#<terminal_ansi>
#<binary_io>
#<date_time>
#<system_env>
#<encode_decode>
#<crypto>
#<debug>
#<http>
#<websocket>
#<tcp>
#<udp>
#<web_server>
etc... etc.. etc..
Isto permite que quando o código é executado, as funções nativas estejam disponíveis para uso imediato, sem a necessidade de importações adicionais, ja economia de processamento e memória é algo desejado.

Todas as funções nativas são separadas por módulos, e cada módulo pode ser carregado ou não dependendo da necessidade do código, usando a diretiva de importação.

```dryad
Funções Nativas: 

Buffer de Console / Terminal #<console_io>

native_input();                        // lê linha do stdin
/*
Lê o buffer de entrada do console, esperando Enter, retorna string.
Esta função é bloqueante, ou seja, espera o usuário digitar e pressionar Enter.
retorna: string
*/

native_input_char();                   // lê 1 caractere (sem esperar Enter)
/*
Lê um único caractere do console, sem esperar Enter.
Útil para inputs rápidos ou interativos.
Entrada: nenhum
*/

native_input_bytes(count);             // lê N bytes do console
/*
Lê um número específico de bytes do console.
Esta função é útil para ler dados binários ou quando o tamanho do input é conhecido.
Entrada: um número inteiro representando a quantidade de bytes a serem lidos.
retorna: array de bytes
*/

native_input_timeout(ms);              // lê com timeout
/*
Lê do console com um tempo limite em milissegundos.
Se o tempo limite for atingido, retorna null.
Entrada: um número inteiro representando o tempo limite em milissegundos.
retorna: string ou null
*/

native_print(data);                    // sem quebra de linha
/*
Imprime dados no console sem adicionar uma nova linha.
Entrada: qualquer tipo de dado (string, número, etc.).
retorna: nenhum
*/

native_println(data);                  // com quebra de linha
/*
Imprime dados no console e adiciona uma nova linha.
Entrada: qualquer tipo de dado (string, número, etc.).
retorna: nenhum
*/

native_write_stdout(bytes);            // escrita binária direta
/*
Escreve bytes diretamente no stdout.
Entrada: um array de bytes.
retorna: nenhum
*/

native_flush();                        // força flush do stdout
/*
Força a saída do buffer do stdout.
Entrada: nenhum
retorna: nenhum
*/


🎨 Controle de terminal (ANSI) #<terminal_ansi>

native_clear_screen();                 // limpa terminal

/*
Limpa a tela do terminal, movendo o cursor para o início.
Entrada: nenhum
retorna: nenhum
*/

native_move_cursor(x, y);              // move cursor
/*
Move o cursor para uma posição específica (x, y).
Entrada: dois números inteiros representando as coordenadas x e y.
retorna: nenhum
*/

native_set_color(fg, bg);              // cores (ex: "red", "blue", hex ou índice)
/*
Define a cor do texto e do fundo.
Entrada: duas strings representando as cores do primeiro plano (fg) e do fundo (bg).
retorna: nenhum
*/

native_set_style(style);               // estilo do texto (ex: "bold", "italic")
/*
Define o estilo do texto.
Entrada: uma string representando o estilo (ex: "bold", "italic", "underline").
retorna: nenhum
*/

native_reset_style();                  // reseta estilo do texto
/*
Reseta o estilo do texto para o padrão do terminal.
Entrada: nenhum
retorna: nenhum
*/


native_hide_cursor();                  // oculta cursor
/*
Oculta o cursor do terminal.
Entrada: nenhum
retorna: nenhum
*/

native_show_cursor();                  // mostra cursor
/*
Mostra o cursor do terminal.
Entrada: nenhum
retorna: nenhum
*/

native_terminal_size();                // retorna (cols, rows)
/*
Retorna o tamanho do terminal como uma tupla (colunas, linhas).
Entrada: nenhum
retorna: uma tupla com dois números inteiros representando as colunas e linhas do terminal.
*/

Escrita binária #<binary_io>

native_write_bytes(path, bytes);       // salva buffer no disco
/*
Escreve um array de bytes em um arquivo.
Entrada: um caminho de arquivo (string) e um array de bytes.
retorna: nenhum
*/

native_append_bytes(path, bytes);      // adiciona ao final
/*
Adiciona bytes ao final de um arquivo existente.
Entrada: um caminho de arquivo (string) e um array de bytes.
retorna: nenhum
*/

native_overwrite_chunk(path, offset, bytes); // sobrescreve parte
/*
Sobrescreve uma parte específica de um arquivo com bytes.
Entrada: um caminho de arquivo (string), um offset (número inteiro) e um array de bytes.
retorna: nenhum
*/

native_read_bytes(path);               // retorna array de bytes (ou string binária)
/*
Lê o conteúdo de um arquivo como um array de bytes.
Entrada: um caminho de arquivo (string).
retorna: um array de bytes.
*/

native_read_chunk(path, offset, size); // lê parte do arquivo
/*
Lê uma parte específica de um arquivo como um array de bytes.
Entrada: um caminho de arquivo (string), um offset (número inteiro) e um tamanho (número inteiro).
retorna: um array de bytes.
*/

native_file_size(path);                // retorna tamanho do arquivo
/*
Retorna o tamanho de um arquivo em bytes.
Entrada: um caminho de arquivo (string).
retorna: um número inteiro representando o tamanho do arquivo.
*/

🗂️ Sistema de Arquivos e Diretórios #<file_io>

native_read_file(path);           // lê conteúdo do arquivo como string
/*
Lê o conteúdo de um arquivo como uma string.
Entrada: um caminho de arquivo (string).
retorna: uma string com o conteúdo do arquivo.
*/

native_write_file(path, data);    // escreve string no arquivo
/*
Escreve uma string em um arquivo, sobrescrevendo o conteúdo existente.
Entrada: um caminho de arquivo (string) e uma string com os dados.
retorna: nenhum
*/

native_append_file(path, data);   // adiciona conteúdo ao fim do arquivo
/*
Adiciona uma string ao final de um arquivo existente.
Entrada: um caminho de arquivo (string) e uma string com os dados.
retorna: nenhum
*/

native_delete_file(path);         // deleta arquivo
/*
Deleta um arquivo do sistema.
Entrada: um caminho de arquivo (string).
retorna: nenhum
*/

native_list_dir(path);            // lista arquivos/pastas no diretório
/*
Lista os arquivos e pastas em um diretório.
Entrada: um caminho de diretório (string).
retorna: um array de strings com os nomes dos arquivos e pastas.
*/

native_copy_file(from, to);       // copia arquivo
/*
Copia um arquivo de um local para outro.
Entrada: dois caminhos de arquivo (string), o primeiro é o arquivo de origem e o segundo é o destino.
retorna: nenhum
*/

native_move_file(from, to);       // move arquivo
/*
Move um arquivo de um local para outro.
Entrada: dois caminhos de arquivo (string), o primeiro é o arquivo de origem e o segundo é o destino.
retorna: nenhum
*/

native_file_exists(path);         // bool
/*
Verifica se um arquivo existe.
Entrada: um caminho de arquivo (string).
retorna: um booleano (true se o arquivo existir, false caso contrário).
*/

native_is_dir(path);              // bool
/*
Verifica se um caminho é um diretório.
Entrada: um caminho de arquivo ou diretório (string).
retorna: um booleano (true se for um diretório, false caso contrário).
*/

native_mkdir(path);               // cria pasta
/*
Cria um diretório.
Entrada: um caminho de diretório (string).
retorna: nenhum
*/

native_getcwd();                  // retorna diretório atual
/*
Retorna o diretório de trabalho atual como uma string.
Entrada: nenhum
retorna: uma string com o caminho do diretório atual.
*/

native_setcwd(path);              // muda o diretório atual
/*
Muda o diretório de trabalho atual para o especificado.
Entrada: um caminho de diretório (string).
retorna: nenhum
*/

native_get_file_info(path);       // retorna info do arquivo
/*
Retorna informações sobre um arquivo, como tamanho, data de modificação, etc.
Entrada: um caminho de arquivo (string).
retorna: um objeto com as informações do arquivo.
*/

native_read_file_content(path); // lê conteúdo do arquivo em string, com uma unica linha retornada.
/*
Lê o conteúdo de um arquivo como uma string, sem quebra de linha.
Entrada: um caminho de arquivo (string).
retorna: uma string com o conteúdo do arquivo.
*/


🕓 Tempo, Datas, Temporização #<time>

native_now();                     // timestamp atual
/*
Retorna o timestamp atual em milissegundos desde a época (epoch).
Entrada: nenhum
retorna: um número inteiro representando o timestamp atual.
*/

native_sleep(ms);                 // pausa em milissegundos
/*
Pausa a execução por um número específico de milissegundos.
Entrada: um número inteiro representando o tempo em milissegundos.
retorna: nenhum
*/

native_timestamp();              // timestamp unix
/*
Retorna o timestamp atual em segundos desde a época (epoch).
Entrada: nenhum
retorna: um número inteiro representando o timestamp atual.
*/

native_date();                   // data atual (ex: "2025-07-11")
/*
Retorna a data atual no formato "YYYY-MM-DD".
Entrada: nenhum
retorna: uma string representando a data atual.
*/

native_time();                   // hora atual (ex: "13:37:42")
/*
Retorna a hora atual no formato "HH:MM:SS".
Entrada: nenhum
retorna: uma string representando a hora atual.
*/

native_format_date(fmt);         // formato customizado
/*
Formata a data atual de acordo com o formato especificado.
Entrada: uma string representando o formato (ex: "YYYY-MM-DD HH:mm:ss").
retorna: uma string com a data formatada.
*/

native_uptime();                 // tempo desde início da execução
/*
Retorna o tempo de execução do programa em milissegundos.
Entrada: nenhum
retorna: um número inteiro representando o tempo de execução.
*/

🧠 Sistema, Ambiente e Processo #<system_env>

native_platform();               // "linux", "windows", "macos"
/*
Retorna o sistema operacional atual.
Entrada: nenhum
retorna: uma string representando o sistema operacional.
*/

native_arch();                   // "x86_64", "aarch64"
/*
Retorna a arquitetura do sistema atual.
Entrada: nenhum
retorna: uma string representando a arquitetura do sistema.
*/

native_env(key);                 // busca variável de ambiente
/*
Busca o valor de uma variável de ambiente.
Entrada: uma string representando o nome da variável de ambiente.
retorna: uma string com o valor da variável ou null se não existir.
*/

native_set_env(key, value);      // define variável de ambiente
/*
Define o valor de uma variável de ambiente.
Entrada: duas strings, a primeira é o nome da variável e a segunda é o valor.
retorna: nenhum
*/

native_exec(cmd);                // executa comando no shell
/*
Executa um comando no shell e retorna o status de saída.
Entrada: uma string representando o comando a ser executado.
retorna: um número inteiro representando o status de saída do comando.
*/

native_exec_output(cmd);         // executa e retorna stdout
/*
Executa um comando no shell e retorna sua saída padrão.
Entrada: uma string representando o comando a ser executado.
retorna: uma string com a saída do comando.
*/

native_pid();                    // ID do processo atual
/*
Retorna o ID do processo atual.
Entrada: nenhum
retorna: um número inteiro representando o ID do processo.
*/

native_exit(code);               // encerra execução com código
/*
Encerra a execução do programa com um código de saída.
Entrada: um número inteiro representando o código de saída (0 para sucesso, outros valores para erro).
retorna: nenhum
*/

Encode/Decode (Codificação e Decodificação) #<encode_decode> (json, xml, csv)

native_json_encode(obj);          // converte objeto para JSON
/*
Converte um objeto Dryad para uma string JSON.
Entrada: um objeto Dryad.
retorna: uma string JSON representando o objeto.
*/

native_json_decode(json_str);    // converte JSON para objeto
/*
Converte uma string JSON para um objeto Dryad.
Entrada: uma string JSON.
retorna: um objeto Dryad representando os dados JSON.
*/

native_csv_encode(array);        // converte array para CSV
/*  
Converte um array de arrays ou objetos para uma string CSV.
Entrada: um array de arrays ou objetos.
retorna: uma string CSV representando os dados.
*/

native_csv_decode(csv_str);      // converte CSV para array
/*
Converte uma string CSV para um array de arrays ou objetos.
Entrada: uma string CSV.
retorna: um array de arrays ou objetos representando os dados CSV.
*/

native_xml_encode(obj);          // converte objeto para XML
/*  
Converte um objeto Dryad para uma string XML.
Entrada: um objeto Dryad.
retorna: uma string XML representando o objeto.
*/

native_xml_decode(xml_str);    // converte XML para objeto
/*
Converte uma string XML para um objeto Dryad.
Entrada: uma string XML.
retorna: um objeto Dryad representando os dados XML.
*/

🔐 Criptografia e Identificadores #<crypto>

native_hash_sha256(data);       // string hash
/*
Calcula o hash SHA-256 de uma string ou array de bytes.
Entrada: uma string ou array de bytes.
retorna: uma string representando o hash SHA-256.
*/

native_hash_md5(data);          // md5
/*
Calcula o hash MD5 de uma string ou array de bytes.
Entrada: uma string ou array de bytes.
retorna: uma string representando o hash MD5.
*/

native_uuid();                  // UUID v4
/*
Gera um UUID v4 aleatório.
Entrada: nenhum
retorna: uma string representando o UUID.
*/

native_base64_encode(str);      // codifica
/*
Codifica uma string ou array de bytes em Base64.
Entrada: uma string ou array de bytes.
retorna: uma string Base64 representando os dados.
*/

native_base64_decode(str);      // decodifica
/*
Decodifica uma string Base64 para uma string ou array de bytes.
Entrada: uma string Base64.
retorna: uma string ou array de bytes representando os dados decodificados.
*/

native_hex_encode(str);         // para hexadecimal
/*
Codifica uma string ou array de bytes em hexadecimal.
Entrada: uma string ou array de bytes.
retorna: uma string hexadecimal representando os dados.
*/

native_hex_decode(str);         // de volta para string
/*
Decodifica uma string hexadecimal para uma string ou array de bytes.
Entrada: uma string hexadecimal.
retorna: uma string ou array de bytes representando os dados decodificados.
*/

native_random_bytes(length); // gera bytes aleatórios
/*
Gera um array de bytes aleatórios de um tamanho específico.
Entrada: um número inteiro representando o tamanho do array.
retorna: um array de bytes aleatórios.
*/

native_random_string(length, charset); // gera string aleatória
/*
Gera uma string aleatória de um tamanho específico usando um conjunto de caracteres.
Entrada: um número inteiro representando o tamanho da string e uma string com os caracteres permitidos
(retorna todos os caracteres ASCII se não for especificado).
retorna: uma string aleatória.
*/

native_encrypt_aes(data, key); // criptografa com AES
/*
Criptografa dados usando AES com uma chave fornecida.
Entrada: um array de bytes (dados) e uma string (chave).
retorna: um array de bytes criptografados.
*/

native_decrypt_aes(data, key); // descriptografa com AES
/*
Descriptografa dados criptografados com AES usando a mesma chave.
Entrada: um array de bytes (dados criptografados) e uma string (chave).
retorna: um array de bytes descriptografados.
*/

native_encrypt_rsa(data, public_key); // criptografa com RSA
/*
Criptografa dados usando RSA com uma chave pública fornecida.
Entrada: um array de bytes (dados) e uma string (chave pública).
retorna: um array de bytes criptografados.
*/

native_decrypt_rsa(data, private_key); // descriptografa com RSA
/*
Descriptografa dados criptografados com RSA usando a chave privada correspondente.
Entrada: um array de bytes (dados criptografados) e uma string (chave privada).
retorna: um array de bytes descriptografados.
*/

native_sign(data, private_key); // assina com RSA
/*
Assina dados usando RSA com uma chave privada fornecida.
Entrada: um array de bytes (dados) e uma string (chave privada).
retorna: um array de bytes com a assinatura.
*/

native_verify(data, signature, public_key); // verifica assinatura RSA
/*
Verifica uma assinatura RSA usando a chave pública correspondente.
Entrada: um array de bytes (dados), um array de bytes (assinatura) e uma
string (chave pública).
retorna: um booleano (true se a assinatura for válida, false caso contrário).
*/

native_generate_rsa_keypair(bits); // gera par de chaves RSA
/*
Gera um par de chaves RSA (pública e privada) com o número de bits
especificado.
Entrada: um número inteiro representando o tamanho da chave em bits.
retorna: um objeto com as chaves pública e privada.
*/


🧪 Debug e Diagnóstico

native_log(value);              // imprime valor bruto (sem print formatado)
native_typeof(value);           // tipo como string
native_memory_usage();          // bytes usados
native_stack_trace();           // stack trace atual
native_perf_start(name);        // inicia timer customizado
native_perf_end(name);          // encerra e mostra tempo decorrido

🧠 Manipulação de Dados Estruturados (futuro)

native_json_parse(json_str);    // converte para objeto
native_json_stringify(obj);     // objeto para string
native_csv_parse(csv_str);      // string CSV para array
native_csv_stringify(array);    // array para CSV.
native_xml_parse(xml_str);      // converte XML para objeto
native_xml_stringify(obj);      // objeto para XML string
native_yaml_parse(yaml_str);    // converte YAML para objeto
native_yaml_stringify(obj);     // objeto para YAML string
native_toml_parse(toml_str);    // converte TOML para objeto
native_toml_stringify(obj);     // objeto para TOML string

🧬 Outros Interessantes / Experimentais

native_eval(code);              // executa código Dryad dinâmico
native_clone(obj);              // cópia profunda de objeto
native_watch_file(path);        // observa mudanças em tempo real
native_prompt(message);         // input no terminal
native_random_int(min, max);    // inteiro aleatório
native_random_float(min, max);  // float aleatório
native_random_string(length);   // string aleatória
native_random_bytes(length);    // bytes aleatórios
native_random_seed(seed);       // semente para gerador aleatório

🧭 HTTP (Cliente)

native_http_get(url);                   // GET simples, retorna string
native_http_post(url, body);            // POST, com string no corpo
native_http_headers(url);               // retorna headers
native_http_download(url, path);        // salva conteúdo em arquivo
native_http_status(url);                // retorna status HTTP (200, 404...)
native_http_json(url);                 // retorna JSON como objeto
native_http_set_timeout(url, ms);       // define timeout para requisições
native_http_set_headers(url, headers); // define headers customizados
native_http_set_user_agent(url, agent); // define User-Agent customizado
native_http_set_proxy(url, proxy); // define proxy para requisições
native_http_set_auth(url, username, password); // define autenticação básica
native_http_set_follow_redirects(url, enable); // segue redirecionamentos
native_http_set_cache(url, enable); // ativa/desativa cache
native_http_set_compression(url, enable); // ativa/desativa compressão
native_http_set_max_redirects(url, count); // define máximo de redirecionamentos
native_http_set_retry(url, count); // define número de tentativas em falhas
native_http_set_cookies(url, cookies); // define cookies para requisição
native_http_set_timeout(url, ms); // define timeout para requisições
native_http_set_keepalive(url, enable); // ativa/desativa keepalive
native_http_set_reuseaddr(url, enable); // ativa/desativa reuseaddr
native_http_set_nodelay(url, enable); // desativa Nagle's algorithm
native_http_set_ssl_verify(url, enable); // ativa/desativa verificação SSL
native_http_set_ssl_cert(url, cert_path); // define certificado SSL
native_http_set_ssl_key(url, key_path); // define chave SSL
native_http_set_ssl_ca(url, ca_path); // define CA SSL
native_http_set_ssl_sni(url, sni); // define SNI para SSL
native_http_set_ssl_protocols(url, protocols); // define protocolos SSL permitidos
native_http_set_ssl_ciphers(url, ciphers); // define cifras SSL permitidas
native_http_set_ssl_session(url, session); // define sessão SSL

📡 WebSocket (Cliente)

    Ideal para integração com sistemas reativos (ex: live data, chats)

native_ws_connect(url);                // conecta e retorna ID
native_ws_send(socket_id, message);   // envia mensagem
native_ws_recv(socket_id);            // recebe próxima mensagem
native_ws_close(socket_id);           // encerra conexão
native_ws_is_connected(socket_id);   // verifica se está conectado
native_ws_set_timeout(socket_id, ms); // define timeout para recv
native_ws_set_nodelay(socket_id);      // desativa Nagle's algorithm
native_ws_set_keepalive(socket_id, enable); // ativa/desativa keepalive
native_ws_set_reuseaddr(socket_id, enable); // ativa/desativa reuseaddr

🌍 TCP (Cliente e Servidor)

// Cliente
native_tcp_connect(host, port);         // conecta e retorna socket_id
native_tcp_send(socket_id, data);       // envia dados
native_tcp_recv(socket_id);             // recebe dados (string ou bytes)
native_tcp_close(socket_id);            // fecha conexão
native_tcp_is_connected(socket_id);    // verifica se está conectado
native_tcp_set_timeout(socket_id, ms); // define timeout para recv
native_tcp_set_nodelay(socket_id);      // desativa Nagle's algorithm
native_tcp_set_keepalive(socket_id, enable); // ativa/desativa keepalive
native_tcp_set_reuseaddr(socket_id, enable); // ativa/desativa reuseaddr

// Servidor
native_tcp_listen(port);                // inicia listener TCP e retorna id
native_tcp_accept(listener_id);         // aceita conexão e retorna socket_id
native_tcp_shutdown(listener_id);       // encerra listener
native_tcp_send_all(socket_id, data); // envia dados para todos os clientes
native_tcp_broadcast(data);             // envia dados para todos os clientes conectados
native_tcp_broadcast_except(data, exclude_socket_id); // envia para todos menos um
native_tcp_set_timeout(socket_id, ms); // define timeout para recv
native_tcp_set_nodelay(socket_id);      // desativa Nagle's algorithm
native_tcp_set_keepalive(socket_id, enable); // ativa/desativa keepalive
native_tcp_set_reuseaddr(socket_id, enable); // ativa/desativa reuseaddr

🌐 UDP (Datagramas)

native_udp_bind(port);                  // inicia socket UDP local
native_udp_send(ip, port, data);        // envia datagrama
native_udp_recv();                      // espera e retorna pacote (ip, port, data)
native_udp_close();                     // encerra socket UDP

🕸️ Web Server (mínimo)

Para facilitar criação de APIs locais:

native_web_listen(port);                // inicia servidor web simples
/*
Inicia um servidor web simples na porta especificada.
Entrada: um número inteiro representando a porta.
retorna: nenhum
*/

native_web_route(method, path, handler); // define rota e função
/*
Define uma rota para o servidor web.
Entrada: uma string representando o método HTTP (ex: "GET", "POST"), uma string representando o caminho da rota e uma função que será chamada quando a rota for acessada.
retorna: nenhum
*/

native_web_route_static(path, dir); // rota para arquivos estáticos
/*
Define uma rota para servir arquivos estáticos de um diretório.
Entrada: uma string representando o caminho da rota e uma string representando o diretório de onde os arquivos serão servidos.
retorna: nenhum
*/

native_web_route_data(path, data); // rota para dados dinâmicos
/*
Define uma rota para retornar dados dinâmicos.
Entrada: uma string representando o caminho da rota e uma string representando os dados que serão retornados.
(ex: JSON, XML, etc.)
retorna: nenhum
*/

native_web_on_error(handler); // define função para erros
/*
Define uma função que será chamada quando ocorrer um erro no servidor web.
Entrada: uma função que recebe os parâmetros (error, socket_id) e retorna uma resposta HTTP.
retorna: nenhum
*/

native_web_on_request(handler); // define função para requisições
/*
Define uma função que será chamada para cada requisição recebida pelo servidor web.
Entrada: uma função que recebe os parâmetros (socket_id, method, path, headers, body
) e retorna uma resposta HTTP.
retorna: nenhum
*/

native_web_shutdown();                  // encerra servidor
/*
Encerra o servidor web.
Entrada: nenhum
retorna: nenhum
*/

native_web_on_request(handler); // define função para requisições
/*
Define uma função que será chamada para cada requisição recebida pelo servidor web.
Entrada: uma função que recebe os parâmetros (socket_id, method, path, headers, body) e retorna uma resposta HTTP.
retorna: nenhum
*/

native_web_send_response(socket_id, status, headers, body); // envia resposta
/*
Envia uma resposta HTTP para o cliente.
Entrada: um número inteiro representando o ID do socket, um número inteiro representando o status HTTP (ex: 200, 404), um array de strings representando os headers e uma string representando o corpo da resposta.
retorna: nenhum
*/



```


#### Funções Anônimas (Lambdas)
```dryad
// Planejado para versões futuras   
let quadrado = (x) => x * x;
let resultado = quadrado(5); // 25
```


---

## 🏛️ Classes e Objetos

### Declaração de Classes

```dryad
class Pessoa {
    function init(nome, idade) {
        this.nome = nome;
        this.idade = idade;
    }
    
    function apresentar() {
        return "Meu nome é " + this.nome + " e tenho " + this.idade + " anos.";
    }
    
    function aniversario() {
        this.idade = this.idade + 1;
        return "Agora tenho " + this.idade + " anos!";
    }
}
```

### Criação de Instâncias

```dryad
let pessoa1 = Pessoa("Ana", 25);
let pessoa2 = Pessoa("Bruno", 30);

print(pessoa1.apresentar()); // "Meu nome é Ana e tenho 25 anos."
print(pessoa2.aniversario()); // "Agora tenho 31 anos!"
```




### Métodos Estáticos

```dryad
class Calculadora {
    static function pi() {
        return 3.14159;
    }
    
    static function circunferencia(raio) {
        return 2 * Calculadora.pi() * raio;
    }
}

let circ = Calculadora.circunferencia(5); // 31.4159
```

### Herança

```dryad
// Planejado para versões futuras
class Estudante extends Pessoa {
    function init(nome, idade, curso) {
        super.init(nome, idade);
        this.curso = curso;
    }
    
    function estudar() {
        return this.nome + " está estudando " + this.curso;
    }
}
```

### Visibilidade

```dryad
// Planejado para versões futuras
class ContaBancaria {
    public let numero;
    private let saldo;
    protected let titular;
    
    public function depositar(valor) {
        this.saldo = this.saldo + valor;
    }
    
    private function validarSaque(valor) {
        return valor <= this.saldo;
    }

    public static function criar(numero) {
        return ContaBancaria(numero, 0);
    }

    public function sacar(valor) {
        if this.validarSaque(valor) {
            this.saldo = this.saldo - valor;
            this.titular.notificarSaque(valor);
            return "Saque de " + valor + " realizado.";
        } else {
            return "Saldo insuficiente.";
        }
    }

    protected function notificarSaque(valor) {
        print("Notificando saque de " + valor + " para " + this.titular.nome);
    }
}

class ContaEspecial extends ContaBancaria {
    public function sacar(valor) {
        if this.validarSaque(valor) {
            this.saldo = this.saldo - valor;
            this.titular.notificarSaque(valor);
            return "Saque de " + valor + " realizado.";
        } else {
            return "Saldo insuficiente.";
        }
    }
}
```
print("Mensagem simples");
print("Valor: " + 42);

// Operações de arquivo
// read_file("arquivo.txt")
// write_file("saida.txt", "conteúdo")
```

### Chamadas de Função
```javascript
// Chamada simples
print("Hello World");

// Com expressões
print("Resultado: " + (2 + 3));

// Com variáveis
let nome = "Maria";
print("Olá, " + nome);

// Aninhadas
print("Debug: " + print("valor interno"));
```
---

## 📦 Módulos e Imports

### Sistema Oak

Dryad utiliza o sistema de módulos **Oak**, que permite instalar e organizar bibliotecas de forma eficiente. O Oak é um gerenciador de pacotes completo que oferece dois tipos de projeto:

#### Tipos de Projeto

**1. Projeto (Project)**
```json
{
  "name": "meu-projeto",
  "version": "0.1.0", 
  "type": "project",
  "main": "main.dryad",
  "dependencies": {},
  "scripts": {
    "start": "dryad run main.dryad",
    "test": "dryad test",
    "check": "dryad check main.dryad"
  }
}
```

**2. Biblioteca (Library)**
```json
{
  "name": "minha-biblioteca",
  "version": "0.1.0",
  "type": "library", 
  "main": "src/main.dryad",
  "dependencies": {
    "dryad-stdlib": "^0.1.0"
  },
  "scripts": {
    "check": "dryad check src/main.dryad",
    "test": "dryad test"
  }
}
```

#### Estrutura de Projetos

**Projeto:**
```
meu-projeto/
├── main.dryad           # Ponto de entrada
├── oaklibs.json         # Configuração do projeto
├── oaklock.json         # Lock file (gerado automaticamente)
├── README.md
├── .gitignore
└── src/                 # Código adicional (opcional)
```

**Biblioteca:**
```
minha-biblioteca/
├── src/
│   └── main.dryad       # Ponto de entrada da biblioteca
├── lib/
│   ├── matematica.dryad # Módulos exportáveis
│   └── utilidades.dryad
├── oaklibs.json         # Configuração da biblioteca
├── oaklock.json         # Mapeamento de módulos
├── README.md
└── .gitignore
```

#### Comandos Oak

**Inicializar Projeto:**
```bash
# Criar projeto
oak init meu-projeto --type project

# Criar biblioteca  
oak init minha-biblioteca --type library
```

**Gerenciar Dependências:**
```bash
# Instalar dependência
oak install matematica-utils --version "^0.1.0"

# Remover dependência
oak remove matematica-utils

# Listar dependências
oak list

# Atualizar dependências
oak update
```

**Resolução de Módulos:**
```bash
# Gerar/atualizar oaklock.json
oak lock

# Informações do projeto
oak info

# Executar scripts
oak run start
oak run test
oak run check
```

#### Arquivo oaklock.json

O `oaklock.json` mapeia aliases para caminhos de arquivos, permitindo importações eficientes:

```json
{
  "modules": {
    "matematica-utils": {
      "paths": {
        "matematica": "./oak_modules/matematica-utils/lib/matematica.dryad",
        "utilidades": "./oak_modules/matematica-utils/lib/utilidades.dryad", 
        "formas": "./oak_modules/matematica-utils/lib/formas.dryad"
      }
    },
    "dryad-stdlib": {
      "paths": {
        "io": "./oak_modules/dryad-stdlib/io.dryad",
        "math": "./oak_modules/dryad-stdlib/math.dryad",
        "string": "./oak_modules/dryad-stdlib/string.dryad"
      }
    }
  }
}
```

### Exports

#### Export de Variáveis
```dryad
// arquivo: matematica.dryad
export let PI = 3.14159;
export let E = 2.71828;
```

#### Export de Funções
```dryad
// arquivo: utilidades.dryad
export function quadrado(x) {
    return x * x;
}

export function cubo(x) {
    return x * x * x;
}
```

#### Export de Classes
```dryad
// arquivo: formas.dryad
export class Retangulo {
    function init(largura, altura) {
        this.largura = largura;
        this.altura = altura;
    }
    
    function area() {
        return this.largura * this.altura;
    }
}
```

#### Export de Métodos Estáticos
```dryad
// arquivo: calculadora.dryad
export class Calculadora {
    static function pi() {
        return 3.14159;
    }
    
    static function circunferencia(raio) {
        return 2 * Calculadora.pi() * raio;
    }
}
```

### Imports

#### Use (Import direto)
```dryad
// Importando por caminho relativo -> começa do arquivo atual e percorre o caminho provido.
use "../../oak_modules/matematica-utils/lib/matematica.dryad";

// Importando por caminho absoluto -> começa com a raiz do projeto e percorre o caminho provido.
use "@/matematica-utils/lib/utilidades.dryad";

// Import de bibliotecas instaladas (oak_modules) -> usa o ficheiro oaklock.json para resolver o caminho. com base no alias provido.
use "matematica-utils/matematica";
use "matematica-utils/utilidades";

// Uso direto das funções importadas.
let resultado = quadrado(4);
let forma = Retangulo(10, 20);
let area = forma.area();
```

#### Estrutura Modular Completa

**Biblioteca matematica-utils/lib/matematica.dryad:**
```dryad
export function fatorial(n) {
    if n <= 1 {
        return 1;
    }
    return n * fatorial(n - 1);
}

export function fibonacci(n) {
    if n <= 1 {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}
```

**Projeto que usa a biblioteca:**
```dryad
// main.dryad
use "matematica";

let fat5 = fatorial(5);    // 120
let fib7 = fibonacci(7);   // 13
print("5! = " + fat5);
print("fibonacci(7) = " + fib7);
```

#### Configuração Completa

**oaklibs.json (projeto que consome):**
```json
{
  "name": "meu-app",
  "version": "1.0.0",
  "type": "project",
  "main": "main.dryad",
  "dependencies": {
    "matematica-utils": "^0.1.0",
    "dryad-stdlib": "^0.1.0"
  }
}
```

**oaklock.json (gerado automaticamente):**
```json
{
  "modules": {
    "matematica-utils": {
      "paths": {
        "matematica": "./oak_modules/matematica-utils/lib/matematica.dryad",
        "utilidades": "./oak_modules/matematica-utils/lib/utilidades.dryad"
      }
    }
  }
}
```

### Estrutura de Módulos

```
projeto/
├── main.dryad
├── oaklibs.json
├── oaklock.json
├── oak_modules/
│   └── matematica-utils/
│       └── lib/
│           ├── matematica.dryad
│           ├── utilidades.dryad
│           └── formas.dryad
└── README.md
```

### Configuração Oak

O arquivo `oaklibs.json` é o coração do sistema de módulos:

```json
{
  "name": "meu-projeto",
  "version": "1.0.0",
  "type": "project",
  "main": "main.dryad",
  "dependencies": {
    "matematica-utils": "^0.1.0"
  },
  "scripts": {
    "start": "dryad run main.dryad",
    "test": "dryad test",
    "check": "dryad check main.dryad"
use "matematica-utils/matematica";

let resultado = quadrado(4); // Usa quadrado diretamente
```

### Estrutura de Módulos

```
projeto/
├── main.dryad
├── lib/
│   ├── matematica.dryad
│   ├── utilidades.dryad
│   └── formas.dryad
└── oaklibs.json
```

### Configuração Oak

```json
{
  "name": "meu-projeto",
  "version": "1.0.0",
  "lib_paths": ["./lib"], // Caminho para os módulos ./lib é a biblioteca padrão (common library)
  "dependencies": {}
}
```

---

## 🔧 Funções Nativas

### I/O (Entrada/Saída)

#### Print Functions
```dryad
print("Olá");           // Imprime sem quebra de linha
println("Mundo");       // Imprime com quebra de linha
```

#### Input Function
```dryad
let nome = input("Digite seu nome: ");
print("Olá, " + nome);
```

### String Functions

#### len() - Comprimento
```dryad
let texto = "Dryad";
let tamanho = len(texto); // 5
```

#### substr() - Substring
```dryad
let frase = "Linguagem Dryad";
let parte = substr(frase, 0, 9); // "Linguagem"
```

#### concat() - Concatenação
```dryad
let primeiro = "Olá";
let segundo = "mundo";
let completo = concat(primeiro, ", " + segundo); // "Olá, mundo"
```

### Math Functions

#### abs() - Valor Absoluto
```dryad
let negativo = -15;
let positivo = abs(negativo); // 15
```

#### sqrt() - Raiz Quadrada
```dryad
let numero = 16;
let raiz = sqrt(numero); // 4.0
```

#### pow() - Potenciação
```dryad
let base = 2;
let expoente = 3;
let resultado = pow(base, expoente); // 8.0
```

### Type Functions

#### type() - Tipo do Valor
```dryad
let numero = 42;
let texto = "Hello";
let booleano = true;

print(type(numero));   // "number"
print(type(texto));    // "string"
print(type(booleano)); // "bool"
```

#### Conversões de Tipo
```dryad
// to_string() - Converter para string
let num = 42;
let str = to_string(num); // "42"

// to_number() - Converter para número
let texto = "3.14";
let numero = to_number(texto); // 3.14
```

---

## 💬 Comentários

### Comentários de Linha

```dryad
// Este é um comentário de linha
let x = 10; // Comentário no final da linha

// Múltiplas linhas de comentário
// Cada linha precisa começar com //
// Como estas linhas aqui
```

### Comentários de Bloco

```dryad
// Planejado para versões futuras
/*
   Este é um comentário
   de múltiplas linhas
   que será implementado no futuro
*/
```

---

## 🔒 Palavras Reservadas

### Palavras-chave da Linguagem

#### Declarações
- `let` - Declaração de variável
- `function` - Declaração de função
- `class` - Declaração de classe
- `export` - Exportar elemento
- `static` - Método/propriedade estática

#### Controle de Fluxo
- `if` - Condicional
- `else` - Alternativa condicional
- `while` - Loop
- `for` - Loop iterativo
- `in` - Palavra-chave para foreach loops
- `return` - Retorno de função
- `break` - Quebra de loop
- `continue` - Continuar loop
- `try` - Bloco de tentativa
- `catch` - Captura de exceção
- `finally` - Bloco sempre executado
- `throw` - Lançar exceção

#### Módulos
- `using` - Import com namespace
- `use` - Import direto

#### Valores Literais
- `true` - Verdadeiro
- `false` - Falso
- `null` - Nulo/vazio

#### Orientação a Objetos
- `this` - Referência ao objeto atual
- `super` - Referência à classe pai
- `public` - Visibilidade pública
- `private` - Visibilidade privada

#### Outras
- `var` - (Reservado para uso futuro)
- `const` - (Reservado para uso futuro)

---

## 📚 Exemplos Práticos

### Calculadora Simples

```dryad
// calculadora.dryad
class Calculadora {
    static function somar(a, b) {
        return a + b;
    }
    
    static function subtrair(a, b) {
        return a - b;
    }
    
    static function multiplicar(a, b) {
        return a * b;
    }
    
    static function dividir(a, b) {
        if b == 0 {
            print("Erro: Divisão por zero!");
            return null;
        }
        return a / b;
    }
}

// Usando a calculadora
let resultado1 = Calculadora.somar(10, 5);     // 15
let resultado2 = Calculadora.dividir(20, 4);   // 5.0
let resultado3 = Calculadora.dividir(10, 0);   // null (erro)
```

### Sistema de Usuários

```dryad
// usuario.dryad
class Usuario {
    function init(nome, email, idade) {
        this.nome = nome;
        this.email = email;
        this.idade = idade;
        this.ativo = true;
    }
    
    function perfil() {
        let status = this.ativo ? "Ativo" : "Inativo";
        return this.nome + " (" + this.email + ") - " + status;
    }
    
    function desativar() {
        this.ativo = false;
        return "Usuário " + this.nome + " foi desativado.";
    }
    
    function podeVotar() {
        return this.idade >= 16;
    }
}

// Criando usuários
let user1 = Usuario("Ana Silva", "ana@email.com", 25);
let user2 = Usuario("João Santos", "joao@email.com", 15);

print(user1.perfil());     // "Ana Silva (ana@email.com) - Ativo"
print(user1.podeVotar());  // true
print(user2.podeVotar());  // false
```

### Módulo de Utilidades

```dryad
// math_utils.dryad
export function ehPar(numero) {
    return numero % 2 == 0;
}

export function ehPrimo(numero) {
    if numero < 2 {
        return false;
    }
    
    let i = 2;
    while i * i <= numero {
        if numero % i == 0 {
            return false;
        }
        i = i + 1;
    }
    return true;
}

export function fibonacci(n) {
    if n <= 1 {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

// main.dryad
use "math_utils";

print(ehPar(4));        // true
print(ehPrimo(17));     // true
print(fibonacci(7));    // 13
```

### Conversor de Temperatura

```dryad
// temperatura.dryad
export class ConversorTemperatura {
    static function celsiusParaFahrenheit(celsius) {
        return (celsius * 9 / 5) + 32;
    }
    
    static function fahrenheitParaCelsius(fahrenheit) {
        return (fahrenheit - 32) * 5 / 9;
    }
    
    static function celsiusParaKelvin(celsius) {
        return celsius + 273.15;
    }
    
    static function kelvinParaCelsius(kelvin) {
        return kelvin - 273.15;
    }
}

// Exemplo de uso
using "temperatura" as temp;

let celsius = 25;
let fahrenheit = temp.ConversorTemperatura.celsiusParaFahrenheit(celsius);
let kelvin = temp.ConversorTemperatura.celsiusParaKelvin(celsius);

println("Temperatura:");
println(celsius + "°C = " + fahrenheit + "°F");
println(celsius + "°C = " + kelvin + "K");
```

---

## 🚀 Funcionalidades Futuras

### Arrays Indexáveis

```dryad
// Planejado para versões futuras
let numeros = [1, 2, 3, 4, 5];
let primeiro = numeros[0];      // 1
numeros[2] = 10;               // Modifica o terceiro elemento
let tamanho = len(numeros);    // 5

// Métodos de array
numeros.push(6);               // Adiciona elemento
let ultimo = numeros.pop();    // Remove e retorna último
```

### Objects/Maps

```dryad
// Planejado para versões futuras
let pessoa = {
    "nome": "Maria",
    "idade": 30,
    "email": "maria@email.com"
};

pessoa["telefone"] = "123-456-7890";  // Adiciona nova propriedade
let nome = pessoa["nome"];            // Acesso por chave
```

### Loops Avançados

```dryad
// Planejado para versões futuras

// While loop
let contador = 0;
while contador < 10 {
    print(contador);
    contador = contador + 1;
}

// For loop
for i in 0..10 {
    print(i);
}

// For-each loop
let lista = [1, 2, 3, 4, 5];
for item in lista {
    print(item);
}
```

### Tratamento de Exceções

```dryad
// Planejado para versões futuras
try {
    let resultado = dividir(10, 0);
    print(resultado);
} catch (erro) {
    print("Erro capturado: " + erro.message);
} finally {
    print("Sempre executado");
}

// Lançar exceções
function validarIdade(idade) {
    if idade < 0 {
        throw "Idade não pode ser negativa";
    }
    return true;
}
```

### Funções de Usuário Avançadas

```dryad
// Planejado para versões futuras

// Parâmetros padrão
function saudar(nome = "Visitante", saudacao = "Olá") {
    return saudacao + ", " + nome + "!");
}

// Parâmetros variáveis
function somar(...numeros) {
    let total = 0;
    for numero in numeros {
        total = total + numero;
    }
    return total;
}

// Funções lambda/anônimas
let quadrado = (x) => x * x;
let filtrados = lista.filter((x) => x > 5);
```

### Modules Avançados

```dryad
// Planejado para versões futuras

// Re-exports
export { funcao1, Classe1 } from "outro_modulo";

// Exports com renomeação
export { minhaFuncao as funcaoUtil };

// Import específico
import { funcao1, Classe1 } from "modulo";

// Import tudo
import * as utils from "utilidades";
```

### Recursos de Sistema

```dryad
// Planejado para versões futuras

// File System
let conteudo = fs.readFile("arquivo.txt");
fs.writeFile("saida.txt", "conteúdo");
let arquivos = fs.listDir("diretorio/");

// JSON
let objeto = json.parse('{"nome": "João", "idade": 25}');
let texto = json.stringify(objeto);

// HTTP (futuro distante)
let resposta = http.get("https://api.exemplo.com/dados");
```

---

## 📖 Guia de Referência Rápida

### Sintaxe Básica
```dryad
// Variáveis
let nome = "Dryad";
let idade = 1;

// Funções
function cumprimentar(nome) {
    return "Olá, " + nome;
}

// Classes
class Pessoa {
    function init(nome) {
        this.nome = nome;
    }
}

// Condicionais
if idade >= 18 {
    print("Adulto");
} else {
    print("Menor");
}
```

### Operadores Essenciais
```dryad
+  -  *  /     // Aritméticos
== != < > <= >= // Comparação
&& || !         // Lógicos
=               // Atribuição
```

### Funções Nativas Principais
```dryad
print()     println()   input()
len()       substr()    concat()
abs()       sqrt()      pow()
type()      to_string() to_number()
```

---

*Esta documentação reflete a sintaxe atual da linguagem Dryad (v0.1.1). Para funcionalidades futuras e roadmap detalhado, consulte a documentação de desenvolvimento.*
