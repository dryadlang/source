#include "dryad/runtime/function.h"
#include "dryad/runtime/class.h"
#include "dryad/compiler/interpreter.h"
#include "dryad/common/utils.h"

namespace dryad {

Value DryadFunction::call(Interpreter* interp, const std::vector<Value>& args) {
    return call_impl(interp, args, nullptr);
}

Value DryadFunction::call_with_receiver(Interpreter* interp, const std::vector<Value>& args,
                                        std::shared_ptr<ClassInstance> receiver) {
    return call_impl(interp, args, std::move(receiver));
}

Value DryadFunction::call_impl(Interpreter* interp, const std::vector<Value>& args,
                               std::shared_ptr<ClassInstance> receiver) {
    if (args.size() != declaration_->parameters.size()) {
        throw DryadException("Function expects " + std::to_string(declaration_->parameters.size()) +
                           " arguments, got " + std::to_string(args.size()));
    }

    auto function_env = std::make_shared<Environment>(closure_);
    auto previous_class = interp->executing_class();

    if (receiver) {
        function_env->define("this", Value(receiver));
    }
    if (home_class_) {
        interp->set_executing_class(home_class_);
    }

    for (size_t i = 0; i < args.size(); ++i) {
        function_env->define(declaration_->parameters[i].name, args[i]);
    }

    interp->increment_call_depth();
    try {
        interp->execute_block(declaration_->body->statements, function_env);
    } catch (const ReturnException& ret) {
        interp->decrement_call_depth();
        interp->set_executing_class(previous_class);
        return ret.value;
    } catch (...) {
        interp->decrement_call_depth();
        interp->set_executing_class(previous_class);
        throw;
    }

    interp->decrement_call_depth();
    interp->set_executing_class(previous_class);
    return Value();
}

} // namespace dryad