#include "dryad/runtime/vm.h"
#include "dryad/runtime/class.h"
#include "dryad/runtime/function.h"
#include "dryad/runtime/intrinsics_registry.h"
#include "dryad/common/utils.h"
#include <cmath>
#include <cstdlib>
#include <cstring>

namespace dryad {

VM::VM() : pc_(0), halted_(false) {}

void VM::load(const BytecodeModule& module) {
    program_ = nullptr;
    current_function_index_ = 0;
    frames_.clear();
    bytecode_ = module.code;
    constants_ = module.constants;
    pc_ = 0;
    halted_ = false;
    stack_.clear();
    slots_.clear();
}

void VM::load_program(const CompiledProgram& program, size_t entry_index) {
    if (entry_index >= program.functions.size()) {
        throw DryadException("VM entry function index out of bounds");
    }
    program_ = &program;
    current_function_index_ = entry_index;
    frames_.clear();
    stack_.clear();
    slots_.clear();
    halted_ = false;
    activate_function(entry_index);
    loops_ = analyze_loops(bytecode_);
    if (jit_delegate_) {
        jit_delegate_->prepare_loops(loops_);
    }
    pc_ = 0;
}

void VM::set_jit_delegate(VmJitDelegate* delegate) {
    jit_delegate_ = delegate;
    if (jit_delegate_) {
        jit_delegate_->prepare_loops(loops_);
    }
}

void VM::set_runtime_delegate(VmRuntimeDelegate* delegate) {
    runtime_delegate_ = delegate;
}

void VM::activate_function(size_t function_index) {
    if (!program_ || function_index >= program_->functions.size()) {
        throw DryadException("VM activate_function: invalid function index");
    }
    const BytecodeModule& module = program_->functions[function_index].module;
    bytecode_ = module.code;
    constants_ = module.constants;
    current_function_index_ = function_index;
}

void VM::load_bytecode(const std::vector<uint8_t>& bytecode) {
    bytecode_ = bytecode;
    constants_.clear();
    pc_ = 0;
    halted_ = false;
    stack_.clear();
    slots_.clear();
}

void VM::set_constants(const std::vector<Value>& constants) {
    constants_ = constants;
}

void VM::reset() {
    program_ = nullptr;
    current_function_index_ = 0;
    frames_.clear();
    pc_ = 0;
    halted_ = false;
    stack_.clear();
    slots_.clear();
    bytecode_.clear();
    constants_.clear();
}

void VM::execute() {
    halted_ = false;
    execute_impl();
}

namespace {

uint64_t vm_max_steps_from_env() {
    const char* env = std::getenv("DRYAD_VM_MAX_STEPS");
    if (env == nullptr || env[0] == '\0') {
        return 0;
    }
    char* end = nullptr;
    const unsigned long long parsed = std::strtoull(env, &end, 10);
    if (end == env || parsed == 0) {
        return 0;
    }
    return static_cast<uint64_t>(parsed);
}

} // namespace

void VM::execute_impl() {
    uint64_t steps = 0;
    const uint64_t max_steps = vm_max_steps_from_env();
    while (!halted_ && pc_ < bytecode_.size()) {
        if (max_steps > 0 && ++steps > max_steps) {
            throw DryadException(
                "VM step limit exceeded (set DRYAD_VM_MAX_STEPS, 0 = unlimited)");
        }
        if (try_run_jit_body()) {
            continue;
        }
        Opcode op = static_cast<Opcode>(bytecode_[pc_]);
        dispatch_opcode(op);
    }
}

bool VM::try_run_jit_body() {
    if (!jit_enabled_ || !jit_delegate_) {
        return false;
    }
    const size_t loop_index = jit_delegate_->loop_index_for_body(static_cast<uint16_t>(pc_));
    if (loop_index == loops_.size() || !jit_delegate_->is_body_ready(loop_index)) {
        return false;
    }
    if (!jit_delegate_->try_run_body(loop_index, slots_)) {
        return false;
    }
    pc_ = loops_[loop_index].back_edge_pc;
    return true;
}

void VM::handle_back_edge(uint16_t back_edge_pc, uint16_t target_pc) {
    if (jit_enabled_ && jit_delegate_) {
        const size_t loop_index = jit_delegate_->loop_index_for_back_edge(back_edge_pc);
        if (loop_index != loops_.size()) {
            jit_delegate_->on_back_edge(loop_index);
            if (jit_delegate_->should_compile(loop_index)) {
                jit_delegate_->try_compile_loop(loop_index);
            }
        }
    }
    pc_ = target_pc;
}

void VM::dispatch_opcode(Opcode op) {
    switch (op) {
        case Opcode::PUSH_NULL:
            stack_.push_back(Value());
            advance(1);
            break;

        case Opcode::PUSH_TRUE:
            stack_.push_back(Value(true));
            advance(1);
            break;

        case Opcode::PUSH_FALSE:
            stack_.push_back(Value(false));
            advance(1);
            break;

        case Opcode::PUSH_INT: {
            if (pc_ + 9 > bytecode_.size()) {
                throw DryadException("Bytecode bounds violation reading PUSH_INT");
            }
            stack_.push_back(Value(read_i64_at(pc_ + 1)));
            advance(9);
            break;
        }

        case Opcode::PUSH_FLOAT: {
            if (pc_ + 9 > bytecode_.size()) {
                throw DryadException("Bytecode bounds violation reading PUSH_FLOAT");
            }
            stack_.push_back(Value(read_f64_at(pc_ + 1)));
            advance(9);
            break;
        }

        case Opcode::PUSH_CONST: {
            uint16_t index = read_u16_at(pc_ + 1);
            if (index >= constants_.size()) {
                throw DryadException("Constant pool index out of bounds: " + std::to_string(index));
            }
            stack_.push_back(constants_[index]);
            advance(3);
            break;
        }

        case Opcode::POP:
            validate_stack(1, "POP");
            stack_.pop_back();
            advance(1);
            break;

        case Opcode::DUP:
            validate_stack(1, "DUP");
            stack_.push_back(stack_.back());
            advance(1);
            break;

        case Opcode::SWAP:
            validate_stack(2, "SWAP");
            std::swap(stack_[stack_.size() - 1], stack_[stack_.size() - 2]);
            advance(1);
            break;

        case Opcode::ADD: {
            validate_stack(2, "ADD");
            Value right = stack_.back();
            stack_.pop_back();
            Value left = stack_.back();
            stack_.pop_back();
            if (left.is_integer() && right.is_integer()) {
                stack_.push_back(Value(left.as_integer() + right.as_integer()));
            } else if (left.is_number() && right.is_number()) {
                stack_.push_back(Value(as_f64(left) + as_f64(right)));
            } else if (left.is_string() && right.is_string()) {
                stack_.push_back(Value(left.as_string() + right.as_string()));
            } else {
                throw DryadException("Invalid operands for ADD");
            }
            advance(1);
            break;
        }

        case Opcode::SUB: {
            validate_stack(2, "SUB");
            Value right = stack_.back();
            stack_.pop_back();
            Value left = stack_.back();
            stack_.pop_back();
            if (!left.is_number() || !right.is_number()) {
                throw DryadException("SUB requires numeric operands");
            }
            if (left.is_float() || right.is_float()) {
                stack_.push_back(Value(as_f64(left) - as_f64(right)));
            } else {
                stack_.push_back(Value(left.as_integer() - right.as_integer()));
            }
            advance(1);
            break;
        }

        case Opcode::MUL: {
            validate_stack(2, "MUL");
            Value right = stack_.back();
            stack_.pop_back();
            Value left = stack_.back();
            stack_.pop_back();
            if (!left.is_number() || !right.is_number()) {
                throw DryadException("MUL requires numeric operands");
            }
            if (left.is_float() || right.is_float()) {
                stack_.push_back(Value(as_f64(left) * as_f64(right)));
            } else {
                stack_.push_back(Value(left.as_integer() * right.as_integer()));
            }
            advance(1);
            break;
        }

        case Opcode::DIV: {
            validate_stack(2, "DIV");
            Value right = stack_.back();
            stack_.pop_back();
            Value left = stack_.back();
            stack_.pop_back();
            if (!left.is_number() || !right.is_number()) {
                throw DryadException("DIV requires numeric operands");
            }
            stack_.push_back(Value(as_f64(left) / as_f64(right)));
            advance(1);
            break;
        }

        case Opcode::MOD: {
            validate_stack(2, "MOD");
            Value right = stack_.back();
            stack_.pop_back();
            Value left = stack_.back();
            stack_.pop_back();
            if (!left.is_integer() || !right.is_integer()) {
                throw DryadException("MOD requires integer operands");
            }
            stack_.push_back(Value(left.as_integer() % right.as_integer()));
            advance(1);
            break;
        }

        case Opcode::ADD_F64: {
            validate_stack(2, "ADD_F64");
            double b = as_f64(stack_.back());
            stack_.pop_back();
            double a = as_f64(stack_.back());
            stack_.pop_back();
            stack_.push_back(Value(a + b));
            advance(1);
            break;
        }

        case Opcode::SUB_F64: {
            validate_stack(2, "SUB_F64");
            double b = as_f64(stack_.back());
            stack_.pop_back();
            double a = as_f64(stack_.back());
            stack_.pop_back();
            stack_.push_back(Value(a - b));
            advance(1);
            break;
        }

        case Opcode::MUL_F64: {
            validate_stack(2, "MUL_F64");
            double b = as_f64(stack_.back());
            stack_.pop_back();
            double a = as_f64(stack_.back());
            stack_.pop_back();
            stack_.push_back(Value(a * b));
            advance(1);
            break;
        }

        case Opcode::DIV_F64: {
            validate_stack(2, "DIV_F64");
            double b = as_f64(stack_.back());
            stack_.pop_back();
            double a = as_f64(stack_.back());
            stack_.pop_back();
            stack_.push_back(Value(a / b));
            advance(1);
            break;
        }

        case Opcode::ADD_I32: {
            validate_stack(2, "ADD_I32");
            int64_t b = as_i32_slot(stack_.back());
            stack_.pop_back();
            int64_t a = as_i32_slot(stack_.back());
            stack_.pop_back();
            stack_.push_back(Value(a + b));
            advance(1);
            break;
        }

        case Opcode::SUB_I32: {
            validate_stack(2, "SUB_I32");
            int64_t b = as_i32_slot(stack_.back());
            stack_.pop_back();
            int64_t a = as_i32_slot(stack_.back());
            stack_.pop_back();
            stack_.push_back(Value(a - b));
            advance(1);
            break;
        }

        case Opcode::MUL_I32: {
            validate_stack(2, "MUL_I32");
            int64_t b = as_i32_slot(stack_.back());
            stack_.pop_back();
            int64_t a = as_i32_slot(stack_.back());
            stack_.pop_back();
            stack_.push_back(Value(a * b));
            advance(1);
            break;
        }

        case Opcode::DIV_I32: {
            validate_stack(2, "DIV_I32");
            int64_t b = as_i32_slot(stack_.back());
            stack_.pop_back();
            int64_t a = as_i32_slot(stack_.back());
            stack_.pop_back();
            if (b == 0) {
                throw DryadException("Division by zero");
            }
            stack_.push_back(Value(a / b));
            advance(1);
            break;
        }

        case Opcode::NOT: {
            validate_stack(1, "NOT");
            bool truthy = stack_.back().is_truthy();
            stack_.pop_back();
            stack_.push_back(Value(!truthy));
            advance(1);
            break;
        }

        case Opcode::EQ:
        case Opcode::NEQ:
        case Opcode::LT:
        case Opcode::LTE:
        case Opcode::GT:
        case Opcode::GTE: {
            validate_stack(2, "comparison");
            Value right = stack_.back();
            stack_.pop_back();
            Value left = stack_.back();
            stack_.pop_back();
            bool result = false;

            if (op == Opcode::EQ || op == Opcode::NEQ) {
                if (left.type() == right.type()) {
                    if (left.is_null()) {
                        result = true;
                    } else if (left.is_boolean()) {
                        result = left.as_boolean() == right.as_boolean();
                    } else if (left.is_integer()) {
                        result = left.as_integer() == right.as_integer();
                    } else if (left.is_float()) {
                        result = left.as_float() == right.as_float();
                    } else if (left.is_string()) {
                        result = left.as_string() == right.as_string();
                    }
                }
                if (op == Opcode::NEQ) {
                    result = !result;
                }
            } else {
                if (!left.is_number() || !right.is_number()) {
                    throw DryadException("Comparison requires numeric operands");
                }
                double l = as_f64(left);
                double r = as_f64(right);
                switch (op) {
                    case Opcode::LT:  result = l < r; break;
                    case Opcode::LTE: result = l <= r; break;
                    case Opcode::GT:  result = l > r; break;
                    case Opcode::GTE: result = l >= r; break;
                    default: break;
                }
            }

            stack_.push_back(Value(result));
            advance(1);
            break;
        }

        case Opcode::JUMP: {
            uint16_t target = read_u16_at(pc_ + 1);
            advance(3);
            if (target < pc_) {
                handle_back_edge(static_cast<uint16_t>(pc_ - 3), target);
            } else {
                pc_ = target;
            }
            break;
        }

        case Opcode::JUMP_IF_FALSE: {
            validate_stack(1, "JUMP_IF_FALSE");
            bool truthy = stack_.back().is_truthy();
            stack_.pop_back();
            uint16_t target = read_u16_at(pc_ + 1);
            pc_ = truthy ? pc_ + 3 : target;
            break;
        }

        case Opcode::JUMP_IF_TRUE: {
            validate_stack(1, "JUMP_IF_TRUE");
            bool truthy = stack_.back().is_truthy();
            stack_.pop_back();
            uint16_t target = read_u16_at(pc_ + 1);
            pc_ = truthy ? target : pc_ + 3;
            break;
        }

        case Opcode::HALT:
            halted_ = true;
            advance(1);
            break;

        case Opcode::CALL: {
            if (!program_) {
                throw DryadException("CALL requires a loaded program");
            }
            uint16_t function_index = read_u16_at(pc_ + 1);
            uint8_t argc = bytecode_[pc_ + 3];
            validate_stack(argc, "CALL");
            advance(4);
            enter_function(function_index, argc);
            break;
        }

        case Opcode::RET:
            return_from_function();
            break;

        case Opcode::LOAD_FAST: {
            uint16_t slot = read_u16_at(pc_ + 1);
            if (slot >= slots_.size()) {
                throw DryadException("LOAD_FAST slot out of bounds: " + std::to_string(slot));
            }
            stack_.push_back(slots_[slot]);
            advance(3);
            break;
        }

        case Opcode::STORE_FAST: {
            validate_stack(1, "STORE_FAST");
            uint16_t slot = read_u16_at(pc_ + 1);
            if (slot >= slots_.size()) {
                slots_.resize(slot + 1);
            }
            slots_[slot] = stack_.back();
            stack_.pop_back();
            advance(3);
            break;
        }

        case Opcode::LOAD_GLOBAL: {
            if (!runtime_delegate_) {
                throw DryadException("LOAD_GLOBAL requires a runtime delegate");
            }
            uint16_t name_index = read_u16_at(pc_ + 1);
            if (name_index >= constants_.size() || !constants_[name_index].is_string()) {
                throw DryadException("LOAD_GLOBAL invalid name constant");
            }
            stack_.push_back(runtime_delegate_->get_global(constants_[name_index].as_string()));
            advance(3);
            break;
        }

        case Opcode::MAKE_FUNCTION: {
            if (!runtime_delegate_) {
                throw DryadException("MAKE_FUNCTION requires a runtime delegate");
            }
            uint16_t function_index = read_u16_at(pc_ + 1);
            auto native_fn = std::make_shared<NativeFunction>(
                [this, function_index](const std::vector<Value>& args) -> Value {
                    return runtime_delegate_->invoke_bytecode_function(function_index, args);
                });
            stack_.push_back(Value(native_fn));
            advance(3);
            break;
        }

        case Opcode::NEW_ARRAY:
            stack_.push_back(Value::create_array());
            advance(1);
            break;

        case Opcode::ARRAY_PUSH: {
            validate_stack(2, "ARRAY_PUSH");
            Value element = stack_.back();
            stack_.pop_back();
            Value array = stack_.back();
            stack_.pop_back();
            if (!array.is_array()) {
                throw DryadException("ARRAY_PUSH requires array");
            }
            array.array_push(element);
            stack_.push_back(array);
            advance(1);
            break;
        }

        case Opcode::SET_ELEMENT: {
            validate_stack(3, "SET_ELEMENT");
            Value value = stack_.back();
            stack_.pop_back();
            Value index_val = stack_.back();
            stack_.pop_back();
            Value array_val = stack_.back();
            stack_.pop_back();
            if (!array_val.is_array()) {
                throw DryadException("SET_ELEMENT requires array");
            }
            if (!index_val.is_integer()) {
                throw DryadException("SET_ELEMENT index must be integer");
            }
            array_val.array_set(static_cast<size_t>(index_val.as_integer()), value);
            stack_.push_back(value);
            advance(1);
            break;
        }

        case Opcode::GET_PROPERTY: {
            validate_stack(1, "GET_PROPERTY");
            uint16_t name_index = read_u16_at(pc_ + 1);
            if (name_index >= constants_.size() || !constants_[name_index].is_string()) {
                throw DryadException("GET_PROPERTY invalid name constant");
            }
            const std::string& name = constants_[name_index].as_string();
            Value object = stack_.back();
            stack_.pop_back();
            if (runtime_delegate_) {
                stack_.push_back(runtime_delegate_->get_property(object, name));
            } else if (object.is_instance()) {
                stack_.push_back(object.as_instance()->get(name));
            } else if (object.is_object()) {
                stack_.push_back(object.object_get(name));
            } else {
                throw DryadException("GET_PROPERTY requires object or instance");
            }
            advance(3);
            break;
        }

        case Opcode::SET_PROPERTY: {
            validate_stack(2, "SET_PROPERTY");
            uint16_t name_index = read_u16_at(pc_ + 1);
            if (name_index >= constants_.size() || !constants_[name_index].is_string()) {
                throw DryadException("SET_PROPERTY invalid name constant");
            }
            const std::string& name = constants_[name_index].as_string();
            Value value = stack_.back();
            stack_.pop_back();
            Value object = stack_.back();
            stack_.pop_back();
            if (runtime_delegate_) {
                runtime_delegate_->set_property(object, name, value);
            } else if (object.is_instance()) {
                object.as_instance()->set(name, value);
            } else if (object.is_object()) {
                object.object_set(name, value);
            } else {
                throw DryadException("SET_PROPERTY requires object or instance");
            }
            stack_.push_back(value);
            advance(3);
            break;
        }

        case Opcode::METHOD_CALL: {
            if (!runtime_delegate_) {
                throw DryadException("METHOD_CALL requires a runtime delegate");
            }
            uint16_t name_index = read_u16_at(pc_ + 1);
            uint8_t argc = bytecode_[pc_ + 3];
            if (name_index >= constants_.size() || !constants_[name_index].is_string()) {
                throw DryadException("METHOD_CALL invalid name constant");
            }
            validate_stack(static_cast<size_t>(argc) + 1, "METHOD_CALL");
            const std::string& method = constants_[name_index].as_string();
            Value receiver = stack_[stack_.size() - static_cast<size_t>(argc) - 1];
            std::vector<Value> args(argc);
            for (int i = static_cast<int>(argc) - 1; i >= 0; --i) {
                args[static_cast<size_t>(i)] = stack_.back();
                stack_.pop_back();
            }
            stack_.pop_back();
            stack_.push_back(runtime_delegate_->invoke_method(receiver, method, args));
            advance(4);
            break;
        }

        case Opcode::NEW_INSTANCE: {
            if (!runtime_delegate_) {
                throw DryadException("NEW_INSTANCE requires a runtime delegate");
            }
            uint16_t class_index = read_u16_at(pc_ + 1);
            uint8_t argc = bytecode_[pc_ + 3];
            if (class_index >= constants_.size() || !constants_[class_index].is_string()) {
                throw DryadException("NEW_INSTANCE invalid class name constant");
            }
            validate_stack(argc, "NEW_INSTANCE");
            std::vector<Value> args(argc);
            for (int i = static_cast<int>(argc) - 1; i >= 0; --i) {
                args[static_cast<size_t>(i)] = stack_.back();
                stack_.pop_back();
            }
            const Value class_value =
                runtime_delegate_->get_global(constants_[class_index].as_string());
            stack_.push_back(runtime_delegate_->new_instance(class_value, args));
            advance(4);
            break;
        }

        case Opcode::CALL_GLOBAL: {
            if (!runtime_delegate_) {
                throw DryadException("CALL_GLOBAL requires a runtime delegate");
            }
            uint16_t name_index = read_u16_at(pc_ + 1);
            uint8_t argc = bytecode_[pc_ + 3];
            if (name_index >= constants_.size() || !constants_[name_index].is_string()) {
                throw DryadException("CALL_GLOBAL invalid name constant");
            }
            validate_stack(argc, "CALL_GLOBAL");
            const std::string& name = constants_[name_index].as_string();
            std::vector<Value> args(argc);
            for (int i = static_cast<int>(argc) - 1; i >= 0; --i) {
                args[static_cast<size_t>(i)] = stack_.back();
                stack_.pop_back();
            }
            stack_.push_back(runtime_delegate_->invoke_global(name, args));
            advance(4);
            break;
        }

        case Opcode::GET_ELEMENT: {
            validate_stack(2, "GET_ELEMENT");
            Value index_val = stack_.back();
            stack_.pop_back();
            Value array_val = stack_.back();
            stack_.pop_back();
            if (!array_val.is_array()) {
                throw DryadException("GET_ELEMENT requires array");
            }
            stack_.push_back(array_val.array_get(static_cast<size_t>(index_val.as_integer())));
            advance(1);
            break;
        }

        case Opcode::GET_ELEMENT_F64: {
            validate_stack(2, "GET_ELEMENT_F64");
            Value index_val = stack_.back();
            stack_.pop_back();
            Value array_val = stack_.back();
            stack_.pop_back();
            if (!array_val.is_array()) {
                throw DryadException("GET_ELEMENT_F64 requires array");
            }
            Value element = array_val.array_get(static_cast<size_t>(index_val.as_integer()));
            if (!element.is_number()) {
                throw DryadException("GET_ELEMENT_F64 element is not numeric");
            }
            stack_.push_back(Value(as_f64(element)));
            advance(1);
            break;
        }

        case Opcode::INTRINSIC_SYSCALL:
            handle_intrinsic_syscall();
            break;

        default:
            throw DryadException(std::string("Unimplemented opcode: ") + opcode_name(op));
    }
}

void VM::enter_function(size_t function_index, uint8_t argc) {
    VmCallFrame frame;
    frame.return_pc = pc_;
    frame.slots = std::move(slots_);
    frame.function_index = current_function_index_;
    frames_.push_back(std::move(frame));

    activate_function(function_index);
    slots_.clear();
    if (argc > 0) {
        slots_.resize(argc);
        for (int i = static_cast<int>(argc) - 1; i >= 0; --i) {
            slots_[static_cast<size_t>(i)] = stack_.back();
            stack_.pop_back();
        }
    }
    pc_ = 0;
}

void VM::return_from_function() {
    validate_stack(1, "RET");
    Value result = stack_.back();
    stack_.pop_back();
    advance(1);

    if (frames_.empty()) {
        stack_.push_back(result);
        halted_ = true;
        return;
    }

    VmCallFrame frame = std::move(frames_.back());
    frames_.pop_back();
    activate_function(frame.function_index);
    slots_ = std::move(frame.slots);
    pc_ = frame.return_pc;
    stack_.push_back(result);
}

void VM::handle_intrinsic_syscall() {
    if (pc_ + 4 > bytecode_.size()) {
        throw DryadException("Bytecode bounds violation in INTRINSIC_SYSCALL");
    }

    uint16_t intrinsic_id =
        (static_cast<uint16_t>(bytecode_[pc_ + 1]) << 8) |
        static_cast<uint16_t>(bytecode_[pc_ + 2]);
    uint8_t argc = bytecode_[pc_ + 3];

    validate_stack(argc, "INTRINSIC_SYSCALL");

    std::vector<Value> args(argc);
    for (int i = static_cast<int>(argc) - 1; i >= 0; --i) {
        args[static_cast<size_t>(i)] = stack_.back();
        stack_.pop_back();
    }

    Value result = IntrinsicsRegistry::instance().call_by_id(intrinsic_id, args);
    stack_.push_back(result);
    advance(4);
}

Value VM::get_stack_top() const {
    if (stack_.empty()) {
        throw DryadException("Stack is empty");
    }
    return stack_.back();
}

void VM::push_stack(const Value& value) {
    stack_.push_back(value);
}

void VM::set_slot(uint16_t index, const Value& value) {
    if (index >= slots_.size()) {
        slots_.resize(index + 1);
    }
    slots_[index] = value;
}

void VM::validate_stack(size_t required, const char* context) {
    if (stack_.size() < required) {
        throw DryadException(std::string("Stack underflow in ") + context +
                             ": need " + std::to_string(required) +
                             ", have " + std::to_string(stack_.size()));
    }
}

uint16_t VM::read_u16_at(size_t offset) const {
    if (offset + 2 > bytecode_.size()) {
        throw DryadException("Bytecode bounds violation reading u16");
    }
    return static_cast<uint16_t>(bytecode_[offset]) |
           (static_cast<uint16_t>(bytecode_[offset + 1]) << 8);
}

int64_t VM::read_i64_at(size_t offset) const {
    if (offset + 8 > bytecode_.size()) {
        throw DryadException("Bytecode bounds violation reading i64");
    }
    int64_t value = 0;
    for (int i = 0; i < 8; ++i) {
        value |= static_cast<int64_t>(bytecode_[offset + i]) << (i * 8);
    }
    return value;
}

double VM::read_f64_at(size_t offset) const {
    int64_t bits = read_i64_at(offset);
    double value = 0.0;
    std::memcpy(&value, &bits, sizeof(double));
    return value;
}

void VM::advance(size_t bytes) {
    pc_ += bytes;
}

double VM::as_f64(const Value& value) {
    if (value.is_integer()) {
        return static_cast<double>(value.as_integer());
    }
    if (value.is_float()) {
        return value.as_float();
    }
    throw DryadException("Expected numeric value for f64 operation");
}

int64_t VM::as_i32_slot(const Value& value) {
    if (value.is_integer()) {
        return value.as_integer();
    }
    throw DryadException("Expected integer value for i32 operation");
}

} // namespace dryad
