#ifndef DRYAD_COMPILER_PROGRAM_ENTRY_H
#define DRYAD_COMPILER_PROGRAM_ENTRY_H

#include "dryad/compiler/ast.h"
#include <memory>

namespace dryad {

constexpr const char* kScriptEntryFunctionName = "__dryad_main";

bool is_script_level_statement(ASTNodeType type);
bool program_has_user_functions(const Program* program);
bool program_has_script_body(const Program* program);
std::unique_ptr<FunctionDeclaration> synthesize_script_entry_function(Program* program);

} // namespace dryad

#endif // DRYAD_COMPILER_PROGRAM_ENTRY_H
