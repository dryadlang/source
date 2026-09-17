#include "dryad/aot/llvm_backend.h"
#include "dryad/compiler/type_profiler.h"
#include "dryad/common/utils.h"
#include "dryad/runtime/opcode.h"
#include <cctype>
#include <fstream>
#include <sstream>

namespace dryad {

namespace {

const char* llvm_type(IrType type) {
    switch (type) {
        case IrType::I32:     return "i32";
        case IrType::F64:     return "double";
        case IrType::Bool:    return "i1";
        case IrType::Ptr:     return "i8*";
        case IrType::Variant: return "i8*";
        case IrType::Void:    return "void";
    }
    return "i8*";
}

std::string reg_name(int id) { return "%v" + std::to_string(id); }

std::string as_i8_ptr(std::ostringstream& out, int reg, int& next_temp) {
    const std::string cast_name = "%as_i8_" + std::to_string(next_temp++);
    out << "  " << cast_name << " = bitcast ptr " << reg_name(reg) << " to i8*\n";
    return cast_name;
}

std::string sanitize_symbol(const std::string& name) {
    if (name.empty()) {
        return "anonymous";
    }
    std::string out = name;
    for (char& ch : out) {
        if (!std::isalnum(static_cast<unsigned char>(ch))) {
            ch = '_';
        }
    }
    return out;
}

void emit_runtime_declarations(std::ostringstream& out) {
    out << "; Dryad AOT LLVM IR (generated)\n";
    out << "declare i8* @dryad_variant_add(i8*, i8*)\n";
    out << "declare i8* @dryad_variant_get_element(i8*, i8*)\n";
    out << "declare i8* @dryad_intrinsic_slow(i16, i8)\n";
    out << "declare i8* @dryad_new_array()\n";
    out << "declare i8* @dryad_array_push(i8*, i8*)\n";
    out << "declare i8* @dryad_array_set(i8*, i8*, i8*)\n";
    out << "declare void @dryad_print(i8*)\n";
    out << "declare i8* @dryad_box_i32(i32)\n";
    out << "declare i8* @dryad_box_f64(double)\n";
    out << "declare i8* @dryad_box_bool(i1)\n";
    out << "declare i32 @dryad_unbox_i32(i8*)\n";
    out << "declare i1 @dryad_variant_eq(i8*, i8*)\n";
    out << "declare i1 @dryad_variant_neq(i8*, i8*)\n";
    out << "declare double @dryad_unbox_f64(i8*)\n";
    out << "declare i1 @dryad_unbox_bool(i8*)\n";
    out << "declare i8* @dryad_box_cstring(i8*)\n";
    out << "declare i8* @dryad_box_lit_cstring(i8*)\n\n";
}

std::string block_label(uint32_t id) { return "bb" + std::to_string(id); }

std::string box_argument(std::ostringstream& out,
                         int reg,
                         IrType type,
                         int& next_temp,
                         const std::string& suffix) {
    if (type == IrType::I32) {
        const std::string boxed = "%lc_box" + suffix;
        out << "  " << boxed << " = call i8* @dryad_box_i32(i32 " << reg_name(reg) << ")\n";
        return boxed;
    }
    if (type == IrType::F64) {
        const std::string boxed = "%lc_box" + suffix;
        out << "  " << boxed << " = call i8* @dryad_box_f64(double " << reg_name(reg) << ")\n";
        return boxed;
    }
    if (type == IrType::Bool) {
        const std::string boxed = "%lc_box" + suffix;
        out << "  " << boxed << " = call i8* @dryad_box_bool(i1 " << reg_name(reg) << ")\n";
        return boxed;
    }
    return as_i8_ptr(out, reg, next_temp);
}

bool emit_linked_global_call(std::ostringstream& out,
                             const IrFunction& function,
                             const IrInstruction& inst,
                             const std::string& symbol,
                             int& next_temp) {
    std::vector<std::string> boxed_args;
    if (inst.intrinsic_argc >= 2) {
        boxed_args.push_back(
            box_argument(out, inst.rhs, IrType::I32, next_temp, std::to_string(inst.result) + "_0"));
        boxed_args.push_back(
            box_argument(out, inst.lhs, IrType::I32, next_temp, std::to_string(inst.result) + "_1"));
    } else if (inst.intrinsic_argc == 1) {
        boxed_args.push_back(
            box_argument(out, inst.rhs, inst.result_type, next_temp, std::to_string(inst.result) + "_0"));
    }

    const std::string ret = "%lc_ret" + std::to_string(inst.result);
    out << "  " << ret << " = call i8* @" << symbol << "(";
    for (size_t i = 0; i < boxed_args.size(); ++i) {
        if (i > 0) {
            out << ", ";
        }
        out << "i8* " << boxed_args[i];
    }
    out << ")\n";

    if (inst.result_type == IrType::I32) {
        out << "  " << reg_name(inst.result) << " = call i32 @dryad_unbox_i32(i8* " << ret
            << ")\n";
    } else if (inst.result_type == IrType::F64) {
        out << "  " << reg_name(inst.result) << " = call double @dryad_unbox_f64(i8* " << ret
            << ")\n";
    } else if (inst.result_type == IrType::Bool) {
        out << "  " << reg_name(inst.result) << " = call i1 @dryad_unbox_bool(i8* " << ret
            << ")\n";
    } else {
        out << "  " << reg_name(inst.result) << " = bitcast i8* " << ret << " to i8*\n";
    }
    return true;
}

bool emit_intrinsic_call(std::ostringstream& out,
                         const IrFunction& function,
                         const IrInstruction& inst,
                         int& next_temp,
                         const IrEmitOptions& options) {
    const uint8_t opcode = static_cast<uint8_t>(inst.intrinsic_id & 0xFFu);
    switch (static_cast<Opcode>(opcode)) {
        case Opcode::NEW_ARRAY:
            out << "  " << reg_name(inst.result) << " = call i8* @dryad_new_array()\n";
            return true;
        case Opcode::ARRAY_PUSH: {
            std::string element = reg_name(inst.rhs);
            if (inst.result_type == IrType::I32) {
                const std::string boxed = "%apush_box" + std::to_string(inst.result);
                out << "  " << boxed << " = call i8* @dryad_box_i32(i32 " << element << ")\n";
                element = boxed;
            } else if (inst.result_type == IrType::F64) {
                const std::string boxed = "%apush_box" + std::to_string(inst.result);
                out << "  " << boxed << " = call i8* @dryad_box_f64(double " << element << ")\n";
                element = boxed;
            } else if (inst.result_type == IrType::Bool) {
                const std::string boxed = "%apush_box" + std::to_string(inst.result);
                out << "  " << boxed << " = call i8* @dryad_box_bool(i1 " << element << ")\n";
                element = boxed;
            } else {
                element = as_i8_ptr(out, inst.rhs, next_temp);
            }
            out << "  " << reg_name(inst.result) << " = call i8* @dryad_array_push(i8* "
                << reg_name(inst.lhs) << ", i8* " << element << ")\n";
            return true;
        }
        case Opcode::SET_ELEMENT:
            out << "  " << reg_name(inst.result) << " = call i8* @dryad_array_set(i8* "
                << reg_name(inst.lhs) << ", i8* " << reg_name(inst.rhs) << ", i8* "
                << reg_name(static_cast<int>(inst.slot)) << ")\n";
            return true;
        case Opcode::EQ:
            if (inst.intrinsic_argc == 2) {
                const std::string lhs_ptr = as_i8_ptr(out, inst.lhs, next_temp);
                const std::string rhs_ptr = as_i8_ptr(out, inst.rhs, next_temp);
                out << "  " << reg_name(inst.result) << " = call i1 @dryad_variant_eq(i8* "
                    << lhs_ptr << ", i8* " << rhs_ptr << ")\n";
                return true;
            }
            break;
        case Opcode::NEQ:
            if (inst.intrinsic_argc == 2) {
                const std::string lhs_ptr = as_i8_ptr(out, inst.lhs, next_temp);
                const std::string rhs_ptr = as_i8_ptr(out, inst.rhs, next_temp);
                out << "  " << reg_name(inst.result) << " = call i1 @dryad_variant_neq(i8* "
                    << lhs_ptr << ", i8* " << rhs_ptr << ")\n";
                return true;
            }
            break;
        case Opcode::MAKE_FUNCTION:
            out << "  " << reg_name(inst.result) << " = inttoptr i64 0 to i8*\n";
            return true;
        case Opcode::CALL_GLOBAL:
            if (inst.const_string_id < function.string_literals.size()) {
                const std::string& global_name = function.string_literals[inst.const_string_id];
                const auto linked = options.linked_exports.find(global_name);
                if (linked != options.linked_exports.end()) {
                    return emit_linked_global_call(out, function, inst, linked->second, next_temp);
                }
            }
            if (inst.intrinsic_argc == 1 && inst.const_string_id < function.string_literals.size() &&
                function.string_literals[inst.const_string_id] == "print") {
                if (inst.result_type == IrType::I32) {
                    const std::string boxed = "%print_box" + std::to_string(inst.result);
                    out << "  " << boxed << " = call i8* @dryad_box_i32(i32 "
                        << reg_name(inst.rhs) << ")\n";
                    out << "  call void @dryad_print(i8* " << boxed << ")\n";
                } else if (inst.result_type == IrType::F64) {
                    const std::string boxed = "%print_box" + std::to_string(inst.result);
                    out << "  " << boxed << " = call i8* @dryad_box_f64(double "
                        << reg_name(inst.rhs) << ")\n";
                    out << "  call void @dryad_print(i8* " << boxed << ")\n";
                } else if (inst.result_type == IrType::Bool) {
                    const std::string boxed = "%print_box" + std::to_string(inst.result);
                    out << "  " << boxed << " = call i8* @dryad_box_bool(i1 "
                        << reg_name(inst.rhs) << ")\n";
                    out << "  call void @dryad_print(i8* " << boxed << ")\n";
                } else {
                    out << "  call void @dryad_print(i8* " << reg_name(inst.rhs) << ")\n";
                }
                out << "  " << reg_name(inst.result) << " = inttoptr i64 0 to i8*\n";
                return true;
            }
            break;
        default:
            break;
    }
    out << "  " << reg_name(inst.result)
        << " = call i8* @dryad_intrinsic_slow(i16 " << inst.intrinsic_id
        << ", i8 " << static_cast<int>(inst.intrinsic_argc) << ")\n";
    return true;
}

void emit_string_literal_globals(std::ostringstream& out, const IrModule& module) {
    for (const auto& function : module.functions) {
        const std::string symbol = sanitize_symbol(function.name);
        for (size_t i = 0; i < function.string_literals.size(); ++i) {
            const std::string& text = function.string_literals[i];
            out << "@dryad_lit_" << symbol << "_" << i << " = private unnamed_addr constant ["
                << (text.size() + 1) << " x i8] c\"";
            for (char ch : text) {
                if (ch == '\\' || ch == '"') {
                    out << '\\' << ch;
                } else if (ch == '\n') {
                    out << "\\0A";
                } else {
                    out << ch;
                }
            }
            out << "\\00\"\n";
        }
    }
    if (!module.functions.empty()) {
        out << "\n";
    }
}

std::string box_for_return(std::ostringstream& out, int value_reg, IrType type, int& next_temp) {
    if (type == IrType::Variant || type == IrType::Ptr) {
        return reg_name(value_reg);
    }
    const std::string boxed = "%boxed" + std::to_string(next_temp++);
    if (type == IrType::I32) {
        out << "  " << boxed << " = call i8* @dryad_box_i32(i32 " << reg_name(value_reg) << ")\n";
    } else if (type == IrType::F64) {
        out << "  " << boxed << " = call i8* @dryad_box_f64(double " << reg_name(value_reg) << ")\n";
    } else if (type == IrType::Bool) {
        out << "  " << boxed << " = call i8* @dryad_box_bool(i1 " << reg_name(value_reg) << ")\n";
    } else {
        out << "  " << boxed << " = inttoptr i64 0 to i8*\n";
    }
    return boxed;
}

void emit_parameter_init(std::ostringstream& out,
                         const IrFunction& function,
                         int& next_temp) {
    const uint8_t param_count = function.metadata.parameter_count;
    for (uint8_t i = 0; i < param_count; ++i) {
        IrType slot_type = IrType::Variant;
        if (i < function.slot_types.size()) {
            slot_type = function.slot_types[i];
        }
        const int index = static_cast<int>(i);
        const std::string arg = "%arg" + std::to_string(index);
        if (slot_type == IrType::I32) {
            const std::string unboxed = "%p" + std::to_string(next_temp++);
            out << "  " << unboxed << " = call i32 @dryad_unbox_i32(i8* " << arg << ")\n";
            out << "  store i32 " << unboxed << ", i32* %slot" << index << ", align 4\n";
        } else if (slot_type == IrType::F64) {
            const std::string unboxed = "%p" + std::to_string(next_temp++);
            out << "  " << unboxed << " = call double @dryad_unbox_f64(i8* " << arg << ")\n";
            out << "  store double " << unboxed << ", double* %slot" << index << ", align 8\n";
        } else if (slot_type == IrType::Bool) {
            const std::string unboxed = "%p" + std::to_string(next_temp++);
            out << "  " << unboxed << " = call i1 @dryad_unbox_bool(i8* " << arg << ")\n";
            out << "  store i1 " << unboxed << ", i1* %slot" << index << ", align 1\n";
        } else {
            out << "  store i8* " << arg << ", i8** %slot" << index << ", align 8\n";
        }
    }
}

void emit_instruction(std::ostringstream& out,
                      const IrFunction& function,
                      const IrInstruction& inst,
                      int& next_temp,
                      const IrEmitOptions& options) {
    switch (inst.op) {
        case IrOpcode::ConstI32:
            out << "  " << reg_name(inst.result) << " = add i32 0, " << inst.const_i32 << "\n";
            break;
        case IrOpcode::ConstF64:
            out << "  " << reg_name(inst.result) << " = fadd double 0.0, "
                << inst.const_f64 << "\n";
            break;
        case IrOpcode::ConstBool:
            out << "  " << reg_name(inst.result) << " = add i1 0, "
                << (inst.const_bool ? "1" : "0") << "\n";
            break;
        case IrOpcode::ConstString: {
            const std::string& literal =
                function.string_literals.at(inst.const_string_id);
            const std::string symbol = sanitize_symbol(function.name);
            out << "  " << reg_name(inst.result) << " = call i8* @dryad_box_lit_cstring(i8* "
                << "getelementptr inbounds ([" << (literal.size() + 1) << " x i8], ["
                << (literal.size() + 1) << " x i8]* @dryad_lit_" << symbol << "_"
                << inst.const_string_id << ", i64 0, i64 0))\n";
            break;
        }
        case IrOpcode::LoadFast: {
            IrType slot_type = IrType::Variant;
            if (inst.slot < function.slot_types.size()) {
                slot_type = function.slot_types[inst.slot];
            }
            if (slot_type == IrType::Variant && inst.result_type != IrType::Variant) {
                const std::string loaded = "%slot_load" + std::to_string(next_temp++);
                out << "  " << loaded << " = load i8*, i8** %slot" << inst.slot << ", align 8\n";
                if (inst.result_type == IrType::Bool) {
                    out << "  " << reg_name(inst.result)
                        << " = call i1 @dryad_unbox_bool(i8* " << loaded << ")\n";
                } else if (inst.result_type == IrType::I32) {
                    out << "  " << reg_name(inst.result)
                        << " = call i32 @dryad_unbox_i32(i8* " << loaded << ")\n";
                } else if (inst.result_type == IrType::F64) {
                    out << "  " << reg_name(inst.result)
                        << " = call double @dryad_unbox_f64(i8* " << loaded << ")\n";
                } else {
                    out << "  " << reg_name(inst.result) << " = load i8*, i8** %slot"
                        << inst.slot << ", align 8\n";
                }
            } else {
                out << "  " << reg_name(inst.result) << " = load "
                    << llvm_type(inst.result_type) << ", "
                    << llvm_type(slot_type) << "* %slot" << inst.slot << ", align 8\n";
            }
            break;
        }
        case IrOpcode::StoreFast: {
            IrType slot_type = IrType::Variant;
            if (inst.slot < function.slot_types.size()) {
                slot_type = function.slot_types[inst.slot];
            }
            if (slot_type == IrType::Variant && inst.result_type != IrType::Variant) {
                std::string boxed = reg_name(inst.result);
                if (inst.result_type == IrType::Bool) {
                    boxed = "%store_box" + std::to_string(next_temp++);
                    out << "  " << boxed << " = call i8* @dryad_box_bool(i1 "
                        << reg_name(inst.result) << ")\n";
                } else if (inst.result_type == IrType::I32) {
                    boxed = "%store_box" + std::to_string(next_temp++);
                    out << "  " << boxed << " = call i8* @dryad_box_i32(i32 "
                        << reg_name(inst.result) << ")\n";
                } else if (inst.result_type == IrType::F64) {
                    boxed = "%store_box" + std::to_string(next_temp++);
                    out << "  " << boxed << " = call i8* @dryad_box_f64(double "
                        << reg_name(inst.result) << ")\n";
                }
                out << "  store i8* " << boxed << ", i8** %slot" << inst.slot << ", align 8\n";
            } else {
                out << "  store " << llvm_type(inst.result_type) << " "
                    << reg_name(inst.result) << ", " << llvm_type(slot_type)
                    << "* %slot" << inst.slot << ", align 8\n";
            }
            break;
        }
        case IrOpcode::AddI32:
            out << "  " << reg_name(inst.result) << " = add i32 "
                << reg_name(inst.lhs) << ", " << reg_name(inst.rhs) << "\n";
            break;
        case IrOpcode::AddF64:
            out << "  " << reg_name(inst.result) << " = fadd double "
                << reg_name(inst.lhs) << ", " << reg_name(inst.rhs) << "\n";
            break;
        case IrOpcode::SubI32:
            out << "  " << reg_name(inst.result) << " = sub i32 "
                << reg_name(inst.lhs) << ", " << reg_name(inst.rhs) << "\n";
            break;
        case IrOpcode::SubF64:
            out << "  " << reg_name(inst.result) << " = fsub double "
                << reg_name(inst.lhs) << ", " << reg_name(inst.rhs) << "\n";
            break;
        case IrOpcode::MulI32:
            out << "  " << reg_name(inst.result) << " = mul i32 "
                << reg_name(inst.lhs) << ", " << reg_name(inst.rhs) << "\n";
            break;
        case IrOpcode::MulF64:
            out << "  " << reg_name(inst.result) << " = fmul double "
                << reg_name(inst.lhs) << ", " << reg_name(inst.rhs) << "\n";
            break;
        case IrOpcode::DivF64:
            out << "  " << reg_name(inst.result) << " = fdiv double "
                << reg_name(inst.lhs) << ", " << reg_name(inst.rhs) << "\n";
            break;
        case IrOpcode::CmpEqI32:
            out << "  " << reg_name(inst.result) << " = icmp eq i32 "
                << reg_name(inst.lhs) << ", " << reg_name(inst.rhs) << "\n";
            break;
        case IrOpcode::CmpNeI32:
            out << "  " << reg_name(inst.result) << " = icmp ne i32 "
                << reg_name(inst.lhs) << ", " << reg_name(inst.rhs) << "\n";
            break;
        case IrOpcode::CmpLtI32:
            out << "  " << reg_name(inst.result) << " = icmp slt i32 "
                << reg_name(inst.lhs) << ", " << reg_name(inst.rhs) << "\n";
            break;
        case IrOpcode::CmpLeI32:
            out << "  " << reg_name(inst.result) << " = icmp sle i32 "
                << reg_name(inst.lhs) << ", " << reg_name(inst.rhs) << "\n";
            break;
        case IrOpcode::CmpGtI32:
            out << "  " << reg_name(inst.result) << " = icmp sgt i32 "
                << reg_name(inst.lhs) << ", " << reg_name(inst.rhs) << "\n";
            break;
        case IrOpcode::CmpGeI32:
            out << "  " << reg_name(inst.result) << " = icmp sge i32 "
                << reg_name(inst.lhs) << ", " << reg_name(inst.rhs) << "\n";
            break;
        case IrOpcode::GenericGetElement: {
            const std::string boxed_idx =
                "%ge_idx" + std::to_string(inst.result);
            out << "  " << boxed_idx << " = call i8* @dryad_box_i32(i32 "
                << reg_name(inst.rhs) << ")\n";
            out << "  " << reg_name(inst.result)
                << " = call i8* @dryad_variant_get_element(i8* "
                << reg_name(inst.lhs) << ", i8* " << boxed_idx << ")\n";
            break;
        }
        case IrOpcode::UnboxI32:
            out << "  " << reg_name(inst.result)
                << " = call i32 @dryad_unbox_i32(i8* " << reg_name(inst.lhs) << ")\n";
            break;
        case IrOpcode::UnboxBool: {
            const std::string operand = as_i8_ptr(out, inst.lhs, next_temp);
            out << "  " << reg_name(inst.result)
                << " = call i1 @dryad_unbox_bool(i8* " << operand << ")\n";
            break;
        }
        case IrOpcode::NotBool:
            out << "  " << reg_name(inst.result) << " = icmp eq i1 "
                << reg_name(inst.lhs) << ", 0\n";
            break;
        case IrOpcode::Phi: {
            IrType phi_type = inst.result_type;
            if (phi_type == IrType::Void) {
                phi_type = IrType::Bool;
            }
            out << "  " << reg_name(inst.result) << " = phi "
                << llvm_type(phi_type) << " ";
            for (size_t i = 0; i < inst.phi_incomings.size(); ++i) {
                if (i > 0) {
                    out << ", ";
                }
                const auto& incoming = inst.phi_incomings[i];
                out << "[" << reg_name(incoming.reg) << ", %"
                    << block_label(incoming.block) << "]";
            }
            out << "\n";
            break;
        }
        case IrOpcode::GenericAdd: {
            const std::string ul = "%ga_ul" + std::to_string(inst.result);
            const std::string ur = "%ga_ur" + std::to_string(inst.result);
            const std::string bl = "%ga_l" + std::to_string(inst.result);
            const std::string br = "%ga_r" + std::to_string(inst.result);
            out << "  " << ul << " = call i32 @dryad_unbox_i32(i8* " << reg_name(inst.lhs) << ")\n";
            out << "  " << ur << " = call i32 @dryad_unbox_i32(i8* " << reg_name(inst.rhs) << ")\n";
            out << "  " << bl << " = call i8* @dryad_box_i32(i32 " << ul << ")\n";
            out << "  " << br << " = call i8* @dryad_box_i32(i32 " << ur << ")\n";
            out << "  " << reg_name(inst.result)
                << " = call i8* @dryad_variant_add(i8* " << bl << ", i8* " << br << ")\n";
            break;
        }
        case IrOpcode::IntrinsicSlow:
            emit_intrinsic_call(out, function, inst, next_temp, options);
            break;
        case IrOpcode::Br:
            out << "  br label %" << block_label(inst.branch_target) << "\n";
            break;
        case IrOpcode::BrCond:
            out << "  br i1 " << reg_name(inst.lhs) << ", label %"
                << block_label(inst.branch_target) << ", label %"
                << block_label(inst.branch_false_target) << "\n";
            break;
        case IrOpcode::Return: {
            if (inst.result < 0) {
                out << "  ret i8* null\n";
            } else {
                const std::string boxed =
                    box_for_return(out, inst.result, inst.result_type, next_temp);
                out << "  ret i8* " << boxed << "\n";
            }
            break;
        }
    }
}

void emit_jit_instruction(std::ostringstream& out,
                          const IrInstruction& inst,
                          int& next_temp) {
    switch (inst.op) {
        case IrOpcode::ConstI32:
            out << "  " << reg_name(inst.result) << " = add i64 0, " << inst.const_i32 << "\n";
            break;
        case IrOpcode::LoadFast: {
            const std::string ptr = "%slot_ptr" + std::to_string(next_temp++);
            out << "  " << ptr << " = getelementptr inbounds i64, i64* %slots, i64 "
                << inst.slot << "\n";
            out << "  " << reg_name(inst.result) << " = load i64, i64* " << ptr << ", align 8\n";
            break;
        }
        case IrOpcode::StoreFast: {
            const std::string ptr = "%slot_ptr" + std::to_string(next_temp++);
            out << "  " << ptr << " = getelementptr inbounds i64, i64* %slots, i64 "
                << inst.slot << "\n";
            out << "  store i64 " << reg_name(inst.result) << ", i64* " << ptr << ", align 8\n";
            break;
        }
        case IrOpcode::AddI32:
            out << "  " << reg_name(inst.result) << " = add i64 "
                << reg_name(inst.lhs) << ", " << reg_name(inst.rhs) << "\n";
            break;
        case IrOpcode::SubI32:
            out << "  " << reg_name(inst.result) << " = sub i64 "
                << reg_name(inst.lhs) << ", " << reg_name(inst.rhs) << "\n";
            break;
        case IrOpcode::MulI32:
            out << "  " << reg_name(inst.result) << " = mul i64 "
                << reg_name(inst.lhs) << ", " << reg_name(inst.rhs) << "\n";
            break;
        default:
            break;
    }
}

bool is_jit_compatible(const IrFunction& function) {
    for (const auto& inst : function.instructions) {
        switch (inst.op) {
            case IrOpcode::ConstI32:
            case IrOpcode::LoadFast:
            case IrOpcode::StoreFast:
            case IrOpcode::AddI32:
            case IrOpcode::SubI32:
            case IrOpcode::MulI32:
                break;
            default:
                return false;
        }
    }
    return !function.instructions.empty();
}

} // namespace

const char* argv_box_helper(IrType slot_type, ObservedType observed) {
    if (slot_type == IrType::I32 || observed == ObservedType::I32) {
        return "dryad_argv_box_i32";
    }
    if (slot_type == IrType::F64 || observed == ObservedType::F64) {
        return "dryad_argv_box_f64";
    }
    if (slot_type == IrType::Bool || observed == ObservedType::Bool) {
        return "dryad_argv_box_bool";
    }
    return "dryad_argv_box_cstring";
}

std::string append_executable_main(const std::string& ir, const IrFunction& entry) {
    const uint8_t param_count = entry.metadata.parameter_count;
    if (param_count > 4) {
        throw DryadException("Executable entry supports at most 4 parameters (got " +
                             std::to_string(param_count) + ")");
    }

    const std::string symbol = sanitize_symbol(entry.name);
    std::ostringstream out;
    out << ir;
    out << "declare i8* @dryad_argv_box_i32(i8**, i32)\n";
    out << "declare i8* @dryad_argv_box_f64(i8**, i32)\n";
    out << "declare i8* @dryad_argv_box_bool(i8**, i32)\n";
    out << "declare i8* @dryad_argv_box_cstring(i8**, i32)\n";
    out << "define i32 @main(i32 %argc, i8** %argv) {\n";
    out << "entry:\n";

    std::vector<std::string> arg_regs;
    for (uint8_t i = 0; i < param_count; ++i) {
        IrType slot_type = IrType::Variant;
        if (i < entry.slot_types.size()) {
            slot_type = entry.slot_types[i];
        }
        ObservedType observed = ObservedType::Any;
        if (i < entry.metadata.slots.size()) {
            observed = entry.metadata.slots[i].type;
        }

        const std::string arg_reg = "%main_arg" + std::to_string(i);
        const int argv_index = static_cast<int>(i) + 1;
        out << "  " << arg_reg << " = call i8* @" << argv_box_helper(slot_type, observed)
            << "(i8** %argv, i32 " << argv_index << ")\n";
        arg_regs.push_back(arg_reg);
    }

    out << "  call i8* @" << symbol << "(";
    for (size_t i = 0; i < arg_regs.size(); ++i) {
        if (i > 0) {
            out << ", ";
        }
        out << "i8* " << arg_regs[i];
    }
    out << ")\n";
    out << "  ret i32 0\n";
    out << "}\n";
    return out.str();
}

std::string emit_llvm_ir(const IrModule& module, const IrEmitOptions& options) {
    std::ostringstream out;
    emit_runtime_declarations(out);
    emit_string_literal_globals(out, module);

    for (const auto& function : module.functions) {
        const std::string symbol = sanitize_symbol(function.name);
        const uint8_t param_count = function.metadata.parameter_count;
        out << "define i8* @" << symbol << "(";
        for (uint8_t i = 0; i < param_count; ++i) {
            if (i > 0) {
                out << ", ";
            }
            out << "i8* %arg" << static_cast<int>(i);
        }
        out << ") {\n";

        for (size_t i = 0; i < function.slot_types.size(); ++i) {
            out << "  %slot" << i << " = alloca " << llvm_type(function.slot_types[i])
                << ", align 8\n";
        }

        int next_temp = 0;
        emit_parameter_init(out, function, next_temp);
        const bool has_cfg = !function.blocks.empty();
        if (has_cfg) {
            out << "  br label %" << block_label(function.entry_block) << "\n\n";
            for (const auto& block : function.blocks) {
                out << block_label(block.id) << ":\n";
                bool terminated = false;
                for (const auto& inst : block.instructions) {
                    emit_instruction(out, function, inst, next_temp, options);
                    if (inst.op == IrOpcode::Return || inst.op == IrOpcode::Br ||
                        inst.op == IrOpcode::BrCond) {
                        terminated = true;
                    }
                }
                if (!terminated) {
                    out << "  ret i8* null\n";
                }
                out << "\n";
            }
        } else {
            out << "entry:\n";
            emit_parameter_init(out, function, next_temp);
            bool returned = false;
            for (const auto& inst : function.instructions) {
                emit_instruction(out, function, inst, next_temp, options);
                if (inst.op == IrOpcode::Return) {
                    returned = true;
                }
            }
            if (!returned) {
                out << "  ret i8* null\n";
            }
        }
        out << "}\n\n";
    }
    return out.str();
}

std::string emit_jit_loop_ir(const IrFunction& function, const std::string& symbol) {
    if (!is_jit_compatible(function)) {
        return {};
    }

    std::ostringstream out;
    emit_runtime_declarations(out);
    out << "define void @" << symbol << "(i64* %slots) {\nentry:\n";

    int next_temp = 0;
    for (const auto& inst : function.instructions) {
        emit_jit_instruction(out, inst, next_temp);
    }
    out << "  ret void\n}\n";
    return out.str();
}

bool write_llvm_ir_file(const IrModule& module,
                        const std::string& path,
                        const IrEmitOptions& options) {
    std::ofstream file(path);
    if (!file) {
        return false;
    }
    file << emit_llvm_ir(module, options);
    return static_cast<bool>(file);
}

bool write_llvm_ir_executable_file(const IrModule& module,
                                   const std::string& path,
                                   const std::string& entry_symbol,
                                   const IrEmitOptions& options) {
    const IrFunction* entry = nullptr;
    for (const auto& function : module.functions) {
        if (function.name == entry_symbol) {
            entry = &function;
            break;
        }
    }
    if (!entry) {
        throw DryadException("Executable entry function not found in IR module: " + entry_symbol);
    }

    std::ofstream file(path);
    if (!file) {
        return false;
    }
    file << append_executable_main(emit_llvm_ir(module, options), *entry);
    return static_cast<bool>(file);
}

bool write_jit_loop_ir_file(const IrFunction& function,
                            const std::string& symbol,
                            const std::string& path) {
    const std::string ir = emit_jit_loop_ir(function, symbol);
    if (ir.empty()) {
        return false;
    }
    std::ofstream file(path);
    if (!file) {
        return false;
    }
    file << ir;
    return static_cast<bool>(file);
}

} // namespace dryad
