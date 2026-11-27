#ifndef DRYAD_COMPILER_BYTECODE_COMPILER_H
#define DRYAD_COMPILER_BYTECODE_COMPILER_H

#include "dryad/common/utils.h"
#include "dryad/compiler/ast.h"
#include "dryad/compiler/type_profiler.h"
#include "dryad/runtime/bytecode_module.h"
#include "dryad/runtime/bytecode_builder.h"
#include <memory>
#include <set>
#include <string>
#include <unordered_map>
#include <vector>

namespace dryad {

struct LoopCompileContext {
    std::vector<size_t> break_jump_patches;
    std::vector<size_t> continue_jump_patches;
};

class Interpreter;

// Thrown when a construct cannot be compiled to bytecode. Callers fall back to
// the interpreter for the affected function or script body.
class BytecodeUnsupported : public DryadException {
public:
    using DryadException::DryadException;
};

class BytecodeCompiler {
public:
    void set_interpreter(Interpreter* interpreter) { interpreter_ = interpreter; }

    CompiledFunction compile_function(FunctionDeclaration* fn,
                                      const FunctionProfile* profile = nullptr);
    CompiledFunction compile_instance_method(FunctionDeclaration* fn,
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
    std::vector<CompiledFunction> lambda_functions_;
    size_t lambda_counter_ = 0;
    size_t lambda_base_index_ = 0;
    // Lambdas are only emitted by compile_program, which appends them to the
    // program; a standalone compile_function has nowhere to put them.
    bool allow_lambdas_ = false;
    std::set<std::string> cell_names_;
    std::unordered_map<std::string, uint16_t> upvalue_index_;
    Interpreter* interpreter_ = nullptr;

    uint16_t allocate_slot(const std::string& name, const std::string& annotation = "");
    uint16_t compile_lambda(FunctionExpression* expr);
    void begin_function(const std::string& name, size_t parameter_count,
                        const FunctionProfile* profile);
    void emit_cell_prologue(const std::vector<Parameter>& parameters,
                            const std::vector<Statement*>& body);
    CompiledFunction end_function(const std::vector<Statement*>& body);
    std::vector<CompiledFunction> compile_named_functions(
        const std::vector<std::pair<FunctionDeclaration*, const FunctionProfile*>>& named,
        std::vector<FunctionDeclaration*>* compiled_decls);
    void emit_load_name(const std::string& name);
    void emit_store_name(const std::string& name);
    bool is_local_name(const std::string& name) const;
    CompiledFunction compile_function_body(const std::string& name,
                                           const std::vector<Parameter>& parameters,
                                           const std::vector<Statement*>& body,
                                           const FunctionProfile* profile);
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
