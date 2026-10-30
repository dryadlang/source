#ifndef DRYAD_RUNTIME_INTRINSICS_REGISTRY_H
#define DRYAD_RUNTIME_INTRINSICS_REGISTRY_H

#include "dryad/runtime/value.h"
#include <cstdint>
#include <functional>
#include <string>
#include <unordered_map>
#include <utility>
#include <vector>

namespace dryad {

using IntrinsicFunction = std::function<Value(const std::vector<Value>&)>;

class IntrinsicsRegistry {
public:
    static IntrinsicsRegistry& instance();
    
    void register_intrinsic(const std::string& name, IntrinsicFunction func);
    Value call(const std::string& name, const std::vector<Value>& args);
    Value call_by_id(uint16_t id, const std::vector<Value>& args);
    uint16_t get_id(const std::string& name) const;
    std::vector<std::pair<uint16_t, std::string>> list_all_intrinsics() const;
    bool has(const std::string& name) const;
    
    void register_file_io_intrinsics();
    void register_network_intrinsics();
    void register_tls_intrinsics();
    void register_filesystem_intrinsics();
    void register_process_intrinsics();
    void register_memory_intrinsics();
    void register_string_intrinsics();
    void register_time_intrinsics();
    void register_misc_intrinsics();
    void register_thread_intrinsics();
    void register_event_loop_intrinsics();
    void register_poller_intrinsics();
    void register_all();

    void set_network_allowed(bool allowed) { network_allowed_ = allowed; }
    bool network_allowed() const { return network_allowed_; }
    void require_network_access() const;
    
private:
    IntrinsicsRegistry() = default;
    std::unordered_map<std::string, IntrinsicFunction> intrinsics_;
    std::unordered_map<uint16_t, IntrinsicFunction> functions_by_id_;
    std::unordered_map<std::string, uint16_t> name_to_id_;
    uint16_t next_id_ = 0;
    bool network_allowed_ = false;
};

} // namespace dryad

#endif // DRYAD_RUNTIME_INTRINSICS_REGISTRY_H