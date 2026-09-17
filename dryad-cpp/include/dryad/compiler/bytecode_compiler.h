#ifndef DRYAD_COMPILER_BYTECODE_COMPILER_H
#define DRYAD_COMPILER_BYTECODE_COMPILER_H

#include "dryad/compiler/ast.h"
#include "dryad/compiler/type_profiler.h"
#include "dryad/runtime/bytecode_module.h"
#include "dryad/runtime/bytecode_builder.h"
#include <string>
#include <unordered_map>
#include <vector>

namespace dryad {

struct LoopCompileContext {
    std::vector<size_t> break_jump_patches;
    std::vector<size_t> continue_jump_patches;
};

class BytecodeCompiler {
public:
    CompiledFunction compile_function(FunctionDeclaration* fn,
                                      const FunctionProfile* profile = nullptr);
    CompiledProgram compile_program(Program* program,
                                    const TypeProfiler& profiler);

private:
    BytecodeBuilder builder_;
    std::unordered_map<std::string, uint16_t> slot_map_;
    std::unordered_map<std::string, uint16_t> function_indices_;
    std::unordered_map<std::string, std::string> slot_annotations_;
    const FunctionProfile* profile_ = nullptr;
    FunctionMetadata metadata_;
    size_t stack_depth_ = 0;
    size_t max_stack_depth_ = 0;
    std::vector<LoopCompileContext> loop_stack_;

    uint16_t allocate_slot(const std::string& name, const std::string& annotation = "");
    uint16_t slot_index(const std::string& name) const;
    SlotProfile slot_profile(const std::string& name) const;

    void push_stack();
    void pop_stack(size_t count = 1);

    Opcode select_add_opcode(const std::string& left, const std::string& right) const;
    Opcode select_sub_opcode(const std::string& left, const std::string& right) const;
    Opcode select_mul_opcode(const std::string& left, const std::string& right) const;
    Opcode select_div_opcode(const std::string& left, const std::string& right) const;
    bool use_f64_element_access(const std::string& array_name) const;
    bool try_resolve_intrinsic_id(const std::string& name, uint16_t& id) const;

    void compile_expression(Expression* expr);
    void compile_unary(UnaryExpression* expr);
    void compile_logical(BinaryExpression* expr);
    void patch_break_jumps(size_t exit_pc);
    void patch_continue_jumps(size_t target_pc);
    void compile_statement(Statement* stmt);
    void compile_block(BlockStatement* block);
    void compile_binary(BinaryExpression* expr);
    void compile_assignment(AssignmentExpression* expr);
    void compile_call(CallExpression* expr);

    void finalize_metadata();
};

FunctionDeclaration* find_function_declaration(Program* program, const std::string& name);

} // namespace dryad

#endif // DRYAD_COMPILER_BYTECODE_COMPILER_H
