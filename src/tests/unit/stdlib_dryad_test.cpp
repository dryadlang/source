#include <gtest/gtest.h>
#include "dryad/common/utils.h"
#include "dryad/compiler/interpreter.h"
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"
#include <filesystem>
#include <fstream>
#include <regex>
#include <sstream>
#include <string>
#include <vector>

// Runs every `function test_*()` in tests/stdlib/**/*.dryad files whose first
// line is `// @auto-test`, one gtest case per function. A test passes by
// returning `true`; returning a string reports it as the failure message.
// Add `// @allow-net` on the second line for tests that open loopback sockets.

using namespace dryad;
namespace fs = std::filesystem;

namespace {

struct DryadCase {
    std::string file;
    std::string function;
    bool allow_net = false;
};

// Readable test parameters in gtest output instead of a byte dump.
void PrintTo(const DryadCase& test, std::ostream* os) {
    *os << test.file << ":" << test.function;
}

std::string read_file(const fs::path& path) {
    std::ifstream file(path);
    std::ostringstream text;
    text << file.rdbuf();
    return text.str();
}

std::vector<DryadCase> discover() {
    std::vector<DryadCase> cases;
    const fs::path root = fs::path(DRYAD_TEST_DIR) / "stdlib";
    std::vector<fs::path> files;
    for (const auto& entry : fs::recursive_directory_iterator(root)) {
        if (entry.is_regular_file() && entry.path().extension() == ".dryad") {
            files.push_back(entry.path());
        }
    }
    std::sort(files.begin(), files.end());
    static const std::regex test_fn(R"(^function\s+(test_\w+)\s*\(\s*\))");
    for (const auto& path : files) {
        const std::string text = read_file(path);
        if (text.rfind("// @auto-test", 0) != 0) {
            continue;
        }
        const bool allow_net = text.find("// @allow-net") != std::string::npos;
        std::istringstream lines(text);
        std::string line;
        while (std::getline(lines, line)) {
            std::smatch match;
            if (std::regex_search(line, match, test_fn)) {
                cases.push_back({fs::relative(path, root).generic_string(), match[1], allow_net});
            }
        }
    }
    return cases;
}

std::string case_name(const testing::TestParamInfo<DryadCase>& info) {
    std::string name = info.param.file.substr(0, info.param.file.size() - 6) + "_" + info.param.function;
    for (char& c : name) {
        if (!std::isalnum(static_cast<unsigned char>(c))) {
            c = '_';
        }
    }
    return name;
}

class StdlibDryadTest : public testing::TestWithParam<DryadCase> {};

} // namespace

TEST_P(StdlibDryadTest, Passes) {
    const DryadCase& test = GetParam();
    const fs::path path = fs::path(DRYAD_TEST_DIR) / "stdlib" / test.file;
    const std::string source = read_file(path) + "\nlet __result = " + test.function + "();\n";

    Lexer lexer(source);
    Parser parser(lexer.tokenize());
    auto program = parser.parse();
    ASSERT_FALSE(parser.has_error()) << test.file << ": " << parser.error_message();

    Interpreter interpreter;
    interpreter.set_stdlib_root(DRYAD_STDLIB_DIR);
    interpreter.set_network_allowed(test.allow_net);
    interpreter.execute(program.get());

    const Value result = interpreter.global_env()->get("__result");
    if (result.is_boolean() && result.as_boolean()) {
        return;
    }
    ADD_FAILURE() << test.file << " " << test.function << " returned "
                  << (result.is_string() ? "\"" + result.as_string() + "\"" : result.to_string());
}

INSTANTIATE_TEST_SUITE_P(Stdlib, StdlibDryadTest, testing::ValuesIn(discover()), case_name);
GTEST_ALLOW_UNINSTANTIATED_PARAMETERIZED_TEST(StdlibDryadTest);
