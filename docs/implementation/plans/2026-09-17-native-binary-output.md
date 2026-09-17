# Phase 6.14 — Native Binary Output

> **Status:** Complete  
> **Depends on:** Native linking (6.8), stdlib via intrinsics (6.13)

## Goal

After compilation, return artifact bytes in memory (`AotCompileResult::binary`) and link runnable native images (ELF on Linux, PE on Windows).

## API

```cpp
struct AotCompileResult {
    bool success;
    std::string output_path;
    std::string message;
    std::vector<uint8_t> binary;
};

AotCompileResult compile_to_native_executable(...);
std::string resolve_entry_function(const CompiledProgram&, const CompileOptions&);
```

## Entry points

| Source shape | Entry symbol |
|--------------|--------------|
| Top-level script (no `function`) | `__dryad_main` (synthesized from script statements) |
| Explicit `--entry fn` | Named function |
| Default (functions present) | First function, or `__dryad_main` if synthesized |

## CLI

```bash
# Shared library
dryad compile app.dryad --aot --link -o app.so    # .dll on Windows

# ELF/PE executable
dryad compile app.dryad --aot --link --executable -o app --entry main_fn
```

## `main(argc, argv)`

Generated LLVM:

```llvm
define i32 @main(i32 %argc, i8** %argv) {
  %main_arg0 = call i8* @dryad_argv_box_i32(i8** %argv, i32 1)
  ...
  call i8* @entry(i8* %main_arg0, ...)
  ret i32 0
}
```

Parameter types map from slot metadata (`i32`, `f64`, `bool`, or C string).

## Platform linking

| OS | Shared | Executable | Magic |
|----|--------|------------|-------|
| Linux | `.so`, `-fPIC -shared` | extensionless | `ELF` |
| Windows | `.dll`, `-shared` | `.exe` | `MZ` |

## Tests

- `TopLevelScriptCompilesToExecutable`
- `NativeExecutableEntryWithArgvParameters`
- `ScriptEntryUsesDryadMainSymbol`
- `is_native_binary_image()` — ELF or PE
