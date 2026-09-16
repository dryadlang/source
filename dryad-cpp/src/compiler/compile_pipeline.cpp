#include "dryad/compiler/compile_pipeline.h"
#include "dryad/compiler/bytecode_compiler.h"
#include "dryad/compiler/interpreter.h"
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
    Interpreter interpreter;
    if (!options.stdlib_root.empty()) {
        interpreter.set_stdlib_root(options.stdlib_root);
    }
    interpreter.set_network_allowed(options.allow_network);

    if (options.run_profiler) {
        interpreter.execute(program.get());
    }

    BytecodeCompiler compiler;
    return compiler.compile_program(program.get(), interpreter.type_profiler());
}

} // namespace dryad
