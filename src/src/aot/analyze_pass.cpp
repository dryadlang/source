#include "dryad/aot/analyze_pass.h"
#include "dryad/aot/micro_ir.h"
#include "dryad/compiler/bytecode_compiler.h"
#include "dryad/runtime/opcode.h"
#include <iomanip>
#include <sstream>

namespace dryad {

namespace {

bool is_slow_opcode(Opcode op) {
    switch (op) {
        case Opcode::ADD:
        case Opcode::SUB:
        case Opcode::MUL:
        case Opcode::DIV:
        case Opcode::MOD:
        case Opcode::GET_ELEMENT:
        case Opcode::INTRINSIC_SYSCALL:
            return true;
        default:
            return false;
    }
}

bool is_fast_opcode(Opcode op) {
    switch (op) {
        case Opcode::ADD_F64:
        case Opcode::ADD_I32:
        case Opcode::SUB_F64:
        case Opcode::SUB_I32:
        case Opcode::MUL_F64:
        case Opcode::MUL_I32:
        case Opcode::DIV_F64:
        case Opcode::GET_ELEMENT_F64:
        case Opcode::LOAD_FAST:
        case Opcode::STORE_FAST:
            return true;
        default:
            return false;
    }
}

void analyze_bytecode(const CompiledFunction& function, AnalyzeReport& report) {
    size_t pc = 0;
    const auto& code = function.module.code;
    while (pc < code.size()) {
        Opcode op = static_cast<Opcode>(code[pc]);
        if (is_slow_opcode(op)) {
            AnalyzeFinding finding;
            finding.function_name = function.metadata.name;
            finding.site = "bytecode@" + std::to_string(pc);
            finding.path_kind = AnalyzePathKind::SlowPath;
            finding.reason = std::string("generic opcode ") + opcode_name(op);
            finding.suggestion = "add type annotations to promote to native opcode";
            report.findings.push_back(finding);
            report.slow_path_sites++;
        } else if (is_fast_opcode(op)) {
            report.fast_path_sites++;
        }

        switch (op) {
            case Opcode::PUSH_INT:
            case Opcode::PUSH_FLOAT:
                pc += 9;
                break;
            case Opcode::PUSH_CONST:
            case Opcode::LOAD_FAST:
            case Opcode::STORE_FAST:
            case Opcode::LOAD_GLOBAL:
            case Opcode::MAKE_FUNCTION:
            case Opcode::JUMP:
            case Opcode::JUMP_IF_FALSE:
            case Opcode::JUMP_IF_TRUE:
                pc += 3;
                break;
            case Opcode::INTRINSIC_SYSCALL:
                pc += 4;
                break;
            default:
                pc += 1;
                break;
        }
    }

    for (const auto& slot : function.metadata.slots) {
        if (slot.confidence == TypeConfidence::UNKNOWN) {
            AnalyzeFinding finding;
            finding.function_name = function.metadata.name;
            finding.site = "slot:" + slot.name;
            finding.path_kind = AnalyzePathKind::SlowPath;
            finding.reason = "slot type UNKNOWN (polymorphic)";
            finding.suggestion = "add type hint for '" + slot.name + "'";
            report.findings.push_back(finding);
            report.slow_path_sites++;
        } else {
            report.fast_path_sites++;
        }
    }
}

void analyze_micro_ir(const IrModule& module, AnalyzeReport& report) {
    for (const auto& function : module.functions) {
        for (const auto& inst : function.instructions) {
            if (inst.op == IrOpcode::GenericAdd ||
                inst.op == IrOpcode::GenericGetElement ||
                inst.op == IrOpcode::IntrinsicSlow) {
                AnalyzeFinding finding;
                finding.function_name = function.name;
                finding.site = "micro_ir:" + std::to_string(inst.result);
                finding.path_kind = AnalyzePathKind::SlowPath;
                finding.reason = "AOT would emit Variant/slow intrinsic dispatch";
                finding.suggestion = "resolve slot types to STATIC or OBSERVED";
                report.findings.push_back(finding);
            }
        }
    }
}

} // namespace

AnalyzeReport run_analyze_pass(const std::string& path, const CompileOptions& options) {
    AnalyzeReport report;
    report.source_file = path;
    CompiledProgram program = compile_file_to_bytecode(path, options);

    for (const auto& function : program.functions) {
        analyze_bytecode(function, report);
    }

    IrModule ir_module = bytecode_to_micro_ir(program);
    analyze_micro_ir(ir_module, report);
    return report;
}

std::string AnalyzeReport::format() const {
    std::ostringstream out;
    out << "Dryad AOT Analysis Report\n";
    out << "━━━━━━━━━━━━━━━━━━━━━━━\n";
    out << "File: " << source_file << "\n\n";

    for (const auto& finding : findings) {
        const char* label = finding.path_kind == AnalyzePathKind::SlowPath ? "SLOW PATH" : "FAST PATH";
        out << "[" << label << "] " << finding.function_name << " :: " << finding.site << "\n";
        out << "  Reason: " << finding.reason << "\n";
        if (!finding.suggestion.empty()) {
            out << "  Fix: " << finding.suggestion << "\n";
        }
        out << "\n";
    }

    const size_t total = fast_path_sites + slow_path_sites;
    const double pct = total == 0 ? 100.0
                                  : (static_cast<double>(fast_path_sites) * 100.0 /
                                     static_cast<double>(total));
    out << "Summary:\n";
    out << "  Fast Path sites:  " << fast_path_sites << "\n";
    out << "  Slow Path sites:  " << slow_path_sites << "\n";
    out << "  Fast Path ratio:  " << std::fixed << std::setprecision(1) << pct << "%\n";
    return out.str();
}

} // namespace dryad
