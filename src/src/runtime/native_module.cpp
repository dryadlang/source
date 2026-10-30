#include "dryad/runtime/native_module.h"
#include "dryad/runtime/bytecode_cache.h"
#include "dryad/common/utils.h"
#include <dlfcn.h>

namespace dryad {

namespace {

using NativeFn0 = void* (*)();
using NativeFn1 = void* (*)(void*);
using NativeFn2 = void* (*)(void*, void*);
using NativeFn3 = void* (*)(void*, void*, void*);
using NativeFn4 = void* (*)(void*, void*, void*, void*);

struct DryadBox {
    int tag;
    int64_t i;
    double f;
    unsigned char b;
};

std::string sanitize_symbol(const std::string& name) {
    std::string out = name;
    for (char& ch : out) {
        if (!((ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') ||
              (ch >= '0' && ch <= '9'))) {
            ch = '_';
        }
    }
    if (out.empty()) {
        return "anonymous";
    }
    return out;
}

Value value_from_box(void* raw) {
    if (!raw) {
        return Value();
    }
    const DryadBox* box = static_cast<const DryadBox*>(raw);
    switch (box->tag) {
        case 1:
            return Value(box->i);
        case 2:
            return Value(box->f);
        case 3:
            return Value(box->b != 0);
        default:
            return Value();
    }
}

void* box_value(const Value& value) {
    DryadBox* box = new DryadBox();
    if (value.is_integer()) {
        box->tag = 1;
        box->i = value.as_integer();
    } else if (value.is_float()) {
        box->tag = 2;
        box->f = value.as_float();
    } else if (value.is_boolean()) {
        box->tag = 3;
        box->b = value.as_boolean() ? 1 : 0;
    } else {
        box->tag = 0;
    }
    return box;
}

void* invoke_native(void* sym, const std::vector<void*>& args) {
    switch (args.size()) {
        case 0:
            return reinterpret_cast<NativeFn0>(sym)();
        case 1:
            return reinterpret_cast<NativeFn1>(sym)(args[0]);
        case 2:
            return reinterpret_cast<NativeFn2>(sym)(args[0], args[1]);
        case 3:
            return reinterpret_cast<NativeFn3>(sym)(args[0], args[1], args[2]);
        case 4:
            return reinterpret_cast<NativeFn4>(sym)(args[0], args[1], args[2], args[3]);
        default:
            throw DryadException("NativeModule: unsupported argument count " +
                                 std::to_string(args.size()) + " (max 4)");
    }
}

} // namespace

NativeModule::NativeModule() = default;

NativeModule::~NativeModule() {
    unload();
}

bool NativeModule::try_load_companion(const std::string& source_path) {
    return try_load_file(companion_native_path(source_path));
}

bool NativeModule::try_load_file(const std::string& path) {
    unload();
    handle_ = dlopen(path.c_str(), RTLD_LAZY | RTLD_LOCAL);
    return handle_ != nullptr;
}

bool NativeModule::has_function(const std::string& name) const {
    return resolve_symbol(name) != nullptr;
}

void* NativeModule::resolve_symbol(const std::string& name) const {
    if (!handle_) {
        return nullptr;
    }
    const std::string symbol = sanitize_symbol(name);
    void* sym = dlsym(handle_, symbol.c_str());
    if (!sym) {
        sym = dlsym(handle_, name.c_str());
    }
    return sym;
}

Value NativeModule::invoke(const std::string& name, const std::vector<Value>& args) {
    if (!handle_) {
        throw DryadException("NativeModule: not loaded");
    }

    void* sym = resolve_symbol(name);
    if (!sym) {
        throw DryadException("NativeModule: symbol not found: " + name);
    }

    std::vector<void*> boxed_args;
    boxed_args.reserve(args.size());
    for (const auto& arg : args) {
        boxed_args.push_back(box_value(arg));
    }

    void* raw_result = invoke_native(sym, boxed_args);

    for (void* boxed : boxed_args) {
        delete static_cast<DryadBox*>(boxed);
    }

    return value_from_box(raw_result);
}

void NativeModule::unload() {
    if (handle_) {
        dlclose(handle_);
        handle_ = nullptr;
    }
}

} // namespace dryad
