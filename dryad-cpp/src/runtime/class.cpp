#include "dryad/runtime/class.h"
#include "dryad/runtime/value.h"
#include "dryad/runtime/function.h"
#include "dryad/compiler/interpreter.h"
#include "dryad/common/utils.h"

namespace dryad {

std::shared_ptr<Function> DryadClass::find_method(const std::string& method_name) {
    auto it = methods.find(method_name);
    if (it != methods.end()) {
        return it->second;
    }
    
    if (super_class) {
        return super_class->find_method(method_name);
    }
    
    return nullptr;
}

std::shared_ptr<ClassInstance> DryadClass::instantiate(const std::vector<Value>& args, Interpreter* interpreter) {
    auto instance = std::make_shared<ClassInstance>(shared_from_this());
    
    if (constructor) {
        auto dryad_ctor = std::dynamic_pointer_cast<DryadFunction>(constructor);
        if (!dryad_ctor) {
            throw DryadException("Constructor must be a DryadFunction");
        }
        
        auto prev_env = interpreter->current_env();
        auto ctor_env = std::make_shared<Environment>(prev_env);
        ctor_env->define("this", Value(instance));
        
        for (size_t i = 0; i < args.size() && i < dryad_ctor->declaration()->parameters.size(); ++i) {
            ctor_env->define(dryad_ctor->declaration()->parameters[i].name, args[i]);
        }
        
        try {
            interpreter->execute_block(dryad_ctor->declaration()->body->statements, ctor_env);
        } catch (const ReturnException&) {
            // Constructors ignore return values
        }
    }
    
    return instance;
}

Value ClassInstance::get(const std::string& name) {
    auto it = fields.find(name);
    if (it != fields.end()) {
        return it->second;
    }
    
    auto method = klass->find_method(name);
    if (method) {
        return Value(method);
    }
    
    return Value();
}

void ClassInstance::set(const std::string& name, const Value& value) {
    fields[name] = value;
}

} // namespace dryad
