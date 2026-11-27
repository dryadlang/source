// End-to-end tests for the `dryad` command-line driver (src/tools/main.cpp).
// Each test runs the real binary in a scratch directory and checks the exit
// code, stdout and stderr, so argument parsing and every subcommand's error
// paths are exercised the way users hit them.

#include <gtest/gtest.h>
#include <sys/wait.h>
#include <cstdlib>
#include <filesystem>
#include <fstream>
#include <sstream>
#include <string>
#include <unistd.h>

namespace fs = std::filesystem;

namespace {

struct CliResult {
    int exit_code = -1;
    std::string out;
    std::string err;
};

std::string read_text(const fs::path& path) {
    std::ifstream file(path);
    std::ostringstream text;
    text << file.rdbuf();
    return text.str();
}

void write_text(const fs::path& path, const std::string& text) {
    std::ofstream file(path);
    file << text;
}

// Single-quotes `value` for /bin/sh.
std::string shell_quote(const std::string& value) {
    std::string quoted = "'";
    for (char ch : value) {
        if (ch == '\'') {
            quoted += "'\\''";
        } else {
            quoted += ch;
        }
    }
    return quoted + "'";
}

class CliTest : public ::testing::Test {
protected:
    void SetUp() override {
        std::string pattern = (fs::temp_directory_path() / "dryad-cli-XXXXXX").string();
        ASSERT_NE(::mkdtemp(pattern.data()), nullptr);
        dir_ = pattern;
    }

    void TearDown() override {
        std::error_code ignored;
        fs::remove_all(dir_, ignored);
    }

    fs::path write(const std::string& name, const std::string& text) const {
        const fs::path path = dir_ / name;
        write_text(path, text);
        return path;
    }

    // Runs `dryad <args>` inside the scratch directory. `env` is a list of
    // NAME=value assignments placed before the command.
    CliResult run(const std::string& args, const std::string& env = "") const {
        const fs::path out = dir_ / ".stdout";
        const fs::path err = dir_ / ".stderr";
        const std::string command = "cd " + shell_quote(dir_.string()) + " && " + env + " " +
                                    shell_quote(DRYAD_CLI_BINARY) + " " + args + " >" +
                                    shell_quote(out.string()) + " 2>" + shell_quote(err.string()) +
                                    " </dev/null";
        const int status = std::system(command.c_str());
        CliResult result;
        result.exit_code = WIFEXITED(status) ? WEXITSTATUS(status) : -1;
        result.out = read_text(out);
        result.err = read_text(err);
        return result;
    }

    fs::path dir_;
};

const char* kHello = "print(\"hello from dryad\");\n";

const char* kFunctions =
    "function add(a, b) {\n"
    "    return a + b;\n"
    "}\n"
    "print(add(20, 22));\n";

bool contains(const std::string& text, const std::string& needle) {
    return text.find(needle) != std::string::npos;
}

} // namespace

TEST_F(CliTest, NoArgumentsPrintsHelpAndFails) {
    const auto r = run("");
    EXPECT_EQ(r.exit_code, 1);
    EXPECT_TRUE(contains(r.out, "Usage:")) << r.out;
}

TEST_F(CliTest, HelpAndVersion) {
    for (const char* flag : {"--help", "-h"}) {
        const auto r = run(flag);
        EXPECT_EQ(r.exit_code, 0) << flag;
        EXPECT_TRUE(contains(r.out, "Usage:")) << flag;
        EXPECT_TRUE(contains(r.out, "DRYAD_STDLIB_DIR")) << flag;
    }
    for (const char* flag : {"--version", "-v"}) {
        const auto r = run(flag);
        EXPECT_EQ(r.exit_code, 0) << flag;
        EXPECT_EQ(r.out, "dryad 2.0.0\n") << flag;
    }
}

TEST_F(CliTest, RunsAScriptInTheInterpreterAndTheVm) {
    write("hello.dryad", kHello);
    write("fns.dryad", kFunctions);
    for (const char* mode : {"", "--vm "}) {
        auto r = run(std::string(mode) + "hello.dryad");
        EXPECT_EQ(r.exit_code, 0) << mode << r.err;
        EXPECT_EQ(r.out, "hello from dryad\n") << mode;
        r = run(std::string(mode) + "fns.dryad");
        EXPECT_EQ(r.exit_code, 0) << mode << r.err;
        EXPECT_EQ(r.out, "42\n") << mode;
    }
}

TEST_F(CliTest, ScriptErrorsExitNonZeroWithAMessage) {
    write("boom.dryad", "let items = [1];\nprint(items[3]);\n");
    auto r = run("boom.dryad");
    EXPECT_EQ(r.exit_code, 1);
    EXPECT_TRUE(contains(r.err, "Error: ")) << r.err;

    r = run("missing.dryad");
    EXPECT_EQ(r.exit_code, 1);
    EXPECT_TRUE(contains(r.err, "Error: ")) << r.err;

    write("syntax.dryad", "let = ;\n");
    r = run("syntax.dryad");
    EXPECT_EQ(r.exit_code, 1);
    EXPECT_TRUE(contains(r.err, "Error: ")) << r.err;
}

TEST_F(CliTest, RejectsUnknownOptionsAndAMissingScript) {
    auto r = run("--bogus hello.dryad");
    EXPECT_EQ(r.exit_code, 1);
    EXPECT_TRUE(contains(r.err, "unknown option: --bogus")) << r.err;

    r = run("--vm --allow-net");
    EXPECT_EQ(r.exit_code, 1);
    EXPECT_TRUE(contains(r.err, "script path is required")) << r.err;
}

TEST_F(CliTest, NetworkAccessFollowsTheFlagAndEnvironment) {
    // Creating a socket needs network permission (no traffic is sent).
    write("net.dryad",
          "import { Socket, SocketDomain_IPv4, SocketType_Stream } from \"@std/net\";\n"
          "let s = new Socket(SocketDomain_IPv4, SocketType_Stream);\n"
          "s.close();\n"
          "print(\"allowed\");\n");
    const auto denied = run("net.dryad");
    const auto flag = run("--allow-net net.dryad");
    const auto env = run("net.dryad", "DRYAD_ALLOW_NET=1");
    const auto env_true = run("net.dryad", "DRYAD_ALLOW_NET=true");
    const auto env_off = run("net.dryad", "DRYAD_ALLOW_NET=0");
    EXPECT_EQ(flag.out, "allowed\n") << flag.err;
    EXPECT_EQ(env.out, "allowed\n") << env.err;
    EXPECT_EQ(env_true.out, "allowed\n") << env_true.err;
    EXPECT_EQ(denied.exit_code, 1);
    EXPECT_TRUE(contains(denied.err, "Network access denied")) << denied.err;
    EXPECT_EQ(env_off.exit_code, 1);
}

TEST_F(CliTest, CheckValidatesSyntax) {
    write("ok.dryad", kFunctions);
    write("bad.dryad", "function (\n");
    auto r = run("check ok.dryad");
    EXPECT_EQ(r.exit_code, 0) << r.err;
    EXPECT_EQ(r.out, "OK: ok.dryad\n");

    r = run("check bad.dryad");
    EXPECT_EQ(r.exit_code, 1);
    EXPECT_TRUE(contains(r.err, "bad.dryad")) << r.err;

    r = run("check");
    EXPECT_EQ(r.exit_code, 1);
    EXPECT_TRUE(contains(r.err, "check requires a file path")) << r.err;
}

TEST_F(CliTest, AnalyzeReportsAndValidatesOptions) {
    write("fns.dryad", kFunctions);
    for (const char* command : {"analyze fns.dryad", "--analyze fns.dryad --no-profile",
                                "analyze fns.dryad --allow-net"}) {
        const auto r = run(command);
        EXPECT_EQ(r.exit_code, 0) << command << r.err;
        EXPECT_TRUE(contains(r.out, "File: fns.dryad")) << command << r.out;
        EXPECT_TRUE(contains(r.out, "Fast Path ratio:")) << command << r.out;
    }
    auto r = run("analyze fns.dryad --fast");
    EXPECT_EQ(r.exit_code, 1);
    EXPECT_TRUE(contains(r.err, "unknown analyze option: --fast")) << r.err;

    r = run("analyze");
    EXPECT_EQ(r.exit_code, 1);
    EXPECT_TRUE(contains(r.err, "analyze requires a file path")) << r.err;

    r = run("analyze missing.dryad");
    EXPECT_EQ(r.exit_code, 1);
    EXPECT_TRUE(contains(r.err, "Error: ")) << r.err;
}

TEST_F(CliTest, CompileWritesBytecodeIrAndLibraries) {
    write("fns.dryad", kFunctions);
    auto r = run("compile fns.dryad --no-profile");
    EXPECT_EQ(r.exit_code, 0) << r.err;
    EXPECT_TRUE(fs::exists(dir_ / "fns.dryadc"));

    r = run("compile fns.dryad -o custom.dryadc --no-profile");
    EXPECT_EQ(r.exit_code, 0) << r.err;
    EXPECT_TRUE(fs::exists(dir_ / "custom.dryadc"));

    r = run("compile fns.dryad --aot --no-profile --entry add");
    EXPECT_EQ(r.exit_code, 0) << r.err;
    EXPECT_TRUE(fs::exists(dir_ / "fns.ll"));

    write("mathlib.dyd",
          "export function twice(x) {\n"
          "    return x * 2;\n"
          "}\n");
    r = run("compile mathlib.dyd --lib --no-profile");
    EXPECT_EQ(r.exit_code, 0) << r.err;
    EXPECT_TRUE(fs::exists(dir_ / "mathlib.dyl"));

    r = run("link mathlib.dyl");
    EXPECT_EQ(r.exit_code, 0) << r.err;
    EXPECT_TRUE(fs::exists(dir_ / "mathlib.ll"));

    r = run("link mathlib.dyl -o out.ll");
    EXPECT_EQ(r.exit_code, 0) << r.err;
    EXPECT_TRUE(fs::exists(dir_ / "out.ll"));
}

TEST_F(CliTest, CompileRejectsInvalidFlagCombinations) {
    write("fns.dryad", kFunctions);
    const std::pair<const char*, const char*> cases[] = {
        {"compile", "compile requires an input file"},
        {"compile fns.dryad --lib --aot", "--lib cannot be combined"},
        {"compile fns.dryad --native", "--native requires --lib"},
        {"compile fns.dryad --fast", "unknown compile option: --fast"},
        {"compile fns.dryad --link", "--link requires --aot"},
        {"compile missing.dryad --no-profile", "Error: "},
    };
    for (const auto& [args, message] : cases) {
        const auto r = run(args);
        EXPECT_EQ(r.exit_code, 1) << args;
        EXPECT_TRUE(contains(r.err, message)) << args << ": " << r.err;
    }
}

TEST_F(CliTest, LinkAndInstallReportErrors) {
    auto r = run("link");
    EXPECT_EQ(r.exit_code, 1);
    EXPECT_TRUE(contains(r.err, "link requires a .dyl library path")) << r.err;

    r = run("link missing.dyl --bogus");
    EXPECT_EQ(r.exit_code, 1);
    EXPECT_TRUE(contains(r.err, "unknown link option: --bogus")) << r.err;

    r = run("link missing.dyl");
    EXPECT_EQ(r.exit_code, 1);
    EXPECT_TRUE(contains(r.err, "Error: ")) << r.err;

    r = run("install --bogus");
    EXPECT_EQ(r.exit_code, 1);
    EXPECT_TRUE(contains(r.err, "unknown install option: --bogus")) << r.err;

    r = run("install");
    EXPECT_EQ(r.exit_code, 1);
    EXPECT_TRUE(contains(r.err, "Error: ")) << r.err;

    r = run("install missing.toml --modules mods");
    EXPECT_EQ(r.exit_code, 1);
    EXPECT_TRUE(contains(r.err, "Error: ")) << r.err;
}

TEST_F(CliTest, StdlibDirCanBeOverridden) {
    write("uses_std.dryad",
          "import { List } from \"@std/collections\";\n"
          "let l = new List();\n"
          "l.add(1);\n"
          "print(l.count);\n");
    auto r = run("uses_std.dryad");
    EXPECT_EQ(r.exit_code, 0) << r.err;
    EXPECT_EQ(r.out, "1\n");

    fs::create_directories(dir_ / "empty_std");
    r = run("uses_std.dryad", "DRYAD_STDLIB_DIR=" + shell_quote((dir_ / "empty_std").string()));
    EXPECT_EQ(r.exit_code, 1);
    EXPECT_TRUE(contains(r.err, "Error: ")) << r.err;
}
