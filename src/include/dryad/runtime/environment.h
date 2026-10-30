#ifndef DRYAD_RUNTIME_ENVIRONMENT_H
#define DRYAD_RUNTIME_ENVIRONMENT_H

#include "dryad/runtime/value.h"
#include <unordered_map>
#include <string>
#include <memory>

namespace dryad {

class Environment {
public:
    Environment();
    explicit Environment(std::shared_ptr<Environment> parent);
    
    void define(const std::string& name, const Value& value);
    Value get(const std::string& name) const;
    void set(const std::string& name, const Value& value);
    void assign(const std::string& name, const Value& value);
    bool has(const std::string& name) const;
    
private:
    std::unordered_map<std::string, Value> values_;
    std::shared_ptr<Environment> parent_;
};

} // namespace dryad

#endif // DRYAD_RUNTIME_ENVIRONMENT_H