#ifndef DRYAD_COMPILER_CLOSURE_ANALYSIS_H
#define DRYAD_COMPILER_CLOSURE_ANALYSIS_H

#include "dryad/compiler/ast.h"
#include <set>
#include <string>
#include <vector>

namespace dryad {

// Name resolution for bytecode closures. Scoping is flat per function, matching
// the bytecode compiler's slot map: every parameter and `let` anywhere in a
// function body (outside nested function expressions) is one local of it.

std::vector<Statement*> block_statements_of(const BlockStatement* block);

// Parameters plus every variable declared in the body.
std::set<std::string> declared_names(const std::vector<Parameter>& parameters,
                                     const std::vector<Statement*>& body);

// Names a function reads or assigns, including those its nested lambdas need,
// that it does not declare itself. Sorted, so capture order is deterministic.
std::vector<std::string> free_names(const std::vector<Parameter>& parameters,
                                    const std::vector<Statement*>& body);

// Locals of this function that some directly nested lambda needs. These must
// live in shared cells so closures and the function see the same binding.
std::set<std::string> captured_names(const std::vector<Parameter>& parameters,
                                     const std::vector<Statement*>& body);

} // namespace dryad

#endif // DRYAD_COMPILER_CLOSURE_ANALYSIS_H
