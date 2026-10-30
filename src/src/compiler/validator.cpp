#include "dryad/compiler/validator.h"
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"
#include "dryad/compiler/semantic_analyzer.h"
#include <fstream>
#include <sstream>

namespace dryad {

std::string ValidationResult::message() const {
    if (diagnostics.empty()) {
        return "";
    }
    std::ostringstream stream;
    for (size_t i = 0; i < diagnostics.size(); ++i) {
        if (i > 0) {
            stream << '\n';
        }
        stream << diagnostics[i].format();
    }
    return stream.str();
}

std::vector<std::string> ValidationResult::errors() const {
    std::vector<std::string> messages;
    for (const auto& diagnostic : diagnostics) {
        messages.push_back(diagnostic.format());
    }
    return messages;
}

ValidationResult ProgramValidator::validate_source(const std::string& source) {
    ValidationResult result;

    Lexer lexer(source);
    auto tokens = lexer.tokenize();

    Parser parser(std::move(tokens));
    auto program = parser.parse();

    for (const auto& diagnostic : parser.diagnostics()) {
        result.ok = false;
        result.diagnostics.push_back(diagnostic);
    }

    if (!program) {
        result.ok = false;
        result.diagnostics.emplace_back(SourceLocation(), DiagnosticSeverity::Error,
                                        "Parser produced no program");
        return result;
    }

    SemanticAnalyzer analyzer;
    auto semantic_diagnostics = analyzer.analyze(program.get());
    for (const auto& diagnostic : semantic_diagnostics) {
        if (diagnostic.severity == DiagnosticSeverity::Error) {
            result.ok = false;
        }
        result.diagnostics.push_back(diagnostic);
    }

    return result;
}

ValidationResult ProgramValidator::validate_file(const std::string& path) {
    std::ifstream file(path);
    if (!file) {
        ValidationResult result;
        result.ok = false;
        result.diagnostics.emplace_back(SourceLocation(), DiagnosticSeverity::Error,
                                        "Cannot open file: " + path);
        return result;
    }

    std::ostringstream buffer;
    buffer << file.rdbuf();
    return validate_source(buffer.str());
}

} // namespace dryad
