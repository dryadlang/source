#include "dryad/compiler/bytecode_compiler.h"
#include "dryad/compiler/interpreter.h"
#include "dryad/compiler/program_entry.h"
#include "dryad/runtime/intrinsics_registry.h"
#include "dryad/common/utils.h"

namespace dryad {

namespace {

bool is_specializable(const SlotProfile& profile) {
    return profile.confidence == TypeConfidence::STATIC ||
           profile.confidence == TypeConfidence::OBSERVED;
}

std::string identifier_name(Expression* expr) {
    if (!expr || expr->type != ASTNodeType::Identifier) {
        return {};
    }
    return static_cast<Identifier*>(expr)->name;
}

} // namespace

FunctionDeclaration* find_function_declaration(Program* program, const std::string& name) {
    if (!program) {
        return nullptr;
    }
    for (auto& stmt : program->statements) {
        if (stmt->type != ASTNodeType::FunctionDeclaration) {
            continue;
        }
        auto* fn = static_cast<FunctionDeclaration*>(stmt.get());
        if (fn->name == name) {
            return fn;
        }
    }
    return nullptr;
}

uint16_t BytecodeCompiler::allocate_slot(const std::string& name, const std::string& annotation) {
    auto existing = slot_map_.find(name);
    if (existing != slot_map_.end()) {
        return existing->second;
    }
    const uint16_t index = static_cast<uint16_t>(slot_map_.size());
    slot_map_[name] = index;
    if (!annotation.empty()) {
        slot_annotations_[name] = annotation;
    }
    return index;
}

uint16_t BytecodeCompiler::slot_index(const std::string& name) const {
    auto it = slot_map_.find(name);
    if (it == slot_map_.end()) {
        throw DryadException("Unknown slot in bytecode compiler: " + name);
    }
    return it->second;
}

SlotProfile BytecodeCompiler::slot_profile(const std::string& name) const {
    if (profile_) {
        auto it = profile_->slots.find(name);
        if (it != profile_->slots.end()) {
            return it->second;
        }
    }
    auto ann = slot_annotations_.find(name);
    if (ann != slot_annotations_.end() && !ann->second.empty()) {
        SlotProfile profile;
        profile.type = TypeProfiler::from_annotation(ann->second);
        profile.confidence = TypeConfidence::STATIC;
        profile.observation_count = 1;
        return profile;
    }
    return {};
}

void BytecodeCompiler::push_stack() {
    stack_depth_++;
    max_stack_depth_ = std::max(max_stack_depth_, stack_depth_);
}

void BytecodeCompiler::pop_stack(size_t count) {
    if (stack_depth_ < count) {
        stack_depth_ = 0;
    } else {
        stack_depth_ -= count;
    }
}

Opcode BytecodeCompiler::select_add_opcode(const std::string& left, const std::string& right) const {
    const SlotProfile l = slot_profile(left);
    const SlotProfile r = slot_profile(right);
    if (is_specializable(l) && is_specializable(r) &&
        l.type == ObservedType::F64 && r.type == ObservedType::F64) {
        return Opcode::ADD_F64;
    }
    if (is_specializable(l) && is_specializable(r) &&
        l.type == ObservedType::I32 && r.type == ObservedType::I32) {
        return Opcode::ADD_I32;
    }
    return Opcode::ADD;
}

Opcode BytecodeCompiler::select_sub_opcode(const std::string& left, const std::string& right) const {
    const SlotProfile l = slot_profile(left);
    const SlotProfile r = slot_profile(right);
    if (is_specializable(l) && is_specializable(r) &&
        l.type == ObservedType::F64 && r.type == ObservedType::F64) {
        return Opcode::SUB_F64;
    }
    if (is_specializable(l) && is_specializable(r) &&
        l.type == ObservedType::I32 && r.type == ObservedType::I32) {
        return Opcode::SUB_I32;
    }
    return Opcode::SUB;
}

Opcode BytecodeCompiler::select_mul_opcode(const std::string& left, const std::string& right) const {
    const SlotProfile l = slot_profile(left);
    const SlotProfile r = slot_profile(right);
    if (is_specializable(l) && is_specializable(r) &&
        l.type == ObservedType::F64 && r.type == ObservedType::F64) {
        return Opcode::MUL_F64;
    }
    if (is_specializable(l) && is_specializable(r) &&
        l.type == ObservedType::I32 && r.type == ObservedType::I32) {
        return Opcode::MUL_I32;
    }
    return Opcode::MUL;
}

Opcode BytecodeCompiler::select_div_opcode(const std::string& left, const std::string& right) const {
    const SlotProfile l = slot_profile(left);
    const SlotProfile r = slot_profile(right);
    if (is_specializable(l) && is_specializable(r) &&
        l.type == ObservedType::F64 && r.type == ObservedType::F64) {
        return Opcode::DIV_F64;
    }
    if (is_specializable(l) && is_specializable(r) &&
        l.type == ObservedType::I32 && r.type == ObservedType::I32) {
        return Opcode::DIV_I32;
    }
    return Opcode::DIV;
}

bool BytecodeCompiler::try_resolve_intrinsic_id(const std::string& name, uint16_t& id) const {
    auto& registry = IntrinsicsRegistry::instance();
    if (registry.has(name)) {
        id = registry.get_id(name);
        return true;
    }
    if (name.rfind("__runtime_", 0) == 0) {
        const std::string runtime_name = "runtime." + name.substr(10);
        if (registry.has(runtime_name)) {
            id = registry.get_id(runtime_name);
            return true;
        }
    }
    if (name.rfind("__", 0) == 0) {
        const std::string syscall_name = "syscall." + name.substr(2);
        if (registry.has(syscall_name)) {
            id = registry.get_id(syscall_name);
            return true;
        }
    }
    return false;
}

void BytecodeCompiler::patch_break_jumps(size_t exit_pc) {
    if (loop_stack_.empty()) {
        return;
    }
    for (size_t patch_offset : loop_stack_.back().break_jump_patches) {
        builder_.patch_u16(patch_offset, static_cast<uint16_t>(exit_pc));
    }
}

void BytecodeCompiler::patch_continue_jumps(size_t target_pc) {
    if (loop_stack_.empty()) {
        return;
    }
    for (size_t patch_offset : loop_stack_.back().continue_jump_patches) {
        builder_.patch_u16(patch_offset, static_cast<uint16_t>(target_pc));
    }
}

bool BytecodeCompiler::use_f64_element_access(const std::string& array_name) const {
    const SlotProfile profile = slot_profile(array_name);
    if (is_specializable(profile) && profile.type == ObservedType::F64) {
        return true;
    }
    auto ann = slot_annotations_.find(array_name);
    if (ann != slot_annotations_.end()) {
        const std::string lower = ann->second;
        if (lower.find("f64") != std::string::npos || lower.find("number") != std::string::npos) {
            return true;
        }
    }
    return false;
}

void BytecodeCompiler::compile_expression(Expression* expr) {
    if (!expr) {
        return;
    }

    switch (expr->type) {
        case ASTNodeType::IntegerLiteral:
            builder_.emit_push_int(static_cast<IntegerLiteral*>(expr)->value);
            push_stack();
            break;

        case ASTNodeType::FloatLiteral:
            builder_.emit_push_float(static_cast<FloatLiteral*>(expr)->value);
            push_stack();
            break;

        case ASTNodeType::BooleanLiteral:
            builder_.emit(static_cast<BooleanLiteral*>(expr)->value ? Opcode::PUSH_TRUE : Opcode::PUSH_FALSE);
            push_stack();
            break;

        case ASTNodeType::NullLiteral:
            builder_.emit(Opcode::PUSH_NULL);
            push_stack();
            break;

        case ASTNodeType::StringLiteral: {
            uint16_t index = builder_.add_constant(Value(static_cast<StringLiteral*>(expr)->value));
            builder_.emit_push_const(index);
            push_stack();
            break;
        }

        case ASTNodeType::Identifier: {
            const std::string& name = static_cast<Identifier*>(expr)->name;
            auto slot_it = slot_map_.find(name);
            if (slot_it != slot_map_.end()) {
                builder_.emit_load_fast(slot_it->second);
            } else {
                const uint16_t name_const = builder_.add_constant(Value(name));
                builder_.emit_load_global(name_const);
            }
            push_stack();
            break;
        }

        case ASTNodeType::BinaryExpression: {
            auto* binary = static_cast<BinaryExpression*>(expr);
            if (binary->op == TokenType::AmpersandAmpersand || binary->op == TokenType::PipePipe) {
                compile_logical(binary);
            } else {
                compile_binary(binary);
            }
            break;
        }

        case ASTNodeType::UnaryExpression:
            compile_unary(static_cast<UnaryExpression*>(expr));
            break;

        case ASTNodeType::ThisExpression:
            builder_.emit_load_fast(slot_index("this"));
            push_stack();
            break;

        case ASTNodeType::AssignmentExpression:
            compile_assignment(static_cast<AssignmentExpression*>(expr));
            break;

        case ASTNodeType::ArrayExpression: {
            auto* array_expr = static_cast<ArrayExpression*>(expr);
            builder_.emit_new_array();
            push_stack();
            for (const auto& element : array_expr->elements) {
                compile_expression(element.get());
                builder_.emit_array_push();
            }
            break;
        }

        case ASTNodeType::MemberExpression: {
            auto* member = static_cast<MemberExpression*>(expr);
            if (member->computed) {
                const std::string array_name = identifier_name(member->object.get());
                compile_expression(member->object.get());
                compile_expression(member->property.get());
                if (!array_name.empty() && use_f64_element_access(array_name)) {
                    builder_.emit(Opcode::GET_ELEMENT_F64);
                } else {
                    builder_.emit(Opcode::GET_ELEMENT);
                }
                pop_stack();
            } else {
                auto* prop = dynamic_cast<Identifier*>(member->property.get());
                if (!prop) {
                    throw DryadException("Bytecode compiler: property must be an identifier");
                }
                compile_expression(member->object.get());
                const uint16_t name_const = builder_.add_constant(Value(prop->name));
                builder_.emit_get_property(name_const);
                push_stack();
            }
            break;
        }

        case ASTNodeType::NewExpression: {
            auto* new_expr = static_cast<NewExpression*>(expr);
            if (new_expr->arguments.size() > 255) {
                throw DryadException("Bytecode compiler: too many constructor arguments");
            }
            for (const auto& arg : new_expr->arguments) {
                compile_expression(arg.get());
            }
            const uint16_t class_const = builder_.add_constant(Value(new_expr->class_name));
            const uint8_t argc = static_cast<uint8_t>(new_expr->arguments.size());
            builder_.emit_new_instance(class_const, argc);
            push_stack();
            break;
        }

        case ASTNodeType::CallExpression:
            compile_call(static_cast<CallExpression*>(expr));
            break;

        case ASTNodeType::FunctionExpression: {
            const uint16_t function_index =
                compile_lambda(static_cast<FunctionExpression*>(expr));
            builder_.emit_make_function(function_index);
            push_stack();
            break;
        }

        case ASTNodeType::AwaitExpression: {
            auto* await_expr = static_cast<AwaitExpression*>(expr);
            compile_expression(await_expr->operand.get());
            uint16_t intrinsic_id = 0;
            if (!try_resolve_intrinsic_id("runtime.await_promise", intrinsic_id)) {
                throw DryadException("Bytecode compiler: runtime.await_promise intrinsic not found");
            }
            builder_.emit_intrinsic_syscall(intrinsic_id, 1);
            push_stack();
            break;
        }

        case ASTNodeType::SuperExpression:
            throw DryadException("Bytecode compiler: super requires interpreter dispatch");

        default:
            throw DryadException("Bytecode compiler: unsupported expression type");
    }
}

void BytecodeCompiler::compile_unary(UnaryExpression* expr) {
    if (!expr) {
        return;
    }
    compile_expression(expr->operand.get());
    switch (expr->op) {
        case TokenType::Bang:
            builder_.emit(Opcode::NOT);
            break;
        case TokenType::Minus:
            builder_.emit_push_int(0);
            push_stack();
            builder_.emit(Opcode::SUB);
            pop_stack();
            break;
        default:
            throw DryadException("Bytecode compiler: unsupported unary operator");
    }
}

void BytecodeCompiler::compile_logical(BinaryExpression* expr) {
    if (!expr) {
        return;
    }
    if (expr->op == TokenType::AmpersandAmpersand) {
        compile_expression(expr->left.get());
        builder_.emit(Opcode::DUP);
        const size_t jump_false = builder_.emit_jump_placeholder(Opcode::JUMP_IF_FALSE);
        builder_.emit(Opcode::POP);
        pop_stack();
        compile_expression(expr->right.get());
        builder_.patch_u16(jump_false, builder_.current_pc());
        return;
    }
    if (expr->op == TokenType::PipePipe) {
        compile_expression(expr->left.get());
        builder_.emit(Opcode::DUP);
        const size_t jump_true = builder_.emit_jump_placeholder(Opcode::JUMP_IF_TRUE);
        builder_.emit(Opcode::POP);
        pop_stack();
        compile_expression(expr->right.get());
        builder_.patch_u16(jump_true, builder_.current_pc());
        return;
    }
    throw DryadException("Bytecode compiler: invalid logical expression");
}

void BytecodeCompiler::compile_call(CallExpression* expr) {
    if (!expr) {
        throw DryadException("Bytecode compiler: invalid call expression");
    }

    if (expr->callee->type == ASTNodeType::MemberExpression) {
        auto* member = static_cast<MemberExpression*>(expr->callee.get());
        if (member->computed) {
            throw DryadException("Bytecode compiler: computed method calls are not supported");
        }
        auto* prop = dynamic_cast<Identifier*>(member->property.get());
        if (!prop) {
            throw DryadException("Bytecode compiler: method name must be an identifier");
        }
        if (expr->arguments.size() > 255) {
            throw DryadException("Bytecode compiler: too many arguments for METHOD_CALL");
        }
        compile_expression(member->object.get());
        for (auto& arg : expr->arguments) {
            compile_expression(arg.get());
        }
        const uint16_t name_const = builder_.add_constant(Value(prop->name));
        const uint8_t argc = static_cast<uint8_t>(expr->arguments.size());
        builder_.emit_method_call(name_const, argc);
        push_stack();
        return;
    }

    if (expr->callee->type != ASTNodeType::Identifier) {
        throw DryadException("Bytecode compiler: only direct function and method calls are supported");
    }
    const std::string& name = static_cast<Identifier*>(expr->callee.get())->name;

    if (expr->arguments.size() > 255) {
        throw DryadException("Bytecode compiler: too many arguments for CALL");
    }

    for (auto& arg : expr->arguments) {
        compile_expression(arg.get());
    }

    const uint8_t argc = static_cast<uint8_t>(expr->arguments.size());

    uint16_t intrinsic_id = 0;
    if (try_resolve_intrinsic_id(name, intrinsic_id)) {
        builder_.emit_intrinsic_syscall(intrinsic_id, argc);
        push_stack();
        return;
    }

    auto target = function_indices_.find(name);
    if (target != function_indices_.end()) {
        builder_.emit_call(target->second, argc);
        push_stack();
        return;
    }

    const uint16_t name_const = builder_.add_constant(Value(name));
    builder_.emit_call_global(name_const, argc);
    push_stack();
}

void BytecodeCompiler::compile_binary(BinaryExpression* expr) {
    const std::string left_name = identifier_name(expr->left.get());
    const std::string right_name = identifier_name(expr->right.get());

    compile_expression(expr->left.get());
    compile_expression(expr->right.get());

    switch (expr->op) {
        case TokenType::Plus:
            builder_.emit(select_add_opcode(left_name, right_name));
            break;
        case TokenType::Minus:
            builder_.emit(select_sub_opcode(left_name, right_name));
            break;
        case TokenType::Star:
            builder_.emit(select_mul_opcode(left_name, right_name));
            break;
        case TokenType::Slash:
            builder_.emit(select_div_opcode(left_name, right_name));
            break;
        case TokenType::Percent:
            builder_.emit(Opcode::MOD);
            break;
        case TokenType::Less:
            builder_.emit(Opcode::LT);
            break;
        case TokenType::LessEqual:
            builder_.emit(Opcode::LTE);
            break;
        case TokenType::Greater:
            builder_.emit(Opcode::GT);
            break;
        case TokenType::GreaterEqual:
            builder_.emit(Opcode::GTE);
            break;
        case TokenType::EqualEqual:
            builder_.emit(Opcode::EQ);
            break;
        case TokenType::BangEqual:
            builder_.emit(Opcode::NEQ);
            break;
        default:
            throw DryadException("Bytecode compiler: unsupported binary operator");
    }
    pop_stack();
}

void BytecodeCompiler::compile_assignment(AssignmentExpression* expr) {
    if (expr->target->type == ASTNodeType::MemberExpression) {
        auto* member = static_cast<MemberExpression*>(expr->target.get());
        if (member->computed) {
            compile_expression(member->object.get());
            compile_expression(member->property.get());
            compile_expression(expr->value.get());
            builder_.emit_set_element();
            return;
        }
        auto* prop = dynamic_cast<Identifier*>(member->property.get());
        if (!prop) {
            throw DryadException("Bytecode compiler: property must be an identifier");
        }
        compile_expression(member->object.get());
        compile_expression(expr->value.get());
        const uint16_t name_const = builder_.add_constant(Value(prop->name));
        builder_.emit_set_property(name_const);
        return;
    }

    if (expr->target->type != ASTNodeType::Identifier) {
        throw DryadException("Bytecode compiler: only identifier and member assignment is supported");
    }
    const std::string& name = static_cast<Identifier*>(expr->target.get())->name;
    compile_expression(expr->value.get());
    builder_.emit(Opcode::DUP);
    builder_.emit_store_fast(slot_index(name));
}

void BytecodeCompiler::compile_statement(Statement* stmt) {
    if (!stmt) {
        return;
    }

    switch (stmt->type) {
        case ASTNodeType::VariableDeclaration: {
            auto* decl = static_cast<VariableDeclaration*>(stmt);
            allocate_slot(decl->name, decl->type_annotation);
            if (decl->initializer) {
                compile_expression(decl->initializer.get());
                builder_.emit_store_fast(slot_index(decl->name));
                pop_stack();
            }
            break;
        }

        case ASTNodeType::ExpressionStatement:
            compile_expression(static_cast<ExpressionStatement*>(stmt)->expression.get());
            builder_.emit(Opcode::POP);
            pop_stack();
            break;

        case ASTNodeType::ReturnStatement: {
            auto* ret = static_cast<ReturnStatement*>(stmt);
            if (ret->value) {
                compile_expression(ret->value.get());
            } else {
                builder_.emit(Opcode::PUSH_NULL);
                push_stack();
            }
            builder_.emit_ret();
            pop_stack();
            break;
        }

        case ASTNodeType::IfStatement: {
            auto* if_stmt = static_cast<IfStatement*>(stmt);
            compile_expression(if_stmt->condition.get());
            const size_t jump_else = builder_.emit_jump_placeholder(Opcode::JUMP_IF_FALSE);
            pop_stack();
            compile_statement(if_stmt->then_branch.get());
            if (if_stmt->else_branch) {
                const size_t jump_end = builder_.emit_jump_placeholder(Opcode::JUMP);
                builder_.patch_u16(jump_else, builder_.current_pc());
                compile_statement(if_stmt->else_branch.get());
                builder_.patch_u16(jump_end, builder_.current_pc());
            } else {
                builder_.patch_u16(jump_else, builder_.current_pc());
            }
            break;
        }

        case ASTNodeType::WhileStatement: {
            auto* while_stmt = static_cast<WhileStatement*>(stmt);
            const size_t loop_start = builder_.current_pc();
            compile_expression(while_stmt->condition.get());
            const size_t jump_exit = builder_.emit_jump_placeholder(Opcode::JUMP_IF_FALSE);
            pop_stack();
            LoopCompileContext loop_ctx;
            loop_stack_.push_back(loop_ctx);
            compile_statement(while_stmt->body.get());
            patch_continue_jumps(loop_start);
            builder_.emit_jump(Opcode::JUMP, loop_start);
            const size_t loop_exit = builder_.current_pc();
            patch_break_jumps(loop_exit);
            loop_stack_.pop_back();
            builder_.patch_u16(jump_exit, loop_exit);
            break;
        }

        case ASTNodeType::ForStatement: {
            auto* for_stmt = static_cast<ForStatement*>(stmt);
            if (for_stmt->initializer) {
                compile_statement(for_stmt->initializer.get());
            }
            const size_t loop_start = builder_.current_pc();
            if (for_stmt->condition) {
                compile_expression(for_stmt->condition.get());
                const size_t jump_exit = builder_.emit_jump_placeholder(Opcode::JUMP_IF_FALSE);
                pop_stack();
                LoopCompileContext loop_ctx;
                loop_stack_.push_back(loop_ctx);
                compile_statement(for_stmt->body.get());
                if (for_stmt->increment) {
                    patch_continue_jumps(builder_.current_pc());
                    compile_expression(for_stmt->increment.get());
                    builder_.emit(Opcode::POP);
                    pop_stack();
                } else {
                    patch_continue_jumps(loop_start);
                }
                builder_.emit_jump(Opcode::JUMP, loop_start);
                const size_t loop_exit = builder_.current_pc();
                patch_break_jumps(loop_exit);
                loop_stack_.pop_back();
                builder_.patch_u16(jump_exit, loop_exit);
            } else {
                LoopCompileContext loop_ctx;
                loop_stack_.push_back(loop_ctx);
                compile_statement(for_stmt->body.get());
                if (for_stmt->increment) {
                    patch_continue_jumps(builder_.current_pc());
                    compile_expression(for_stmt->increment.get());
                    builder_.emit(Opcode::POP);
                    pop_stack();
                } else {
                    patch_continue_jumps(loop_start);
                }
                builder_.emit_jump(Opcode::JUMP, loop_start);
                const size_t loop_exit = builder_.current_pc();
                patch_break_jumps(loop_exit);
                loop_stack_.pop_back();
            }
            break;
        }

        case ASTNodeType::BreakStatement: {
            if (loop_stack_.empty()) {
                throw DryadException("Bytecode compiler: break outside of loop");
            }
            const size_t patch = builder_.emit_jump_placeholder(Opcode::JUMP);
            loop_stack_.back().break_jump_patches.push_back(patch);
            break;
        }

        case ASTNodeType::ContinueStatement: {
            if (loop_stack_.empty()) {
                throw DryadException("Bytecode compiler: continue outside of loop");
            }
            const size_t patch = builder_.emit_jump_placeholder(Opcode::JUMP);
            loop_stack_.back().continue_jump_patches.push_back(patch);
            break;
        }

        case ASTNodeType::BlockStatement:
            compile_block(static_cast<BlockStatement*>(stmt));
            break;

        case ASTNodeType::CompileBlockStatement:
            compile_block(static_cast<CompileBlockStatement*>(stmt)->body.get());
            break;

        default:
            throw DryadException("Bytecode compiler: unsupported statement type");
    }
}

void BytecodeCompiler::compile_block(BlockStatement* block) {
    if (!block) {
        return;
    }
    for (auto& stmt : block->statements) {
        compile_statement(stmt.get());
    }
}

void BytecodeCompiler::finalize_metadata() {
    metadata_.slots.clear();
    for (const auto& entry : slot_map_) {
        SlotMetadata slot;
        slot.name = entry.first;
        slot.slot_index = entry.second;
        const SlotProfile profile = slot_profile(entry.first);
        slot.type = profile.type;
        slot.confidence = profile.confidence;
        metadata_.slots.push_back(slot);
    }
    metadata_.stack_depth = static_cast<uint16_t>(max_stack_depth_);
    metadata_.opcode_count = static_cast<uint32_t>(builder_.code().size());
}

uint16_t BytecodeCompiler::compile_lambda(FunctionExpression* expr) {
    if (!expr) {
        throw DryadException("Bytecode compiler: invalid function expression");
    }

    FunctionDeclaration* decl = nullptr;
    if (expr->body) {
        const std::string name = "__lambda_" + std::to_string(lambda_counter_++);
        auto owned = std::make_unique<FunctionDeclaration>(
            name,
            std::move(expr->parameters),
            std::move(expr->body),
            std::move(expr->return_type));
        owned_lambdas_.push_back(std::move(owned));
        decl = owned_lambdas_.back().get();
    } else if (interpreter_) {
        decl = interpreter_->lookup_lambda_code(expr);
    }
    if (!decl || !decl->body) {
        throw DryadException("Bytecode compiler: invalid function expression");
    }
    if (decl->name.empty()) {
        decl->name = "__lambda_" + std::to_string(lambda_counter_++);
    }

    const uint16_t index =
        static_cast<uint16_t>(lambda_base_index_ + lambda_functions_.size());

    BytecodeBuilder saved_builder = std::move(builder_);
    auto saved_slot_map = slot_map_;
    auto saved_slot_annotations = slot_annotations_;
    FunctionMetadata saved_metadata = metadata_;
    size_t saved_stack_depth = stack_depth_;
    size_t saved_max_stack_depth = max_stack_depth_;
    auto saved_loop_stack = loop_stack_;
    const FunctionProfile* saved_profile = profile_;

    lambda_functions_.push_back(compile_function(decl, nullptr));

    builder_ = std::move(saved_builder);
    slot_map_ = std::move(saved_slot_map);
    slot_annotations_ = std::move(saved_slot_annotations);
    metadata_ = std::move(saved_metadata);
    stack_depth_ = saved_stack_depth;
    max_stack_depth_ = saved_max_stack_depth;
    loop_stack_ = std::move(saved_loop_stack);
    profile_ = saved_profile;

    return index;
}

CompiledFunction BytecodeCompiler::compile_function(FunctionDeclaration* fn,
                                                    const FunctionProfile* profile) {
    if (!fn || !fn->body) {
        throw DryadException("Bytecode compiler: invalid function declaration");
    }

    builder_ = BytecodeBuilder{};
    slot_map_.clear();
    slot_annotations_.clear();
    if (function_indices_.empty()) {
        function_indices_[fn->name] = 0;
    }
    profile_ = profile;
    metadata_ = FunctionMetadata{};
    metadata_.name = fn->name;
    metadata_.parameter_count = static_cast<uint8_t>(fn->parameters.size());
    stack_depth_ = 0;
    max_stack_depth_ = 0;
    loop_stack_.clear();

    for (const auto& param : fn->parameters) {
        allocate_slot(param.name, param.type_annotation);
    }

    compile_block(fn->body.get());
    builder_.emit_halt();
    finalize_metadata();

    CompiledFunction compiled;
    compiled.metadata = metadata_;
    compiled.module = builder_.build();
    return compiled;
}

CompiledFunction BytecodeCompiler::compile_instance_method(FunctionDeclaration* fn,
                                                           const FunctionProfile* profile) {
    if (!fn || !fn->body) {
        throw DryadException("Bytecode compiler: invalid method declaration");
    }

    builder_ = BytecodeBuilder{};
    slot_map_.clear();
    slot_annotations_.clear();
    profile_ = profile;
    metadata_ = FunctionMetadata{};
    metadata_.name = fn->name;
    metadata_.parameter_count = static_cast<uint8_t>(fn->parameters.size());
    stack_depth_ = 0;
    max_stack_depth_ = 0;
    loop_stack_.clear();

    allocate_slot("this");
    for (const auto& param : fn->parameters) {
        allocate_slot(param.name, param.type_annotation);
    }

    compile_block(fn->body.get());
    builder_.emit_halt();
    finalize_metadata();

    CompiledFunction compiled;
    compiled.metadata = metadata_;
    compiled.module = builder_.build();
    return compiled;
}

CompiledProgram BytecodeCompiler::compile_program(Program* program, const TypeProfiler& profiler) {
    CompiledProgram result;
    if (!program) {
        return result;
    }

    function_indices_.clear();
    uint16_t next_index = 0;

    for (auto& stmt : program->statements) {
        if (stmt->type != ASTNodeType::FunctionDeclaration) {
            continue;
        }
        auto* fn = static_cast<FunctionDeclaration*>(stmt.get());
        const FunctionProfile* profile = nullptr;
        for (const auto& entry : profiler.profiles()) {
            if (entry.first == fn) {
                profile = &entry.second;
                break;
            }
        }
        try {
            result.functions.push_back(compile_function(fn, profile));
            function_indices_[fn->name] = next_index++;
        } catch (const DryadException& ex) {
            const std::string message = ex.what();
            if (message.find("function expressions") != std::string::npos ||
                message.find("await is not supported") != std::string::npos ||
                message.find("super requires interpreter") != std::string::npos) {
                continue;
            }
            throw;
        }
    }

    std::unique_ptr<FunctionDeclaration> script_entry = synthesize_script_entry_function(program);
    if (script_entry) {
        const FunctionProfile script_profile =
            profiler.build_script_function_profile(kScriptEntryFunctionName);
        const FunctionProfile* profile_ptr = script_profile.slots.empty() ? nullptr : &script_profile;
        lambda_functions_.clear();
        owned_lambdas_.clear();
        lambda_counter_ = 0;
        lambda_base_index_ = result.functions.size();
        try {
            CompiledFunction main_fn = compile_function(script_entry.get(), profile_ptr);
            result.functions.insert(result.functions.end(), lambda_functions_.begin(),
                                    lambda_functions_.end());
            result.functions.push_back(main_fn);
        } catch (const DryadException&) {
            // Fall through to statement restore below.
        }

        auto* body = dynamic_cast<BlockStatement*>(script_entry->body.get());
        if (body) {
            for (auto& stmt : body->statements) {
                program->statements.push_back(std::move(stmt));
            }
            body->statements.clear();
        }
    }

    return result;
}

} // namespace dryad
