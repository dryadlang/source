#ifndef IPE_CONFIG_H
#define IPE_CONFIG_H

/* Platform detection */
#if defined(_WIN32) || defined(_WIN64) || defined(__MINGW32__) || defined(__MINGW64__)
    #define IPE_PLATFORM_WINDOWS 1
    #define IPE_PLATFORM_NAME "Windows"
#elif defined(__APPLE__) && defined(__MACH__)
    #define IPE_PLATFORM_MACOS 1
    #define IPE_PLATFORM_NAME "macOS"
#elif defined(__linux__)
    #define IPE_PLATFORM_LINUX 1
    #define IPE_PLATFORM_NAME "Linux"
#else
    #error "Unsupported platform"
#endif

/* Compiler detection */
#if defined(_MSC_VER)
    #define IPE_COMPILER_MSVC 1
    #define IPE_EXPORT __declspec(dllexport)
#elif defined(__GNUC__) || defined(__clang__)
    #define IPE_COMPILER_GCC 1
    #define IPE_EXPORT __attribute__((visibility("default")))
#else
    #define IPE_EXPORT
#endif

/* SDL2 base includes */
#include <SDL2/SDL.h>

/* Platform-specific SDL2 includes (if needed) */
#ifdef IPE_PLATFORM_WINDOWS
    #include <SDL2/SDL_syswm.h>
#endif

#endif /* IPE_CONFIG_H */
