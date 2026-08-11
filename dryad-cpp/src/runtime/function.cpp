#include "dryad/runtime/function.h"
#include "dryad/compiler/interpreter.h"
#include "dryad/common/utils.h"

namespace dryad {

Value DryadFunction::call(Interpreter* interp, const std::vector<Value>& args) {
    if (args.size() != declaration_->parameters.size()) {
        throw DryadException("Function expects " + std::to_string(declaration_->parameters.size()) + 
                           " arguments, got " + std::to_string(args.size()));
    }
    
    auto function_env = std::make_shared<Environment>(closure_);
    
    for (size_t i = 0; i < args.size(); ++i) {
        function_env->define(declaration_->parameters[i].name, args[i]);
    }
    
    try {
        interp->execute_block(declaration_->body->statements, function_env);
    } catch (const ReturnException& ret) {
        return ret.value;
    }
    
    return Value();
}

} // namespace dryad