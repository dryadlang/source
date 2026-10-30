#include "dryad/runtime/library_module.h"
#include "dryad/compiler/interpreter.h"
#include "dryad/runtime/class.h"
#include "dryad/runtime/function.h"
#include "dryad/runtime/vm.h"
#include "dryad/runtime/vm_runtime_delegate.h"
#include "dryad/common/utils.h"
#include <algorithm>
#include <chrono>
#include <cstring>
#include <filesystem>
#include <fstream>
#include <sstream>

namespace dryad {

namespace {

constexpr char kMagic[] = "DRYADYL";
constexpr uint32_t kFormatVersion = 1;

void write_u32(std::vector<uint8_t>& out, uint32_t value) {
    for (int i = 0; i < 4; ++i) {
        out.push_back(static_cast<uint8_t>((value >> (i * 8)) & 0xFF));
    }
}

uint32_t read_u32(const std::vector<uint8_t>& data, size_t& offset) {
    if (offset + 4 > data.size()) {
        throw DryadException("dyl: truncated u32");
    }
    uint32_t value = 0;
    for (int i = 0; i < 4; ++i) {
        value |= static_cast<uint32_t>(data[offset + i]) << (i * 8);
    }
    offset += 4;
    return value;
}

void write_string(std::vector<uint8_t>& out, const std::string& value) {
    write_u32(out, static_cast<uint32_t>(value.size()));
    out.insert(out.end(), value.begin(), value.end());
}

std::string read_string(const std::vector<uint8_t>& data, size_t& offset) {
    const uint32_t len = read_u32(data, offset);
    if (offset + len > data.size()) {
        throw DryadException("dyl: truncated string");
    }
    std::string value(data.begin() + offset, data.begin() + offset + len);
    offset += len;
    return value;
}

void write_constant(std::vector<uint8_t>& out, const Value& value) {
    switch (value.type()) {
        case ValueType::Null:
            out.push_back(0);
            break;
        case ValueType::Boolean:
            out.push_back(1);
            out.push_back(value.as_boolean() ? 1 : 0);
            break;
        case ValueType::Integer:
            out.push_back(2);
            for (int i = 0; i < 8; ++i) {
                const int64_t v = value.as_integer();
                out.push_back(static_cast<uint8_t>((v >> (i * 8)) & 0xFF));
            }
            break;
        case ValueType::Float:
            out.push_back(3);
            for (int i = 0; i < 8; ++i) {
                const double d = value.as_float();
                int64_t bits = 0;
                std::memcpy(&bits, &d, sizeof(double));
                out.push_back(static_cast<uint8_t>((bits >> (i * 8)) & 0xFF));
            }
            break;
        case ValueType::String:
            out.push_back(4);
            write_string(out, value.as_string());
            break;
        default:
            throw DryadException("dyl: unsupported exported value type");
    }
}

Value read_constant(const std::vector<uint8_t>& data, size_t& offset) {
    if (offset >= data.size()) {
        throw DryadException("dyl: truncated constant tag");
    }
    const uint8_t tag = data[offset++];
    switch (tag) {
        case 0:
            return Value();
        case 1: {
            if (offset >= data.size()) {
                throw DryadException("dyl: truncated bool constant");
            }
            return Value(data[offset++] != 0);
        }
        case 2: {
            if (offset + 8 > data.size()) {
                throw DryadException("dyl: truncated integer constant");
            }
            int64_t value = 0;
            for (int i = 0; i < 8; ++i) {
                value |= static_cast<int64_t>(data[offset + i]) << (i * 8);
            }
            offset += 8;
            return Value(value);
        }
        case 3: {
            if (offset + 8 > data.size()) {
                throw DryadException("dyl: truncated float constant");
            }
            int64_t bits = 0;
            for (int i = 0; i < 8; ++i) {
                bits |= static_cast<int64_t>(data[offset + i]) << (i * 8);
            }
            offset += 8;
            double value = 0.0;
            std::memcpy(&value, &bits, sizeof(double));
            return Value(value);
        }
        case 4:
            return Value(read_string(data, offset));
        default:
            throw DryadException("dyl: unknown constant tag");
    }
}

std::vector<Parameter> parameters_from_slots(const std::vector<SlotMetadata>& slots) {
    std::vector<SlotMetadata> ordered = slots;
    std::sort(ordered.begin(), ordered.end(),
              [](const SlotMetadata& left, const SlotMetadata& right) {
                  return left.slot_index < right.slot_index;
              });

    std::vector<Parameter> parameters;
    for (const auto& slot : ordered) {
        if (slot.name == "this") {
            continue;
        }
        parameters.push_back(Parameter{slot.name, "", nullptr});
    }
    return parameters;
}

uint16_t slot_index_for_name(const CompiledFunction& function, const std::string& name) {
    for (const auto& slot : function.metadata.slots) {
        if (slot.name == name) {
            return slot.slot_index;
        }
    }
    throw DryadException("dyl: slot not found in bytecode metadata: " + name);
}

Value invoke_bytecode_with_slots(const CompiledProgram& program,
                                 uint16_t function_index,
                                 const std::unordered_map<std::string, Value>& slots,
                                 VmRuntimeDelegate* delegate) {
    if (function_index >= program.functions.size()) {
        throw DryadException("dyl: invalid function index");
    }

    VM vm;
    vm.load_program(program, function_index);
    vm.set_runtime_delegate(delegate);

    const auto& function = program.functions[function_index];
    for (const auto& entry : slots) {
        vm.set_slot(slot_index_for_name(function, entry.first), entry.second);
    }

    vm.execute();
    if (vm.stack_size() == 0) {
        return Value();
    }
    return vm.get_stack_top();
}

} // namespace

class LibraryVmRuntime final : public VmRuntimeDelegate {
public:
    LibraryVmRuntime(Interpreter* interpreter,
                     std::shared_ptr<Environment> module_env,
                     const CompiledProgram* program)
        : interpreter_(interpreter),
          module_env_(std::move(module_env)),
          program_(program) {}

    Value get_global(const std::string& name) override {
        if (module_env_->has(name)) {
            return module_env_->get(name);
        }
        return interpreter_->runtime_env()->get(name);
    }

    Value get_property(const Value& object, const std::string& name) override {
        if (object.is_instance()) {
            auto instance = object.as_instance();
            return instance->get(name);
        }
        if (object.is_object()) {
            return object.object_get(name);
        }
        throw DryadException("GET_PROPERTY requires object or instance");
    }

    void set_property(const Value& object, const std::string& name, const Value& value) override {
        if (object.is_instance()) {
            object.as_instance()->set(name, value);
            return;
        }
        if (object.is_object()) {
            const_cast<Value&>(object).object_set(name, value);
            return;
        }
        throw DryadException("SET_PROPERTY requires object or instance");
    }

    Value invoke_method(const Value& receiver,
                        const std::string& method,
                        const std::vector<Value>& args) override {
        if (!receiver.is_instance()) {
            throw DryadException("METHOD_CALL requires instance receiver");
        }
        auto instance = receiver.as_instance();
        Value method_val = instance->get(method);
        if (!method_val.is_function()) {
            throw DryadException("Cannot call non-function value: " + method);
        }
        auto dryad_func = std::dynamic_pointer_cast<DryadFunction>(method_val.as_function());
        if (dryad_func) {
            return dryad_func->call_with_receiver(interpreter_, args, instance);
        }
        return method_val.as_function()->call(interpreter_, args);
    }

    Value new_instance(const Value& class_value, const std::vector<Value>& args) override {
        if (!class_value.is_class()) {
            throw DryadException("NEW_INSTANCE requires class value");
        }
        return Value(class_value.as_class()->instantiate(args, interpreter_));
    }

    Value invoke_global(const std::string& name, const std::vector<Value>& args) override {
        Value fn_val = get_global(name);
        if (!fn_val.is_function()) {
            throw DryadException("CALL_GLOBAL: not a function: " + name);
        }
        auto dryad_func = std::dynamic_pointer_cast<DryadFunction>(fn_val.as_function());
        if (dryad_func) {
            return dryad_func->call(interpreter_, args);
        }
        return fn_val.as_function()->call(interpreter_, args);
    }

    Value invoke_bytecode_function(uint16_t function_index,
                                   const std::vector<Value>& args) override {
        std::unordered_map<std::string, Value> slots;
        const auto& function = program_->functions[function_index];
        size_t arg_index = 0;
        for (const auto& slot : function.metadata.slots) {
            if (slot.name == "this") {
                continue;
            }
            if (arg_index < args.size()) {
                slots[slot.name] = args[arg_index++];
            }
        }
        return invoke_bytecode_with_slots(*program_, function_index, slots, this);
    }

private:
    Interpreter* interpreter_;
    std::shared_ptr<Environment> module_env_;
    const CompiledProgram* program_;
};

class LibraryDryadFunction final : public DryadFunction {
public:
    LibraryDryadFunction(std::unique_ptr<FunctionDeclaration> decl,
                         std::shared_ptr<Environment> closure,
                         std::shared_ptr<DryadClass> home_class,
                         std::shared_ptr<CompiledLibrary> library,
                         uint16_t function_index,
                         std::shared_ptr<LibraryVmRuntime> runtime)
        : DryadFunction(std::move(decl), std::move(closure), std::move(home_class)),
          library_(std::move(library)),
          function_index_(function_index),
          runtime_(std::move(runtime)) {}

    Value call(Interpreter* interp, const std::vector<Value>& args) override {
        return invoke_with_slots(interp, args, nullptr);
    }

    Value call_with_receiver(Interpreter* interp,
                             const std::vector<Value>& args,
                             std::shared_ptr<ClassInstance> receiver) override {
        return invoke_with_slots(interp, args, std::move(receiver));
    }

    void invoke_constructor(const std::shared_ptr<ClassInstance>& instance,
                            const std::vector<Value>& args,
                            Interpreter* interp) {
        invoke_with_slots(interp, args, instance);
    }

private:
    Value invoke_with_slots(Interpreter* interp,
                            const std::vector<Value>& args,
                            const std::shared_ptr<ClassInstance>& receiver) {
        std::unordered_map<std::string, Value> slots;
        if (receiver) {
            slots["this"] = Value(receiver);
        }

        const auto& parameters = declaration()->parameters;
        for (size_t i = 0; i < parameters.size(); ++i) {
            if (i < args.size()) {
                slots[parameters[i].name] = args[i];
            }
        }

        return invoke_bytecode_with_slots(library_->program, function_index_, slots, runtime_.get());
    }

    std::shared_ptr<CompiledLibrary> library_;
    uint16_t function_index_;
    std::shared_ptr<LibraryVmRuntime> runtime_;
};

bool save_compiled_library(const CompiledLibrary& library, const std::string& path) {
    try {
        std::vector<uint8_t> out;
        out.insert(out.end(), kMagic, kMagic + 7);
        out.push_back(0);
        write_u32(out, kFormatVersion);

        write_string(out, library.package_name);
        write_string(out, library.package_version);
        write_string(out, library.source_path);

        out.push_back(library.metadata.has_explicit_exports ? 1 : 0);
        write_u32(out, static_cast<uint32_t>(library.metadata.exports.size()));
        for (const auto& name : library.metadata.exports) {
            write_string(out, name);
        }
        write_u32(out, static_cast<uint32_t>(library.metadata.internal_symbols.size()));
        for (const auto& name : library.metadata.internal_symbols) {
            write_string(out, name);
        }

        write_u32(out, static_cast<uint32_t>(library.values.size()));
        for (const auto& value_export : library.values) {
            write_string(out, value_export.name);
            write_constant(out, value_export.value);
        }

        write_u32(out, static_cast<uint32_t>(library.functions.size()));
        for (const auto& fn_export : library.functions) {
            write_string(out, fn_export.name);
            write_u32(out, fn_export.function_index);
        }

        write_u32(out, static_cast<uint32_t>(library.classes.size()));
        for (const auto& class_export : library.classes) {
            write_string(out, class_export.name);
            write_u32(out, class_export.constructor_index);
            write_u32(out, static_cast<uint32_t>(class_export.methods.size()));
            for (const auto& method : class_export.methods) {
                write_string(out, method.name);
                write_u32(out, method.function_index);
            }
        }

        const auto bytecode = serialize_program_to_bytes(library.program);
        write_u32(out, static_cast<uint32_t>(bytecode.size()));
        out.insert(out.end(), bytecode.begin(), bytecode.end());

        const auto parent = std::filesystem::path(path).parent_path();
        if (!parent.empty()) {
            std::filesystem::create_directories(parent);
        }

        std::ofstream file(path, std::ios::binary);
        if (!file) {
            return false;
        }
        file.write(reinterpret_cast<const char*>(out.data()),
                   static_cast<std::streamsize>(out.size()));
        return static_cast<bool>(file);
    } catch (...) {
        return false;
    }
}

bool load_compiled_library(const std::string& path, CompiledLibrary& library) {
    try {
        std::ifstream file(path, std::ios::binary);
        if (!file) {
            return false;
        }
        std::vector<uint8_t> data((std::istreambuf_iterator<char>(file)),
                                  std::istreambuf_iterator<char>());
        if (data.size() < 12 || std::memcmp(data.data(), kMagic, 7) != 0) {
            throw DryadException("dyl: invalid magic header");
        }

        size_t offset = 8;
        const uint32_t version = read_u32(data, offset);
        if (version != kFormatVersion) {
            throw DryadException("dyl: unsupported format version");
        }

        library.package_name = read_string(data, offset);
        library.package_version = read_string(data, offset);
        library.source_path = read_string(data, offset);

        library.metadata.has_explicit_exports = data[offset++] != 0;
        const uint32_t export_count = read_u32(data, offset);
        for (uint32_t i = 0; i < export_count; ++i) {
            library.metadata.exports.insert(read_string(data, offset));
        }
        const uint32_t internal_count = read_u32(data, offset);
        for (uint32_t i = 0; i < internal_count; ++i) {
            library.metadata.internal_symbols.insert(read_string(data, offset));
        }

        library.values.clear();
        const uint32_t value_count = read_u32(data, offset);
        for (uint32_t i = 0; i < value_count; ++i) {
            LibraryValueExport value_export;
            value_export.name = read_string(data, offset);
            value_export.value = read_constant(data, offset);
            library.values.push_back(std::move(value_export));
        }

        library.functions.clear();
        const uint32_t function_count = read_u32(data, offset);
        for (uint32_t i = 0; i < function_count; ++i) {
            LibraryFunctionExport fn_export;
            fn_export.name = read_string(data, offset);
            fn_export.function_index = read_u32(data, offset);
            library.functions.push_back(std::move(fn_export));
        }

        library.classes.clear();
        const uint32_t class_count = read_u32(data, offset);
        for (uint32_t i = 0; i < class_count; ++i) {
            LibraryClassExport class_export;
            class_export.name = read_string(data, offset);
            class_export.constructor_index = read_u32(data, offset);
            const uint32_t method_count = read_u32(data, offset);
            for (uint32_t m = 0; m < method_count; ++m) {
                LibraryClassMethod method;
                method.name = read_string(data, offset);
                method.function_index = read_u32(data, offset);
                class_export.methods.push_back(std::move(method));
            }
            library.classes.push_back(std::move(class_export));
        }

        const uint32_t bytecode_size = read_u32(data, offset);
        if (offset + bytecode_size > data.size()) {
            throw DryadException("dyl: truncated bytecode section");
        }
        std::vector<uint8_t> bytecode(data.begin() + offset, data.begin() + offset + bytecode_size);
        library.program = deserialize_program_from_bytes(bytecode);
        return true;
    } catch (...) {
        return false;
    }
}

LibraryMaterialization materialize_library(CompiledLibrary& library, Interpreter* interpreter) {
    LibraryMaterialization result;
    auto library_holder = std::make_shared<CompiledLibrary>(std::move(library));
    result.library = library_holder;

    auto module_env = std::make_shared<Environment>(interpreter->runtime_env());
    result.runtime_delegate = std::make_shared<LibraryVmRuntime>(
        interpreter, module_env, &library_holder->program);

    for (const auto& value_export : library_holder->values) {
        module_env->define(value_export.name, value_export.value);
    }

    for (const auto& class_export : library_holder->classes) {
        auto klass = std::make_shared<DryadClass>(class_export.name, nullptr);

        if (class_export.constructor_index < library_holder->program.functions.size()) {
            const auto& ctor_meta =
                library_holder->program.functions[class_export.constructor_index].metadata;
            auto ctor_decl = std::make_unique<FunctionDeclaration>(
                "constructor",
                parameters_from_slots(ctor_meta.slots),
                std::make_unique<BlockStatement>(std::vector<std::unique_ptr<Statement>>{}));

            auto ctor_fn = std::make_shared<LibraryDryadFunction>(
                std::move(ctor_decl),
                module_env,
                klass,
                library_holder,
                static_cast<uint16_t>(class_export.constructor_index),
                std::static_pointer_cast<LibraryVmRuntime>(result.runtime_delegate));
            klass->constructor = ctor_fn;
        }

        for (const auto& method : class_export.methods) {
            if (method.function_index >= library_holder->program.functions.size()) {
                continue;
            }
            const auto& method_meta =
                library_holder->program.functions[method.function_index].metadata;
            auto method_decl = std::make_unique<FunctionDeclaration>(
                method.name,
                parameters_from_slots(method_meta.slots),
                std::make_unique<BlockStatement>(std::vector<std::unique_ptr<Statement>>{}));

            auto method_fn = std::make_shared<LibraryDryadFunction>(
                std::move(method_decl),
                module_env,
                klass,
                library_holder,
                static_cast<uint16_t>(method.function_index),
                std::static_pointer_cast<LibraryVmRuntime>(result.runtime_delegate));
            klass->methods[method.name] = method_fn;
            klass->owned_declarations.push_back(std::move(method_decl));
        }

        module_env->define(class_export.name, Value(klass));
    }

    for (const auto& fn_export : library_holder->functions) {
        if (fn_export.function_index >= library_holder->program.functions.size()) {
            continue;
        }
        const auto& fn_meta = library_holder->program.functions[fn_export.function_index].metadata;
        auto fn_decl = std::make_unique<FunctionDeclaration>(
            fn_export.name,
            parameters_from_slots(fn_meta.slots),
            std::make_unique<BlockStatement>(std::vector<std::unique_ptr<Statement>>{}));

        auto fn = std::make_shared<LibraryDryadFunction>(
            std::move(fn_decl),
            module_env,
            nullptr,
            library_holder,
            static_cast<uint16_t>(fn_export.function_index),
            std::static_pointer_cast<LibraryVmRuntime>(result.runtime_delegate));
        module_env->define(fn_export.name, Value(fn));
    }

    result.environment = module_env;
    return result;
}

std::string default_dyl_output(const std::string& input_path) {
    const auto dot = input_path.rfind('.');
    if (dot == std::string::npos) {
        return input_path + ".dyl";
    }
    return input_path.substr(0, dot) + ".dyl";
}

std::string library_path_for_specifier(const std::string& specifier) {
    return specifier + ".dyl";
}

bool library_file_is_fresh(const std::string& library_path, const std::string& source_path) {
    std::ifstream library_file(library_path, std::ios::binary);
    if (!library_file) {
        return false;
    }

    std::ifstream source_file(source_path);
    if (!source_file) {
        return true;
    }

    const auto library_time = std::filesystem::last_write_time(library_path);
    const auto source_time = std::filesystem::last_write_time(source_path);
    return library_time >= source_time;
}

bool try_invoke_library_constructor(const std::shared_ptr<Function>& constructor,
                                    const std::shared_ptr<ClassInstance>& instance,
                                    const std::vector<Value>& args,
                                    Interpreter* interpreter) {
    auto library_ctor = std::dynamic_pointer_cast<LibraryDryadFunction>(constructor);
    if (!library_ctor) {
        return false;
    }
    library_ctor->invoke_constructor(instance, args, interpreter);
    return true;
}

} // namespace dryad
