#ifndef DRYAD_RUNTIME_INTRINSICS_REGISTRY_H
#define DRYAD_RUNTIME_INTRINSICS_REGISTRY_H

#include "dryad/runtime/value.h"
#include <string>
#include <functional>
#include <unordered_map>
#include <vector>

namespace dryad {

using IntrinsicFunction = std::function<Value(const std::vector<Value>&)>;

class IntrinsicsRegistry {
public:
    static IntrinsicsRegistry& instance();
    
    void register_intrinsic(const std::string& name, IntrinsicFunction func);
    Value call(const std::string& name, const std::vector<Value>& args);
    bool has(const std::string& name) const;
    
    void register_file_io_intrinsics();
    void register_network_intrinsics();
    void register_filesystem_intrinsics();
    void register_process_intrinsics();
    void register_memory_intrinsics();
    void register_time_intrinsics();
    void register_misc_intrinsics();
    void register_all();
    
private:
    IntrinsicsRegistry() = default;
    std::unordered_map<std::string, IntrinsicFunction> intrinsics_;
};

} // namespace dryad

#endif // DRYAD_RUNTIME_INTRINSICS_REGISTRY_H