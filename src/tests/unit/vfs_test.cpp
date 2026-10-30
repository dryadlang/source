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

Value run_vfs_test(const std::string& test_fn) {
    static const std::string tests = read_file(
        std::string(DRYAD_TEST_DIR) + "/stdlib/io/vfs_test.dryad");

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

TEST(VFSTest, BackendInterfaceDefined) {
    EXPECT_TRUE(run_vfs_test("test_backend_interface_defined").is_truthy());
}

TEST(VFSTest, CreationWithMemoryBackend) {
    EXPECT_TRUE(run_vfs_test("test_vfs_creation_with_memory_backend").is_truthy());
}

TEST(VFSTest, CreationWithNativeBackend) {
    EXPECT_TRUE(run_vfs_test("test_vfs_creation_with_native_backend").is_truthy());
}

TEST(VFSTest, BasicWriteRead) {
    EXPECT_TRUE(run_vfs_test("test_vfs_basic_read_write").is_truthy());
}

TEST(VFSTest, ExistsCheck) {
    EXPECT_TRUE(run_vfs_test("test_vfs_exists_check").is_truthy());
}

TEST(VFSTest, RemoveFile) {
    EXPECT_TRUE(run_vfs_test("test_vfs_remove_file").is_truthy());
}

TEST(VFSTest, Disposal) {
    EXPECT_TRUE(run_vfs_test("test_vfs_disposal").is_truthy());
}

TEST(VFSTest, MultipleFileOperations) {
    EXPECT_TRUE(run_vfs_test("test_multiple_file_operations").is_truthy());
}

TEST(VFSTest, MemoryBackendIndependence) {
    EXPECT_TRUE(run_vfs_test("test_memory_backend_independence").is_truthy());
}

TEST(VFSTest, MemoryBackendListDirectory) {
    EXPECT_TRUE(run_vfs_test("test_memory_backend_list_directory").is_truthy());
}

TEST(VFSTest, MemoryBackendMissingFile) {
    EXPECT_TRUE(run_vfs_test("test_memory_backend_missing_file").is_truthy());
}

TEST(VFSTest, NativeBackendWriteRead) {
    EXPECT_TRUE(run_vfs_test("test_native_backend_write_read").is_truthy());
}

TEST(VFSTest, NativeBackendEmptyFile) {
    EXPECT_TRUE(run_vfs_test("test_native_backend_empty_file").is_truthy());
}

TEST(VFSTest, NativeBackendMkdirAndList) {
    EXPECT_TRUE(run_vfs_test("test_native_backend_mkdir_and_list").is_truthy());
}

TEST(VFSTest, NativeBackendMissingFile) {
    EXPECT_TRUE(run_vfs_test("test_native_backend_missing_file").is_truthy());
}
