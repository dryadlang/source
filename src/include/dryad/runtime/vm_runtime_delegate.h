#ifndef DRYAD_RUNTIME_VM_RUNTIME_DELEGATE_H
#define DRYAD_RUNTIME_VM_RUNTIME_DELEGATE_H

#include "dryad/common/utils.h"
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
    virtual Value invoke_bytecode_function(uint16_t function_index,
                                           const std::vector<Value>& args) = 0;

    // Closure and first-class call support. Delegates that only run
    // capture-free code (e.g. compiled libraries) can keep these defaults.
    virtual Value invoke_bytecode_closure(uint16_t function_index,
                                          const std::vector<Value>& args,
                                          const std::vector<Value>& upvalues) {
        if (upvalues.empty()) {
            return invoke_bytecode_function(function_index, args);
        }
        throw DryadException("MAKE_CLOSURE is not supported by this runtime");
    }
    virtual Value invoke_value(const Value& callee, const std::vector<Value>& args) {
        (void)callee;
        (void)args;
        throw DryadException("CALL_VALUE is not supported by this runtime");
    }
    virtual void set_global(const std::string& name, const Value& value) {
        (void)name;
        (void)value;
        throw DryadException("STORE_GLOBAL is not supported by this runtime");
    }
};

} // namespace dryad

#endif // DRYAD_RUNTIME_VM_RUNTIME_DELEGATE_H
