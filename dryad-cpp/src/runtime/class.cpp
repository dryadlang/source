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

std::shared_ptr<Function> DryadClass::find_getter(const std::string& property_name) {
    auto it = getters.find(property_name);
    if (it != getters.end()) {
        return it->second;
    }

    if (super_class) {
        return super_class->find_getter(property_name);
    }

    return nullptr;
}

std::shared_ptr<Function> DryadClass::find_setter(const std::string& property_name) {
    auto it = setters.find(property_name);
    if (it != setters.end()) {
        return it->second;
    }

    if (super_class) {
        return super_class->find_setter(property_name);
    }

    return nullptr;
}

void DryadClass::run_constructor(const std::shared_ptr<ClassInstance>& instance,
                                   const std::vector<Value>& args,
                                   Interpreter* interpreter) {
    if (!constructor) {
        return;
    }

    auto dryad_ctor = std::dynamic_pointer_cast<DryadFunction>(constructor);
    if (!dryad_ctor) {
        throw DryadException("Constructor must be a DryadFunction");
    }

    auto prev_class = interpreter->executing_class();
    bool prev_in_constructor = interpreter->in_constructor();
    auto ctor_env = std::make_shared<Environment>(interpreter->current_env());
    ctor_env->define("this", Value(instance));
    interpreter->set_executing_class(shared_from_this());
    interpreter->set_in_constructor(true);

    for (size_t i = 0; i < args.size() && i < dryad_ctor->declaration()->parameters.size(); ++i) {
        ctor_env->define(dryad_ctor->declaration()->parameters[i].name, args[i]);
    }

    try {
        interpreter->execute_block(dryad_ctor->declaration()->body->statements, ctor_env);
    } catch (const ReturnException&) {
        // Constructors ignore return values
    } catch (...) {
        interpreter->set_executing_class(prev_class);
        interpreter->set_in_constructor(prev_in_constructor);
        throw;
    }
    interpreter->set_executing_class(prev_class);
    interpreter->set_in_constructor(prev_in_constructor);
}

bool DryadClass::is_readonly_property(const std::string& property_name) const {
    if (readonly_properties.count(property_name) != 0) {
        return true;
    }
    if (super_class) {
        return super_class->is_readonly_property(property_name);
    }
    return false;
}

bool DryadClass::has_member(const std::string& member_name) const {
    if (methods.count(member_name) != 0 || properties.count(member_name) != 0 ||
        getters.count(member_name) != 0 || setters.count(member_name) != 0) {
        return true;
    }
    if (super_class) {
        return super_class->has_member(member_name);
    }
    return false;
}

std::string DryadClass::find_property_type(const std::string& property_name) const {
    auto it = property_types.find(property_name);
    if (it != property_types.end()) {
        return it->second;
    }
    if (super_class) {
        return super_class->find_property_type(property_name);
    }
    return "";
}

std::shared_ptr<ClassInstance> DryadClass::instantiate(const std::vector<Value>& args, Interpreter* interpreter) {
    if (is_interface) {
        throw DryadException("Cannot instantiate interface: " + name);
    }
    if (is_abstract || !abstract_methods.empty() || !abstract_properties.empty()) {
        throw DryadException("Cannot instantiate abstract class: " + name);
    }

    auto instance = std::make_shared<ClassInstance>(shared_from_this());
    run_constructor(instance, args, interpreter);
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
