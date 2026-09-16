#include "dryad/compiler/bytecode_compiler.h"
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
            builder_.emit_load_fast(slot_index(name));
            push_stack();
            break;
        }

        case ASTNodeType::BinaryExpression:
            compile_binary(static_cast<BinaryExpression*>(expr));
            break;

        case ASTNodeType::AssignmentExpression:
            compile_assignment(static_cast<AssignmentExpression*>(expr));
            break;

        case ASTNodeType::MemberExpression: {
            auto* member = static_cast<MemberExpression*>(expr);
            if (!member->computed) {
                throw DryadException("Bytecode compiler: only computed member access is supported");
            }
            const std::string array_name = identifier_name(member->object.get());
            compile_expression(member->object.get());
            compile_expression(member->property.get());
            if (!array_name.empty() && use_f64_element_access(array_name)) {
                builder_.emit(Opcode::GET_ELEMENT_F64);
            } else {
                builder_.emit(Opcode::GET_ELEMENT);
            }
            pop_stack();
            break;
        }

        default:
            throw DryadException("Bytecode compiler: unsupported expression type");
    }
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
            builder_.emit(Opcode::DIV);
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
    if (expr->target->type != ASTNodeType::Identifier) {
        throw DryadException("Bytecode compiler: only identifier assignment is supported");
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
            compile_statement(while_stmt->body.get());
            builder_.emit_jump(Opcode::JUMP, loop_start);
            builder_.patch_u16(jump_exit, builder_.current_pc());
            break;
        }

        case ASTNodeType::BlockStatement:
            compile_block(static_cast<BlockStatement*>(stmt));
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

CompiledFunction BytecodeCompiler::compile_function(FunctionDeclaration* fn,
                                                    const FunctionProfile* profile) {
    if (!fn || !fn->body) {
        throw DryadException("Bytecode compiler: invalid function declaration");
    }

    builder_ = BytecodeBuilder{};
    slot_map_.clear();
    slot_annotations_.clear();
    profile_ = profile;
    metadata_ = FunctionMetadata{};
    metadata_.name = fn->name;
    stack_depth_ = 0;
    max_stack_depth_ = 0;

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
        result.functions.push_back(compile_function(fn, profile));
    }
    return result;
}

} // namespace dryad
