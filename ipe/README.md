# IPE Framework - Biblioteca Gráfica Nativa para Dryad

## 📋 Visão Geral

IPE é um framework GUI nativo que permite criar interfaces gráficas simples usando Dryad através de FFI (Foreign Function Interface). A biblioteca oferece suporte multiplataforma com Windows, Linux e macOS.

## 🖥️ Platform Support

### Supported Platforms
- **Windows** (Windows Vista+) - via SDL2 with hardware acceleration
- **Linux** (X11/Wayland) - via SDL2 with GCC
- **macOS** (10.7+) - via SDL2 with Clang (Apple Silicon & Intel)

### Installation

#### Prerequisites
- **Compiler:** 
  - Windows: GCC/MinGW
  - Linux: GCC or Clang
  - macOS: Xcode Command Line Tools (Clang)
- **SDL2 Development Libraries**

#### Install SDL2

**Windows (MSYS2/MinGW):**
```bash
pacman -S mingw-w64-x86_64-SDL2 mingw-w64-x86_64-SDL2_ttf mingw-w64-x86_64-SDL2_image
```

**Linux (Debian/Ubuntu):**
```bash
sudo apt-get install -y libsdl2-dev libsdl2-ttf-dev libsdl2-image-dev build-essential
```

**macOS (Homebrew):**
```bash
brew install sdl2 sdl2_ttf sdl2_image
```

#### Building

**Using Makefile (all platforms):**
```bash
cd ipe/native
make clean
make sdl2
```

**Using shell script:**
- **Windows:** `build.cmd`
- **Linux/macOS:** `./build.sh`

**Using legacy GDI (Windows only):**
```bash
make gdi      # or build.cmd --gdi
```

### Verification

After building, verify the library works:

```bash
# Run integration test
dryad run ipe/tests/test_integration.dryad

# Run interactive demo
dryad run ipe/tests/demo.dryad
```

## 🏗️ Arquitetura

```
ipe/
├── lib/
│   └── ipe.dryad          # Wrapper Dryad (FFI bindings)
├── native/
│   ├── ipe_helper.c       # Implementação nativa (Windows GDI)
│   ├── ipe_helper_sdl2.c  # Implementação alternativa SDL2
│   ├── Makefile           # Build cross-platform
│   ├── build.sh           # Script build para Linux/macOS
│   ├── build.cmd          # Script build para Windows
│   └── ipe.dll/libipe.so  # Bibliotecas compiladas
├── tests/
│   ├── demo.dryad         # Demo da API
│   ├── test_ffi.dryad     # Teste FFI básico
│   ├── test_native.dryad  # Teste de detecção de plataforma
│   ├── test_integration.dryad # Suite de testes de integração
│   └── kitchen_sink.dryad # Demo completa
└── oaklibs.json           # Manifest do pacote
```

## 🚀 Início Rápido

### Windows

1. **Compilar a biblioteca nativa:**
   ```batch
   cd ipe\native
   build.cmd
   ```

2. **Executar teste de integração:**
   ```bash
   cargo run --bin dryad run ipe/tests/test_integration.dryad
   ```

3. **Executar demo:**
   ```bash
   cargo run --bin dryad run ipe/tests/demo.dryad
   ```

### Linux/macOS

1. **Compilar a biblioteca nativa:**
   ```bash
   cd ipe/native
   chmod +x build.sh
   ./build.sh
   ```

2. **Executar teste de integração:**
   ```bash
   cargo run --bin dryad run ipe/tests/test_integration.dryad
   ```

3. **Executar demo:**
   ```bash
   cargo run --bin dryad run ipe/tests/demo.dryad
   ```

## 📚 Uso da API

### Inicializar e Criar Janela

```dryad
use "./ipe/lib/ipe";

fn main() {
    // Inicializar o framework
    se (!ipe::init()) {
        println("Erro ao inicializar");
        retorna;
    }

    // Criar janela
    ipe::createWindow(800, 600, "Minha Janela");

    // Loop principal
    enquanto (ipe::isOpen()) {
        // Limpar background
        ipe::clear(0x333333);

        // Desenhar retângulo
        ipe::drawRect(100, 100, 200, 150, 0xFF0000);

        // Processar eventos
        ipe::processEvents();
    }

    // Fechar
    ipe::close();
}

main();
```

### API Completa

#### `ipe::init() -> bool`
Inicializa o framework IPE. Deve ser chamada antes de qualquer outra operação.

**Retorna:** `true` se bem-sucedido, `false` caso contrário.

#### `ipe::createWindow(width: int, height: int, title: string) -> pointer`
Cria uma janela com as dimensões especificadas.

**Parâmetros:**
- `width`: Largura em pixels
- `height`: Altura em pixels  
- `title`: Título da janela

**Retorna:** Ponteiro para a janela (uso interno)

#### `ipe::clear(color: int)`
Limpa a janela com a cor especificada.

**Parâmetros:**
- `color`: Cor em formato RGB (0xRRGGBB)

#### `ipe::drawRect(x: int, y: int, w: int, h: int, color: int)`
Desenha um retângulo preenchido.

**Parâmetros:**
- `x`, `y`: Coordenadas do canto superior-esquerdo
- `w`, `h`: Largura e altura
- `color`: Cor em formato RGB (0xRRGGBB)

#### `ipe::processEvents() -> int`
Processa eventos da janela (teclado, mouse, etc).

**Retorna:** Status de execução (0 = parar, não-zero = continuar)

#### `ipe::isOpen() -> bool`
Verifica se a janela está aberta.

**Retorna:** `true` se aberta, `false` caso contrário

#### `ipe::close()`
Fecha a janela e libera recursos.

#### `ipe::unload() -> bool`
Descarrega a biblioteca nativa.

**Retorna:** `true` se bem-sucedido

#### `ipe::get_status() -> object`
Retorna o status atual do framework.

**Retorna:** Objeto com campos `{ loaded, path, alias }`

## 🔧 Build & Compilação

### SDL2 (Recommended, All Platforms)

**Windows:**
```batch
cd ipe\native
build.cmd
```

**Linux/macOS:**
```bash
cd ipe/native
chmod +x build.sh
./build.sh
```

### GDI (Legacy, Windows Only)

**Windows:**
```batch
cd ipe\native
build.cmd --gdi
```

### Manual Compilation

**Windows (SDL2):**
```batch
gcc -shared -O2 -fPIC ipe_helper_sdl2.c -o ipe.dll -lSDL2 -lSDL2_ttf -lSDL2_image -lgdi32 -luser32 -lkernel32
```

**Windows (GDI):**
```batch
gcc -shared -O2 -fPIC ipe_helper.c -o ipe.dll -lgdi32 -luser32 -lkernel32
```

**Linux:**
```bash
gcc -shared -O2 -fPIC -Wl,--no-undefined ipe_helper_sdl2.c -o libipe.so -lSDL2 -lSDL2_ttf -lSDL2_image -ldl -lm
```

**macOS:**
```bash
clang -shared -O2 -fPIC ipe_helper_sdl2.c -o libipe.dylib -lSDL2 -lSDL2_ttf -lSDL2_image -framework CoreFoundation -framework CoreVideo
```

## 🧪 Testes

### Test FFI Básico
```bash
cargo run --bin dryad run ipe/tests/test_ffi.dryad
```

### Test de Plataforma
```bash
cargo run --bin dryad run ipe/tests/test_native.dryad
```

### Suite de Integração
```bash
cargo run --bin dryad run ipe/tests/test_integration.dryad
```

A suite de testes verifica:
- ✓ Detecção de plataforma
- ✓ Carregamento de biblioteca
- ✓ Verificação de símbolos
- ✓ Chamadas FFI

### Demo
```bash
cargo run --bin dryad run ipe/tests/demo.dryad
```

## 📋 Requisitos do Sistema

### Windows
- MinGW-w64 ou MSVC
- Windows 7 ou superior
- GDI+ (incluído no Windows)

### Linux
- GCC 5.0+
- libX11 development headers
- GLIBC 2.14+

### macOS
- Clang/LLVM
- macOS 10.10 ou superior
- Xcode Command Line Tools

## ⚠️ Limitações Atuais

- Suporte apenas a rendering 2D simples (retângulos)
- Sem suporte a input avançado (mouse, teclado)
- Sem animações ou efeitos
- Sem suporte a fontes custom

## 📝 Roadmap

### Fase 1 ✓ (Concluído)
- [x] FFI bindings básicos
- [x] Inicialização e criação de janela
- [x] Desenho de retângulos
- [x] Processamento de eventos

### Fase 2 (Em Progresso)
- [ ] Desenho de círculos e polígonos
- [ ] Suporte a imagens/texturas
- [ ] Input de teclado/mouse
- [ ] Fontes e texto renderizado

### Fase 3 (Planejado)
- [ ] Sistema de layout automático
- [ ] Controles (botões, inputs, etc)
- [ ] Temas e estilos
- [ ] Animações

### Fase 4 (Futuro)
- [ ] Gradientes e efeitos
- [ ] 3D básico
- [ ] Performance optimizations
- [ ] Publicar como pacote Oak

## 🐛 Debugging

### Erros Comuns

**`[X] Biblioteca não carregada`**
- Certifique-se de que a DLL/SO foi compilada
- Verifique que o arquivo está no caminho correto
- Use `test_integration.dryad` para diagnosticar

**`[X] Símbolo não encontrado`**
- Recompile a biblioteca nativa
- Verifique que `__declspec(dllexport)` está presente em Windows
- Use `nm` (Linux) ou `dumpbin` (Windows) para verificar símbolos

**`[X] Erro ao inicializar Ipe`**
- Verifique se a janela foi criada corretamente
- Tente em fullscreen false
- Verifique os logs do sistema

## 📖 Exemplo Completo

```dryad
#<ffi>
#<console_io>
use "./ipe/lib/ipe";

fn main() {
    println("=== IPE Demo ===");
    
    se (!ipe::init()) {
        println("Erro ao inicializar IPE");
        retorna;
    }

    ipe::createWindow(1024, 768, "IPE Framework Demo");
    
    var x = 0;
    var y = 0;
    var vx = 5;
    var vy = 3;
    
    enquanto (ipe::isOpen() && x < 5000) {
        ipe::clear(0x1a1a1a);
        
        // Desenha um retângulo que se move
        ipe::drawRect(x, y, 50, 50, 0xFF5500);
        
        // Update posição
        x = x + vx;
        y = y + vy;
        
        // Collision detection simples
        se (y < 0 || y > 768 - 50) vy = -vy;
        se (x < 0 || x > 1024 - 50) vx = -vx;
        
        ipe::processEvents();
    }

    ipe::close();
    println("Demo encerrado!");
}

main();
```

## 📄 Licença

MIT - Veja LICENSE para detalhes

## 👤 Autor

Pedro Jesus

## 🙏 Contribuições

Contribuições são bem-vindas! Abra uma issue ou PR no repositório.

---

**Status:** ✅ Funcional em Windows | ⚠️ Em testes em Linux/macOS
