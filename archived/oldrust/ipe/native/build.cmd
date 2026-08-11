@echo off
REM Build script para compilar ipe library com SDL2 no Windows
REM Usage: build.cmd [--clean] [--help] [--gdi]

setlocal enabledelayedexpansion

set "GDI=0"
if "%1%"=="--gdi" (
    set "GDI=1"
    shift
)

if "%1%"=="--clean" (
    echo [→] Removendo arquivos compilados...
    del /q *.dll 2>nul
    del /q *.o 2>nul
    del /q *.a 2>nul
    echo [✓] Limpeza concluída!
    goto :eof
)

if "%1%"=="--help" (
    echo Usage: build.cmd [options]
    echo.
    echo Options:
    echo   --clean     Remove arquivos compilados
    echo   --gdi       Compila versão GDI (padrão de legado)
    echo   --help      Mostra este menu
    echo.
    echo Examples:
    echo   build.cmd              # Compila biblioteca SDL2
    echo   build.cmd --clean      # Limpa e compila
    echo   build.cmd --gdi        # Compila versão GDI (legado)
    goto :eof
)

REM Verificar GCC
where gcc >nul 2>&1
if !errorlevel! neq 0 (
    echo [ERROR] GCC não encontrado. Instale MinGW ou adicione ao PATH.
    exit /b 1
)

echo.
echo ═══════════════════════════════════════════════════════
echo ░ IPE Native Library Build (SDL2 Default)
echo ═══════════════════════════════════════════════════════
echo Platform: Windows
echo Compiler: GCC/MinGW
echo ═══════════════════════════════════════════════════════
echo.

if "%GDI%"=="1" (
    echo [→] Compilando versão GDI (legado)...
    gcc -shared -O2 -fPIC ipe_helper.c -o ipe.dll -lgdi32 -luser32 -lkernel32
    if not exist ipe.dll (
        echo [ERROR] Erro ao compilar ipe.dll (GDI)
        exit /b 1
    )
) else (
    echo [→] Compilando versão SDL2...
    gcc -shared -O2 -fPIC ipe_helper_sdl2.c -o ipe.dll -lSDL2 -lSDL2_ttf -lSDL2_image -lgdi32 -luser32 -lkernel32
    if not exist ipe.dll (
        echo [ERROR] Erro ao compilar ipe.dll (SDL2)
        exit /b 1
    )
)

echo [✓] ipe.dll criado com sucesso!
dir /b ipe.dll
echo [✓] Build completo!
