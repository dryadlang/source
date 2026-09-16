#include "dryad/runtime/vm.h"
#include "dryad/runtime/intrinsics_registry.h"
#include "dryad/common/utils.h"
#include <cmath>
#include <cstring>

namespace dryad {

VM::VM() : pc_(0), halted_(false) {}

void VM::load(const BytecodeModule& module) {
    bytecode_ = module.code;
    constants_ = module.constants;
    pc_ = 0;
    halted_ = false;
    stack_.clear();
    slots_.clear();
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
    pc_ = 0;
    halted_ = false;
    stack_.clear();
    slots_.clear();
}

void VM::execute() {
    halted_ = false;
    execute_impl();
}

void VM::execute_impl() {
    while (!halted_ && pc_ < bytecode_.size()) {
        Opcode op = static_cast<Opcode>(bytecode_[pc_]);
        dispatch_opcode(op);
    }
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
            pc_ = target;
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
