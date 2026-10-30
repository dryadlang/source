#ifndef DRYAD_RUNTIME_LIBRARY_MODULE_H
#define DRYAD_RUNTIME_LIBRARY_MODULE_H

#include "dryad/runtime/bytecode_module.h"
#include "dryad/runtime/value.h"
#include <cstdint>
#include <memory>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>

namespace dryad {

class Interpreter;
class Environment;
class DryadClass;
class ClassInstance;
class Function;
class VmRuntimeDelegate;
class LibraryDryadFunction;

struct ModuleMetadata {
    std::unordered_set<std::string> exports;
    std::unordered_set<std::string> internal_symbols;
    bool has_explicit_exports = false;
};

enum class LibraryExportKind : uint8_t {
    Function = 0,
    Class = 1,
    Value = 2
};

struct LibraryClassMethod {
    std::string name;
    uint32_t function_index = 0;
};

struct LibraryClassExport {
    std::string name;
    uint32_t constructor_index = 0;
    std::vector<LibraryClassMethod> methods;
};

struct LibraryFunctionExport {
    std::string name;
    uint32_t function_index = 0;
};

struct LibraryValueExport {
    std::string name;
    Value value;
};

struct CompiledLibrary {
    std::string package_name;
    std::string package_version;
    std::string source_path;
    ModuleMetadata metadata;
    CompiledProgram program;
    std::vector<LibraryFunctionExport> functions;
    std::vector<LibraryClassExport> classes;
    std::vector<LibraryValueExport> values;
};

struct LibraryMaterialization {
    std::shared_ptr<Environment> environment;
    std::shared_ptr<CompiledLibrary> library;
    std::shared_ptr<VmRuntimeDelegate> runtime_delegate;
};

bool save_compiled_library(const CompiledLibrary& library, const std::string& path);
bool load_compiled_library(const std::string& path, CompiledLibrary& library);
LibraryMaterialization materialize_library(CompiledLibrary& library, Interpreter* interpreter);

std::string default_dyl_output(const std::string& input_path);
std::string library_path_for_specifier(const std::string& specifier);
bool library_file_is_fresh(const std::string& library_path, const std::string& source_path);
bool try_invoke_library_constructor(const std::shared_ptr<Function>& constructor,
                                    const std::shared_ptr<ClassInstance>& instance,
                                    const std::vector<Value>& args,
                                    Interpreter* interpreter);

} // namespace dryad

#endif // DRYAD_RUNTIME_LIBRARY_MODULE_H
