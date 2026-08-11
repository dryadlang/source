#ifndef DRYAD_RUNTIME_CLASS_H
#define DRYAD_RUNTIME_CLASS_H

#include <string>
#include <memory>
#include <unordered_map>
#include <vector>

namespace dryad {

class Value;
class Function;
class Environment;
class ClassInstance;

class DryadClass : public std::enable_shared_from_this<DryadClass> {
public:
    std::string name;
    std::shared_ptr<DryadClass> super_class;
    std::unordered_map<std::string, std::shared_ptr<Function>> methods;
    std::shared_ptr<Function> constructor;
    std::vector<std::unique_ptr<class FunctionDeclaration>> owned_declarations;
    
    DryadClass(std::string name, std::shared_ptr<DryadClass> super = nullptr)
        : name(std::move(name)), super_class(super), constructor(nullptr) {}
    
    std::shared_ptr<Function> find_method(const std::string& method_name);
    std::shared_ptr<ClassInstance> instantiate(const std::vector<Value>& args, class Interpreter* interpreter);
};

class ClassInstance {
public:
    std::shared_ptr<DryadClass> klass;
    std::unordered_map<std::string, Value> fields;
    
    ClassInstance(std::shared_ptr<DryadClass> klass)
        : klass(std::move(klass)) {}
    
    Value get(const std::string& name);
    void set(const std::string& name, const Value& value);
};

} // namespace dryad

#endif // DRYAD_RUNTIME_CLASS_H
