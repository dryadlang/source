#!/bin/bash
# Build script para compilar ipe library com SDL2 em Linux/macOS
# Usage: ./build.sh [--clean] [--help] [--gdi]

set -e

# Detectar SO
OS_TYPE=$(uname -s)
ARCH=$(uname -m)

# Check for --gdi flag (Windows GDI version)
USE_GDI=0
if [ "$1" == "--gdi" ]; then
    USE_GDI=1
    shift
fi

case "$OS_TYPE" in
    Linux*)
        PLATFORM="Linux"
        if [ "$USE_GDI" == "1" ]; then
            echo "❌ GDI não é suportado em Linux. Use SDL2."
            exit 1
        fi
        OUTPUT="libipe.so"
        CFLAGS="-shared -O2 -fPIC -Wl,--no-undefined"
        LIBS="-lSDL2 -lSDL2_ttf -lSDL2_image -ldl -lm"
        ;;
    Darwin*)
        PLATFORM="macOS"
        if [ "$USE_GDI" == "1" ]; then
            echo "❌ GDI não é suportado em macOS. Use SDL2."
            exit 1
        fi
        OUTPUT="libipe.dylib"
        CC="clang"
        CFLAGS="-shared -O2 -fPIC"
        LIBS="-lSDL2 -lSDL2_ttf -lSDL2_image -framework CoreFoundation -framework CoreVideo"
        ;;
    *)
        echo "❌ SO não suportado: $OS_TYPE"
        exit 1
        ;;
esac

CC="${CC:-gcc}"
SOURCE="ipe_helper_sdl2.c"

# Funções auxiliares
print_info() {
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "📦 IPE Native Library Build (SDL2)"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "Platform: $PLATFORM ($ARCH)"
    echo "Compiler: $CC"
    echo "Output: $OUTPUT"
    echo "Source: $SOURCE"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
}

check_sdl2() {
    echo "[→] Verificando SDL2..."
    if ! pkg-config --exists sdl2; then
        echo "❌ SDL2 não encontrado."
        if [ "$PLATFORM" == "Linux" ]; then
            echo "   Instale com: sudo apt-get install libsdl2-dev libsdl2-ttf-dev libsdl2-image-dev"
        else
            echo "   Instale com: brew install sdl2 sdl2_ttf sdl2_image"
        fi
        exit 1
    fi
    echo "[✓] SDL2 disponível"
}

clean() {
    echo "[→] Removendo arquivos compilados..."
    rm -f *.so *.dylib *.o *.a
    echo "[✓] Limpeza concluída!"
}

build() {
    echo "[→] Compilando $SOURCE para $PLATFORM..."
    $CC $CFLAGS $SOURCE -o $OUTPUT $LIBS
    
    if [ -f "$OUTPUT" ]; then
        echo "[✓] $OUTPUT criado com sucesso!"
        ls -lh "$OUTPUT"
    else
        echo "❌ Erro ao compilar $OUTPUT"
        exit 1
    fi
}

show_help() {
    echo "Usage: $0 [options]"
    echo ""
    echo "Options:"
    echo "  --clean     Remove arquivos compilados"
    echo "  --gdi       Tenta compilar versão GDI (Windows apenas)"
    echo "  --help      Mostra este menu"
    echo ""
    echo "Examples:"
    echo "  $0              # Compila biblioteca SDL2"
    echo "  $0 --clean      # Limpa e compila"
}

# Main
if [ "$1" == "--help" ]; then
    show_help
    exit 0
fi

if [ "$1" == "--clean" ]; then
    clean
    shift
fi

print_info
check_sdl2
build
echo "[✓] Build completo!"
