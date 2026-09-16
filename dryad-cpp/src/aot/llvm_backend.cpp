#include "dryad/aot/llvm_backend.h"
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

void emit_instruction(std::ostringstream& out, const IrInstruction& inst) {
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
        case IrOpcode::LoadFast:
            out << "  " << reg_name(inst.result) << " = load "
                << llvm_type(inst.result_type) << ", "
                << llvm_type(inst.result_type) << "* %slot" << inst.slot
                << ", align 8\n";
            break;
        case IrOpcode::StoreFast:
            out << "  store " << llvm_type(IrType::Variant) << " "
                << reg_name(inst.result) << ", " << llvm_type(IrType::Variant)
                << "* %slot" << inst.slot << ", align 8\n";
            break;
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
        case IrOpcode::CmpLtI32:
            out << "  " << reg_name(inst.result) << " = icmp slt i32 "
                << reg_name(inst.lhs) << ", " << reg_name(inst.rhs) << "\n";
            break;
        case IrOpcode::GenericAdd:
            out << "  " << reg_name(inst.result)
                << " = call i8* @dryad_variant_add(i8* " << reg_name(inst.lhs)
                << ", i8* " << reg_name(inst.rhs) << ")\n";
            break;
        case IrOpcode::GenericGetElement:
            out << "  " << reg_name(inst.result)
                << " = call i8* @dryad_variant_get_element(i8* "
                << reg_name(inst.lhs) << ", i8* " << reg_name(inst.rhs) << ")\n";
            break;
        case IrOpcode::IntrinsicSlow:
            out << "  " << reg_name(inst.result)
                << " = call i8* @dryad_intrinsic_slow(i16 " << inst.intrinsic_id
                << ", i8 " << inst.intrinsic_argc << ")\n";
            break;
        case IrOpcode::Return:
            out << "  ret " << llvm_type(IrType::Variant) << " "
                << reg_name(inst.result) << "\n";
            break;
    }
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

} // namespace

std::string emit_llvm_ir(const IrModule& module) {
    std::ostringstream out;
    out << "; Dryad AOT LLVM IR (generated)\n";
    out << "declare i8* @dryad_variant_add(i8*, i8*)\n";
    out << "declare i8* @dryad_variant_get_element(i8*, i8*)\n";
    out << "declare i8* @dryad_intrinsic_slow(i16, i8)\n\n";

    for (const auto& function : module.functions) {
        const std::string symbol = sanitize_symbol(function.name);
        out << "define i8* @" << symbol << "() {\n";
        out << "entry:\n";
        for (size_t i = 0; i < function.slot_types.size(); ++i) {
            out << "  %slot" << i << " = alloca " << llvm_type(function.slot_types[i])
                << ", align 8\n";
        }
        for (const auto& inst : function.instructions) {
            emit_instruction(out, inst);
        }
        out << "  ret i8* null\n";
        out << "}\n\n";
    }
    return out.str();
}

bool write_llvm_ir_file(const IrModule& module, const std::string& path) {
    std::ofstream file(path);
    if (!file) {
        return false;
    }
    file << emit_llvm_ir(module);
    return static_cast<bool>(file);
}

} // namespace dryad
