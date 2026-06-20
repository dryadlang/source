#ifndef DRYAD_RUNTIME_FUNCTION_H
#define DRYAD_RUNTIME_FUNCTION_H

#include "dryad/runtime/value.h"
#include "dryad/runtime/environment.h"
#include "dryad/compiler/ast.h"
#include <memory>
#include <functional>
#include <vector>

namespace dryad {

class Function {
public:
    virtual ~Function() = default;
    virtual Value call(class Interpreter* interp, const std::vector<Value>& args) = 0;
};

class NativeFunction : public Function {
public:
    using NativeFn = std::function<Value(const std::vector<Value>&)>;
    
    explicit NativeFunction(NativeFn fn) : fn_(std::move(fn)) {}
    
    Value call(Interpreter* interp, const std::vector<Value>& args) override {
        return fn_(args);
    }
    
private:
    NativeFn fn_;
};

class DryadFunction : public Function {
public:
    DryadFunction(FunctionDeclaration* decl, std::shared_ptr<Environment> closure)
        : declaration_(decl), closure_(std::move(closure)) {}
    
    Value call(Interpreter* interp, const std::vector<Value>& args) override;
    
    FunctionDeclaration* declaration() { return declaration_; }
    std::shared_ptr<Environment> closure() { return closure_; }
    
private:
    FunctionDeclaration* declaration_;
    std::shared_ptr<Environment> closure_;
};

} // namespace dryad

#endif // DRYAD_RUNTIME_FUNCTION_H