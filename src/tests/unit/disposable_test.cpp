#include <gtest/gtest.h>
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"
#include "dryad/compiler/interpreter.h"

using namespace dryad;

TEST(DisposableTest, InterfaceDeclarationParses) {
    std::string code = R"(
        interface IDisposable {
            dispose(): void;
        }
    )";

    Lexer lexer(code);
    auto tokens = lexer.tokenize();
    Parser parser(tokens);
    auto program = parser.parse();

    ASSERT_FALSE(parser.has_error());
    ASSERT_EQ(program->statements.size(), 1u);
    EXPECT_EQ(program->statements[0]->type, ASTNodeType::InterfaceDeclaration);
}

TEST(DisposableTest, ClassImplementsDisposable) {
    std::string code = R"(
        interface IDisposable {
            dispose(): void;
        }

        class Resource implements IDisposable {
            constructor() {
                this.disposed = false;
            }

            dispose() {
                this.disposed = true;
            }
        }

        let r = new Resource();
        r.dispose();
        let disposed = r.disposed;
    )";

    Lexer lexer(code);
    auto tokens = lexer.tokenize();
    Parser parser(tokens);
    auto program = parser.parse();

    ASSERT_FALSE(parser.has_error());

    Interpreter interpreter;
    interpreter.execute(program.get());

    auto flag = interpreter.global_env()->get("disposed");
    ASSERT_TRUE(flag.is_boolean());
    EXPECT_TRUE(flag.as_boolean());
}

TEST(DisposableTest, DisposePatternWithFunctions) {
    std::string code = R"(
        let disposed = false;

        function dispose() {
            disposed = true;
        }

        dispose();
    )";

    Lexer lexer(code);
    auto tokens = lexer.tokenize();
    Parser parser(tokens);
    auto program = parser.parse();

    ASSERT_FALSE(parser.has_error());

    Interpreter interpreter;
    interpreter.execute(program.get());

    auto disposed = interpreter.global_env()->get("disposed");
    ASSERT_TRUE(disposed.is_boolean());
    EXPECT_TRUE(disposed.as_boolean());
}
