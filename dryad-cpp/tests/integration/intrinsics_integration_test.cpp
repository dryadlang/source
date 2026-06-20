#include <gtest/gtest.h>
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"
#include "dryad/compiler/interpreter.h"

using namespace dryad;

TEST(IntrinsicsIntegration, FileIOWorkflow) {
    std::string source = R"(
        let fd = __open("/tmp/dryad_integration.txt", 577);
        __write(fd, "Dryad intrinsics work!");
        __close(fd);
        
        let fd_read = __open("/tmp/dryad_integration.txt", 0);
        let content = __read(fd_read, 1024);
        __close(fd_read);
        
        print(content);
        
        __unlink("/tmp/dryad_integration.txt");
    )";
    
    Lexer lexer(source);
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();
    
    Interpreter interp;
    
    testing::internal::CaptureStdout();
    interp.execute(program.get());
    std::string output = testing::internal::GetCapturedStdout();
    
    EXPECT_EQ(output, "Dryad intrinsics work!\n");
}

TEST(IntrinsicsIntegration, EnvironmentVariables) {
    std::string source = R"(
        __setenv("DRYAD_TEST", "hello_world");
        let value = __getenv("DRYAD_TEST");
        print(value);
    )";
    
    Lexer lexer(source);
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();
    
    Interpreter interp;
    
    testing::internal::CaptureStdout();
    interp.execute(program.get());
    std::string output = testing::internal::GetCapturedStdout();
    
    EXPECT_EQ(output, "hello_world\n");
}

TEST(IntrinsicsIntegration, FileSystemOperations) {
    std::string source = R"(
        let cwd = __getcwd();
        print(cwd);
        
        __mkdir("/tmp/dryad_test_dir");
        __chdir("/tmp/dryad_test_dir");
        let new_cwd = __getcwd();
        __chdir(cwd);
        __rmdir("/tmp/dryad_test_dir");
        
        print(new_cwd);
    )";
    
    Lexer lexer(source);
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();
    
    Interpreter interp;
    
    testing::internal::CaptureStdout();
    interp.execute(program.get());
    std::string output = testing::internal::GetCapturedStdout();
    
    EXPECT_TRUE(output.find("/tmp/dryad_test_dir") != std::string::npos);
}

TEST(IntrinsicsIntegration, ProcessInfo) {
    std::string source = R"(
        let pid = __getpid();
        print(pid);
    )";
    
    Lexer lexer(source);
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();
    
    Interpreter interp;
    
    testing::internal::CaptureStdout();
    interp.execute(program.get());
    std::string output = testing::internal::GetCapturedStdout();
    
    EXPECT_FALSE(output.empty());
    EXPECT_NE(output, "0\n");
}
