# 📚 Guia Completo de Erros - Linguagem Dryad

**Versão:** 2.0  
**Data:** 26 de setembro de 2025  
**Acesso Rápido:** Use Ctrl+F para buscar pelo código do erro (ex: E1001)  

---

## 🔍 Índice por Categoria

- [🔤 Erros Léxicos (1000-1999)](#erros-léxicos-1000-1999)
- [📝 Erros de Parser (2000-2999)](#erros-de-parser-2000-2999)
- [⚡ Erros de Runtime (3000-3999)](#erros-de-runtime-3000-3999)
- [🏷️ Erros de Tipo (4000-4999)](#erros-de-tipo-4000-4999)
- [📁 Erros de I/O (5000-5999)](#erros-de-io-5000-5999)
- [📦 Erros de Módulo (6000-6999)](#erros-de-módulo-6000-6999)
- [📖 Erros de Sintaxe (7000-7999)](#erros-de-sintaxe-7000-7999)
- [⚠️ Avisos/Warnings (8000-8999)](#avisoswarnings-8000-8999)
- [🖥️ Erros de Sistema (9000-9999)](#erros-de-sistema-9000-9999)

---

## 🔤 Erros Léxicos (1000-1999)

### E1001: Caracter Inesperado

**Descrição:** O lexer encontrou um caracter que não é válido na linguagem Dryad.

**Exemplo de Código Problemático:**
```dryad
let invalid@variable = 10;
//       ^ E1001: Caracter inesperado '@'
```

**Solução:**
```dryad
let invalid_variable = 10;  // Use underscore em vez de @
```

**Causas Comuns:**
- Uso de símbolos não permitidos em identificadores
- Caracteres de controle invisíveis
- Cópia/cola de código com caracteres especiais

**Dica:** Identificadores podem conter apenas letras, números e underscore, e devem começar com letra ou underscore.

---

### E1002: String Literal Não Fechada

**Descrição:** Uma string foi iniciada mas não foi fechada corretamente.

**Exemplo de Código Problemático:**
```dryad
let message = "Hello World;
//            ^ E1002: String literal não fechada
```

**Solução:**
```dryad
let message = "Hello World";  // Adicione aspas de fechamento
```

**Causas Comuns:**
- Esquecimento de aspas de fechamento
- Quebra de linha não intencional dentro da string
- Uso de aspas diferentes para abrir e fechar

**Dica:** Use `\"` para incluir aspas dentro de strings.

---

### E1003: Comentário de Bloco Não Fechado

**Descrição:** Um comentário de bloco `/* */` foi iniciado mas não foi fechado.

**Exemplo de Código Problemático:**
```dryad
/* Este comentário nunca fecha
let x = 5;
// ^ E1003: Comentário de bloco não fechado
```

**Solução:**
```dryad
/* Este comentário está correto */
let x = 5;
```

**Causas Comuns:**
- Esquecimento do `*/` de fechamento
- Comentários de bloco aninhados incorretamente
- Edição que removeu acidentalmente o fechamento

**Dica:** Para comentários de múltiplas linhas, considere usar `//` em cada linha.

---

### E1004: Formato de Número Inválido

**Descrição:** O formato do número não está correto.

**Exemplo de Código Problemático:**
```dryad
let value = 123.45.67;  // E1004: Múltiplos pontos decimais
let binary = 0b102;     // E1004: Dígito inválido em binário
```

**Solução:**
```dryad
let value = 123.45;     // Apenas um ponto decimal
let binary = 0b101;     // Apenas 0 e 1 em binário
```

**Formatos Válidos:**
- Decimais: `123`, `123.45`, `0.5`
- Binários: `0b1010`, `0B1010`
- Octais: `0o755`, `0O755`  
- Hexadecimais: `0xFF`, `0xff`, `0XFF`

---

### E1005: Sequência de Escape Inválida

**Descrição:** Uma sequência de escape em string não é válida.

**Exemplo de Código Problemático:**
```dryad
let text = "Hello\q World";  // E1005: \q não é válido
```

**Solução:**
```dryad
let text = "Hello\\n World"; // Use \n para nova linha
```

**Sequências de Escape Válidas:**
- `\n` - Nova linha
- `\t` - Tab
- `\r` - Retorno de carro
- `\\` - Barra invertida
- `\"` - Aspas duplas
- `\'` - Aspas simples
- `\uXXXX` - Unicode (4 dígitos hex)

---

## 📝 Erros de Parser (2000-2999)

### E2001: Token Inesperado

**Descrição:** O parser encontrou um token que não era esperado no contexto atual.

**Exemplo de Código Problemático:**
```dryad
if (x > 0 {  // E2001: Esperado ')', encontrado '{'
    println("positive");
}
```

**Solução:**
```dryad
if (x > 0) {  // Adicione o parêntese de fechamento
    println("positive");
}
```

**Contextos Comuns:**
- Parênteses não balanceados
- Chaves não balanceadas
- Vírgulas ausentes em listas
- Operadores em posições incorretas

---

### E2002: Expressão Inválida

**Descrição:** A expressão não pode ser analisada corretamente.

**Exemplo de Código Problemático:**
```dryad
let result = 5 + * 3;  // E2002: Operadores consecutivos
```

**Solução:**
```dryad
let result = 5 + 3;    // Remova o operador extra
let result = 5 * 3;    // Ou use o operador correto
```

---

### E2003: Ponto e Vírgula Esperado

**Descrição:** Uma declaração deveria terminar com ponto e vírgula.

**Exemplo de Código Problemático:**
```dryad
let x = 5
let y = 10  // E2003: Esperado ';' após declaração anterior
```

**Solução:**
```dryad
let x = 5;  // Adicione ponto e vírgula
let y = 10;
```

---

## ⚡ Erros de Runtime (3000-3999)

### E3001: Divisão por Zero

**Descrição:** Tentativa de dividir um número por zero.

**Exemplo de Código Problemático:**
```dryad
let result = 10 / 0;  // E3001: Divisão por zero
```

**Solução:**
```dryad
let divisor = 0;
if (divisor != 0) {
    let result = 10 / divisor;
} else {
    println("Erro: divisor não pode ser zero");
}
```

**Verificação Preventiva:**
```dryad
function safe_divide(a, b) {
    if (b == 0) {
        throw "Divisão por zero não permitida";
    }
    return a / b;
}
```

---

### E3002: Variável Não Definida

**Descrição:** Tentativa de usar uma variável que não foi declarada.

**Exemplo de Código Problemático:**
```dryad
println(undefined_var);  // E3002: Variável não definida
```

**Solução:**
```dryad
let undefined_var = "agora está definida";
println(undefined_var);
```

---

### E3003: Função Não Encontrada

**Descrição:** Tentativa de chamar uma função que não existe.

**Exemplo de Código Problemático:**
```dryad
nonexistent_function();  // E3003: Função não encontrada
```

**Solução:**
```dryad
function nonexistent_function() {
    println("Agora existe!");
}
nonexistent_function();
```

---

## 🏷️ Erros de Tipo (4000-4999)

### E4001: Tipos Incompatíveis

**Descrição:** Operação entre tipos que não são compatíveis.

**Exemplo de Código Problemático:**
```dryad
let result = "texto" + 42;  // E4001: String + Number
```

**Solução:**
```dryad
// Conversão explícita
let result = "texto" + String(42);  // "texto42"
let result = Number("42") + 42;     // 84 (se "42" for válido)
```

**Conversões Disponíveis:**
- `String(valor)` - Converte para string
- `Number(valor)` - Converte para número
- `Boolean(valor)` - Converte para booleano

---

### E4002: Conversão Inválida

**Descrição:** Tentativa de conversão que não pode ser realizada.

**Exemplo de Código Problemático:**
```dryad
let num = Number("não é número");  // E4002: Conversão inválida
```

**Solução:**
```dryad
let text = "não é número";
if (is_numeric(text)) {
    let num = Number(text);
} else {
    println("Valor não é numérico");
}
```

---

## 📁 Erros de I/O (5000-5999)

### E5001: Arquivo Não Encontrado

**Descrição:** Tentativa de acessar um arquivo que não existe.

**Exemplo de Código Problemático:**
```dryad
let content = read_file("arquivo_inexistente.txt");  // E5001
```

**Solução:**
```dryad
if (file_exists("arquivo.txt")) {
    let content = read_file("arquivo.txt");
    println(content);
} else {
    println("Arquivo não encontrado");
}
```

---

### E5002: Permissão Negada

**Descrição:** Sem permissões para acessar o arquivo ou diretório.

**Exemplo de Código Problemático:**
```dryad
write_file("/root/protected.txt", "data");  // E5002: Sem permissão
```

**Solução:**
- Verifique as permissões do arquivo/diretório
- Execute com privilégios adequados
- Use um local com permissões de escrita

---

## 📦 Erros de Módulo (6000-6999)

### E6001: Módulo Não Encontrado

**Descrição:** Tentativa de importar um módulo que não existe.

**Exemplo de Código Problemático:**
```dryad
import "modulo_inexistente";  // E6001
```

**Solução:**
```dryad
import "modulo_existente";  // Verifique o nome e caminho
```

---

### E6002: Import Circular

**Descrição:** Dependência circular entre módulos.

**Exemplo de Código Problemático:**
```dryad
// arquivo_a.dryad
import "arquivo_b";

// arquivo_b.dryad  
import "arquivo_a";  // E6002: Import circular
```

**Solução:**
- Reorganize o código para evitar dependências circulares
- Extraia código comum para um terceiro módulo
- Use injeção de dependência

---

## 📖 Erros de Sintaxe (7000-7999)

### E7001: Sintaxe de Declaração Inválida

**Descrição:** A sintaxe para declarar variáveis está incorreta.

**Exemplo de Código Problemático:**
```dryad
var x = 5;  // E7001: Use 'let' ou 'const'
```

**Solução:**
```dryad
let x = 5;     // Variável mutável
const y = 10;  // Constante
```

---

## ⚠️ Avisos/Warnings (8000-8999)

### W8001: Variável Não Utilizada

**Severidade:** 🟡 Média

**Descrição:** Variável foi declarada mas nunca usada.

**Exemplo de Código:**
```dryad
let unused_var = 42;  // W8001: Nunca utilizada
let used_var = 10;
println(used_var);
```

**Soluções:**
1. Remova a variável se não for necessária
2. Use a variável no código
3. Prefixe com `_` para indicar que é intencional: `let _unused_var = 42;`

---

### W8002: Função Deprecated

**Severidade:** 🟠 Alta

**Descrição:** Função está marcada como deprecated e pode ser removida em versões futuras.

**Exemplo de Código:**
```dryad
old_function();  // W8002: Função deprecated
```

**Solução:**
```dryad
new_function();  // Use a nova versão da função
```

---

### W8003: Potencial Vazamento de Memória

**Severidade:** 🟠 Alta

**Descrição:** Código pode causar vazamento de memória.

**Exemplo de Código:**
```dryad
while (true) {
    let data = create_large_object();  // W8003: Possível vazamento
    // data nunca é liberada
}
```

**Solução:**
```dryad
while (condition) {
    let data = create_large_object();
    process_data(data);
    data = null;  // Libere explicitamente
}
```

---

## 🖥️ Erros de Sistema (9000-9999)

### E9001: Memória Insuficiente

**Descrição:** Sistema ficou sem memória disponível.

**Soluções:**
- Otimize uso de memória
- Processe dados em lotes menores
- Libere recursos não utilizados

---

### E9002: Stack Overflow

**Descrição:** Pilha de chamadas excedeu o limite.

**Causa Comum:** Recursão infinita

**Exemplo de Código Problemático:**
```dryad
function recursive() {
    recursive();  // E9002: Stack overflow
}
```

**Solução:**
```dryad
function recursive(depth) {
    if (depth <= 0) return;  // Condição de parada
    recursive(depth - 1);
}
```

---

## 🔧 Ferramentas de Debug

### Análise de Stack Trace

Quando um erro de runtime ocorre, o stack trace mostra:

```
📚 Stack Trace:
┌─ main (programa.dryad:1:1)
├─ calculate (programa.dryad:15:5) - function call  
├─ divide (programa.dryad:25:10) - within for loop
```

**Interpretação:**
1. **main**: Função principal onde tudo começou
2. **calculate**: Função chamada que contém o problema  
3. **divide**: Função onde o erro realmente ocorreu

### Contexto de Variáveis

```
🔍 Variáveis locais:
   numerator = 100
   denominator = 0
   iteration = 5
```

**Como usar:**
- Verifique se os valores estão corretos
- Identifique variáveis com valores inesperados
- Trace como as variáveis chegaram naquele estado

---

## 📱 Acesso Rápido por Código

**Formato:** `E` + 4 dígitos (Ex: E1001, E2003, E3001)
**Busca:** Use Ctrl+F e digite o código do erro
**Categorias:** Primeiro dígito indica a categoria (1=Lexer, 2=Parser, etc.)

---

## 🆘 Obtendo Ajuda

1. **Procure o código do erro** neste documento
2. **Leia a descrição** e exemplos
3. **Aplique a solução** sugerida
4. **Se persistir**: Verifique o stack trace e contexto de variáveis
5. **Documentação oficial**: https://docs.dryad-lang.org
6. **Comunidade**: https://github.com/Dryad-lang/discussions

