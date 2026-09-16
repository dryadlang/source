#include "dryad/runtime/function.h"
#include "dryad/runtime/class.h"
#include "dryad/compiler/interpreter.h"
#include "dryad/runtime/promise_runtime.h"
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
    const auto& parameters = declaration_->parameters;
    size_t required_count = 0;
    for (const auto& param : parameters) {
        if (!param.default_value) {
            required_count++;
        }
    }

    if (args.size() < required_count || args.size() > parameters.size()) {
        throw DryadException("Function expects between " + std::to_string(required_count) +
                           " and " + std::to_string(parameters.size()) +
                           " arguments, got " + std::to_string(args.size()));
    }

    auto function_env = std::make_shared<Environment>(closure_);
    auto previous_class = interp->executing_class();
    auto previous_env = interp->current_env();

    if (receiver) {
        function_env->define("this", Value(receiver));
    }
    if (home_class_) {
        interp->set_executing_class(home_class_);
    }

    for (size_t i = 0; i < parameters.size(); ++i) {
        Value arg_value;
        if (i < args.size()) {
            arg_value = args[i];
        } else if (parameters[i].default_value) {
            interp->set_current_env(closure_);
            arg_value = interp->evaluate(parameters[i].default_value.get());
            interp->set_current_env(previous_env);
        } else {
            throw DryadException("Missing argument for parameter '" + parameters[i].name + "'");
        }
        function_env->define(parameters[i].name, arg_value);
    }

    const bool is_async = declaration_->is_async;

    interp->increment_call_depth();
    try {
        interp->execute_block(declaration_->body->statements, function_env);
    } catch (const ReturnException& ret) {
        interp->decrement_call_depth();
        interp->set_executing_class(previous_class);
        interp->set_current_env(previous_env);
        if (is_async) {
            return PromiseRuntime::resolve(interp, ret.value);
        }
        return ret.value;
    } catch (const DryadException&) {
        interp->decrement_call_depth();
        interp->set_executing_class(previous_class);
        interp->set_current_env(previous_env);
        throw;
    } catch (...) {
        interp->decrement_call_depth();
        interp->set_executing_class(previous_class);
        interp->set_current_env(previous_env);
        throw;
    }

    interp->decrement_call_depth();
    interp->set_executing_class(previous_class);
    interp->set_current_env(previous_env);
    if (is_async) {
        return PromiseRuntime::resolve(interp, Value());
    }
    return Value();
}

} // namespace dryad