#ifndef DRYAD_COMPILER_EVALUATOR_H
#define DRYAD_COMPILER_EVALUATOR_H

#include "dryad/compiler/ast.h"
#include "dryad/runtime/value.h"
#include "dryad/runtime/environment.h"
#include <memory>

namespace dryad {

class Interpreter;

class ExpressionEvaluator {
public:
    explicit ExpressionEvaluator(Interpreter* interpreter);
    
    Value evaluate(Expression* expr);
    
private:
    Interpreter* interpreter_;
    
    Value evaluate_literal(Expression* expr);
    Value evaluate_identifier(Identifier* expr);
    Value evaluate_binary(BinaryExpression* expr);
    Value evaluate_unary(UnaryExpression* expr);
    Value evaluate_call(CallExpression* expr);
    Value evaluate_assignment(AssignmentExpression* expr);
    Value evaluate_member(MemberExpression* expr);
    Value evaluate_array(ArrayExpression* expr);
    Value evaluate_new(NewExpression* expr);
    Value evaluate_this(ThisExpression* expr);
    
    Value evaluate_arithmetic(TokenType op, const Value& left, const Value& right);
    Value evaluate_comparison(TokenType op, const Value& left, const Value& right);
    Value evaluate_logical(TokenType op, const Value& left, const Value& right);
};

} // namespace dryad

#endif // DRYAD_COMPILER_EVALUATOR_H