#ifndef DRYAD_AOT_LIBRARY_LINKER_H
#define DRYAD_AOT_LIBRARY_LINKER_H

#include "dryad/aot/aot_compiler.h"
#include "dryad/aot/llvm_backend.h"
#include "dryad/aot/micro_ir.h"
#include "dryad/runtime/library_module.h"
#include <string>
#include <unordered_map>
#include <vector>

namespace dryad {

std::string mangle_library_export(const std::string& package_name, const std::string& export_name);
std::string mangle_library_class_ctor(const std::string& package_name, const std::string& class_name);
std::string mangle_library_class_method(const std::string& package_name,
                                        const std::string& class_name,
                                        const std::string& method_name);
std::string library_native_companion_path(const std::string& dyl_path);
IrModule library_to_ir_module(const CompiledLibrary& library);
void merge_ir_modules(IrModule& target, const IrModule& source);
std::unordered_map<std::string, std::string> build_linked_export_map(const CompiledLibrary& library);
void merge_linked_export_maps(std::unordered_map<std::string, std::string>& target,
                              const std::unordered_map<std::string, std::string>& source);
std::vector<CompiledLibrary> load_link_libraries(const std::vector<std::string>& paths);
IrModule link_libraries_into_module(const IrModule& app_module,
                                    const std::vector<std::string>& library_paths,
                                    IrEmitOptions& emit_options);

AotCompileResult compile_library_to_llvm_ir(const std::string& dyl_path,
                                             const std::string& llvm_output_path);

AotCompileResult compile_library_to_native(const std::string& dyl_path,
                                            const std::string& output_path,
                                            const std::string& aot_runtime_source,
                                            const std::string& include_dir);

} // namespace dryad

#endif // DRYAD_AOT_LIBRARY_LINKER_H
