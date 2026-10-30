#ifndef DRYAD_COMPILER_SEMANTIC_ANALYZER_H
#define DRYAD_COMPILER_SEMANTIC_ANALYZER_H

#include "dryad/compiler/ast.h"
#include "dryad/compiler/diagnostic.h"
#include <memory>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>

namespace dryad {

class SemanticAnalyzer {
public:
    std::vector<Diagnostic> analyze(Program* program);

private:
    struct SymbolInfo {
        std::string type;
        bool used;
        bool report_unused;
        SourceLocation location;
    };

    struct Scope {
        std::unordered_map<std::string, SymbolInfo> symbols;
        std::shared_ptr<Scope> parent;

        explicit Scope(std::shared_ptr<Scope> parent_scope = nullptr)
            : parent(std::move(parent_scope)) {}

        void define(const std::string& name, const std::string& type, const SourceLocation& location,
                    bool report_unused = true);
        void mark_used(const std::string& name);
        bool has(const std::string& name) const;
        std::string type_of(const std::string& name) const;
    };

    struct FunctionSignature {
        std::vector<std::string> parameter_types;
        size_t required_parameter_count = 0;
        std::string return_type;
    };

    std::unordered_set<std::string> globals_;
    std::unordered_set<std::string> known_types_;
    std::unordered_map<std::string, FunctionSignature> function_signatures_;
    std::vector<Diagnostic> diagnostics_;
    size_t async_depth_ = 0;

    void register_builtin_types();
    void collect_globals(Program* program);
    void analyze_statement(Statement* stmt, const std::shared_ptr<Scope>& scope);
    void analyze_block(const std::vector<std::unique_ptr<Statement>>& statements,
                       const std::shared_ptr<Scope>& scope, bool check_unused = true);
    std::string analyze_expression(Expression* expr, const std::shared_ptr<Scope>& scope);
    void analyze_call(CallExpression* call, const std::shared_ptr<Scope>& scope);
    void analyze_class_body(ClassDeclaration* stmt);
    void analyze_interface_body(InterfaceDeclaration* stmt);

    void report_error(const std::string& message, const SourceLocation& location);
    void report_warning(const std::string& message, const SourceLocation& location);
    void report_undefined(const std::string& name, const SourceLocation& location);
    void report_unknown_type(const std::string& type_name, const SourceLocation& location);
    void report_type_mismatch(const std::string& expected, const std::string& actual,
                              const SourceLocation& location);
    void validate_type_annotation(const std::string& type_name, const SourceLocation& location);
    bool is_defined(const std::string& name, const std::shared_ptr<Scope>& scope) const;
    bool is_known_type(const std::string& type_name) const;
    bool is_assignable(const std::string& expected, const std::string& actual) const;
    std::string infer_literal_type(Expression* expr) const;
    std::string infer_binary_type(TokenType op, const std::string& left, const std::string& right) const;
    bool is_comparison_or_logical(TokenType op) const;
    bool is_arithmetic(TokenType op) const;
};

} // namespace dryad

#endif // DRYAD_COMPILER_SEMANTIC_ANALYZER_H
