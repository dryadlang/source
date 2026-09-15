#include "dryad/compiler/interpreter.h"
#include "dryad/compiler/validator.h"
#include "dryad/common/utils.h"
#include <cstdlib>
#include <iostream>
#include <string>

#ifndef DRYAD_STDLIB_DIR
#define DRYAD_STDLIB_DIR "stdlib"
#endif

namespace {

void print_help() {
    std::cout << "Dryad Programming Language v2.0.0\n\n";
    std::cout << "Usage:\n";
    std::cout << "  dryad <script.dryad>   Run a Dryad script\n";
    std::cout << "  dryad check <file>     Validate syntax without executing\n";
    std::cout << "  dryad --help           Show this help\n";
    std::cout << "  dryad --version        Show version information\n\n";
    std::cout << "Environment:\n";
    std::cout << "  DRYAD_STDLIB_DIR       Override stdlib path (default: "
              << DRYAD_STDLIB_DIR << ")\n";
}

std::string stdlib_root() {
    const char* env = std::getenv("DRYAD_STDLIB_DIR");
    if (env != nullptr && env[0] != '\0') {
        return env;
    }
    return DRYAD_STDLIB_DIR;
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

    try {
        dryad::Interpreter interpreter;
        interpreter.set_stdlib_root(stdlib_root());
        interpreter.run_file(arg);
        return 0;
    } catch (const dryad::DryadException& ex) {
        std::cerr << "Error: " << ex.what() << "\n";
        return 1;
    } catch (const std::exception& ex) {
        std::cerr << "Error: " << ex.what() << "\n";
        return 1;
    }
}
