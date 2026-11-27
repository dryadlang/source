#include <gtest/gtest.h>
#include "dryad/compiler/interpreter.h"
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"
#include <filesystem>
#include <fstream>
#include <regex>
#include <sstream>
#include <string>
#include <vector>

using namespace dryad;
namespace fs = std::filesystem;

// Enforces the @std naming convention (docs/stdlib/README.md, "Naming"):
//   - public functions are Type_PascalCase (or a single PascalCase word),
//   - helpers start with an underscore,
//   - pre-convention names survive only as `// @deprecated` aliases that
//     resolve to the same function as their replacement.

namespace {

struct Alias {
    std::string module;
    std::string old_name;
    std::string new_name;
};

std::string read_file(const fs::path& path) {
    std::ifstream file(path);
    std::ostringstream text;
    text << file.rdbuf();
    return text.str();
}

std::string module_of(const fs::path& path) {
    std::string rel = fs::relative(path, DRYAD_STDLIB_DIR).replace_extension("").generic_string();
    return rel;
}

std::vector<fs::path> stdlib_files() {
    std::vector<fs::path> files;
    for (const auto& entry : fs::recursive_directory_iterator(DRYAD_STDLIB_DIR)) {
        if (entry.is_regular_file() && entry.path().extension() == ".dryad") {
            files.push_back(entry.path());
        }
    }
    return files;
}

std::vector<Alias> deprecated_aliases() {
    static const std::regex alias(R"(// @deprecated[^\n]*\nlet (\w+) = (\w+);)");
    std::vector<Alias> out;
    for (const auto& path : stdlib_files()) {
        const std::string text = read_file(path);
        for (auto it = std::sregex_iterator(text.begin(), text.end(), alias); it != std::sregex_iterator(); ++it) {
            out.push_back({module_of(path), (*it)[1], (*it)[2]});
        }
    }
    return out;
}

} // namespace

TEST(StdlibNamingTest, TopLevelFunctionsFollowTheConvention) {
    static const std::regex function_decl(R"(^(?:async\s+)?function\s+(\w+)\s*\()");
    static const std::regex conforming(R"(^(_\w+|[A-Z][A-Za-z0-9]*(_[A-Z][A-Za-z0-9]*)?)$)");
    std::vector<std::string> offenders;
    for (const auto& path : stdlib_files()) {
        std::istringstream lines(read_file(path));
        std::string line;
        while (std::getline(lines, line)) {
            std::smatch match;
            if (std::regex_search(line, match, function_decl) && !std::regex_match(match[1].str(), conforming)) {
                offenders.push_back(module_of(path) + ": " + match[1].str());
            }
        }
    }
    std::string list;
    for (const auto& name : offenders) {
        list += "\n  " + name;
    }
    EXPECT_TRUE(offenders.empty()) << "functions breaking the naming convention:" << list;
}

TEST(StdlibNamingTest, DeprecatedAliasesResolveToTheirReplacement) {
    const std::vector<Alias> aliases = deprecated_aliases();
    ASSERT_FALSE(aliases.empty());
    for (const Alias& alias : aliases) {
        SCOPED_TRACE(alias.module + ": " + alias.old_name + " -> " + alias.new_name);
        const std::string source = "import { " + alias.old_name + ", " + alias.new_name + " } from \"" +
                                   alias.module + "\";";
        Lexer lexer(source);
        Parser parser(lexer.tokenize());
        auto program = parser.parse();
        ASSERT_FALSE(parser.has_error()) << parser.error_message();

        Interpreter interp;
        interp.set_stdlib_root(DRYAD_STDLIB_DIR);
        interp.execute(program.get());
        const Value old_value = interp.global_env()->get(alias.old_name);
        const Value new_value = interp.global_env()->get(alias.new_name);
        ASSERT_TRUE(old_value.is_function());
        ASSERT_TRUE(new_value.is_function());
        EXPECT_EQ(old_value.as_function().get(), new_value.as_function().get());
    }
}

TEST(StdlibNamingTest, BarrelsStillExportDeprecatedNames) {
    const char* source = R"(
        import { Promise_resolve, Thread_sleepMs, EventLoop_pump } from "@std/async/async";
        import { TcpClient_connect, Net_resolveHost, Socket_fromFd } from "@std/net/net";
        import { readFile, File_open, MemoryStream_fromString } from "@std/io/file";
    )";
    Lexer lexer(source);
    Parser parser(lexer.tokenize());
    auto program = parser.parse();
    ASSERT_FALSE(parser.has_error()) << parser.error_message();
    Interpreter interp;
    interp.set_stdlib_root(DRYAD_STDLIB_DIR);
    interp.execute(program.get());
    for (const char* name : {"Promise_resolve", "Thread_sleepMs", "EventLoop_pump", "TcpClient_connect",
                             "Net_resolveHost", "Socket_fromFd", "readFile", "File_open",
                             "MemoryStream_fromString"}) {
        EXPECT_TRUE(interp.global_env()->get(name).is_function()) << name;
    }
}

TEST(StdlibNamingTest, EveryPackageHasABarrel) {
    // `import ... from "@std/<package>"` works for every package, resolving to
    // <package>/<package>.dryad (or a top-level <package>.dryad).
    const char* source = R"(
        import { ArgumentNullException, IDisposable } from "@std/core";
        import { Buffer_FromString } from "@std/buffers";
        import { Bytes_Equals } from "@std/bytes";
        import { Text_Length } from "@std/text";
        import { List } from "@std/collections";
        import { Fs_ReadFile, Path, MemoryBackend, FileStream } from "@std/io";
        import { Promise_Resolve, Thread } from "@std/async";
        import { TcpClient, HttpClient } from "@std/net";
    )";
    Lexer lexer(source);
    Parser parser(lexer.tokenize());
    auto program = parser.parse();
    ASSERT_FALSE(parser.has_error()) << parser.error_message();
    Interpreter interp;
    interp.set_stdlib_root(DRYAD_STDLIB_DIR);
    interp.execute(program.get());
    for (const char* name : {"ArgumentNullException", "Buffer_FromString", "Bytes_Equals", "Text_Length", "List",
                             "Fs_ReadFile", "Path", "MemoryBackend", "FileStream", "Promise_Resolve", "Thread",
                             "TcpClient", "HttpClient"}) {
        EXPECT_FALSE(interp.global_env()->get(name).is_null()) << name;
    }
}
