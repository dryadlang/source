#include "dryad/compiler/diagnostic.h"
#include <sstream>

namespace dryad {

std::string Diagnostic::format(const std::string& file) const {
    std::ostringstream stream;
    if (!file.empty()) {
        stream << file << ':';
    }
    stream << location.line << ':' << location.column << ": ";
    stream << (severity == DiagnosticSeverity::Warning ? "warning: " : "error: ");
    stream << message;
    return stream.str();
}

} // namespace dryad
