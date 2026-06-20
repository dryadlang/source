#include "dryad/compiler/evaluator.h"
#include "dryad/compiler/interpreter.h"
#include "dryad/runtime/function.h"
#include "dryad/runtime/class.h"
#include "dryad/common/utils.h"

namespace dryad {

ExpressionEvaluator::ExpressionEvaluator(Interpreter* interpreter) 
    : interpreter_(interpreter) {}

Value ExpressionEvaluator::evaluate(Expression* expr) {
    switch (expr->type) {
        case ASTNodeType::IntegerLiteral:
        case ASTNodeType::FloatLiteral:
        case ASTNodeType::StringLiteral:
        case ASTNodeType::BooleanLiteral:
        case ASTNodeType::NullLiteral:
            return evaluate_literal(expr);
        
        case ASTNodeType::Identifier:
            return evaluate_identifier(static_cast<Identifier*>(expr));
        
        case ASTNodeType::BinaryExpression:
            return evaluate_binary(static_cast<BinaryExpression*>(expr));
        
        case ASTNodeType::UnaryExpression:
            return evaluate_unary(static_cast<UnaryExpression*>(expr));
        
        case ASTNodeType::AssignmentExpression:
            return evaluate_assignment(static_cast<AssignmentExpression*>(expr));
        
        case ASTNodeType::CallExpression:
            return evaluate_call(static_cast<CallExpression*>(expr));
        
        case ASTNodeType::MemberExpression:
            return evaluate_member(static_cast<MemberExpression*>(expr));
        
        case ASTNodeType::ArrayExpression:
            return evaluate_array(static_cast<ArrayExpression*>(expr));
        
        case ASTNodeType::NewExpression:
            return evaluate_new(static_cast<NewExpression*>(expr));
        
        case ASTNodeType::ThisExpression:
            return evaluate_this(static_cast<ThisExpression*>(expr));
        
        default:
            throw DryadException("Unknown expression type");
    }
}

Value ExpressionEvaluator::evaluate_literal(Expression* expr) {
    switch (expr->type) {
        case ASTNodeType::IntegerLiteral:
            return Value(static_cast<IntegerLiteral*>(expr)->value);
        case ASTNodeType::FloatLiteral:
            return Value(static_cast<FloatLiteral*>(expr)->value);
        case ASTNodeType::StringLiteral:
            return Value(static_cast<StringLiteral*>(expr)->value);
        case ASTNodeType::BooleanLiteral:
            return Value(static_cast<BooleanLiteral*>(expr)->value);
        case ASTNodeType::NullLiteral:
            return Value();
        default:
            throw DryadException("Not a literal expression");
    }
}

Value ExpressionEvaluator::evaluate_identifier(Identifier* expr) {
    return interpreter_->current_env()->get(expr->name);
}

Value ExpressionEvaluator::evaluate_binary(BinaryExpression* expr) {
    Value left = evaluate(expr->left.get());
    Value right = evaluate(expr->right.get());
    
    switch (expr->op) {
        case TokenType::Plus:
        case TokenType::Minus:
        case TokenType::Star:
        case TokenType::Slash:
        case TokenType::Percent:
            return evaluate_arithmetic(expr->op, left, right);
        
        case TokenType::Less:
        case TokenType::LessEqual:
        case TokenType::Greater:
        case TokenType::GreaterEqual:
        case TokenType::EqualEqual:
        case TokenType::BangEqual:
            return evaluate_comparison(expr->op, left, right);
        
        case TokenType::AmpersandAmpersand:
        case TokenType::PipePipe:
            return evaluate_logical(expr->op, left, right);
        
        default:
            throw DryadException("Unknown binary operator");
    }
}

Value ExpressionEvaluator::evaluate_arithmetic(TokenType op, const Value& left, const Value& right) {
    if (op == TokenType::Plus) {
        if (left.is_integer() && right.is_integer()) {
            return Value(left.as_integer() + right.as_integer());
        }
        if (left.is_number() && right.is_number()) {
            return Value(left.as_float() + right.as_float());
        }
        if (left.is_string() && right.is_string()) {
            return Value(left.as_string() + right.as_string());
        }
        throw DryadException("Invalid operands for +");
    }
    
    if (!left.is_number() || !right.is_number()) {
        throw DryadException("Arithmetic requires numeric operands");
    }
    
    bool is_float = left.is_float() || right.is_float();
    
    switch (op) {
        case TokenType::Minus:
            return is_float ? Value(left.as_float() - right.as_float()) 
                           : Value(left.as_integer() - right.as_integer());
        case TokenType::Star:
            return is_float ? Value(left.as_float() * right.as_float())
                           : Value(left.as_integer() * right.as_integer());
        case TokenType::Slash:
            return is_float ? Value(left.as_float() / right.as_float())
                           : Value(left.as_integer() / right.as_integer());
        case TokenType::Percent:
            if (is_float) throw DryadException("Modulo requires integer operands");
            return Value(left.as_integer() % right.as_integer());
        default:
            throw DryadException("Unknown arithmetic operator");
    }
}

Value ExpressionEvaluator::evaluate_comparison(TokenType op, const Value& left, const Value& right) {
    if (op == TokenType::EqualEqual) {
        if (left.type() != right.type()) return Value(false);
        if (left.is_null()) return Value(true);
        if (left.is_boolean()) return Value(left.as_boolean() == right.as_boolean());
        if (left.is_integer()) return Value(left.as_integer() == right.as_integer());
        if (left.is_float()) return Value(left.as_float() == right.as_float());
        if (left.is_string()) return Value(left.as_string() == right.as_string());
        return Value(false);
    }
    
    if (op == TokenType::BangEqual) {
        Value eq = evaluate_comparison(TokenType::EqualEqual, left, right);
        return Value(!eq.as_boolean());
    }
    
    if (!left.is_number() || !right.is_number()) {
        throw DryadException("Comparison requires numeric operands");
    }
    
    double l = left.as_float();
    double r = right.as_float();
    
    switch (op) {
        case TokenType::Less:         return Value(l < r);
        case TokenType::LessEqual:    return Value(l <= r);
        case TokenType::Greater:      return Value(l > r);
        case TokenType::GreaterEqual: return Value(l >= r);
        default: throw DryadException("Unknown comparison operator");
    }
}

Value ExpressionEvaluator::evaluate_logical(TokenType op, const Value& left, const Value& right) {
    bool left_truthy = left.is_truthy();
    
    if (op == TokenType::AmpersandAmpersand) {
        return left_truthy ? right : left;
    } else if (op == TokenType::PipePipe) {
        return left_truthy ? left : right;
    }
    
    throw DryadException("Unknown logical operator");
}

Value ExpressionEvaluator::evaluate_unary(UnaryExpression* expr) {
    Value operand = evaluate(expr->operand.get());
    
    switch (expr->op) {
        case TokenType::Minus:
            if (operand.is_integer()) {
                return Value(-operand.as_integer());
            }
            if (operand.is_float()) {
                return Value(-operand.as_float());
            }
            throw DryadException("Unary minus requires numeric operand");
        
        case TokenType::Bang:
            return Value(!operand.is_truthy());
        
        default:
            throw DryadException("Unknown unary operator");
    }
}

Value ExpressionEvaluator::evaluate_call(CallExpression* expr) {
    Value callee = evaluate(expr->callee.get());
    
    if (!callee.is_function()) {
        throw DryadException("Cannot call non-function value");
    }
    
    std::vector<Value> arguments;
    for (auto& arg : expr->arguments) {
        arguments.push_back(evaluate(arg.get()));
    }
    
    auto func = callee.as_function();
    return func->call(interpreter_, arguments);
}

Value ExpressionEvaluator::evaluate_assignment(AssignmentExpression* expr) {
    Value value = evaluate(expr->value.get());
    
    if (expr->target->type == ASTNodeType::Identifier) {
        auto* ident = static_cast<Identifier*>(expr->target.get());
        interpreter_->current_env()->assign(ident->name, value);
        return value;
    }
    
    if (expr->target->type == ASTNodeType::MemberExpression) {
        auto* member = static_cast<MemberExpression*>(expr->target.get());
        Value obj = evaluate(member->object.get());
        
        if (member->computed) {
            Value index = evaluate(member->property.get());
            if (obj.is_array()) {
                int64_t idx = index.as_integer();
                obj.array_set(idx, value);
            } else if (obj.is_object()) {
                std::string key = index.to_string();
                obj.object_set(key, value);
            } else if (obj.is_instance()) {
                std::string key = index.to_string();
                obj.as_instance()->set(key, value);
            }
        } else {
            auto* prop_ident = static_cast<Identifier*>(member->property.get());
            if (obj.is_object()) {
                obj.object_set(prop_ident->name, value);
            } else if (obj.is_instance()) {
                obj.as_instance()->set(prop_ident->name, value);
            }
        }
        return value;
    }
    
    throw DryadException("Invalid assignment target");
}

Value ExpressionEvaluator::evaluate_member(MemberExpression* expr) {
    Value obj = evaluate(expr->object.get());
    
    if (expr->computed) {
        Value index = evaluate(expr->property.get());
        
        if (obj.is_array()) {
            if (!index.is_integer()) {
                throw DryadException("Array index must be an integer");
            }
            return obj.array_get(static_cast<size_t>(index.as_integer()));
        } else if (obj.is_object()) {
            if (!index.is_string()) {
                throw DryadException("Object key must be a string");
            }
            return obj.object_get(index.as_string());
        } else {
            throw DryadException("Cannot access member of non-object/array value");
        }
    } else {
        if (!obj.is_object() && !obj.is_instance()) {
            throw DryadException("Non-computed member access requires an object or instance");
        }
        
        auto* id_expr = dynamic_cast<Identifier*>(expr->property.get());
        if (!id_expr) {
            throw DryadException("Property must be an identifier");
        }
        
        if (obj.is_instance()) {
            return obj.as_instance()->get(id_expr->name);
        } else {
            return obj.object_get(id_expr->name);
        }
    }
}

Value ExpressionEvaluator::evaluate_array(ArrayExpression* expr) {
    Value arr = Value::create_array();
    
    for (const auto& elem_expr : expr->elements) {
        Value elem = evaluate(elem_expr.get());
        arr.array_push(elem);
    }
    
    return arr;
}


Value ExpressionEvaluator::evaluate_new(NewExpression* expr) {
    Value class_value = interpreter_->current_env()->get(expr->class_name);
    
    if (!class_value.is_class()) {
        throw DryadException("Cannot instantiate non-class value: " + expr->class_name);
    }
    
    auto klass = class_value.as_class();
    
    std::vector<Value> args;
    for (const auto& arg_expr : expr->arguments) {
        args.push_back(evaluate(arg_expr.get()));
    }
    
    auto instance = klass->instantiate(args, interpreter_);
    return Value(instance);
}

Value ExpressionEvaluator::evaluate_this(ThisExpression* expr) {
    (void)expr;
    return interpreter_->current_env()->get("this");
}

} // namespace dryad
