# 📚 Módulos de Funções Nativas do Dryad

## Visão Geral

Os módulos nativos do Dryad fornecem funcionalidades essenciais do sistema e bibliotecas avançadas implementadas em Rust. Essas funções são pré-definidas e não precisam ser declaradas pelo usuário, oferecendo acesso direto a operações de baixo nível e APIs do sistema.

## Sistema de Módulos por Demanda

Para otimizar performance e uso de memória, o Dryad utiliza um sistema de carregamento seletivo de módulos através de diretivas. Apenas os módulos especificados são carregados na memória durante a execução.

### Sintaxe das Diretivas

```dryad
// Carregamento de módulos individuais
#<console_io>     // Entrada/saída do console
#<file_io>        // Operações de arquivo
#<terminal_ansi>  // Controle de terminal ANSI
#<binary_io>      // I/O binário
#<time>           // Data e tempo
#<system_env>     // Ambiente do sistema
#<encode_decode>  // Codificação/decodificação
#<crypto>         // Criptografia
#<debug>          // Ferramentas de debug
#<utils>          // Utilitários diversos
#<http_client>    // Cliente HTTP
#<http_server>    // Servidor HTTP
#<tcp>            // Protocolo TCP
#<udp>            // Protocolo UDP

// Múltiplos módulos
#<console_io>
#<file_io>
#<crypto>
```

### Benefícios do Sistema

- ✅ **Performance Otimizada**: Apenas módulos necessários são carregados
- ✅ **Uso Eficiente de Memória**: Reduz overhead desnecessário
- ✅ **Carregamento Rápido**: Inicialização mais rápida da aplicação
- ✅ **Modularidade**: Funcionalidades organizadas por domínio
- ✅ **Escalabilidade**: Fácil adição de novos módulos

---

## 📋 Índice de Módulos Disponíveis

| Módulo | Diretiva | Status | Descrição |
|--------|----------|--------|-----------|
| Console I/O | `#<console_io>` | ✅ | Entrada/saída do console |
| Terminal ANSI | `#<terminal_ansi>` | ✅ | Controle avançado de terminal |
| Binary I/O | `#<binary_io>` | ✅ | Operações binárias |
| File I/O | `#<file_io>` | ✅ | Manipulação de arquivos |
| Time | `#<time>` | ✅ | Data, hora e temporização |
| System Env | `#<system_env>` | ✅ | Ambiente do sistema |
| Encode/Decode | `#<encode_decode>` | ✅ | JSON, CSV, XML, Base64 |
| Crypto | `#<crypto>` | ✅ | Criptografia e hashing |
| Debug | `#<debug>` | ✅ | Ferramentas de debug |
| Utils | `#<utils>` | ✅ | Utilitários diversos |
| HTTP Client | `#<http_client>` | ✅ | Cliente HTTP/HTTPS |
| HTTP Server | `#<http_server>` | ✅ | Servidor web |
| TCP | `#<tcp>` | ✅ | Comunicação TCP |
| UDP | `#<udp>` | ✅ | Comunicação UDP |

---

# 📦 Documentação Detalhada dos Módulos

## 🖥️ Console I/O `#<console_io>`

**Descrição**: Funções para interação com o console/terminal, incluindo entrada de dados do usuário e saída formatada.

**Casos de Uso**: Aplicações interativas, prompt de comando, jogos de console, ferramentas CLI. 

### Funções de Entrada (Input)

```dryad
native_input();
```
**Descrição**: Lê uma linha completa do stdin (entrada padrão).
- **Parâmetros**: Nenhum
- **Retorno**: `string` - Linha lida do console
- **Comportamento**: Bloqueante - espera o usuário pressionar Enter
- **Exemplo**:
```dryad
print("Digite seu nome: ");
let nome = native_input();
print("Olá, " + nome + "!");
```

---

```dryad
native_input_char();
```
**Descrição**: Lê um único caractere do console.
- **Parâmetros**: Nenhum
- **Retorno**: `string` - Primeiro caractere da linha
- **Uso**: Ideal para menus interativos, confirmações rápidas
- **Exemplo**:
```dryad
print("Pressione qualquer tecla...");
let tecla = native_input_char();
print("Você pressionou: " + tecla);
```

---

```dryad
native_input_bytes(count);
```
**Descrição**: Lê um número específico de bytes do console.
- **Parâmetros**: 
  - `count`: `number` - Quantidade de bytes para ler
- **Retorno**: `string` - Dados lidos como string
- **Uso**: Leitura de dados binários ou tamanho conhecido
- **Exemplo**:
```dryad
let dados = native_input_bytes(10);
print("Lidos " + dados.length + " bytes");
```

---

```dryad
native_input_timeout(ms);
```
**Descrição**: Lê entrada do console com timeout.
- **Parâmetros**:
  - `ms`: `number` - Timeout em milissegundos
- **Retorno**: `string` ou `null` - Dados lidos ou null se timeout
- **Uso**: Interfaces que não devem travar indefinidamente
- **Exemplo**:
```dryad
print("Você tem 5 segundos para responder...");
let resposta = native_input_timeout(5000);
if (resposta == null) {
    print("Timeout! Resposta padrão será usada.");
} else {
    print("Resposta: " + resposta);
}
```

### Funções de Saída (Output)

```dryad
native_print(data);
```
**Descrição**: Imprime dados no console sem quebra de linha.
- **Parâmetros**:
  - `data`: `any` - Dados para imprimir
- **Retorno**: `null`
- **Uso**: Saída contínua, barras de progresso
- **Exemplo**:
```dryad
for (let i = 1; i <= 5; i++) {
    native_print("[" + i + "] ");
}
// Saída: [1] [2] [3] [4] [5] 
```

---

```dryad
native_println(data);
```
**Descrição**: Imprime dados no console com quebra de linha.
- **Parâmetros**:
  - `data`: `any` - Dados para imprimir
- **Retorno**: `null`
- **Uso**: Saída de linhas completas, logs
- **Exemplo**:
```dryad
native_println("Primeira linha");
native_println("Segunda linha");
// Saída:
// Primeira linha
// Segunda linha
```

---

```dryad
native_write_stdout(bytes);
```
**Descrição**: Escreve bytes diretamente no stdout.
- **Parâmetros**:
  - `bytes`: `string` - Dados binários como string
- **Retorno**: `null`
- **Uso**: Saída binária, controle de baixo nível
- **Exemplo**:
```dryad
// Escrever códigos de escape ANSI diretamente
native_write_stdout("\x1b[31mTexto vermelho\x1b[0m");
```

---

```dryad
native_flush();
```
**Descrição**: Força o esvaziamento do buffer de saída.
- **Parâmetros**: Nenhum
- **Retorno**: `null`
- **Uso**: Garantir saída imediata, animações em tempo real
- **Exemplo**:
```dryad
for (let i = 0; i < 10; i++) {
    native_print(".");
    native_flush(); // Garante que o ponto apareça imediatamente
    native_sleep(500);
}
```


---

## 🎨 Terminal ANSI `#<terminal_ansi>`

**Descrição**: Controle avançado de terminal usando sequências de escape ANSI para manipulação de cursor, cores e estilos.

**Casos de Uso**: Interfaces de usuário em terminal, jogos de console, editores de texto, dashboards.

**Compatibilidade**: Funciona em terminais que suportam ANSI (Linux, macOS, Windows 10+).

### Controle de Tela e Cursor

```dryad
native_clear_screen();
```
**Descrição**: Limpa completamente a tela do terminal e move o cursor para o início.
- **Parâmetros**: Nenhum
- **Retorno**: `null`
- **Código ANSI**: `\x1b[2J\x1b[H`
- **Exemplo**:
```dryad
native_clear_screen();
native_println("Tela limpa!");
```

---

```dryad
native_move_cursor(x, y);
```
**Descrição**: Move o cursor para coordenadas específicas na tela.
- **Parâmetros**:
  - `x`: `number` - Coluna (0-baseado)
  - `y`: `number` - Linha (0-baseado)
- **Retorno**: `null`
- **Observação**: Coordenadas começam em (0,0) no canto superior esquerdo
- **Exemplo**:
```dryad
native_move_cursor(10, 5);  // Coluna 10, Linha 5
native_print("Texto posicionado!");
```

---

```dryad
native_hide_cursor();
native_show_cursor();
```
**Descrição**: Oculta ou mostra o cursor do terminal.
- **Parâmetros**: Nenhum
- **Retorno**: `null`
- **Uso**: Animações, interfaces que não precisam do cursor
- **Exemplo**:
```dryad
native_hide_cursor();
// Fazer animação...
native_show_cursor();
```

### Controle de Cores

```dryad
native_set_color(fg, bg);
```
**Descrição**: Define cores do texto (foreground) e fundo (background).
- **Parâmetros**:
  - `fg`: `string` - Cor do texto
  - `bg`: `string` - Cor do fundo
- **Retorno**: `null`
- **Cores Suportadas**: 
  - Nomes: `"black"`, `"red"`, `"green"`, `"yellow"`, `"blue"`, `"magenta"`, `"cyan"`, `"white"`
  - Códigos RGB: `"#FF0000"`, `"#00FF00"`, `"#0000FF"`
  - Códigos 256: `"1"`, `"196"`, `"46"`
- **Exemplo**:
```dryad
native_set_color("red", "black");
native_println("Texto vermelho em fundo preto");

native_set_color("#00FF00", "#000080");
native_println("Verde em azul escuro");
```

### Controle de Estilos

```dryad
native_set_style(style);
```
**Descrição**: Aplica estilos de formatação ao texto.
- **Parâmetros**:
  - `style`: `string` - Estilo a aplicar
- **Retorno**: `null`
- **Estilos Disponíveis**:
  - `"bold"` - Negrito
  - `"italic"` - Itálico
  - `"underline"` - Sublinhado
  - `"blink"` - Piscante
  - `"reverse"` - Cores invertidas
  - `"strikethrough"` - Riscado
- **Exemplo**:
```dryad
native_set_style("bold");
native_println("Texto em negrito");

native_set_style("underline");
native_println("Texto sublinhado");
```

---

```dryad
native_reset_style();
```
**Descrição**: Remove todos os estilos e cores, voltando ao padrão do terminal.
- **Parâmetros**: Nenhum
- **Retorno**: `null`
- **Código ANSI**: `\x1b[0m`
- **Exemplo**:
```dryad
native_set_color("red", "yellow");
native_set_style("bold");
native_println("Texto estilizado");
native_reset_style();
native_println("Texto normal");
```

### Informações do Terminal

```dryad
native_terminal_size();
```
**Descrição**: Retorna as dimensões atuais do terminal.
- **Parâmetros**: Nenhum
- **Retorno**: `object` com propriedades:
  - `width`: `number` - Largura em colunas
  - `height`: `number` - Altura em linhas
- **Exemplo**:
```dryad
let tamanho = native_terminal_size();
native_println("Terminal: " + tamanho.width + "x" + tamanho.height);
```

### Função de Conveniência

```dryad
ansi_red(texto);
```
**Descrição**: Retorna texto formatado em vermelho.
- **Parâmetros**:
  - `texto`: `string` - Texto para colorir
- **Retorno**: `string` - Texto com códigos ANSI
- **Exemplo**:
```dryad
let aviso = ansi_red("ERRO: Operação falhada!");
native_println(aviso);
```

### Exemplo Completo: Interface Colorida

```dryad
#<terminal_ansi>
#<console_io>

// Limpar tela e preparar interface
native_clear_screen();
native_hide_cursor();

// Título
native_move_cursor(20, 2);
native_set_color("cyan", "black");
native_set_style("bold");
native_println("=== SISTEMA DRYAD ===");
native_reset_style();

// Menu
native_move_cursor(15, 5);
native_set_color("green", "black");
native_println("1. Nova Tarefa");

native_move_cursor(15, 6);
native_set_color("yellow", "black");
native_println("2. Ver Tarefas");

native_move_cursor(15, 7);
native_set_color("red", "black");
native_println("3. Sair");

// Barra de status
let tamanho = native_terminal_size();
native_move_cursor(0, tamanho.height - 1);
native_set_color("white", "blue");
native_print(" Status: Sistema Online ");

// Restaurar cursor
native_reset_style();
native_show_cursor();
native_move_cursor(15, 9);
native_print("Escolha uma opção: ");
```

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

---

## 💾 Binary I/O `#<binary_io>`

**Descrição**: Operações de entrada e saída binárias para manipulação de arquivos em nível de bytes.

**Casos de Uso**: Processamento de imagens, arquivos compactados, protocolos binários, análise forense.

### Escrita de Dados Binários

```dryad
native_write_bytes(path, bytes);
```
**Descrição**: Escreve um array de bytes diretamente em um arquivo.
- **Parâmetros**:
  - `path`: `string` - Caminho do arquivo
  - `bytes`: `array` - Array de números (0-255) ou string
- **Retorno**: `null`
- **Comportamento**: Sobrescreve o arquivo se existir
- **Exemplo**:
```dryad
// Criar arquivo binário simples
let dados = [0x48, 0x65, 0x6C, 0x6C, 0x6F]; // "Hello" em ASCII
native_write_bytes("arquivo.bin", dados);
```

```dryad
native_append_bytes(path, bytes);
```
**Descrição**: Adiciona bytes ao final de um arquivo existente.
- **Parâmetros**:
  - `path`: `string` - Caminho do arquivo
  - `bytes`: `array` - Dados para adicionar
- **Retorno**: `null`
- **Exemplo**:
```dryad
let timestamp = [0x60, 0x9F, 0x4A, 0x12];
native_append_bytes("log.bin", timestamp);
```

```dryad
native_overwrite_chunk(path, offset, bytes);
```
**Descrição**: Sobrescreve uma porção específica de um arquivo.
- **Parâmetros**:
  - `path`: `string` - Caminho do arquivo
  - `offset`: `number` - Posição inicial (em bytes)
  - `bytes`: `array` - Dados para escrever
- **Retorno**: `null`
- **Exemplo**:
```dryad
native_overwrite_chunk("dados.bin", 100, [0xFF]);
```

### Leitura de Dados Binários

```dryad
native_read_bytes(path);
```
**Descrição**: Lê todo o conteúdo de um arquivo como array de bytes.
- **Parâmetros**:
  - `path`: `string` - Caminho do arquivo
- **Retorno**: `array` - Array de números (0-255)
- **Exemplo**:
```dryad
let dados = native_read_bytes("imagem.jpg");
print("Arquivo tem " + dados.length + " bytes");
```

```dryad
native_read_chunk(path, offset, size);
```
**Descrição**: Lê uma porção específica de um arquivo.
- **Parâmetros**:
  - `path`: `string` - Caminho do arquivo
  - `offset`: `number` - Posição inicial
  - `size`: `number` - Número de bytes para ler
- **Retorno**: `array` - Bytes lidos
- **Exemplo**:
```dryad
// Ler header JPEG (primeiros 10 bytes)
let header = native_read_chunk("foto.jpg", 0, 10);
if (header[0] == 0xFF && header[1] == 0xD8) {
    print("É um arquivo JPEG válido!");
}
```

### Utilitários

```dryad
native_file_size(path);
```
**Descrição**: Retorna o tamanho de um arquivo em bytes.
- **Parâmetros**: `path`: `string` - Caminho do arquivo
- **Retorno**: `number` - Tamanho em bytes

```dryad
to_hex(bytes);
```
**Descrição**: Converte array de bytes para representação hexadecimal.
- **Parâmetros**: `bytes`: `array` - Array de números (0-255)
- **Retorno**: `string` - Representação hexadecimal

---

## 📁 File I/O `#<file_io>`

**Descrição**: Operações completas de manipulação de arquivos e diretórios do sistema de arquivos.

**Casos de Uso**: Gerenciamento de arquivos, processamento de logs, backup de dados, organização de documentos.

### Leitura e Escrita de Arquivos

```dryad
native_read_file(path);
```
**Descrição**: Lê o conteúdo completo de um arquivo como string.
- **Parâmetros**: `path`: `string` - Caminho do arquivo
- **Retorno**: `string` - Conteúdo do arquivo
- **Codificação**: UTF-8
- **Exemplo**:
```dryad
let conteudo = native_read_file("config.txt");
print("Arquivo contém: " + conteudo.length + " caracteres");
```

```dryad
native_write_file(path, data);
```
**Descrição**: Escreve dados em um arquivo, substituindo o conteúdo existente.
- **Parâmetros**:
  - `path`: `string` - Caminho do arquivo
  - `data`: `string` - Dados para escrever
- **Retorno**: `null`
- **Exemplo**:
```dryad
let config = "port=8080\nhost=localhost";
native_write_file("server.conf", config);
```

```dryad
native_append_file(path, data);
```
**Descrição**: Adiciona dados ao final de um arquivo existente.
- **Parâmetros**:
  - `path`: `string` - Caminho do arquivo
  - `data`: `string` - Dados para adicionar
- **Retorno**: `null`
- **Exemplo**:
```dryad
let timestamp = native_date() + " " + native_time();
native_append_file("log.txt", timestamp + " - Sistema iniciado\n");
```

### Gerenciamento de Arquivos

```dryad
native_delete_file(path);
```
**Descrição**: Remove um arquivo do sistema.
- **Parâmetros**: `path`: `string` - Caminho do arquivo
- **Retorno**: `null`
- **Exemplo**:
```dryad
if (native_file_exists("temp.txt")) {
    native_delete_file("temp.txt");
    print("Arquivo temporário removido");
}
```

```dryad
native_copy_file(from, to);
```
**Descrição**: Copia um arquivo para outro local.
- **Parâmetros**:
  - `from`: `string` - Arquivo origem
  - `to`: `string` - Arquivo destino
- **Retorno**: `null`
- **Exemplo**:
```dryad
native_copy_file("original.txt", "backup/original_backup.txt");
```

```dryad
native_move_file(from, to);
```
**Descrição**: Move ou renomeia um arquivo.
- **Parâmetros**:
  - `from`: `string` - Arquivo origem
  - `to`: `string` - Novo local/nome
- **Retorno**: `null`
- **Exemplo**:
```dryad
native_move_file("temp.txt", "processed/final.txt");
```

### Verificações e Informações

```dryad
native_file_exists(path);
```
**Descrição**: Verifica se um arquivo existe.
- **Parâmetros**: `path`: `string` - Caminho do arquivo
- **Retorno**: `boolean` - true se existe, false caso contrário

```dryad
native_is_dir(path);
```
**Descrição**: Verifica se um caminho é um diretório.
- **Parâmetros**: `path`: `string` - Caminho para verificar
- **Retorno**: `boolean` - true se for diretório

```dryad
native_get_file_info(path);
```
**Descrição**: Obtém informações detalhadas sobre um arquivo.
- **Parâmetros**: `path`: `string` - Caminho do arquivo
- **Retorno**: `object` com propriedades:
  - `size`: `number` - Tamanho em bytes
  - `modified`: `string` - Data de modificação
  - `created`: `string` - Data de criação
  - `is_dir`: `boolean` - Se é diretório
  - `permissions`: `string` - Permissões do arquivo

### Gerenciamento de Diretórios

```dryad
native_list_dir(path);
```
**Descrição**: Lista arquivos e pastas em um diretório.
- **Parâmetros**: `path`: `string` - Caminho do diretório
- **Retorno**: `array` - Lista de nomes de arquivos/pastas
- **Exemplo**:
```dryad
let arquivos = native_list_dir("./documents");
for (arquivo in arquivos) {
    print("Encontrado: " + arquivo);
}
```

```dryad
native_mkdir(path);
```
**Descrição**: Cria um novo diretório.
- **Parâmetros**: `path`: `string` - Caminho do novo diretório
- **Retorno**: `null`
- **Comportamento**: Cria diretórios pais se necessário

```dryad
native_getcwd();
```
**Descrição**: Retorna o diretório de trabalho atual.
- **Parâmetros**: Nenhum
- **Retorno**: `string` - Caminho absoluto do diretório atual

```dryad
native_setcwd(path);
```
**Descrição**: Altera o diretório de trabalho atual.
- **Parâmetros**: `path`: `string` - Novo diretório de trabalho
- **Retorno**: `null`

---

## ⏰ Time `#<time>`

**Descrição**: Funções para manipulação de tempo, datas e temporização.

**Casos de Uso**: Timestamps, logs com data/hora, temporizações, agenda de tarefas, medição de performance.

### Obtenção de Timestamps

```dryad
native_now();
```
**Descrição**: Retorna timestamp atual em milissegundos desde epoch.
- **Parâmetros**: Nenhum
- **Retorno**: `number` - Timestamp em milissegundos
- **Uso**: Medições de tempo de alta precisão
- **Exemplo**:
```dryad
let inicio = native_now();
// operações...
let fim = native_now();
let duracao = fim - inicio;
print("Operação levou: " + duracao + "ms");
```

```dryad
native_timestamp();
```
**Descrição**: Retorna timestamp Unix em segundos desde epoch.
- **Parâmetros**: Nenhum
- **Retorno**: `number` - Timestamp em segundos
- **Uso**: Compatibilidade com sistemas Unix
- **Exemplo**:
```dryad
let ts = native_timestamp();
print("Timestamp Unix: " + ts);
```

### Data e Hora Formatadas

```dryad
native_date();
```
**Descrição**: Retorna a data atual no formato ISO.
- **Parâmetros**: Nenhum
- **Retorno**: `string` - Data no formato "YYYY-MM-DD"
- **Exemplo**:
```dryad
let hoje = native_date();
print("Data atual: " + hoje); // ex: "2025-07-11"
```

```dryad
native_time();
```
**Descrição**: Retorna a hora atual no formato 24h.
- **Parâmetros**: Nenhum
- **Retorno**: `string` - Hora no formato "HH:MM:SS"
- **Exemplo**:
```dryad
let agora = native_time();
print("Hora atual: " + agora); // ex: "13:37:42"
```

```dryad
native_format_date(format);
```
**Descrição**: Formata a data atual com formato customizado.
- **Parâmetros**: `format`: `string` - Padrão de formatação
- **Retorno**: `string` - Data formatada
- **Formato**: Use códigos como %Y (ano), %m (mês), %d (dia), %H (hora)
- **Exemplo**:
```dryad
let custom = native_format_date("%d/%m/%Y %H:%M");
print(custom); // "11/07/2025 13:37"
```

### Temporização e Performance

```dryad
native_sleep(milliseconds);
```
**Descrição**: Pausa a execução por um tempo determinado.
- **Parâmetros**: `milliseconds`: `number` - Tempo em milissegundos
- **Retorno**: `null`
- **Comportamento**: Thread atual fica bloqueada
- **Exemplo**:
```dryad
print("Iniciando contagem...");
for (let i = 3; i > 0; i--) {
    print(i);
    native_sleep(1000);
}
print("Go!");
```

```dryad
native_uptime();
```
**Descrição**: Tempo decorrido desde o início da execução.
- **Parâmetros**: Nenhum
- **Retorno**: `number` - Tempo em milissegundos
- **Uso**: Medição de tempo total de execução
- **Exemplo**:
```dryad
let tempo_execucao = native_uptime();
print("Programa rodando há: " + tempo_execucao + "ms");
```

---

## 💻 System Environment `#<system_env>`

**Descrição**: Interação com o sistema operacional, variáveis de ambiente e execução de comandos.

**Casos de Uso**: Scripts de sistema, configuração, automação, deploys, integração com ferramentas externas.

### Informações do Sistema

```dryad
native_platform();
```
**Descrição**: Identifica o sistema operacional atual.
- **Parâmetros**: Nenhum
- **Retorno**: `string` - "linux", "windows", "macos", "freebsd"
- **Uso**: Lógica condicional por plataforma
- **Exemplo**:
```dryad
let os = native_platform();
if (os == "windows") {
    print("Executando no Windows");
    // lógica específica do Windows
} else if (os == "linux") {
    print("Executando no Linux");
    // lógica específica do Linux
}
```

```dryad
native_arch();
```
**Descrição**: Retorna a arquitetura do processador.
- **Parâmetros**: Nenhum
- **Retorno**: `string` - "x86_64", "aarch64", "arm", "i386"
- **Uso**: Compatibilidade com diferentes arquiteturas
- **Exemplo**:
```dryad
let arch = native_arch();
print("Arquitetura: " + arch);
```

### Variáveis de Ambiente

```dryad
native_env(key);
```
**Descrição**: Obtém o valor de uma variável de ambiente.
- **Parâmetros**: `key`: `string` - Nome da variável
- **Retorno**: `string | null` - Valor da variável ou null se não existir
- **Exemplo**:
```dryad
let path = native_env("PATH");
if (path != null) {
    print("PATH configurado: " + path);
} else {
    print("PATH não encontrado");
}

let home = native_env("HOME"); // Linux/Mac
let userprofile = native_env("USERPROFILE"); // Windows
```

```dryad
native_set_env(key, value);
```
**Descrição**: Define ou modifica uma variável de ambiente.
- **Parâmetros**:
  - `key`: `string` - Nome da variável
  - `value`: `string` - Valor a definir
- **Retorno**: `null`
- **Escopo**: Apenas para o processo atual e filhos
- **Exemplo**:
```dryad
native_set_env("DATABASE_URL", "sqlite:app.db");
native_set_env("LOG_LEVEL", "debug");
print("Variáveis configuradas");
```

### Execução de Comandos

```dryad
native_exec(command);
```
**Descrição**: Executa um comando no shell do sistema.
- **Parâmetros**: `command`: `string` - Comando para executar
- **Retorno**: `number` - Código de saída (0 = sucesso)
- **Comportamento**: Execução síncrona (bloqueia até terminar)
- **Saída**: Imprime diretamente no console
- **Exemplo**:
```dryad
// Listar arquivos
let resultado = native_exec("ls -la"); // Linux/Mac
if (resultado == 0) {
    print("Comando executado com sucesso");
} else {
    print("Erro na execução: código " + resultado);
}

// Comandos específicos por plataforma
let os = native_platform();
if (os == "windows") {
    native_exec("dir");
} else {
    native_exec("ls");
}
```

### Exemplos Avançados

**Script de Deploy Multiplataforma**:
```dryad
let os = native_platform();
let arch = native_arch();

print("Deploy para " + os + " " + arch);

// Configurações por ambiente
if (native_env("PRODUCTION") != null) {
    native_set_env("LOG_LEVEL", "error");
    print("Modo produção ativado");
} else {
    native_set_env("LOG_LEVEL", "debug");
    print("Modo desenvolvimento");
}

// Execução condicional
if (os == "linux") {
    native_exec("sudo systemctl restart myapp");
} else if (os == "windows") {
    native_exec("sc stop myapp && sc start myapp");
}
```

native_exec_output(command);
```
**Descrição**: Executa comando e retorna sua saída.
- **Parâmetros**: `command`: `string` - Comando para executar
- **Retorno**: `string` - Saída padrão do comando
- **Uso**: Captura da saída de comandos

```dryad
native_pid();
```
**Descrição**: Retorna o ID do processo atual.
- **Parâmetros**: Nenhum
- **Retorno**: `number` - Process ID

```dryad
native_exit(code);
```
**Descrição**: Encerra o programa com código de saída.
- **Parâmetros**: `code`: `number` - Código de saída (0 = sucesso)
- **Retorno**: Nunca retorna (encerra programa)

---

## 📝 Encode/Decode `#<encode_decode>`

**Descrição**: Codificação e decodificação de formatos de dados estruturados.

**Casos de Uso**: APIs JSON, processamento de CSV, configurações XML, interchange de dados, persistência.

### JSON (JavaScript Object Notation)

```dryad
native_json_encode(object);
```
**Descrição**: Converte objeto Dryad para string JSON.
- **Parâmetros**: `object`: `object|array` - Estrutura de dados para serializar
- **Retorno**: `string` - Representação JSON
- **Suporte**: Objetos, arrays, strings, números, booleanos, null
- **Exemplo**:
```dryad
let dados = {
    "nome": "João",
    "idade": 30,
    "ativo": true,
    "hobbies": ["leitura", "programação"]
};
let json = native_json_encode(dados);
print(json); // {"nome":"João","idade":30,...}
```

```dryad
native_json_decode(json_string);
```
**Descrição**: Converte string JSON para objeto Dryad.
- **Parâmetros**: `json_string`: `string` - JSON válido
- **Retorno**: `object|array` - Estrutura de dados deserializada
- **Tratamento de Erro**: Retorna null para JSON inválido
- **Exemplo**:
```dryad
let json = '{"status":"ok","count":42}';
let obj = native_json_decode(json);
if (obj != null) {
    print("Status: " + obj.status);
    print("Count: " + obj.count);
}
```

### CSV (Comma-Separated Values)

```dryad
native_csv_encode(data);
```
**Descrição**: Converte array bidimensional para CSV.
- **Parâmetros**: `data`: `array` - Array de arrays ou objetos
- **Retorno**: `string` - Formato CSV com cabeçalho
- **Exemplo**:
```dryad
let dados = [
    ["Nome", "Idade", "Cidade"],
    ["Ana", "25", "São Paulo"],
    ["Carlos", "32", "Rio de Janeiro"]
];
let csv = native_csv_encode(dados);
print(csv);
// Nome,Idade,Cidade
// Ana,25,São Paulo
// Carlos,32,Rio de Janeiro
```

```dryad
native_csv_decode(csv_string);
```
**Descrição**: Converte string CSV para array bidimensional.
- **Parâmetros**: `csv_string`: `string` - Dados CSV
- **Retorno**: `array` - Array de arrays com dados
- **Exemplo**:
```dryad
let csv = "nome,idade\nJoão,30\nMaria,28";
let dados = native_csv_decode(csv);
for (linha in dados) {
    print("Linha: " + native_json_encode(linha));
}
```

### XML (eXtensible Markup Language)

```dryad
native_xml_encode(object);
```
**Descrição**: Converte objeto Dryad para XML.
- **Parâmetros**: `object`: `object` - Estrutura de dados
- **Retorno**: `string` - Documento XML
- **Formato**: Elementos aninhados baseados na estrutura do objeto
- **Exemplo**:
```dryad
let config = {
    "servidor": {
        "porta": 8080,
        "host": "localhost"
    }
};
let xml = native_xml_encode(config);
print(xml);
```

```dryad
native_xml_decode(xml_string);
```
**Descrição**: Converte string XML para objeto Dryad.
- **Parâmetros**: `xml_string`: `string` - Documento XML válido
- **Retorno**: `object` - Estrutura de dados deserializada
- **Exemplo**:
```dryad
let xml = "<config><port>8080</port></config>";
let obj = native_xml_decode(xml);
print("Porta: " + obj.config.port);
```

---

## � Crypto `#<crypto>`

**Descrição**: Funções criptográficas, hashing e geração de identificadores seguros.

**Casos de Uso**: Autenticação, integridade de dados, tokens seguros, senhas, certificates.

### Hashing Criptográfico

```dryad
native_hash_sha256(data);
```
**Descrição**: Calcula hash SHA-256 (mais seguro).
- **Parâmetros**: `data`: `string|array` - Dados para hash
- **Retorno**: `string` - Hash hexadecimal (64 caracteres)
- **Uso**: Senhas, integridade de arquivos, assinaturas digitais
- **Exemplo**:
```dryad
let senha = "minhasenha123";
let hash = native_hash_sha256(senha);
print("SHA-256: " + hash);
// SHA-256: a665a45920422f9d417e4867efdc4fb8a04a1f3fff1fa07e998e86f7f7a27ae3

// Verificação de integridade
let arquivo = native_read_file("important.txt");
let checksum = native_hash_sha256(arquivo);
print("Checksum: " + checksum);
```

```dryad
native_hash_md5(data);
```
**Descrição**: Calcula hash MD5 (legado, menos seguro).
- **Parâmetros**: `data`: `string|array` - Dados para hash
- **Retorno**: `string` - Hash hexadecimal (32 caracteres)
- **Uso**: Compatibilidade legada, checksums rápidos
- **Aviso**: Não use para segurança crítica
- **Exemplo**:
```dryad
let data = "Hello World";
let md5 = native_hash_md5(data);
print("MD5: " + md5); // b10a8db164e0754105b7a99be72e3fe5
```

### Identificação Única

```dryad
native_uuid();
```
**Descrição**: Gera UUID v4 (universalmente único).
- **Parâmetros**: Nenhum
- **Retorno**: `string` - UUID no formato padrão
- **Formato**: "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx"
- **Uso**: IDs de entidades, sessões, tokens
- **Exemplo**:
```dryad
let id = native_uuid();
print("ID gerado: " + id);
// ID gerado: 550e8400-e29b-41d4-a716-446655440000

// Sistema de sessões
let session_id = native_uuid();
native_set_env("SESSION_ID", session_id);
```

### Codificação Base64

```dryad
native_base64_encode(data);
```
**Descrição**: Codifica dados em Base64.
- **Parâmetros**: `data`: `string|array` - Dados para codificar
- **Retorno**: `string` - String Base64
- **Uso**: Transmissão segura, embedding de dados
- **Exemplo**:
```dryad
let texto = "Olá, mundo!";
let encoded = native_base64_encode(texto);
print("Base64: " + encoded); // T2zDoSwgbXVuZG8h

// Para dados binários
let bytes = [72, 101, 108, 108, 111]; // "Hello"
let b64 = native_base64_encode(bytes);
print("Bytes em B64: " + b64);
```

```dryad
native_base64_decode(encoded_data);
```
**Descrição**: Decodifica string Base64.
- **Parâmetros**: `encoded_data`: `string` - String Base64
- **Retorno**: `string|array` - Dados decodificados
- **Exemplo**:
```dryad
let encoded = "T2zDoSwgbXVuZG8h";
let decoded = native_base64_decode(encoded);
print("Decodificado: " + decoded); // Olá, mundo!
```

### Codificação Hexadecimal

```dryad
native_hex_encode(data);
```
**Descrição**: Converte dados para hexadecimal.
- **Parâmetros**: `data`: `string|array` - Dados para converter
- **Retorno**: `string` - Representação hexadecimal
- **Exemplo**:
```dryad
let texto = "ABC";
let hex = native_hex_encode(texto);
print("Hex: " + hex); // 414243
```

```dryad
native_hex_decode(hex_string);
```
**Descrição**: Converte hexadecimal para dados originais.
- **Parâmetros**: `hex_string`: `string` - String hexadecimal
- **Retorno**: `string|array` - Dados decodificados

### Geração de Dados Aleatórios

```dryad
native_random_bytes(length);
```
**Descrição**: Gera bytes aleatórios criptograficamente seguros.
- **Parâmetros**: `length`: `number` - Número de bytes
- **Retorno**: `array` - Array de bytes (0-255)
- **Uso**: Chaves, salts, tokens seguros
- **Exemplo**:
```dryad
let salt = native_random_bytes(16);
print("Salt gerado: " + native_hex_encode(salt));
```

```dryad
native_random_string(length, charset);
```
**Descrição**: Gera string aleatória com caracteres específicos.
- **Parâmetros**:
  - `length`: `number` - Tamanho da string
  - `charset`: `string` - Caracteres permitidos (opcional)
- **Retorno**: `string` - String aleatória
- **Exemplo**:
```dryad
// Token alfanumérico
let token = native_random_string(32, "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789");
print("Token: " + token);

// Senha forte
let password = native_random_string(16, "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*");
print("Senha: " + password);
```

### Criptografia Simétrica (AES)

```dryad
native_encrypt_aes(data, key);
```
**Descrição**: Criptografa dados com AES.
- **Parâmetros**:
  - `data`: `array` - Dados binários para criptografar
  - `key`: `string` - Chave de criptografia
- **Retorno**: `array` - Dados criptografados
- **Uso**: Armazenamento seguro, comunicação privada

```dryad
native_decrypt_aes(encrypted_data, key);
```
**Descrição**: Descriptografa dados AES.
- **Parâmetros**:
  - `encrypted_data`: `array` - Dados criptografados
  - `key`: `string` - Mesma chave usada na criptografia
- **Retorno**: `array` - Dados originais

### Criptografia Assimétrica (RSA)

```dryad
native_encrypt_rsa(data, public_key);
```
**Descrição**: Criptografa com chave pública RSA.
- **Parâmetros**:
  - `data`: `array` - Dados para criptografar
  - `public_key`: `string` - Chave pública RSA
- **Retorno**: `array` - Dados criptografados
- **Uso**: Troca segura de chaves, assinatura digital

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


🧪 Debug e Diagnóstico e Testes #<debug> (implementado)

native_log(value);              // imprime valor bruto (sem print formatado)
/*
Imprime o valor bruto no console, útil para depuração.
Entrada: qualquer tipo de dado (string, número, objeto, etc.).
retorna: nenhum
*/

native_typeof(value);           // tipo como string
/*
Retorna o tipo de dado de uma variável como uma string.
Entrada: qualquer tipo de dado (string, número, objeto, etc.).
retorna: uma string representando o tipo (ex: "string", "number", "object").
*/

native_memory_usage();          // bytes usados
/*
Retorna a quantidade de memória usada pelo programa em bytes.
Entrada: nenhum
retorna: um número inteiro representando a memória usada.
*/

native_stack_trace();           // stack trace atual
/*
Retorna o stack trace atual do programa.
Entrada: nenhum
retorna: uma string representando o stack trace.
*/

native_perf_start(name);        // inicia timer customizado
/*
Inicia um timer para medir o desempenho de uma seção do código.
Entrada: uma string representando o nome do timer.
retorna: nenhum
*/

native_perf_end(name);          // encerra e mostra tempo decorrido
/*
Encerra o timer iniciado com `native_perf_start` e imprime o tempo decorrido.
Entrada: uma string representando o nome do timer.
retorna: nenhum
*/

native_assert(condition, message); // verifica condição
/*
Verifica uma condição e lança um erro se for falsa.
Entrada: uma expressão booleana (condition) e uma string (message) para o erro.
retorna: nenhum
*/

native_assert_equal(actual, expected, message); // compara valores
/*
Compara dois valores e lança um erro se forem diferentes.
Entrada: dois valores (actual e expected) e uma string (message) para o erro.
retorna: nenhum
*/

native_assert_not_equal(actual, expected, message); // compara valores diferentes
/*
Compara dois valores e lança um erro se forem iguais.
Entrada: dois valores (actual e expected) e uma string (message) para o erro.
retorna: nenhum
*/

native_assert_true(value, message); // verifica se é verdadeiro
/*
Verifica se um valor é verdadeiro e lança um erro se não for.
Entrada: um valor (value) e uma string (message) para o erro.
retorna: nenhum
*/

native_assert_false(value, message); // verifica se é falso
/*
Verifica se um valor é falso e lança um erro se não for.
Entrada: um valor (value) e uma string (message) para o erro.
retorna: nenhum
*/

native_assert_type(value, expected_type, message); // verifica tipo
/*
Verifica se o tipo de um valor corresponde ao tipo esperado e lança um erro se não corresponder.
Entrada: um valor (value), uma string representando o tipo esperado (expected_type) e uma string (message) para o erro.
retorna: nenhum
*/

native_test_regex(pattern, string, message); // testa regex
/*
Testa uma expressão regular em uma string e lança um erro se não corresponder.
Entrada: uma string representando o padrão da regex (pattern), uma string (string) para testar e uma string (message) para o erro.
retorna: nenhum
*/

🧬 Outros Interessantes / Experimentais #<utils> (implementado)

native_eval(code);              // executa código Dryad dinâmico
/*
Executa um código Dryad dinâmico passado como string.
Entrada: uma string contendo o código Dryad a ser executado.
retorna: o resultado da execução do código.
*/

native_clone(obj);              // cópia profunda de objeto
/*
Cria uma cópia profunda de um objeto Dryad.
Entrada: um objeto Dryad.
retorna: uma nova instância do objeto com os mesmos dados.
*/

native_watch_file(path);        // observa mudanças em tempo real
/*
Observa um arquivo para mudanças em tempo real e executa uma função de callback quando o arquivo é modificado.
Entrada: um caminho de arquivo (string) e uma função de callback que será chamada com o novo conteúdo do arquivo.
retorna: um ID de observação que pode ser usado para parar a observação.
*/

native_random_int(min, max);    // inteiro aleatório
/*
Gera um número inteiro aleatório entre os valores mínimo e máximo especificados.
Entrada: dois números inteiros representando o mínimo e o máximo.
retorna: um número inteiro aleatório entre o mínimo e o máximo.
*/

native_random_float(min, max);  // float aleatório
/*
Gera um número de ponto flutuante aleatório entre os valores mínimo e máximo especificados.
Entrada: dois números representando o mínimo e o máximo.
retorna: um número de ponto flutuante aleatório entre o mínimo e o máximo.
*/

native_random_string(length, charset);   // string aleatória
/*
Gera uma string aleatória de um determinado comprimento usando um conjunto de caracteres especificado.
Entrada: um número inteiro representando o comprimento da string e uma string com os caracteres permitidos.
retorna: uma string aleatória gerada a partir do conjunto de caracteres.
*/

native_random_bytes(length);    // bytes aleatórios
/*
Gera um array de bytes aleatórios de um determinado comprimento.
Entrada: um número inteiro representando o comprimento do array.
retorna: um array de bytes aleatórios.
*/

native_random_seed(seed);       // semente para gerador aleatório
/*
Define uma semente para o gerador de números aleatórios.
Entrada: um valor que pode ser um número inteiro ou uma string.
retorna: nenhum
*/

native_regex_match(pattern, string); // verifica correspondência de regex
/*
Verifica se uma expressão regular corresponde a uma string e retorna os grupos capturados.
Entrada: uma string representando o padrão da regex (pattern) e uma string (string) para testar.
retorna: um array com os grupos capturados ou null se não houver correspondência.
*/

native_regex_replace(pattern, replacement, string); // substitui regex
/*
Substitui todas as ocorrências de uma expressão regular em uma string por um valor de substituição.
Entrada: uma string representando o padrão da regex (pattern), uma string de substituição (
replacement) e uma string (string) para testar.
retorna: uma nova string com as substituições feitas.
*/

native_regex_split(pattern, string); // divide string por regex
/*
Divide uma string em um array usando uma expressão regular como delimitador.
Entrada: uma string representando o padrão da regex (pattern) e uma string (string)
para dividir.
retorna: um array de strings resultantes da divisão.
*/

native_regex_test(pattern, string); // testa regex sem captura
/*
Testa se uma expressão regular corresponde a uma string sem capturar grupos.
Entrada: uma string representando o padrão da regex (pattern) e uma string (string)
para testar.
retorna: um booleano (true se houver correspondência, false caso contrário).
*/

🧭 HTTP (Cliente) #<http> (implementado)

native_http_get(url);                   // GET simples, retorna string
/*
Realiza uma requisição HTTP GET para a URL especificada e retorna o conteúdo como uma string.
Entrada: uma string representando a URL.
retorna: uma string com o conteúdo da resposta.
*/

native_http_post(url, body);            // POST, com string no corpo
/*
Realiza uma requisição HTTP POST para a URL especificada com um corpo de string.
Entrada: uma string representando a URL e uma string com o corpo da requisição.
retorna: uma string com o conteúdo da resposta.
*/

native_http_headers(url);               // retorna headers
/*
Retorna os cabeçalhos HTTP da resposta para a URL especificada.
Entrada: uma string representando a URL.
retorna: um objeto com os cabeçalhos HTTP.
*/

native_http_download(url, path);        // salva conteúdo em arquivo
/*
Realiza uma requisição HTTP GET para a URL especificada e salva o conteúdo em um arquivo.
Entrada: uma string representando a URL e uma string com o caminho do arquivo onde o conteúdo será salvo.
retorna: nenhum
*/

native_http_status(url);                // retorna status HTTP (200, 404...)
/*
Retorna o código de status HTTP da resposta para a URL especificada.
Entrada: uma string representando a URL.
retorna: um número inteiro representando o código de status HTTP.
*/

native_http_json(url);                 // retorna JSON como objeto
/*
Realiza uma requisição HTTP GET para a URL especificada e retorna o conteúdo como um objeto JSON.
Entrada: uma string representando a URL.
retorna: um objeto representando os dados JSON da resposta.
*/

native_http_set_timeout(url, ms);       // define timeout para requisições
/*
Define o tempo limite para requisições HTTP.
Entrada: uma string representando a URL e um número inteiro representando o tempo limite em milissegundos.
retorna: nenhum
*/

native_http_set_headers(url, headers); // define headers customizados
/*
Define cabeçalhos HTTP personalizados para a requisição.
Entrada: uma string representando a URL e um objeto com os cabeçalhos HTTP.
retorna: nenhum
*/

native_http_set_user_agent(url, agent); // define User-Agent customizado
/*
Define o cabeçalho User-Agent para a requisição HTTP.
Entrada: uma string representando a URL e uma string com o User-Agent.
retorna: nenhum
*/

native_http_set_proxy(url, proxy); // define proxy para requisições
/*
Define um proxy para as requisições HTTP.
Entrada: uma string representando a URL e uma string com o endereço do proxy (ex:
"http://proxy.example.com:8080").
retorna: nenhum
*/

native_http_set_auth(url, username, password); // define autenticação básica
/*
Define autenticação básica para a requisição HTTP.
Entrada: uma string representando a URL, uma string com o nome de usuário e uma string com a senha.
retorna: nenhum
*/

native_http_set_follow_redirects(url, enable); // segue redirecionamentos
/*
Ativa ou desativa o seguimento automático de redirecionamentos HTTP.
Entrada: uma string representando a URL e um booleano (true para ativar, false para desativar).
retorna: nenhum
*/

native_http_set_cache(url, enable); // ativa/desativa cache
/*
Ativa ou desativa o cache para as requisições HTTP.
Entrada: uma string representando a URL e um booleano (true para ativar, false para desativar).
retorna: nenhum
*/

native_http_set_compression(url, enable); // ativa/desativa compressão
/*
Ativa ou desativa a compressão de resposta HTTP (ex: gzip).
Entrada: uma string representando a URL e um booleano (true para ativar, false para desativar).
retorna: nenhum
*/

native_http_set_max_redirects(url, count); // define máximo de redirecionamentos
/*
Define o número máximo de redirecionamentos HTTP a seguir.
Entrada: uma string representando a URL e um número inteiro representando o máximo de redirecionamentos.
retorna: nenhum
*/

native_http_set_retry(url, count); // define número de tentativas em falhas
/*
Define o número de tentativas em caso de falha na requisição HTTP.
Entrada: uma string representando a URL e um número inteiro representando o número de tentativas.
retorna: nenhum
*/

native_http_set_cookies(url, cookies); // define cookies para requisição
/*
Define cookies para a requisição HTTP.
Entrada: uma string representando a URL e um objeto com os cookies (nome: valor).
retorna: nenhum
*/

native_http_set_timeout(url, ms); // define timeout para requisições
/*
Define o tempo limite para requisições HTTP.
Entrada: uma string representando a URL e um número inteiro representando o tempo limite em milissegundos.
retorna: nenhum
*/

native_http_set_keepalive(url, enable); // ativa/desativa keepalive
/*
Ativa ou desativa o uso de conexões persistentes (keepalive) para requisições HTTP.
Entrada: uma string representando a URL e um booleano (true para ativar, false para desativar).
retorna: nenhum
*/

native_http_set_reuseaddr(url, enable); // ativa/desativa reuseaddr
/*
Ativa ou desativa o uso de endereços reutilizáveis (reuseaddr) para conexões HTTP.
Entrada: uma string representando a URL e um booleano (true para ativar, false para desativar).
retorna: nenhum
*/

native_http_set_nodelay(url, enable); // desativa Nagle's algorithm
/*
Desativa o algoritmo de Nagle para conexões HTTP, melhorando a latência em conexões de baixa latência.
Entrada: uma string representando a URL e um booleano (true para desativar, false para ativar).
retorna: nenhum
*/

native_http_set_ssl_verify(url, enable); // ativa/desativa verificação SSL
/*
Ativa ou desativa a verificação de certificados SSL para conexões HTTPS.
Entrada: uma string representando a URL e um booleano (true para ativar, false para desativar).
retorna: nenhum
*/

native_http_set_ssl_cert(url, cert_path); // define certificado SSL
/*
Define o caminho para o certificado SSL a ser usado na conexão HTTPS.
Entrada: uma string representando a URL e uma string com o caminho do certificado SSL.
retorna: nenhum
*/

native_http_set_ssl_key(url, key_path); // define chave SSL
/*
Define o caminho para a chave privada SSL a ser usada na conexão HTTPS.
Entrada: uma string representando a URL e uma string com o caminho da chave privada SSL.
retorna: nenhum
*/

native_http_set_ssl_ca(url, ca_path); // define CA SSL
/*
Define o caminho para o certificado da autoridade certificadora (CA) SSL a ser usado na conexão HTTPS.
Entrada: uma string representando a URL e uma string com o caminho do certificado CA SSL.
retorna: nenhum
*/

native_http_set_ssl_sni(url, sni); // define SNI para SSL
/*
Define o nome do servidor virtual (SNI) para conexões SSL/TLS.
Entrada: uma string representando a URL e uma string com o nome do servidor virtual (SNI).
retorna: nenhum
*/

native_http_set_ssl_protocols(url, protocols); // define protocolos SSL permitidos
/*
Define os protocolos SSL/TLS permitidos para a conexão HTTPS.
Entrada: uma string representando a URL e uma string com os protocolos permitidos (ex: "TLSv1.2,TLSv1.3").
retorna: nenhum
*/

native_http_set_ssl_ciphers(url, ciphers); // define cifras SSL permitidas
/*
Define as cifras criptográficas permitidas para a conexão HTTPS.
Entrada: uma string representando a URL e uma string com as cifras permitidas (ex:
"ECDHE-RSA-AES128-GCM-SHA256,ECDHE-RSA-AES256-GCM-SHA384").
retorna: nenhum
*/

native_http_set_ssl_session(url, session); // define sessão SSL
/*
Define uma sessão SSL para reutilização em conexões HTTPS.
Entrada: uma string representando a URL e uma string com os dados da sessão SSL.
retorna: nenhum
*/

📡 WebSocket (Cliente/Servidor) #<websocket>


#### 🌐 UDP (Datagramas) `#<udp>`

```dryad
// ========================
// SERVIDOR UDP
// ========================

udp_server_create(server_id, host?, port?);
/*
Cria uma nova instância de servidor UDP.
Entrada: 
  - server_id: string identificadora do servidor
  - host: string do endereço IP (opcional, padrão: "127.0.0.1")  
  - port: número da porta (opcional, padrão: 8080)
Retorna: null
*/

udp_server_start(server_id);
/*
Inicia o servidor UDP especificado em modo echo.
O servidor responderá com "Echo: <mensagem>" para qualquer datagrama recebido.
Entrada: server_id (string)
Retorna: null
*/

udp_server_stop(server_id);
/*
Para o servidor UDP especificado.
Entrada: server_id (string)
Retorna: null
*/

udp_server_status(server_id);
/*
Retorna o status atual do servidor UDP.
Entrada: server_id (string)
Retorna: objeto com as propriedades:
  - server_id: string
  - host: string  
  - port: número
  - is_running: boolean
*/

// ========================
// CLIENTE UDP  
// ========================

udp_client_create(client_id, host?, port?);
/*
Cria uma nova instância de cliente UDP.
Entrada:
  - client_id: string identificadora do cliente
  - host: string do servidor de destino (opcional, padrão: "127.0.0.1")
  - port: número da porta de destino (opcional, padrão: 8080)
Retorna: null
*/

udp_client_bind(client_id, local_port?);
/*
Vincula o cliente UDP a uma porta local para enviar/receber dados.
Entrada:
  - client_id: string
  - local_port: número da porta local (opcional, 0 = automática)
Retorna: boolean (true se sucesso, false se falha)
*/

udp_client_send(client_id, message);
/*
Envia dados para o servidor configurado no cliente.
Entrada:
  - client_id: string
  - message: string/número/boolean com os dados
Retorna: boolean (true se enviado com sucesso)
*/

udp_client_receive(client_id);
/*
Recebe dados do socket UDP (última mensagem).
Operação com timeout baseado na configuração do cliente.
Entrada: client_id (string)
Retorna: string com dados recebidos (vazia se timeout/erro)
*/

udp_client_send_to(client_id, message, host, port);
/*
Envia dados para um endereço específico (não necessariamente o servidor configurado).
Entrada:
  - client_id: string
  - message: string/número/boolean com os dados
  - host: string do endereço de destino
  - port: número da porta de destino
Retorna: boolean (true se enviado com sucesso)
*/

udp_client_receive_from(client_id);
/*
Recebe dados e informações do remetente.
Entrada: client_id (string)
Retorna: objeto com propriedades:
  - data: string com os dados recebidos
  - sender: string com endereço do remetente (formato "IP:porta")
*/

udp_client_status(client_id);
/*
Retorna o status atual do cliente UDP.
Entrada: client_id (string)
Retorna: objeto com as propriedades:
  - client_id: string
  - host: string
  - port: número
  - timeout_secs: número
  - is_bound: boolean
*/

udp_client_set_timeout(client_id, timeout_secs);
/*
Define o timeout para operações de recepção.
Entrada:
  - client_id: string
  - timeout_secs: número de segundos
Retorna: null
*/

udp_client_close(client_id);
/*
Fecha e remove o cliente UDP.
Entrada: client_id (string)
Retorna: null
*/

// ========================
// UTILITÁRIOS UDP
// ========================

udp_resolve_hostname(hostname);
/*
Resolve um hostname para endereço IP.
Entrada: hostname (string)
Retorna: string com o IP resolvido
*/

udp_get_local_ip();
/*
Retorna o endereço IP local da máquina.
Entrada: nenhum
Retorna: string com o IP local
*/

udp_port_available(port);
/*
Verifica se uma porta está disponível para bind UDP.
Entrada: port (número)
Retorna: boolean (true se disponível, false se ocupada)
*/
```

#### 🌍 TCP (Cliente e Servidor) `#<tcp>`

native_tcp_server_create(server_id, host?, port?, max_clients?); // cria servidor TCP
/*
Cria uma nova instância de servidor TCP.
Entrada: 
  - server_id (string): identificador único do servidor
  - host (string, opcional): endereço IP para bind (padrão: "127.0.0.1")
  - port (number, opcional): porta para bind (padrão: 8080)
  - max_clients (number, opcional): número máximo de clientes simultâneos (padrão: 10)
retorna: nenhum
*/

native_tcp_server_start(server_id); // inicia servidor TCP
/*
Inicia o servidor TCP especificado.
O servidor rodará em uma thread separada e aceitará conexões de clientes.
Entrada: server_id (string): identificador do servidor
retorna: nenhum
*/

native_tcp_server_stop(server_id); // para servidor TCP
/*
Para o servidor TCP especificado.
Entrada: server_id (string): identificador do servidor
retorna: nenhum
*/

native_tcp_server_status(server_id); // obtém status do servidor
/*
Retorna informações sobre o status atual do servidor TCP.
Entrada: server_id (string): identificador do servidor
retorna: object com propriedades:
  - server_id: identificador do servidor
  - host: endereço IP do servidor
  - port: porta do servidor
  - is_running: se o servidor está rodando
  - max_clients: número máximo de clientes
*/

native_tcp_server_set_max_clients(server_id, max_clients); // define máximo de clientes
/*
Define o número máximo de clientes simultâneos para um servidor TCP.
Só pode ser chamado quando o servidor estiver parado.
Entrada: 
  - server_id (string): identificador do servidor
  - max_clients (number): novo número máximo de clientes
retorna: nenhum
*/

native_tcp_client_create(client_id, host, port); // cria cliente TCP
/*
Cria uma nova instância de cliente TCP.
Entrada:
  - client_id (string): identificador único do cliente
  - host (string): endereço IP ou hostname do servidor
  - port (number): porta do servidor
retorna: nenhum
*/

native_tcp_client_connect(client_id); // conecta cliente ao servidor
/*
Estabelece conexão TCP com o servidor especificado.
Entrada: client_id (string): identificador do cliente
retorna: bool (true se conectou com sucesso, false caso contrário)
*/

native_tcp_client_disconnect(client_id); // desconecta cliente
/*
Encerra a conexão TCP do cliente.
Entrada: client_id (string): identificador do cliente
retorna: nenhum
*/

native_tcp_client_send(client_id, data); // envia dados via cliente
/*
Envia dados através da conexão TCP do cliente.
Entrada:
  - client_id (string): identificador do cliente
  - data (string): dados a serem enviados
retorna: bool (true se enviou com sucesso, false caso contrário)
*/

native_tcp_client_receive(client_id); // recebe dados via cliente
/*
Recebe dados através da conexão TCP do cliente.
Esta função é bloqueante e aguardará até receber dados.
Entrada: client_id (string): identificador do cliente
retorna: string com os dados recebidos
*/

native_tcp_client_status(client_id); // obtém status do cliente
/*
Retorna informações sobre o status atual do cliente TCP.
Entrada: client_id (string): identificador do cliente
retorna: object com propriedades:
  - client_id: identificador do cliente
  - host: endereço IP do servidor
  - port: porta do servidor
  - is_connected: se o cliente está conectado
  - timeout_secs: timeout em segundos para operações
*/

native_tcp_client_set_timeout(client_id, timeout_secs); // define timeout do cliente
/*
Define timeout para operações de conexão e I/O do cliente TCP.
Entrada:
  - client_id (string): identificador do cliente
  - timeout_secs (number): timeout em segundos
retorna: nenhum
*/

native_tcp_resolve_hostname(hostname); // resolve hostname para IP
/*
Resolve um hostname para seu endereço IP correspondente.
Entrada: hostname (string): nome do host a ser resolvido
retorna: string com o endereço IP
*/

native_tcp_get_local_ip(); // obtém IP local da máquina
/*
Retorna o endereço IP local da máquina.
Entrada: nenhum
retorna: string com o endereço IP local
*/

native_tcp_port_available(port); // verifica se porta está disponível
/*
Verifica se uma porta específica está disponível para uso.
Entrada: port (number): porta a ser verificada
retorna: bool (true se disponível, false se em uso)
*/

Exemplo de uso TCP:

```dryad
#<tcp>

// === SERVIDOR TCP ===

// Criar servidor
tcp_server_create("meu_servidor", "0.0.0.0", 8080, 20);

// Configurar limite de clientes
tcp_server_set_max_clients("meu_servidor", 50);

// Verificar status antes de iniciar
let status = tcp_server_status("meu_servidor");
print("Servidor criado: " + status.server_id);
print("Porta: " + status.port);
print("Rodando: " + status.is_running);

// Iniciar servidor
tcp_server_start("meu_servidor");
print("Servidor TCP iniciado em 0.0.0.0:8080");

// === CLIENTE TCP ===

// Verificar se porta está disponível (em outro host)
let porta_disponivel = tcp_port_available(8081);
if (porta_disponivel) {
    print("Porta 8081 está livre");
}

// Obter IP local
let meu_ip = tcp_get_local_ip();
print("Meu IP local: " + meu_ip);

// Resolver hostname
let ip_servidor = tcp_resolve_hostname("exemplo.com");
print("IP do servidor: " + ip_servidor);

// Criar cliente
tcp_client_create("cliente1", "127.0.0.1", 8080);

// Configurar timeout
tcp_client_set_timeout("cliente1", 30);

// Conectar ao servidor
let conectado = tcp_client_connect("cliente1");

if (conectado) {
    print("Conectado ao servidor!");
    
    // Enviar dados
    let enviado = tcp_client_send("cliente1", "Olá servidor TCP!");
    if (enviado) {
        print("Mensagem enviada com sucesso");
        
        // Receber resposta
        let resposta = tcp_client_receive("cliente1");
        print("Resposta do servidor: " + resposta);
    }
    
    // Desconectar
    tcp_client_disconnect("cliente1");
    print("Desconectado do servidor");
}

// Parar servidor
tcp_server_stop("meu_servidor");
print("Servidor TCP parado");
```


#### 🌐 UDP (Datagramas) `#<udp>`

O módulo UDP fornece comunicação por datagramas usando o protocolo UDP (User Datagram Protocol). Ideal para comunicação rápida, broadcast e aplicações que não requerem garantia de entrega.

**Funções do Servidor UDP:**

```dryad
udp_server_create(server_id, host, port);
```
Cria um servidor UDP com identificador único.
- `server_id`: string - Identificador único do servidor
- `host`: string - Endereço IP para bind (ex: "127.0.0.1")
- `port`: number - Porta para escutar (ex: 8080)
- Retorna: null

```dryad
udp_server_start(server_id);
```
Inicia o servidor UDP para começar a receber datagramas.
- `server_id`: string - Identificador do servidor
- Retorna: boolean - true se iniciado com sucesso

```dryad
udp_server_stop(server_id);
```
Para o servidor UDP e libera a porta.
- `server_id`: string - Identificador do servidor
- Retorna: boolean - true se parado com sucesso

```dryad
udp_server_status(server_id);
```
Verifica o status atual do servidor UDP.
- `server_id`: string - Identificador do servidor
- Retorna: object com campos:
  - `server_id`: string - ID do servidor
  - `host`: string - Host configurado
  - `port`: number - Porta configurada
  - `is_running`: boolean - Se está rodando

**Funções do Cliente UDP:**

```dryad
udp_client_create(client_id, host, port);
```
Cria um cliente UDP para envio de datagramas.
- `client_id`: string - Identificador único do cliente
- `host`: string - Host padrão (pode ser sobrescrito no send)
- `port`: number - Porta padrão (0 para porta automática)
- Retorna: null

```dryad
udp_client_send(client_id, target_host, target_port, data);
```
Envia um datagrama UDP para destino específico.
- `client_id`: string - Identificador do cliente
- `target_host`: string - IP de destino
- `target_port`: number - Porta de destino
- `data`: string - Dados para enviar
- Retorna: boolean - true se enviado com sucesso

```dryad
udp_client_receive(client_id);
```
Tenta receber um datagrama UDP (não-bloqueante).
- `client_id`: string - Identificador do cliente
- Retorna: string - Dados recebidos ou string vazia

```dryad
udp_client_status(client_id);
```
Verifica o status do cliente UDP.
- `client_id`: string - Identificador do cliente
- Retorna: object com campos:
  - `client_id`: string - ID do cliente
  - `host`: string - Host configurado
  - `port`: number - Porta configurada
  - `timeout_ms`: number - Timeout em milissegundos

```dryad
udp_client_set_timeout(client_id, timeout_ms);
```
Configura timeout para operações de recepção.
- `client_id`: string - Identificador do cliente
- `timeout_ms`: number - Timeout em milissegundos
- Retorna: boolean - true se configurado com sucesso

**Funções Utilitárias UDP:**

```dryad
udp_get_local_ip();
```
Obtém o IP local da máquina.
- Retorna: string - IP local detectado

```dryad
udp_resolve_hostname(hostname);
```
Resolve um hostname para endereço IP.
- `hostname`: string - Nome do host para resolver
- Retorna: string - IP resolvido

```dryad
udp_port_available(port);
```
Verifica se uma porta UDP está disponível.
- `port`: number - Porta para verificar
- Retorna: boolean - true se disponível

**Exemplo de uso:**

```dryad
#<udp>
#<console_io>

// Servidor UDP
udp_server_create("echo_server", "127.0.0.1", 8080);
udp_server_start("echo_server");

// Cliente UDP  
udp_client_create("client", "127.0.0.1", 0);
udp_client_send("client", "127.0.0.1", 8080, "Hello UDP!");

let response = udp_client_receive("client");
print("Resposta: " + response);

udp_server_stop("echo_server");
```

**Características do UDP:**
- ✅ Comunicação rápida e eficiente
- ✅ Baixo overhead de protocolo
- ✅ Suporte a broadcast/multicast
- ⚠️ Não garante entrega de datagramas
- ⚠️ Não garante ordem de chegada
- ⚠️ Sem controle de fluxo automático
