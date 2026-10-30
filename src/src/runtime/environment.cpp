#include "dryad/runtime/environment.h"
#include "dryad/common/utils.h"

namespace dryad {

Environment::Environment() : parent_(nullptr) {}

Environment::Environment(std::shared_ptr<Environment> parent)
    : parent_(std::move(parent)) {}

void Environment::define(const std::string& name, const Value& value) {
    values_[name] = value;
}

Value Environment::get(const std::string& name) const {
    auto it = values_.find(name);
    if (it != values_.end()) {
        return it->second;
    }
    
    if (parent_) {
        return parent_->get(name);
    }
    
    throw DryadException("Undefined variable '" + name + "'");
}

void Environment::set(const std::string& name, const Value& value) {
    auto it = values_.find(name);
    if (it != values_.end()) {
        it->second = value;
        return;
    }
    
    if (parent_) {
        parent_->set(name, value);
        return;
    }
    
    throw DryadException("Undefined variable '" + name + "'");
}

void Environment::assign(const std::string& name, const Value& value) {
    set(name, value);
}

bool Environment::has(const std::string& name) const {
    if (values_.find(name) != values_.end()) {
        return true;
    }
    
    if (parent_) {
        return parent_->has(name);
    }
    
    return false;
}

} // namespace dryad