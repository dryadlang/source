#ifndef DRYAD_AOT_LLVM_BACKEND_H
#define DRYAD_AOT_LLVM_BACKEND_H

#include "dryad/aot/micro_ir.h"
#include <string>

namespace dryad {

std::string emit_llvm_ir(const IrModule& module);
std::string append_executable_main(const std::string& ir, const IrFunction& entry);
bool write_llvm_ir_file(const IrModule& module, const std::string& path);
bool write_llvm_ir_executable_file(const IrModule& module,
                                   const std::string& path,
                                   const std::string& entry_symbol);

std::string emit_jit_loop_ir(const IrFunction& function, const std::string& symbol);
bool write_jit_loop_ir_file(const IrFunction& function,
                            const std::string& symbol,
                            const std::string& path);

} // namespace dryad

#endif // DRYAD_AOT_LLVM_BACKEND_H
