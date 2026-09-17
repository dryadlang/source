#ifndef DRYAD_COMPILER_LIBRARY_COMPILER_H
#define DRYAD_COMPILER_LIBRARY_COMPILER_H

#include "dryad/compiler/compile_pipeline.h"
#include "dryad/runtime/library_module.h"
#include <string>

namespace dryad {

CompiledLibrary compile_library_file(const std::string& path, const CompileOptions& options);

} // namespace dryad

#endif // DRYAD_COMPILER_LIBRARY_COMPILER_H
