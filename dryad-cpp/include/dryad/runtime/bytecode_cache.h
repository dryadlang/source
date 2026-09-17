#ifndef DRYAD_RUNTIME_BYTECODE_CACHE_H
#define DRYAD_RUNTIME_BYTECODE_CACHE_H

#include "dryad/runtime/bytecode_module.h"
#include <string>

namespace dryad {

std::string companion_dryadc_path(const std::string& source_path);
std::string companion_native_path(const std::string& source_path);

class BytecodeModuleCache {
public:
    void set_source_path(const std::string& source_path);
    const std::string& source_path() const { return source_path_; }

    bool try_load_companion();
    bool try_load_file(const std::string& path);
    bool is_loaded() const { return loaded_; }
    const CompiledProgram& program() const;
    void clear();

private:
    std::string source_path_;
    CompiledProgram program_;
    bool loaded_ = false;
};

} // namespace dryad

#endif // DRYAD_RUNTIME_BYTECODE_CACHE_H
