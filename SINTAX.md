# SINTAX.md - Dryad Language & Ecosystem Reference

Este arquivo documenta a sintaxe completa da linguagem Dryad, sua biblioteca padrão e bibliotecas principais do ecossistema. Ele serve como referência para desenvolvedores e agentes de IA.

---

## 1. Estrutura Léxica

### Comentários
```javascript
// Comentário de uma linha
/* Comentário
   de múltiplas linhas */
```

### Identificadores
Nomes de variáveis, funções e classes devem começar com letra ou `_`, seguidos de letras, números ou `_`.

### Literais
| Tipo | Exemplo |
|------|---------|
| **String** | `"Olá Mundo"`, `'Dryad'` |
| **Number** | `42`, `3.14`, `0xFF` (hex), `0b101` (bin) |
| **Boolean** | `true`, `false` |
| **Null** | `null` |
| **Template** | \`Valor: ${x}\` |

---

## 2. Variáveis e Constantes

### Declaração
Dryad possui tipagem dinâmica com suporte opcional a anotações de tipo.

```javascript
// Variável mutável
let x = 10;
let nome: string = "Pedro"; // Com tipo explícito

// Constante (imutável)
const PI = 3.14159;
const MAX_USERS: number = 100;
```

---

## 3. Tipos de Dados

- `number`: Inteiros e ponto flutuante (f64).
- `string`: Texto UTF-8.
- `bool`: Booleano.
- `null`: Valor nulo.
- `any`: Qualquer tipo (dinâmico).
- `void`: Sem retorno (funções).
- `array`: Listas dinâmicas `[]`.
- `tuple`: Sequências de tamanho fixo `(1, "a")`.
- `object`: Dicionários/Instâncias `{ key: val }`.
- `function`: Funções.

---

## 4. Operadores

### Aritméticos
`+`, `-`, `*`, `/`, `%` (módulo), `**` (potência).

### Atribuição
`=`, `+=`, `-=`, `*=`, `/=`.

### Comparação
`==`, `!=`, `<`, `>`, `<=`, `>=`.

### Lógicos
`&&` (AND), `||` (OR), `!` (NOT).

### Bitwise
`&`, `|`, `^`, `<<`, `>>`, `<<<`, `>>>`.

### Especiais
- `++`, `--`: Incremento/Decremento (pré/pós).
- `??`: Null coalesce (ex: `a ?? b`).

---

## 5. Controle de Fluxo

### If / Else
```javascript
if (x > 10) {
    println("Maior");
} else if (x == 10) {
    println("Igual");
} else {
    println("Menor");
}
```

### Loops
```javascript
// While
while (cond) { ... }

// Do-While
do { ... } while (cond);

// For (estilo C)
for (let i = 0; i < 10; i++) { ... }

// For-In (Iteração)
for (item in lista) { ... }

// Controle
break;
continue;
```

### Match (Pattern Matching)
```javascript
match valor {
    1 => println("Um"),
    x if x > 10 => println("Maior que 10"), // Guard
    _ => println("Outro") // Default
}
```

### Tratamento de Erros
```javascript
try {
    throw "Erro!";
} catch (e) {
    println("Capturado: " + e);
} finally {
    println("Sempre executado");
}
```

---

## 6. Funções

### Declaração Padrão
```javascript
function soma(a, b) {
    return a + b;
}

// Com tipos
function soma(a: number, b: number): number {
    return a + b;
}
```

### Lambdas / Arrow Functions
```javascript
let dobro = (x) => x * 2;
let somar = (a, b) => { return a + b; };
```

### Assíncronas
```javascript
async function carregarDados() {
    let dados = await fetch("url");
    return dados;
}
```

### Threads
```javascript
// Função executada em nova thread
thread function worker() {
    println("Executando em paralelo");
}

// Executar função existente em thread
thread(soma, 10, 20);
```

### Mutex
```javascript
let m = mutex();
// lock/unlock implícitos ou via métodos (depende da implementação da std)
```

---

## 7. Orientação a Objetos

### Classes
```javascript
class Animal {
    let nome: string;
    
    // Construtor é o corpo da classe ou método específico (depende da impl. atual, assumindo estilo C#/TS)
    // Na AST atual: ClassDeclaration tem membros. Inicialização costuma ser via 'new'.

    public function falar() {
        println(this.nome + " faz som.");
    }
}

// Herança
class Cachorro extends Animal {
    public function falar() {
        super.falar();
        println("Au Au!");
    }
}

let dog = new Cachorro();
dog.nome = "Rex";
dog.falar();
```

### Interfaces
```javascript
interface Voavel {
    function voar();
}

class Passaro implements Voavel {
    function voar() { println("Voando..."); }
}
```

### Visibilidade e Modificadores
- `public`, `private`, `protected`.
- `static`: Membros de classe.
- `get`, `set`: Propriedades computadas.

---

## 8. Módulos e Diretivas

### Import / Export
```javascript
// Exportar
export function teste() { ... }
export class MinhaClasse { ... }

// Importar
import { teste } from "./meu_modulo";
import * as Mod from "./outro_modulo";
```

### Diretivas Nativas
Carregam módulos nativos da VM (Rust).
```javascript
#<console_io>  // Habilita println, input
#<file_io>     // Habilita fs_*
#<events>      // Habilita events_*
```

---

## 9. Biblioteca Padrão (Native Modules)

Funções globais disponíveis após ativar a diretiva correspondente.

### `#<console_io>`
- `println(msg)`: Imprime e quebra linha.
- `print(msg)`: Imprime sem quebra.
- `input(prompt)`: Lê entrada do usuário.

### `#<file_io>`
- `fs_read_text(path)`
- `fs_write_text(path, content)`
- `fs_exists(path)`
- `fs_mkdir(path)`
- `fs_delete(path)`

### `#<events>`
- `events_new()`: Cria emitter.
- `events_on(emitter, event, cb)`
- `events_off(emitter, event, cb)`
- `events_emit(emitter, event)`

### `#<time>`
- `time_now()`
- `sleep(ms)`

### `#<http_client>` / `#<http_server>`
- `http_get(url)`
- `http_server_new(port)`

### Outros Módulos
- `#<system_env>`: Variáveis de ambiente.
- `#<json_stream>`: Parser JSON.
- `#<crypto>`: Criptografia.
- `#<ffi>`: Foreign Function Interface (carregar DLLs/.so).
- `#<websocket>`, `#<tcp>`, `#<udp>`: Rede.

---

## 10. Ecossistema (Libs em Dryad)

### Fern (Matemática e Ciência)
Biblioteca inspirada em NumPy/Pandas.
**Import:** `fern/src/lib.dryad`

- **Arrays**: `import { Array } from "./math/arrays";`
- **DataFrames**: `import { DataFrame } from "./data/frames";`
- **Estatística**: `import { Statistics } from "./math/statistics";`
- **Álgebra**: `import { Symbol } from "./symbolic/algebra";`

**Exemplo:**
```javascript
let arr = new Array([1, 2, 3, 4, 5]);
println(arr.mean());
```

### Ipe (Interface Gráfica)
Framework UI Cross-Platform.
**Import:** `ipe/lib/ipe.dryad`

- **Classes**: `Application`, `Form`, `Button`, `Label`.
- **Eventos**: Baseado em `EventEmitter`.

**Exemplo:**
```javascript
import { Application, Form, Button } from "./ipe/lib/ipe";

class MainForm extends Form {
    function constructor() {
        this.title = "Demo Ipe";
        
        let btn = new Button();
        btn.text = "Clique-me";
        btn.on("click", (d) => println("Clicado!"));
        
        this.add(btn);
    }
}

Application.run(new MainForm());
```
