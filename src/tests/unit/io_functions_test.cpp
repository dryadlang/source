#include <gtest/gtest.h>
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"
#include "dryad/compiler/interpreter.h"
#include <fstream>
#include <sstream>
#include <stdexcept>

using namespace dryad;

namespace {

std::string read_file(const std::string& path) {
    std::ifstream f(path);
    if (!f) {
        throw std::runtime_error("Cannot open file: " + path);
    }
    std::ostringstream ss;
    ss << f.rdbuf();
    return ss.str();
}

Value run_io_test(const std::string& test_fn) {
    static const std::string tests = read_file(
        std::string(DRYAD_TEST_DIR) + "/stdlib/io/io_functions_test.dryad");

    std::string source = tests + "\nlet __ok = " + test_fn + "();";

    Lexer lexer(source);
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();
    if (parser.has_error()) {
        throw std::runtime_error("Parser error: " + parser.error_message());
    }

    Interpreter interpreter;
    interpreter.set_stdlib_root(DRYAD_STDLIB_DIR);
    interpreter.execute(program.get());
    return interpreter.global_env()->get("__ok");
}

} // namespace

TEST(IOFunctionsTest, ReadWriteFile) {
    EXPECT_TRUE(run_io_test("test_read_write_file").is_truthy());
}

TEST(IOFunctionsTest, ExistsFunction) {
    EXPECT_TRUE(run_io_test("test_exists_function").is_truthy());
}

TEST(IOFunctionsTest, RemoveFileFunction) {
    EXPECT_TRUE(run_io_test("test_remove_file_function").is_truthy());
}

TEST(IOFunctionsTest, SetVFSBackendIsolation) {
    EXPECT_TRUE(run_io_test("test_set_vfs_backend_isolation").is_truthy());
}

TEST(IOFunctionsTest, MkdirFunction) {
    EXPECT_TRUE(run_io_test("test_mkdir_function").is_truthy());
}

TEST(IOFunctionsTest, ListDirectoryFunction) {
    EXPECT_TRUE(run_io_test("test_list_directory_function").is_truthy());
}

TEST(IOFunctionsTest, NativeReadWriteViaFileApi) {
    EXPECT_TRUE(run_io_test("test_native_read_write_via_file_api").is_truthy());
}
