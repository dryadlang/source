# Códigos de Erro Implementados

Lista completa e correta dos códigos de erro implementados no sistema `dryad_errors`.

## 1xxx: Erros Léxicos

| Código | Descrição |
| :--- | :--- |
| **1001** | Caractere inesperado |
| **1002** | Literal de string não terminado |
| **1003** | Bloco de comentário não terminado |
| **1004** | Formato de número inválido |
| **1005** | Sequência de escape inválida |
| **1006** | Diretiva nativa inválida |

## 2xxx: Erros de Parser (Sintaxe)

| Código | Descrição |
| :--- | :--- |
| **2001** | Token inesperado |
| **2003** | Ponto e vírgula hiante (missing semicolon) |
| **2005** | Parêntese de fechamento ausente |
| **2011** | Declaração de variável inválida |
| **2017** | Parâmetros de função ausentes (esperado) |
| **2018** | Condição de while ausente (esperado) |
| **2019** | Componentes de for ausentes (esperado) |

## 3xxx: Erros de Runtime

| Código | Descrição |
| :--- | :--- |
| **3001** | Variável não definida |
| **3005** | Operação aritmética inválida |
| **3006** | Multiplicação inválida |
| **3007** | Divisão por zero |
| **3009** | Comparação inválida |
| **3010** | Break fora de loop |
| **3011** | Continue fora de loop |
| **3020** | Exceção lançada (usuário) |
| **3021** | Retorno de função inválido |
| **3022** | Contexto `this` inválido |
| **3023** | `super` não implementado |
| **3034** | Atribuição de propriedade inválida |

## 4xxx: Erros de Tipo (Planejados)

| Código | Descrição |
| :--- | :--- |
| **4001** | Tipos incompatíveis |
| **4002** | Conversão inválida |

## 5xxx: Erros de I/O

| Código | Descrição |
| :--- | :--- |
| **5001** | Arquivo não encontrado |
| **5002** | Permissão negada |

## 6xxx: Erros de Módulo

| Código | Descrição |
| :--- | :--- |
| **6001** | Módulo nativo desconhecido |
| **6002** | Importação circular |

## 8xxx: Warnings

| Código | Descrição |
| :--- | :--- |
| **8001** | Variável não utilizada |
| **8002** | Função depreciada |
| **8003** | Potencial vazamento de memória |

## 9xxx: Erros de Sistema

| Código | Descrição |
| :--- | :--- |
| **9001** | Memória insuficiente |
| **9002** | Estouros de pilha (Stack Overflow) |
