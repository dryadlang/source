#include "dryad/aot/aot_compiler.h"
#include "dryad/aot/library_linker.h"
#include "dryad/aot/native_linker.h"
#include "dryad/aot/analyze_pass.h"
#include "dryad/compiler/compile_pipeline.h"
#include "dryad/compiler/execution_mode.h"
#include "dryad/compiler/interpreter.h"
#include "dryad/compiler/validator.h"
#include "dryad/runtime/library_module.h"
#include "dryad/runtime/package_installer.h"
#include "dryad/common/utils.h"
#include <cstdlib>
#include <cstring>
#include <iostream>
#include <string>
#include <vector>

#ifndef DRYAD_STDLIB_DIR
#define DRYAD_STDLIB_DIR "stdlib"
#endif

#ifndef DRYAD_AOT_RUNTIME_SRC
#define DRYAD_AOT_RUNTIME_SRC "src/aot/aot_runtime.c"
#endif

#ifndef DRYAD_INCLUDE_DIR
#define DRYAD_INCLUDE_DIR "include"
#endif

namespace {

void print_help() {
    std::cout << "Dryad Programming Language v2.0.0\n\n";
    std::cout << "Usage:\n";
    std::cout << "  dryad [--allow-net] [--vm] <script.dryad>     Run a Dryad script\n";
    std::cout << "  dryad compile <file> [-o out] [--lib] [--aot] [--link] [--executable] [--entry fn] [--link-lib dyl] [--no-profile]\n";
    std::cout << "  dryad link <lib.dyl> [-o out.ll] [--shared]   Emit LLVM IR (or .so) from a .dyl library\n";
    std::cout << "  dryad analyze <file> [--no-profile]           AOT fast/slow path report\n";
    std::cout << "  dryad check <file>                            Validate syntax only\n";
    std::cout << "  dryad install [dryad.toml] [--modules dir]      Install package dependencies\n";
    std::cout << "  dryad --analyze <file>                      Alias for analyze\n";
    std::cout << "  dryad --help                                Show this help\n";
    std::cout << "  dryad --version                             Show version information\n\n";
    std::cout << "Options:\n";
    std::cout << "  --allow-net     Enable socket/network syscalls for this run\n";
    std::cout << "  --vm            Default to bytecode VM execution mode\n";
    std::cout << "  --no-profile    Skip interpreter profiling during compile/analyze\n";
    std::cout << "  --link          Link LLVM IR to native .so (requires --aot)\n";
    std::cout << "  --executable    Link to native ELF executable (requires --aot --link)\n";
    std::cout << "  --entry <fn>    Entry function for --executable (default: first function)\n";
    std::cout << "  --lib           Compile Dryad library to .dyl (importable binary)\n";
    std::cout << "  --link-lib      Link a compiled .dyl when using --aot/--link/--executable\n\n";
    std::cout << "Environment:\n";
    std::cout << "  DRYAD_STDLIB_DIR      Override stdlib path (default: "
              << DRYAD_STDLIB_DIR << ")\n";
    std::cout << "  DRYAD_ALLOW_NET       Set to 1 or true to enable network access\n";
    std::cout << "  DRYAD_LIBRARY_PATH    Extra library search paths (colon-separated)\n";
    std::cout << "  DRYAD_VM_MAX_STEPS    VM instruction budget (0 = unlimited)\n";
    std::cout << "  DRYAD_ARENA_BYTES     AOT runtime box arena size (default 65536)\n";
    std::cout << "  DRYAD_RUN_TIMEOUT     Example/benchmark run timeout seconds (default 30)\n";
}

std::string stdlib_root() {
    const char* env = std::getenv("DRYAD_STDLIB_DIR");
    if (env != nullptr && env[0] != '\0') {
        return env;
    }
    return DRYAD_STDLIB_DIR;
}

bool network_allowed_from_env() {
    const char* env = std::getenv("DRYAD_ALLOW_NET");
    if (env == nullptr || env[0] == '\0') {
        return false;
    }
    return std::strcmp(env, "1") == 0 || std::strcmp(env, "true") == 0 ||
           std::strcmp(env, "TRUE") == 0;
}

dryad::CompileOptions default_compile_options(bool allow_net, bool run_profiler) {
    dryad::CompileOptions options;
    options.stdlib_root = stdlib_root();
    options.allow_network = allow_net;
    options.run_profiler = run_profiler;
    return options;
}

std::string default_dryadc_output(const std::string& input_path) {
    const auto dot = input_path.rfind('.');
    if (dot == std::string::npos) {
        return input_path + ".dryadc";
    }
    return input_path.substr(0, dot) + ".dryadc";
}

std::string default_llvm_output(const std::string& input_path) {
    const auto dot = input_path.rfind('.');
    if (dot == std::string::npos) {
        return input_path + ".ll";
    }
    return input_path.substr(0, dot) + ".ll";
}

int run_install_command(int argc, char** argv) {
    std::string manifest_path = "dryad.toml";
    std::string modules_dir = "dryad_modules";

    for (int i = 2; i < argc; ++i) {
        const std::string token = argv[i];
        if (token == "--modules" && i + 1 < argc) {
            modules_dir = argv[++i];
            continue;
        }
        if (token.rfind("--", 0) == 0) {
            std::cerr << "Error: unknown install option: " << token << "\n";
            return 1;
        }
        manifest_path = token;
    }

    try {
        auto result = dryad::install_package(manifest_path, modules_dir);
        if (!result.success) {
            std::cerr << "Error: " << result.message << "\n";
            return 1;
        }
        std::cout << result.message << "\n";
        return 0;
    } catch (const dryad::DryadException& ex) {
        std::cerr << "Error: " << ex.what() << "\n";
        return 1;
    }
}

std::vector<std::string> library_paths_from_env() {
    std::vector<std::string> paths;
    const char* env = std::getenv("DRYAD_LIBRARY_PATH");
    if (env && env[0] != '\0') {
        paths = dryad::split(env, ':');
    }
    return paths;
}

int run_link_command(int argc, char** argv) {
    if (argc < 3) {
        std::cerr << "Error: link requires a .dyl library path\n";
        return 1;
    }

    const std::string input_path = argv[2];
    bool link_shared = false;
    std::string output_path;

    for (int i = 3; i < argc; ++i) {
        const std::string token = argv[i];
        if (token == "--shared") {
            link_shared = true;
            continue;
        }
        if (token == "-o" && i + 1 < argc) {
            output_path = argv[++i];
            continue;
        }
        std::cerr << "Error: unknown link option: " << token << "\n";
        return 1;
    }

    if (output_path.empty()) {
        const auto dot = input_path.rfind('.');
        const std::string stem =
            dot == std::string::npos ? input_path : input_path.substr(0, dot);
        output_path = link_shared ? stem + dryad::default_shared_extension()
                                  : stem + ".ll";
    }

    try {
        if (link_shared) {
            if (!dryad::clang_available()) {
                std::cerr << "Error: clang not found in PATH\n";
                return 1;
            }
            const std::string llvm_path = output_path + ".ll";
            auto ir_result = dryad::compile_library_to_llvm_ir(input_path, llvm_path);
            if (!ir_result.success) {
                std::cerr << "Error: " << ir_result.message << "\n";
                return 1;
            }
            auto link_result = dryad::link_native_binary(dryad::NativeLinkKind::SharedObject,
                                                         llvm_path,
                                                         output_path,
                                                         DRYAD_AOT_RUNTIME_SRC,
                                                         DRYAD_INCLUDE_DIR);
            if (!link_result.success) {
                std::cerr << "Error: " << link_result.message << "\n";
                return 1;
            }
            std::cout << link_result.message << "\n";
            return 0;
        }

        auto result = dryad::compile_library_to_llvm_ir(input_path, output_path);
        if (!result.success) {
            std::cerr << "Error: " << result.message << "\n";
            return 1;
        }
        std::cout << result.message << "\n";
        return 0;
    } catch (const std::exception& ex) {
        std::cerr << "Error: " << ex.what() << "\n";
        return 1;
    }
}

int run_compile_command(int argc, char** argv) {
    if (argc < 3) {
        std::cerr << "Error: compile requires an input file\n";
        return 1;
    }

    const std::string input_path = argv[2];
    bool allow_net = network_allowed_from_env();
    bool run_profiler = true;
    bool aot = false;
    bool compile_lib = false;
    bool link_native = false;
    bool link_executable = false;
    std::string output_path;
    std::string entry_function;
    std::vector<std::string> link_libraries;

    for (int i = 3; i < argc; ++i) {
        const std::string token = argv[i];
        if (token == "--allow-net") {
            allow_net = true;
            continue;
        }
        if (token == "--no-profile") {
            run_profiler = false;
            continue;
        }
        if (token == "--lib") {
            compile_lib = true;
            continue;
        }
        if (token == "--aot") {
            aot = true;
            continue;
        }
        if (token == "--link") {
            link_native = true;
            continue;
        }
        if (token == "--executable") {
            link_executable = true;
            link_native = true;
            aot = true;
            continue;
        }
        if (token == "--entry" && i + 1 < argc) {
            entry_function = argv[++i];
            continue;
        }
        if (token == "--link-lib" && i + 1 < argc) {
            link_libraries.push_back(argv[++i]);
            continue;
        }
        if (token == "-o" && i + 1 < argc) {
            output_path = argv[++i];
            continue;
        }
        std::cerr << "Error: unknown compile option: " << token << "\n";
        return 1;
    }

    if (compile_lib && (aot || link_native || link_executable)) {
        std::cerr << "Error: --lib cannot be combined with --aot/--link/--executable\n";
        return 1;
    }
    if (link_executable && !aot) {
        std::cerr << "Error: --executable requires --aot\n";
        return 1;
    }
    if (link_native && !aot) {
        std::cerr << "Error: --link requires --aot\n";
        return 1;
    }

    if (output_path.empty()) {
        const auto dot = input_path.rfind('.');
        const std::string stem =
            dot == std::string::npos ? input_path : input_path.substr(0, dot);
        if (link_executable) {
            output_path = stem + dryad::default_executable_extension();
        } else if (link_native) {
            output_path = stem + dryad::default_shared_extension();
        } else if (compile_lib) {
            output_path = dryad::default_dyl_output(input_path);
        } else {
            output_path = aot ? default_llvm_output(input_path) : default_dryadc_output(input_path);
        }
    }

    try {
        auto options = default_compile_options(allow_net, run_profiler);
        options.entry_function = entry_function;
        options.link_libraries = link_libraries;
        options.library_paths = library_paths_from_env();

        dryad::AotCompileResult result;
        if (compile_lib) {
            result = dryad::compile_to_dyl(input_path, output_path, options);
        } else if (link_executable) {
            result = dryad::compile_to_native_executable(
                input_path, output_path, options, DRYAD_AOT_RUNTIME_SRC, DRYAD_INCLUDE_DIR);
        } else if (link_native) {
            result = dryad::compile_to_native_module(
                input_path, output_path, options, DRYAD_AOT_RUNTIME_SRC, DRYAD_INCLUDE_DIR);
        } else if (aot) {
            result = dryad::compile_to_llvm_ir(input_path, output_path, options);
        } else {
            result = dryad::compile_to_dryadc(input_path, output_path, options);
        }

        if (!result.success) {
            std::cerr << "Error: " << result.message << "\n";
            return 1;
        }
        std::cout << result.message << "\n";
        return 0;
    } catch (const std::exception& ex) {
        std::cerr << "Error: " << ex.what() << "\n";
        return 1;
    }
}

int run_analyze_command(const std::string& input_path, bool allow_net, bool run_profiler) {
    try {
        const auto report = dryad::run_analyze_pass(
            input_path, default_compile_options(allow_net, run_profiler));
        std::cout << report.format();
        return 0;
    } catch (const std::exception& ex) {
        std::cerr << "Error: " << ex.what() << "\n";
        return 1;
    }
}

} // namespace

int main(int argc, char** argv) {
    if (argc < 2) {
        print_help();
        return 1;
    }

    const std::string arg = argv[1];
    if (arg == "--help" || arg == "-h") {
        print_help();
        return 0;
    }
    if (arg == "--version" || arg == "-v") {
        std::cout << "dryad 2.0.0\n";
        return 0;
    }
    if (arg == "check") {
        if (argc < 3) {
            std::cerr << "Error: check requires a file path\n";
            return 1;
        }
        const std::string file_path = argv[2];
        auto result = dryad::ProgramValidator::validate_file(file_path);
        for (const auto& diagnostic : result.diagnostics) {
            std::cerr << diagnostic.format(file_path) << "\n";
        }
        if (!result.ok) {
            return 1;
        }
        std::cout << "OK: " << argv[2] << "\n";
        return 0;
    }
    if (arg == "compile") {
        return run_compile_command(argc, argv);
    }
    if (arg == "install") {
        return run_install_command(argc, argv);
    }
    if (arg == "link") {
        return run_link_command(argc, argv);
    }
    if (arg == "analyze" || arg == "--analyze") {
        if (argc < 3) {
            std::cerr << "Error: analyze requires a file path\n";
            return 1;
        }
        bool allow_net = network_allowed_from_env();
        bool run_profiler = true;
        for (int i = 3; i < argc; ++i) {
            const std::string token = argv[i];
            if (token == "--allow-net") {
                allow_net = true;
            } else if (token == "--no-profile") {
                run_profiler = false;
            } else {
                std::cerr << "Error: unknown analyze option: " << token << "\n";
                return 1;
            }
        }
        return run_analyze_command(argv[2], allow_net, run_profiler);
    }

    bool allow_net = network_allowed_from_env();
    bool use_vm = false;
    std::string script_path;

    for (int i = 1; i < argc; ++i) {
        const std::string token = argv[i];
        if (token == "--allow-net") {
            allow_net = true;
            continue;
        }
        if (token == "--vm") {
            use_vm = true;
            continue;
        }
        if (token.rfind("--", 0) == 0) {
            std::cerr << "Error: unknown option: " << token << "\n";
            return 1;
        }
        script_path = token;
        break;
    }

    if (script_path.empty()) {
        std::cerr << "Error: script path is required\n";
        print_help();
        return 1;
    }

    try {
        dryad::Interpreter interpreter;
        interpreter.set_stdlib_root(stdlib_root());
        interpreter.set_network_allowed(allow_net);
        if (use_vm) {
            interpreter.set_default_execution_mode(dryad::ExecutionMode::Bytecode);
        }
        interpreter.run_file(script_path);
        return 0;
    } catch (const dryad::DryadException& ex) {
        std::cerr << "Error: " << ex.what() << "\n";
        return 1;
    } catch (const std::exception& ex) {
        std::cerr << "Error: " << ex.what() << "\n";
        return 1;
    }
}
