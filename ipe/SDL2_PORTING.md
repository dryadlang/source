# SDL2 Porting Guide

## Overview
Complete migration from Windows-only GDI to cross-platform SDL2 graphics library, maintaining 100% API compatibility.

## What Changed

### API Contract: UNCHANGED ✅

The exported C API remains **identical**:

```c
int ipe_init();
void* ipe_window_create(int width, int height, const char* title);
void ipe_clear_background(uint32_t color);
void ipe_draw_rect(int x, int y, int w, int h, uint32_t color);
int ipe_process_events();
int ipe_is_window_open();
void ipe_window_close();
```

Dryad FFI code requires **zero changes** - just rebuild the native library.

### Implementation Changes

#### Windows GDI (Before)
```c
// Windows-specific GDI API
HWND hwnd = CreateWindowEx(0, "Class", "Title", WS_OVERLAPPEDWINDOW, ...);
HDC hdc = GetDC(hwnd);
HBRUSH brush = CreateSolidBrush(RGB(r, g, b));
FillRect(hdc, &rect, brush);
MSG msg;
while (PeekMessage(&msg, ...)) { ... }
DestroyWindow(hwnd);
```

#### Cross-Platform SDL2 (After)
```c
// Cross-platform SDL2 API
SDL_Window *window = SDL_CreateWindow("Title", SDL_WINDOWPOS_CENTERED, ..., width, height, ...);
SDL_Renderer *renderer = SDL_CreateRenderer(window, -1, SDL_RENDERER_ACCELERATED);
SDL_SetRenderDrawColor(renderer, r, g, b, 255);
SDL_RenderFillRect(renderer, &rect);
SDL_RenderPresent(renderer);
SDL_Event event;
while (SDL_PollEvent(&event)) { ... }
SDL_DestroyRenderer(renderer);
SDL_DestroyWindow(window);
```

## File Structure

**New/Modified Files:**

| File | Status | Purpose |
|------|--------|---------|
| `ipe_helper_sdl2.c` | Created | Full SDL2 implementation (1248 lines) |
| `config.h` | Created | Platform/compiler detection macros |
| `Makefile` | Modified | Added `make sdl2` and `make gdi` targets |
| `build.sh` | Modified | SDL2 default, `--gdi` option |
| `build.cmd` | Modified | SDL2 default, `--gdi` option |
| `ipe_helper.c` | Kept | Legacy GDI implementation (still available) |

## Building

### SDL2 (Default, Recommended)
```bash
cd ipe/native

# Windows
build.cmd

# Linux/macOS
./build.sh
```

### GDI (Legacy, Windows Only)
```bash
# Windows
build.cmd --gdi

# Linux/macOS
./build.sh --gdi  # Will error (GDI not supported)
```

## Platform-Specific Details

### Windows
- **Library:** `ipe.dll` (127 KB typical)
- **Export:** `__declspec(dllexport)` via `IPE_EXPORT` macro
- **Compiler:** GCC/MinGW with `-shared -O2 -fPIC`
- **Libraries:** SDL2, SDL2_ttf, SDL2_image + Windows (GDI32, User32, Kernel32)

### Linux
- **Library:** `libipe.so` (expected ~140 KB)
- **Export:** `__attribute__((visibility("default")))` via `IPE_EXPORT` macro
- **Compiler:** GCC with `-shared -O2 -fPIC -Wl,--no-undefined`
- **Libraries:** SDL2, SDL2_ttf, SDL2_image + dl, m
- **Verification:** `nm -D libipe.so | grep ipe_`

### macOS
- **Library:** `libipe.dylib` (expected ~150 KB)
- **Export:** `__attribute__((visibility("default")))` via `IPE_EXPORT` macro
- **Compiler:** Clang with `-shared -O2 -fPIC`
- **Libraries:** SDL2, SDL2_ttf, SDL2_image + Cocoa frameworks
- **Verification:** `nm -gU libipe.dylib | grep ipe_`
- **Architectures:** Apple Silicon (arm64) and Intel (x86_64)

## Symbol Verification

### Windows
```bash
dumpbin /EXPORTS ipe.dll | findstr "ipe_"
```

### Linux
```bash
nm -D libipe.so | grep ipe_
```

### macOS
```bash
nm -gU libipe.dylib | grep ipe_
```

### Expected Output (all platforms)
```
ipe_init
ipe_window_create
ipe_clear_background
ipe_draw_rect
ipe_process_events
ipe_is_window_open
ipe_window_close
```

## Testing with Dryad

### Existing code (unchanged)
```dryad
#<ffi>
#<system_env>

function main() {
    let platform = native_platform();
    let ext = ".so";
    if (platform == "windows") { ext = ".dll"; }
    
    let path = "./ipe/native/ipe" + ext;
    if (ffi_load_library(path, "ipe")) {
        let result = ffi_call("ipe", "ipe_init", "i32");
        println("Init returned: " + result);
    }
}
```

No changes needed - just rebuild the native library.

## Troubleshooting

### Compilation Fails: SDL2 Headers Not Found
**Windows:**
```bash
pacman -S mingw-w64-x86_64-SDL2
```

**Linux:**
```bash
sudo apt-get install libsdl2-dev libsdl2-ttf-dev libsdl2-image-dev
```

**macOS:**
```bash
brew install sdl2 sdl2_ttf sdl2_image
```

### Symbol Export Issues
Check `config.h` is included as first header:
```c
#include "config.h"  // Must be first
#include <SDL2/SDL.h>
```

### Library Load Fails in Dryad
Verify correct path for your platform:
- Windows: `./ipe/native/ipe.dll`
- Linux: `./ipe/native/libipe.so`
- macOS: `./ipe/native/libipe.dylib`

### Performance Regression
If SDL2 version seems slower:
1. Verify hardware acceleration enabled in Makefile (-DHAVE_OPENGL, etc.)
2. Check SDL2_HINT_RENDER_VSYNC setting
3. Profile with system tools (Instruments, perf, etc.)

## Migration Checklist

- [x] SDL2 installed on all target platforms
- [x] `ipe_helper_sdl2.c` implemented with all 7 functions
- [x] `config.h` created for cross-platform compatibility
- [x] Makefile updated with SDL2 support
- [x] Build scripts (build.sh, build.cmd) tested
- [x] Windows build verified (ipe.dll working)
- [x] Symbol export verified on Windows
- [x] Dryad FFI integration tested (demo.dryad passing)
- [ ] Linux build tested on Linux machine
- [ ] macOS build tested on macOS machine
- [ ] CI/CD pipeline setup (GitHub Actions, etc.)

## Next Steps

1. **Test on Linux:** Run on Linux machine to verify libipe.so
2. **Test on macOS:** Run on macOS machine to verify libipe.dylib
3. **Setup CI/CD:** Automate cross-platform builds in GitHub Actions
4. **Performance:** Benchmark against GDI version (if applicable)
5. **Enhanced Graphics:** Add additional primitives (circles, polygons, etc.)

## References

- [SDL2 Documentation](https://wiki.libsdl.org/)
- [SDL2 Installation](https://wiki.libsdl.org/Installation)
- [SDL2 Cross-Compilation](https://wiki.libsdl.org/FAQ#Cross-compiling)
