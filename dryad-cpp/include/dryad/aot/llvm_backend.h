#ifndef DRYAD_AOT_LLVM_BACKEND_H
#define DRYAD_AOT_LLVM_BACKEND_H

#include "dryad/aot/micro_ir.h"
#include <string>

namespace dryad {

std::string emit_llvm_ir(const IrModule& module);
bool write_llvm_ir_file(const IrModule& module, const std::string& path);

} // namespace dryad

#endif // DRYAD_AOT_LLVM_BACKEND_H
