#ifndef DRYAD_RUNTIME_NATIVE_MODULE_H
#define DRYAD_RUNTIME_NATIVE_MODULE_H

#include "dryad/runtime/value.h"
#include <string>
#include <vector>

namespace dryad {

class NativeModule {
public:
    NativeModule();
    ~NativeModule();

    NativeModule(const NativeModule&) = delete;
    NativeModule& operator=(const NativeModule&) = delete;

    bool try_load_companion(const std::string& source_path);
    bool try_load_file(const std::string& path);
    bool is_loaded() const { return handle_ != nullptr; }
    bool has_function(const std::string& name) const;

    Value invoke(const std::string& name, const std::vector<Value>& args);

    void unload();

private:
    void* handle_ = nullptr;

    void* resolve_symbol(const std::string& name) const;
};

} // namespace dryad

#endif // DRYAD_RUNTIME_NATIVE_MODULE_H
