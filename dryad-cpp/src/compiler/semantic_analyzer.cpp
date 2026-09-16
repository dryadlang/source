#include "dryad/compiler/semantic_analyzer.h"
#include "dryad/common/utils.h"

namespace dryad {

void SemanticAnalyzer::Scope::define(const std::string& name, const std::string& type,
                                     const SourceLocation& location, bool report_unused) {
    symbols[name] = SymbolInfo{type, false, report_unused, location};
}

void SemanticAnalyzer::Scope::mark_used(const std::string& name) {
    auto it = symbols.find(name);
    if (it != symbols.end()) {
        it->second.used = true;
        return;
    }
    if (parent) {
        parent->mark_used(name);
    }
}

bool SemanticAnalyzer::Scope::has(const std::string& name) const {
    if (symbols.count(name) != 0) {
        return true;
    }
    if (parent) {
        return parent->has(name);
    }
    return false;
}

std::string SemanticAnalyzer::Scope::type_of(const std::string& name) const {
    auto it = symbols.find(name);
    if (it != symbols.end()) {
        return it->second.type;
    }
    if (parent) {
        return parent->type_of(name);
    }
    return "";
}

void SemanticAnalyzer::register_builtin_types() {
    known_types_.insert("number");
    known_types_.insert("boolean");
    known_types_.insert("string");
    known_types_.insert("void");
    known_types_.insert("byte");
}

std::vector<Diagnostic> SemanticAnalyzer::analyze(Program* program) {
    diagnostics_.clear();
    globals_.clear();
    known_types_.clear();
    function_signatures_.clear();

    register_builtin_types();
    globals_.insert("print");
    function_signatures_["print"] = FunctionSignature{{}, 0, "void"};

    collect_globals(program);

    auto global_scope = std::make_shared<Scope>();
    for (auto& stmt : program->statements) {
        analyze_statement(stmt.get(), global_scope);
    }

    for (const auto& entry : global_scope->symbols) {
        const auto& info = entry.second;
        if (info.report_unused && !info.used) {
            report_warning("Unused variable '" + entry.first + "'", info.location);
        }
    }

    return diagnostics_;
}

void SemanticAnalyzer::collect_globals(Program* program) {
    for (auto& stmt : program->statements) {
        switch (stmt->type) {
            case ASTNodeType::FunctionDeclaration: {
                auto* func_decl = static_cast<FunctionDeclaration*>(stmt.get());
                globals_.insert(func_decl->name);
                FunctionSignature signature;
                signature.return_type = func_decl->return_type;
                for (const auto& param : func_decl->parameters) {
                    signature.parameter_types.push_back(param.type_annotation);
                    if (!param.default_value) {
                        signature.required_parameter_count++;
                    }
                }
                function_signatures_[func_decl->name] = std::move(signature);
                break;
            }
            case ASTNodeType::ClassDeclaration: {
                auto* class_decl = static_cast<ClassDeclaration*>(stmt.get());
                globals_.insert(class_decl->name);
                known_types_.insert(class_decl->name);
                break;
            }
            case ASTNodeType::InterfaceDeclaration: {
                auto* iface_decl = static_cast<InterfaceDeclaration*>(stmt.get());
                globals_.insert(iface_decl->name);
                known_types_.insert(iface_decl->name);
                break;
            }
            case ASTNodeType::ImportDeclaration: {
                auto* import_stmt = static_cast<ImportDeclaration*>(stmt.get());
                for (const auto& name : import_stmt->names) {
                    globals_.insert(name);
                }
                break;
            }
            default:
                break;
        }
    }
}

bool SemanticAnalyzer::is_defined(const std::string& name, const std::shared_ptr<Scope>& scope) const {
    return scope->has(name) || globals_.count(name) != 0;
}

bool SemanticAnalyzer::is_known_type(const std::string& type_name) const {
    return !type_name.empty() && known_types_.count(type_name) != 0;
}

bool SemanticAnalyzer::is_assignable(const std::string& expected, const std::string& actual) const {
    if (expected.empty() || actual.empty() || actual == "unknown") {
        return true;
    }
    if (expected == actual) {
        return true;
    }
    if (expected == "number" && actual == "number") {
        return true;
    }
    return false;
}

bool SemanticAnalyzer::is_arithmetic(TokenType op) const {
    switch (op) {
        case TokenType::Plus:
        case TokenType::Minus:
        case TokenType::Star:
        case TokenType::Slash:
        case TokenType::Percent:
            return true;
        default:
            return false;
    }
}

bool SemanticAnalyzer::is_comparison_or_logical(TokenType op) const {
    switch (op) {
        case TokenType::EqualEqual:
        case TokenType::BangEqual:
        case TokenType::Less:
        case TokenType::LessEqual:
        case TokenType::Greater:
        case TokenType::GreaterEqual:
        case TokenType::AmpersandAmpersand:
        case TokenType::PipePipe:
            return true;
        default:
            return false;
    }
}

std::string SemanticAnalyzer::infer_binary_type(TokenType op, const std::string& left,
                                                const std::string& right) const {
    if (is_comparison_or_logical(op)) {
        return "boolean";
    }
    if (op == TokenType::Plus && left == "string" && right == "string") {
        return "string";
    }
    if ((op == TokenType::Plus || op == TokenType::Minus || op == TokenType::Star ||
         op == TokenType::Slash || op == TokenType::Percent) &&
        left == "number" && right == "number") {
        return "number";
    }
    if (!left.empty() && !right.empty() && left == right) {
        return left;
    }
    return "unknown";
}

std::string SemanticAnalyzer::infer_literal_type(Expression* expr) const {
    if (!expr) {
        return "";
    }

    switch (expr->type) {
        case ASTNodeType::IntegerLiteral:
        case ASTNodeType::FloatLiteral:
            return "number";
        case ASTNodeType::StringLiteral:
            return "string";
        case ASTNodeType::BooleanLiteral:
            return "boolean";
        case ASTNodeType::NullLiteral:
            return "null";
        default:
            return "";
    }
}

void SemanticAnalyzer::report_error(const std::string& message, const SourceLocation& location) {
    diagnostics_.emplace_back(location, DiagnosticSeverity::Error, message);
}

void SemanticAnalyzer::report_warning(const std::string& message, const SourceLocation& location) {
    diagnostics_.emplace_back(location, DiagnosticSeverity::Warning, message);
}

void SemanticAnalyzer::report_undefined(const std::string& name, const SourceLocation& location) {
    report_error("Undefined symbol '" + name + "'", location);
}

void SemanticAnalyzer::report_unknown_type(const std::string& type_name, const SourceLocation& location) {
    report_error("Unknown type '" + type_name + "'", location);
}

void SemanticAnalyzer::report_type_mismatch(const std::string& expected, const std::string& actual,
                                            const SourceLocation& location) {
    report_error("Type mismatch: expected '" + expected + "', got '" + actual + "'", location);
}

void SemanticAnalyzer::validate_type_annotation(const std::string& type_name, const SourceLocation& location) {
    if (!type_name.empty() && !is_known_type(type_name)) {
        report_unknown_type(type_name, location);
    }
}

void SemanticAnalyzer::analyze_call(CallExpression* call, const std::shared_ptr<Scope>& scope) {
    if (call->callee->type != ASTNodeType::Identifier) {
        analyze_expression(call->callee.get(), scope);
        for (auto& arg : call->arguments) {
            analyze_expression(arg.get(), scope);
        }
        return;
    }

    auto* ident = static_cast<Identifier*>(call->callee.get());
    scope->mark_used(ident->name);

    auto signature_it = function_signatures_.find(ident->name);
    if (signature_it == function_signatures_.end()) {
        if (!is_defined(ident->name, scope)) {
            report_undefined(ident->name, ident->location);
        }
        for (auto& arg : call->arguments) {
            analyze_expression(arg.get(), scope);
        }
        return;
    }

    const auto& signature = signature_it->second;
    if (call->arguments.size() < signature.required_parameter_count ||
        call->arguments.size() > signature.parameter_types.size()) {
        report_error("Function '" + ident->name + "' expects between " +
                         std::to_string(signature.required_parameter_count) + " and " +
                         std::to_string(signature.parameter_types.size()) + " arguments, got " +
                         std::to_string(call->arguments.size()),
                     call->location);
    }

    for (size_t i = 0; i < call->arguments.size(); ++i) {
        std::string arg_type = analyze_expression(call->arguments[i].get(), scope);
        if (i < signature.parameter_types.size() && !signature.parameter_types[i].empty() &&
            !arg_type.empty() && !is_assignable(signature.parameter_types[i], arg_type)) {
            report_type_mismatch(signature.parameter_types[i], arg_type, call->arguments[i]->location);
        }
    }
}

std::string SemanticAnalyzer::analyze_expression(Expression* expr, const std::shared_ptr<Scope>& scope) {
    if (!expr) {
        return "";
    }

    switch (expr->type) {
        case ASTNodeType::ErrorExpression:
            return "unknown";
        case ASTNodeType::Identifier: {
            auto* ident = static_cast<Identifier*>(expr);
            if (!is_defined(ident->name, scope)) {
                report_undefined(ident->name, ident->location);
                return "unknown";
            }
            scope->mark_used(ident->name);
            std::string type = scope->type_of(ident->name);
            return type.empty() ? "unknown" : type;
        }
        case ASTNodeType::BinaryExpression: {
            auto* binary = static_cast<BinaryExpression*>(expr);
            std::string left = analyze_expression(binary->left.get(), scope);
            std::string right = analyze_expression(binary->right.get(), scope);
            std::string inferred = infer_binary_type(binary->op, left, right);
            if (inferred == "unknown" && is_arithmetic(binary->op) && !left.empty() && !right.empty() &&
                left != right) {
                report_error("Invalid operands for binary expression: '" + left + "' and '" + right + "'",
                             binary->location);
            }
            return inferred;
        }
        case ASTNodeType::AwaitExpression: {
            if (async_depth_ == 0) {
                report_error("'await' is only valid inside async functions",
                             static_cast<AwaitExpression*>(expr)->location);
            }
            return analyze_expression(static_cast<AwaitExpression*>(expr)->operand.get(), scope);
        }
        case ASTNodeType::UnaryExpression: {
            auto* unary = static_cast<UnaryExpression*>(expr);
            std::string operand = analyze_expression(unary->operand.get(), scope);
            if (unary->op == TokenType::Bang) {
                return "boolean";
            }
            return operand;
        }
        case ASTNodeType::AssignmentExpression: {
            auto* assignment = static_cast<AssignmentExpression*>(expr);
            std::string value_type = analyze_expression(assignment->value.get(), scope);
            if (assignment->target->type == ASTNodeType::Identifier) {
                auto* ident = static_cast<Identifier*>(assignment->target.get());
                scope->mark_used(ident->name);
                if (!is_defined(ident->name, scope)) {
                    report_undefined(ident->name, ident->location);
                } else {
                    std::string expected = scope->type_of(ident->name);
                    if (!expected.empty() && !value_type.empty() && !is_assignable(expected, value_type)) {
                        report_type_mismatch(expected, value_type, assignment->location);
                    }
                }
            } else {
                analyze_expression(assignment->target.get(), scope);
            }
            return value_type;
        }
        case ASTNodeType::CallExpression: {
            auto* call = static_cast<CallExpression*>(expr);
            if (call->callee->type == ASTNodeType::Identifier) {
                auto* ident = static_cast<Identifier*>(call->callee.get());
                analyze_call(call, scope);
                auto signature_it = function_signatures_.find(ident->name);
                if (signature_it != function_signatures_.end() && !signature_it->second.return_type.empty()) {
                    return signature_it->second.return_type;
                }
                return "unknown";
            }
            analyze_expression(call->callee.get(), scope);
            for (auto& arg : call->arguments) {
                analyze_expression(arg.get(), scope);
            }
            return "unknown";
        }
        case ASTNodeType::MemberExpression: {
            auto* member = static_cast<MemberExpression*>(expr);
            if (member->object->type != ASTNodeType::SuperExpression &&
                member->object->type != ASTNodeType::ThisExpression) {
                analyze_expression(member->object.get(), scope);
            }
            if (member->computed) {
                analyze_expression(member->property.get(), scope);
            }
            return "unknown";
        }
        case ASTNodeType::ArrayExpression: {
            auto* array = static_cast<ArrayExpression*>(expr);
            for (auto& element : array->elements) {
                analyze_expression(element.get(), scope);
            }
            return "unknown";
        }
        case ASTNodeType::NewExpression: {
            auto* new_expr = static_cast<NewExpression*>(expr);
            if (globals_.count(new_expr->class_name) == 0) {
                report_undefined(new_expr->class_name, new_expr->location);
            }
            for (auto& arg : new_expr->arguments) {
                analyze_expression(arg.get(), scope);
            }
            return new_expr->class_name;
        }
        case ASTNodeType::FunctionExpression: {
            auto* func_expr = static_cast<FunctionExpression*>(expr);
            auto body_scope = std::make_shared<Scope>(scope);
            for (const auto& param : func_expr->parameters) {
                body_scope->define(param.name, param.type_annotation, func_expr->location, false);
                if (param.default_value) {
                    analyze_expression(param.default_value.get(), scope);
                }
            }
            if (func_expr->body) {
                analyze_block(func_expr->body->statements, body_scope, false);
            }
            return func_expr->return_type.empty() ? "unknown" : func_expr->return_type;
        }
        case ASTNodeType::IntegerLiteral:
        case ASTNodeType::FloatLiteral:
        case ASTNodeType::StringLiteral:
        case ASTNodeType::BooleanLiteral:
        case ASTNodeType::NullLiteral:
            return infer_literal_type(expr);
        case ASTNodeType::ThisExpression:
        case ASTNodeType::SuperExpression:
            return "unknown";
        default:
            return "unknown";
    }
}

void SemanticAnalyzer::analyze_block(const std::vector<std::unique_ptr<Statement>>& statements,
                                     const std::shared_ptr<Scope>& scope, bool check_unused) {
    for (auto& stmt : statements) {
        analyze_statement(stmt.get(), scope);
    }

    if (!check_unused) {
        return;
    }

    for (const auto& entry : scope->symbols) {
        const auto& info = entry.second;
        if (info.report_unused && !info.used) {
            report_warning("Unused variable '" + entry.first + "'", info.location);
        }
    }
}

void SemanticAnalyzer::analyze_class_body(ClassDeclaration* stmt) {
    for (const auto& property_decl : stmt->properties) {
        validate_type_annotation(property_decl->type_annotation, stmt->location);
    }

    auto method_scope = std::make_shared<Scope>();
    method_scope->define("this", "", stmt->location, false);

    for (auto& method_decl : stmt->methods) {
        for (const auto& param : method_decl->parameters) {
            validate_type_annotation(param.type_annotation, stmt->location);
        }

        if (!method_decl->body) {
            continue;
        }

        auto body_scope = std::make_shared<Scope>(method_scope);
        for (const auto& param : method_decl->parameters) {
            body_scope->define(param.name, param.type_annotation, stmt->location, false);
        }
        analyze_block(method_decl->body->statements, body_scope, false);
    }
}

void SemanticAnalyzer::analyze_interface_body(InterfaceDeclaration* stmt) {
    for (const auto& property_decl : stmt->properties) {
        validate_type_annotation(property_decl->type_annotation, stmt->location);
    }
    for (const auto& method_decl : stmt->methods) {
        for (const auto& param : method_decl->parameters) {
            validate_type_annotation(param.type_annotation, stmt->location);
        }
    }
}

void SemanticAnalyzer::analyze_statement(Statement* stmt, const std::shared_ptr<Scope>& scope) {
    if (!stmt) {
        return;
    }

    switch (stmt->type) {
        case ASTNodeType::VariableDeclaration: {
            auto* var_decl = static_cast<VariableDeclaration*>(stmt);
            validate_type_annotation(var_decl->type_annotation, var_decl->location);
            std::string value_type;
            if (var_decl->initializer) {
                value_type = analyze_expression(var_decl->initializer.get(), scope);
            }
            if (!var_decl->type_annotation.empty() && !value_type.empty() &&
                !is_assignable(var_decl->type_annotation, value_type)) {
                report_type_mismatch(var_decl->type_annotation, value_type, var_decl->location);
            }
            std::string resolved_type = var_decl->type_annotation;
            if (resolved_type.empty() && !value_type.empty()) {
                resolved_type = value_type;
            }
            scope->define(var_decl->name, resolved_type, var_decl->location);
            break;
        }
        case ASTNodeType::FunctionDeclaration: {
            auto* func_decl = static_cast<FunctionDeclaration*>(stmt);
            validate_type_annotation(func_decl->return_type, func_decl->location);
            for (const auto& param : func_decl->parameters) {
                validate_type_annotation(param.type_annotation, func_decl->location);
            }
            auto body_scope = std::make_shared<Scope>(scope);
            for (const auto& param : func_decl->parameters) {
                body_scope->define(param.name, param.type_annotation, func_decl->location, false);
            }
            if (func_decl->is_async) {
                ++async_depth_;
            }
            analyze_block(func_decl->body->statements, body_scope, true);
            if (func_decl->is_async) {
                --async_depth_;
            }
            break;
        }
        case ASTNodeType::ClassDeclaration:
            analyze_class_body(static_cast<ClassDeclaration*>(stmt));
            break;
        case ASTNodeType::InterfaceDeclaration:
            analyze_interface_body(static_cast<InterfaceDeclaration*>(stmt));
            break;
        case ASTNodeType::ExpressionStatement:
            analyze_expression(static_cast<ExpressionStatement*>(stmt)->expression.get(), scope);
            break;
        case ASTNodeType::BlockStatement: {
            auto block_scope = std::make_shared<Scope>(scope);
            analyze_block(static_cast<BlockStatement*>(stmt)->statements, block_scope, true);
            break;
        }
        case ASTNodeType::IfStatement: {
            auto* if_stmt = static_cast<IfStatement*>(stmt);
            analyze_expression(if_stmt->condition.get(), scope);
            analyze_statement(if_stmt->then_branch.get(), scope);
            if (if_stmt->else_branch) {
                analyze_statement(if_stmt->else_branch.get(), scope);
            }
            break;
        }
        case ASTNodeType::WhileStatement: {
            auto* while_stmt = static_cast<WhileStatement*>(stmt);
            analyze_expression(while_stmt->condition.get(), scope);
            analyze_statement(while_stmt->body.get(), scope);
            break;
        }
        case ASTNodeType::ReturnStatement: {
            auto* return_stmt = static_cast<ReturnStatement*>(stmt);
            if (return_stmt->value) {
                analyze_expression(return_stmt->value.get(), scope);
            }
            break;
        }
        case ASTNodeType::ImportDeclaration:
            break;
        default:
            break;
    }
}

} // namespace dryad
