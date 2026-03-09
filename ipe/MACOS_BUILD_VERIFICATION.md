# macOS Build Verification Report

**Date:** Mon Mar 09 2026
**Platform:** macOS (Apple Silicon / Intel)
**Status:** Ready for Testing

## Prerequisites

### System Requirements
- **macOS:** 10.7 (Lion) or later
- **Compiler:** Clang/LLVM (part of Xcode Command Line Tools)
- **Architecture:** Apple Silicon (ARM64) or Intel (x86_64)

### Check Xcode Installation
```bash
# Install if needed
xcode-select --install

# Verify installation
clang --version
```

## SDL2 Installation

### Option 1: Homebrew (Recommended)
```bash
# Install Homebrew if needed
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install SDL2 and development libraries
brew install sdl2 sdl2_ttf sdl2_image
```

### Option 2: MacPorts
```bash
sudo port install libsdl2 libsdl2_ttf libsdl2_image
```

### Verify Installation
```bash
sdl2-config --cflags --libs
# Expected output: -I/opt/homebrew/include/SDL2 -L/opt/homebrew/lib -lSDL2 ...
```

## Build Procedure

### Navigate to Native Directory
```bash
cd ipe/native
```

### Clean Previous Builds
```bash
make clean
```

### Compile SDL2 Version for macOS
```bash
make sdl2
# or
./build.sh
```

### Expected Output
```
[→] Compilando versão SDL2...
[✓] libipe.dylib criado com sucesso!
```

## Verification Steps

### 1. Verify Library File Created
```bash
ls -lh libipe.dylib
```

Expected output:
```
-rw-r--r--  1 user  staff  150K [date] libipe.dylib
```

### 2. Verify Exported Symbols
```bash
nm -gU libipe.dylib | grep ipe_
```

Expected output:
```
00000000000029a0 T _ipe_clear_background
00000000000029c0 T _ipe_draw_rect
0000000000002a00 T _ipe_init
0000000000002a20 T _ipe_is_window_open
0000000000002a40 T _ipe_process_events
0000000000002a60 T _ipe_window_close
0000000000002a80 T _ipe_window_create
```

(Note: Functions have `_` prefix on macOS due to C symbol naming conventions)

### 3. Check Library Dependencies
```bash
otool -L libipe.dylib
```

Expected output should show SDL2 libraries:
```
libipe.dylib:
	/opt/homebrew/lib/libSDL2-2.0.0.dylib (compatibility version 2.0.0, current version 2.0.0)
	/opt/homebrew/lib/libSDL2_ttf-2.0.0.dylib (compatibility version 2.0.0, current version 2.0.0)
	/opt/homebrew/lib/libSDL2_image-2.0.0.dylib (compatibility version 2.0.0, current version 2.0.0)
	/usr/lib/libSystem.B.dylib
```

### 4. Verify File Type
```bash
file libipe.dylib
```

Expected output:
```
libipe.dylib: Mach-O 64-bit dynamically linked shared library arm64
```
(or `x86_64` for Intel Macs)

### 5. Test with Architecture Check
```bash
# For Apple Silicon Mac
lipo -info libipe.dylib
# Expected: Non-fat file: arm64

# For Intel Mac
lipo -info libipe.dylib  
# Expected: Non-fat file: i386 x86_64
```

## Symbol Verification Details

All 7 required core functions must be present:

| Symbol | Purpose | FFI Type |
|--------|---------|----------|
| `ipe_init` | Initialize graphics system | Returns int |
| `ipe_window_create` | Create graphics window | Returns pointer |
| `ipe_clear_background` | Clear screen with color | Returns void |
| `ipe_draw_rect` | Draw filled rectangle | Returns void |
| `ipe_process_events` | Process system events | Returns int |
| `ipe_is_window_open` | Check window state | Returns int |
| `ipe_window_close` | Close and cleanup | Returns void |

**Verification Command:**
```bash
for func in ipe_init ipe_window_create ipe_clear_background ipe_draw_rect ipe_process_events ipe_is_window_open ipe_window_close; do
  if nm -gU libipe.dylib | grep -q "_$func"; then
    echo "✓ $func"
  else
    echo "✗ $func (MISSING)"
  fi
done
```

## Using with Dryad

The built library can be loaded and used from Dryad:

```dryad
#<ffi>
#<system_env>

function main() {
    let platform = native_platform();
    println("Platform: " + platform);
    
    if (ffi_load_library("./ipe/native/libipe.dylib", "ipe")) {
        let result = ffi_call("ipe", "ipe_init", "i32");
        println("ipe_init returned: " + result);
    }
}

main();
```

## Troubleshooting

### Issue: SDL2 libraries not found during compilation
**Solution:**
```bash
# Check SDL2 installation
brew list sdl2
brew list sdl2_ttf
brew list sdl2_image

# If missing, reinstall
brew install sdl2 sdl2_ttf sdl2_image

# Verify pkg-config
pkg-config --cflags --libs sdl2
```

### Issue: `arch: command not found` during compilation
**Solution:**
- This is expected on some macOS systems
- The build system falls back to single architecture
- Library will still be functional

### Issue: Permission denied when running
**Solution:**
```bash
# Make script executable
chmod +x ./build.sh
./build.sh
```

### Issue: Different architecture needed (e.g., universal binary)
**Solution:**
```bash
# For universal binary (both arm64 + x86_64)
# Requires special compilation flags - document separately if needed
# Current default: native architecture only
```

## CI/CD Integration

### GitHub Actions (macOS-latest)
```yaml
- name: Install SDL2
  run: brew install sdl2 sdl2_ttf sdl2_image

- name: Build IPE
  run: cd ipe/native && make clean && make sdl2

- name: Verify Symbols
  run: nm -gU ipe/native/libipe.dylib | grep ipe_

- name: Check Dependencies
  run: otool -L ipe/native/libipe.dylib
```

## Build Verification Checklist

- [ ] Xcode Command Line Tools installed (`clang --version`)
- [ ] SDL2 installed via Homebrew (`brew list sdl2`)
- [ ] `sdl2-config` available in PATH
- [ ] `make clean && make sdl2` succeeds without errors
- [ ] `libipe.dylib` file created (> 100KB)
- [ ] All 7 symbols exported (`nm -gU` shows them)
- [ ] Library dependencies correct (`otool -L` shows SDL2)
- [ ] File type correct (Mach-O 64-bit)
- [ ] Correct architecture (arm64 or x86_64)
- [ ] Dryad can load and call functions

## Expected Result

✅ **SDL2 macOS build verified and working**
- Library compiles without errors
- All 7 symbols properly exported
- Can be loaded and used from Dryad
- Cross-platform compatibility confirmed

## References

- [Homebrew SDL2](https://formulae.brew.sh/formula/sdl2)
- [SDL2 Official Documentation](https://wiki.libsdl.org/)
- [macOS Mach-O Format](https://developer.apple.com/library/archive/documentation/DeveloperTools/Conceptual/MachORuntime/index.html)
