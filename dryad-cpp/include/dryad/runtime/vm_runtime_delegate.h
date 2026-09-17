#ifndef DRYAD_RUNTIME_VM_RUNTIME_DELEGATE_H
#define DRYAD_RUNTIME_VM_RUNTIME_DELEGATE_H

#include "dryad/runtime/value.h"
#include <string>
#include <vector>

namespace dryad {

class VmRuntimeDelegate {
public:
    virtual ~VmRuntimeDelegate() = default;

    virtual Value get_global(const std::string& name) = 0;
    virtual Value get_property(const Value& object, const std::string& name) = 0;
    virtual void set_property(const Value& object, const std::string& name, const Value& value) = 0;
    virtual Value invoke_method(const Value& receiver,
                                const std::string& method,
                                const std::vector<Value>& args) = 0;
    virtual Value new_instance(const Value& class_value, const std::vector<Value>& args) = 0;
    virtual Value invoke_global(const std::string& name, const std::vector<Value>& args) = 0;
};

} // namespace dryad

#endif // DRYAD_RUNTIME_VM_RUNTIME_DELEGATE_H
