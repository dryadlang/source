#ifndef DRYAD_COMPILER_VALIDATOR_H
#define DRYAD_COMPILER_VALIDATOR_H

#include "dryad/compiler/diagnostic.h"
#include <string>
#include <vector>

namespace dryad {

struct ValidationResult {
    bool ok = true;
    std::vector<Diagnostic> diagnostics;

    std::string message() const;
    std::vector<std::string> errors() const;
};

class ProgramValidator {
public:
    static ValidationResult validate_source(const std::string& source);
    static ValidationResult validate_file(const std::string& path);
};

} // namespace dryad

#endif // DRYAD_COMPILER_VALIDATOR_H
