#ifndef DRYAD_RUNTIME_PROMISE_RUNTIME_H
#define DRYAD_RUNTIME_PROMISE_RUNTIME_H

#include "dryad/runtime/value.h"
#include <string>

namespace dryad {

class Interpreter;

class PromiseRuntime {
public:
    static void ensure_loaded(Interpreter* interpreter);
    static Value resolve(Interpreter* interpreter, const Value& value);
    static Value reject(Interpreter* interpreter, const Value& error);
    static Value await_value(Interpreter* interpreter, const Value& promise);

    static bool is_promise(const Value& value);
    static std::string state(const Value& promise);
    static Value value(const Value& promise);
    static Value error(const Value& promise);

private:
    static Value call_export(Interpreter* interpreter, const std::string& name,
                             const std::vector<Value>& args);
};

} // namespace dryad

#endif // DRYAD_RUNTIME_PROMISE_RUNTIME_H
