#include "dryad/compiler/statement_executor.h"
#include "dryad/compiler/interpreter.h"
#include "dryad/runtime/function.h"
#include "dryad/runtime/class.h"
#include "dryad/common/utils.h"
#include <unordered_set>

namespace dryad {

namespace {

constexpr size_t kMaxInheritanceDepth = 256;

void register_property_accessor(const std::shared_ptr<DryadClass>& klass,
                                PropertyDeclaration* property_decl,
                                Interpreter* interpreter,
                                bool is_getter) {
    std::unique_ptr<BlockStatement> body = is_getter ? std::move(property_decl->getter_body)
                                                     : std::move(property_decl->setter_body);
    if (!body) {
        return;
    }

    std::vector<Parameter> parameters;
    if (!is_getter) {
        parameters.push_back({property_decl->setter_parameter, property_decl->type_annotation});
    }

    auto func_decl = std::make_unique<FunctionDeclaration>(
        property_decl->name,
        std::move(parameters),
        std::move(body),
        "",
        false
    );

    auto accessor_func = std::make_shared<DryadFunction>(
        func_decl.get(),
        interpreter->current_env(),
        klass
    );

    if (is_getter) {
        klass->getters[property_decl->name] = accessor_func;
    } else {
        klass->setters[property_decl->name] = accessor_func;
    }

    klass->owned_declarations.push_back(std::move(func_decl));
}

void validate_inheritance_chain(const std::string& class_name, const std::shared_ptr<DryadClass>& super) {
    std::unordered_set<std::string> seen;
    seen.insert(class_name);

    size_t depth = 0;
    auto current = super;
    while (current) {
        if (seen.count(current->name) != 0) {
            throw DryadException("Circular inheritance detected involving class '" + class_name + "'");
        }
        seen.insert(current->name);
        if (++depth > kMaxInheritanceDepth) {
            throw DryadException("Inheritance depth limit exceeded for class '" + class_name + "'");
        }
        current = current->super_class;
    }
}

} // namespace

StatementExecutor::StatementExecutor(Interpreter* interpreter)
    : interpreter_(interpreter) {}

void StatementExecutor::execute(Statement* stmt) {
    switch (stmt->type) {
        case ASTNodeType::ExpressionStatement:
            execute_expression(static_cast<ExpressionStatement*>(stmt));
            break;
        
        case ASTNodeType::VariableDeclaration:
            execute_variable_declaration(static_cast<VariableDeclaration*>(stmt));
            break;
        
        case ASTNodeType::FunctionDeclaration:
            execute_function_declaration(static_cast<FunctionDeclaration*>(stmt));
            break;
        
        case ASTNodeType::ClassDeclaration:
            execute_class_declaration(static_cast<ClassDeclaration*>(stmt));
            break;

        case ASTNodeType::InterfaceDeclaration:
            execute_interface_declaration(static_cast<InterfaceDeclaration*>(stmt));
            break;
        
        case ASTNodeType::BlockStatement:
            execute_block_statement(static_cast<BlockStatement*>(stmt));
            break;
        
        case ASTNodeType::IfStatement:
            execute_if(static_cast<IfStatement*>(stmt));
            break;
        
        case ASTNodeType::WhileStatement:
            execute_while(static_cast<WhileStatement*>(stmt));
            break;
        
        case ASTNodeType::ReturnStatement:
            execute_return(static_cast<ReturnStatement*>(stmt));
            break;

        case ASTNodeType::ImportDeclaration:
            execute_import(static_cast<ImportDeclaration*>(stmt));
            break;
        
        default:
            throw DryadException("Unknown statement type");
    }
}

void StatementExecutor::execute_expression(ExpressionStatement* stmt) {
    interpreter_->evaluate(stmt->expression.get());
}

void StatementExecutor::execute_variable_declaration(VariableDeclaration* stmt) {
    Value value;
    if (stmt->initializer) {
        value = interpreter_->evaluate(stmt->initializer.get());
    }
    interpreter_->current_env()->define(stmt->name, value);
}

void StatementExecutor::execute_function_declaration(FunctionDeclaration* stmt) {
    auto func = std::make_shared<DryadFunction>(stmt, interpreter_->current_env());
    interpreter_->current_env()->define(stmt->name, Value(func));
}

void StatementExecutor::execute_block_statement(BlockStatement* stmt) {
    auto block_env = std::make_shared<Environment>(interpreter_->current_env());
    execute_block(stmt->statements, block_env);
}

void StatementExecutor::execute_block(const std::vector<std::unique_ptr<Statement>>& statements,
                                     std::shared_ptr<Environment> env) {
    auto previous = interpreter_->current_env();
    interpreter_->set_current_env(env);
    
    try {
        for (auto& statement : statements) {
            execute(statement.get());
        }
    } catch (...) {
        interpreter_->set_current_env(previous);
        throw;
    }
    
    interpreter_->set_current_env(previous);
}

void StatementExecutor::execute_if(IfStatement* stmt) {
    Value condition = interpreter_->evaluate(stmt->condition.get());
    
    if (condition.is_truthy()) {
        execute(stmt->then_branch.get());
    } else if (stmt->else_branch) {
        execute(stmt->else_branch.get());
    }
}

void StatementExecutor::execute_while(WhileStatement* stmt) {
    while (true) {
        Value condition = interpreter_->evaluate(stmt->condition.get());
        if (!condition.is_truthy()) break;
        
        try {
            execute(stmt->body.get());
        } catch (const BreakException&) {
            break;
        } catch (const ContinueException&) {
            continue;
        }
    }
}

void StatementExecutor::execute_return(ReturnStatement* stmt) {
    Value value;
    if (stmt->value) {
        value = interpreter_->evaluate(stmt->value.get());
    }
    throw ReturnException(value);
}

void StatementExecutor::execute_import(ImportDeclaration* stmt) {
    interpreter_->module_loader().execute_import(stmt, interpreter_);
}

void StatementExecutor::execute_interface_declaration(InterfaceDeclaration* stmt) {
    auto iface = std::make_shared<DryadClass>(stmt->name);
    iface->is_interface = true;
    iface->is_abstract = true;

    for (const auto& property_decl : stmt->properties) {
        iface->abstract_properties.insert(property_decl->name);
        if (property_decl->is_readonly) {
            iface->readonly_properties.insert(property_decl->name);
        }
    }

    for (const auto& method_decl : stmt->methods) {
        iface->abstract_methods.insert(method_decl->name);
    }

    interpreter_->current_env()->define(stmt->name, Value(iface));
}

void StatementExecutor::execute_class_declaration(ClassDeclaration* stmt) {
    std::shared_ptr<DryadClass> super;
    if (!stmt->super_class.empty()) {
        Value super_value = interpreter_->current_env()->get(stmt->super_class);
        if (!super_value.is_class()) {
            throw DryadException("Superclass is not a class: " + stmt->super_class);
        }
        super = super_value.as_class();
        if (super->is_interface) {
            throw DryadException("Cannot extend interface: " + stmt->super_class);
        }
        validate_inheritance_chain(stmt->name, super);
    }

    auto klass = std::make_shared<DryadClass>(stmt->name, super);
    klass->is_abstract = stmt->is_abstract;

    if (super) {
        klass->abstract_methods = super->abstract_methods;
        klass->abstract_properties = super->abstract_properties;
        klass->readonly_properties = super->readonly_properties;
    }

    for (const auto& iface_name : stmt->interfaces) {
        Value iface_value = interpreter_->current_env()->get(iface_name);
        if (!iface_value.is_class()) {
            throw DryadException("Interface is not defined: " + iface_name);
        }
        auto iface = iface_value.as_class();
        if (!iface->is_interface) {
            throw DryadException("'" + iface_name + "' is not an interface");
        }
        klass->abstract_methods.insert(iface->abstract_methods.begin(), iface->abstract_methods.end());
        klass->abstract_properties.insert(iface->abstract_properties.begin(), iface->abstract_properties.end());
    }

    for (auto& property_decl : stmt->properties) {
        if (property_decl->is_abstract) {
            if (!stmt->is_abstract && !super) {
                throw DryadException("Abstract properties are only allowed in abstract classes");
            }
            klass->abstract_properties.insert(property_decl->name);
            continue;
        }

        klass->properties.insert(property_decl->name);
        klass->abstract_properties.erase(property_decl->name);
        klass->abstract_methods.erase(property_decl->name);

        if (property_decl->is_readonly) {
            klass->readonly_properties.insert(property_decl->name);
        }
        if (!property_decl->type_annotation.empty()) {
            klass->property_types[property_decl->name] = property_decl->type_annotation;
        }

        if (property_decl->has_getter && property_decl->getter_body) {
            register_property_accessor(klass, property_decl.get(), interpreter_, true);
        }
        if (property_decl->has_setter && property_decl->setter_body) {
            register_property_accessor(klass, property_decl.get(), interpreter_, false);
        }
    }

    for (auto& method_decl : stmt->methods) {
        if (method_decl->is_abstract) {
            if (!stmt->is_abstract && !super) {
                throw DryadException("Abstract methods are only allowed in abstract classes");
            }
            klass->abstract_methods.insert(method_decl->name);
            continue;
        }

        if (!method_decl->body) {
            throw DryadException("Method '" + method_decl->name + "' is missing a body");
        }

        auto func_decl = std::make_unique<FunctionDeclaration>(
            method_decl->name,
            method_decl->parameters,
            std::move(method_decl->body),
            "",
            false
        );

        auto method_func = std::make_shared<DryadFunction>(
            func_decl.get(),
            interpreter_->current_env(),
            klass
        );

        klass->abstract_methods.erase(method_decl->name);
        klass->abstract_properties.erase(method_decl->name);

        if (method_decl->is_getter) {
            klass->getters[method_decl->name] = method_func;
        } else if (method_decl->is_setter) {
            klass->setters[method_decl->name] = method_func;
        } else if (method_decl->is_constructor) {
            klass->constructor = method_func;
        } else {
            klass->methods[method_decl->name] = method_func;
        }

        klass->owned_declarations.push_back(std::move(func_decl));
    }

    for (const auto& member_name : std::vector<std::string>(klass->abstract_methods.begin(), klass->abstract_methods.end())) {
        if (klass->has_member(member_name)) {
            klass->abstract_methods.erase(member_name);
        }
    }
    for (const auto& property_name : std::vector<std::string>(klass->abstract_properties.begin(), klass->abstract_properties.end())) {
        if (klass->has_member(property_name)) {
            klass->abstract_properties.erase(property_name);
        }
    }

    if (!stmt->is_abstract && (!klass->abstract_methods.empty() || !klass->abstract_properties.empty())) {
        throw DryadException("Class '" + stmt->name + "' must implement all abstract members");
    }

    interpreter_->current_env()->define(stmt->name, Value(klass));
}

} // namespace dryad
