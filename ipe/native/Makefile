# Makefile cross-platform para compilar ipe library
# Funciona em Windows (MinGW) e Linux

UNAME_S := $(shell uname -s)
UNAME_M := $(shell uname -m)

# Detecção de SO
ifeq ($(OS),Windows_NT)
    PLATFORM := windows
else ifeq ($(UNAME_S),Linux)
    PLATFORM := linux
else ifeq ($(UNAME_S),Darwin)
    PLATFORM := macos
else
    PLATFORM := windows
endif

# Compilador
ifeq ($(PLATFORM),windows)
    CC = gcc
    CFLAGS_BASE = -shared -O2 -fPIC
    LIBS_GDI = -lgdi32 -luser32 -lkernel32
    LIBS_SDL2 = -lSDL2 -lSDL2_ttf -lSDL2_image
    CFLAGS_SDL2 = -IC:/msys64/ucrt64/include/SDL2 -Dmain=SDL_main
    LIBS_SDL2_FULL = -LC:/msys64/ucrt64/lib -lmingw32 -mwindows -lSDL2main -lSDL2 -lSDL2_ttf -lSDL2_image
    OUTPUT_GDI = ipe.dll
    OUTPUT_SDL2 = ipe.dll
else ifeq ($(PLATFORM),linux)
    CC = gcc
    CFLAGS_BASE = -shared -O2 -fPIC -Wl,--no-undefined
    LIBS_GDI = -ldl
    LIBS_SDL2 = $(shell pkg-config --libs SDL2 SDL2_ttf SDL2_image)
    CFLAGS_SDL2 = $(shell pkg-config --cflags SDL2 SDL2_ttf SDL2_image)
    OUTPUT_GDI = libipe.so
    OUTPUT_SDL2 = libipe.so
else ifeq ($(PLATFORM),macos)
    CC = clang
    CFLAGS_BASE = -shared -O2 -fPIC
    LIBS_GDI = -framework CoreFoundation
    LIBS_SDL2 = $(shell pkg-config --libs SDL2 SDL2_ttf SDL2_image)
    CFLAGS_SDL2 = $(shell pkg-config --cflags SDL2 SDL2_ttf SDL2_image)
    OUTPUT_GDI = libipe.dylib
    OUTPUT_SDL2 = libipe.dylib
endif

SOURCE_GDI = ipe_helper.c
SOURCE_SDL2 = ipe_helper_sdl2.c

.PHONY: all clean info help gdi sdl2 test test-windows test-linux test-macos

# Default target: SDL2
all: sdl2
	@echo "[✓] Build completo para $(PLATFORM): $(OUTPUT_SDL2)"

# GDI target (Windows only)
gdi: $(OUTPUT_GDI)
	@echo "[✓] GDI build completo: $(OUTPUT_GDI)"

$(OUTPUT_GDI): $(SOURCE_GDI)
	@echo "[→] Compilando $(SOURCE_GDI) para $(PLATFORM) (GDI)..."
	$(CC) $(CFLAGS_BASE) $(SOURCE_GDI) -o $(OUTPUT_GDI) $(LIBS_GDI)
	@echo "[✓] $(OUTPUT_GDI) criado com sucesso!"

# SDL2 target (primary)
sdl2: $(OUTPUT_SDL2)_sdl2
	@echo "[✓] SDL2 build completo: $(OUTPUT_SDL2)"

$(OUTPUT_SDL2)_sdl2: $(SOURCE_SDL2)
	@echo "[→] Compilando versão SDL2..."
	$(CC) $(CFLAGS_BASE) $(CFLAGS_SDL2) $(SOURCE_SDL2) -o $(OUTPUT_SDL2) $(LIBS_SDL2_FULL)
	@echo "[✓] $(OUTPUT_SDL2) criado com sucesso!"

info:
	@echo "=== IPE Native Library Build Info ==="
	@echo "Platform: $(PLATFORM)"
	@echo "Compiler: $(CC)"
	@echo "Output SDL2: $(OUTPUT_SDL2)"
	@echo "Output GDI: $(OUTPUT_GDI)"
	@echo "CFLAGS_BASE: $(CFLAGS_BASE)"
	@echo "LIBS_SDL2: $(LIBS_SDL2)"
	@echo "LIBS_GDI: $(LIBS_GDI)"

help:
	@echo "=== IPE Native Library - Makefile Commands ==="
	@echo "make         - Compila versão SDL2 (padrão)"
	@echo "make sdl2    - Compila versão SDL2"
	@echo "make gdi     - Compila versão GDI (Windows)"
	@echo "make clean   - Remove arquivos compilados"
	@echo "make info    - Mostra info de build"
	@echo "make help    - Mostra este menu"

clean:
	@echo "[→] Removendo arquivos compilados..."
ifeq ($(PLATFORM),windows)
	del /q *.dll 2>nul || true
	del /q *.o 2>nul || true
else
	rm -f *.so *.dylib *.o
endif
	@echo "[✓] Limpeza concluída!"

test: $(OUTPUT_SDL2)
	@echo "[→] Executando testes de integração SDL2..."
	@cd ../.. && ./target/debug/deps/dryad.exe run ipe/tests/test_sdl2_cross_platform.dryad
	@echo "[✓] Testes completados!"

test-windows: sdl2
	@echo "[→] Compilando versão SDL2 para Windows..."
	@echo "[✓] ipe.dll criado"
	@cd ../.. && ./target/debug/deps/dryad.exe run ipe/tests/test_sdl2_cross_platform.dryad

test-linux: sdl2
	@echo "[→] Compilando versão SDL2 para Linux..."
	@echo "[✓] libipe.so criado"
	@cd ../.. && ./target/debug/deps/dryad.exe run ipe/tests/test_sdl2_cross_platform.dryad

test-macos: sdl2
	@echo "[→] Compilando versão SDL2 para macOS..."
	@echo "[✓] libipe.dylib criado"
	@cd ../.. && ./target/debug/deps/dryad.exe run ipe/tests/test_sdl2_cross_platform.dryad

