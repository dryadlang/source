#include "dryad/compiler/compile_pipeline.h"
#include "dryad/compiler/bytecode_compiler.h"
#include "dryad/compiler/import_cycle_detector.h"
#include "dryad/compiler/interpreter.h"
#include "dryad/runtime/module_loader.h"
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"
#include "dryad/common/utils.h"
#include <fstream>
#include <sstream>

namespace dryad {

std::unique_ptr<Program> parse_file(const std::string& path) {
    std::ifstream file(path);
    if (!file) {
        throw DryadException("Cannot open file: " + path);
    }
    std::ostringstream buffer;
    buffer << file.rdbuf();

    Lexer lexer(buffer.str());
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();
    if (parser.has_error()) {
        throw DryadException("Parser error: " + parser.error_message());
    }
    return program;
}

CompiledProgram compile_file_to_bytecode(const std::string& path, const CompileOptions& options) {
    auto program = parse_file(path);

    ModuleLoader cycle_loader(options.stdlib_root.empty() ? "stdlib" : options.stdlib_root);
    if (!options.library_paths.empty()) {
        cycle_loader.set_library_paths(options.library_paths);
    }
    cycle_loader.push_import_context(path);
    const ImportCycleReport cycle_report = detect_import_cycles(path, cycle_loader);
    cycle_loader.pop_import_context();
    if (cycle_report.has_cycle) {
        std::string message = "Circular import detected:";
        for (const auto& node : cycle_report.cycle) {
            message += " -> " + node;
        }
        throw DryadException(message);
    }

    Interpreter interpreter;
    if (!options.stdlib_root.empty()) {
        interpreter.set_stdlib_root(options.stdlib_root);
    }
    if (!options.library_paths.empty()) {
        interpreter.set_library_paths(options.library_paths);
    }
    interpreter.set_network_allowed(options.allow_network);
    interpreter.module_loader().push_import_context(path);

    if (options.run_profiler) {
        try {
            interpreter.execute(program.get());
        } catch (...) {
            interpreter.module_loader().pop_import_context();
            throw;
        }
    }
    interpreter.module_loader().pop_import_context();

    BytecodeCompiler compiler;
    compiler.set_interpreter(&interpreter);
    return compiler.compile_program(program.get(), interpreter.type_profiler());
}

} // namespace dryad
