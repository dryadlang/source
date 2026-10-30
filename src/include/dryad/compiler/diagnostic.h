#ifndef DRYAD_COMPILER_DIAGNOSTIC_H
#define DRYAD_COMPILER_DIAGNOSTIC_H

#include "dryad/compiler/token.h"
#include <string>

namespace dryad {

enum class DiagnosticSeverity {
    Error,
    Warning
};

struct Diagnostic {
    SourceLocation location;
    DiagnosticSeverity severity;
    std::string message;

    Diagnostic(SourceLocation location, DiagnosticSeverity severity, std::string message)
        : location(location), severity(severity), message(std::move(message)) {}

    std::string format(const std::string& file = "") const;
};

} // namespace dryad

#endif // DRYAD_COMPILER_DIAGNOSTIC_H
