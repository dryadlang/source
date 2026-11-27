#include "dryad/aot/native_linker.h"
#include <cstdlib>
#include <fstream>
#include <iterator>
#include <vector>

#if defined(_WIN32)
#include <process.h>
#else
#include <fcntl.h>
#include <spawn.h>
#include <sys/wait.h>
#include <unistd.h>
extern char** environ;
#endif

namespace dryad {

namespace {

// Runs `argv[0]` with the given arguments directly via exec (posix_spawn on
// POSIX, _spawnvp on Windows) -- NOT through a shell. link_native_binary()
// below builds command lines out of caller-controlled file paths (source
// file names, -o output paths, library paths), which on a shell-string path
// (the previous implementation used std::system()) would be an arbitrary
// command injection vector: a path like `out.ll"; rm -rf ~; echo "` or one
// containing `$(...)`/backticks would be interpreted by /bin/sh even though
// each argument was individually double-quoted, since double quotes do not
// suppress command substitution. Passing an argv vector straight to exec
// avoids any shell parsing, so such paths are inert data again.
int run_command(const std::vector<std::string>& args) {
    if (args.empty()) {
        return -1;
    }

#if defined(_WIN32)
    std::vector<const char*> argv;
    argv.reserve(args.size() + 1);
    for (const auto& arg : args) {
        argv.push_back(arg.c_str());
    }
    argv.push_back(nullptr);
    return _spawnvp(_P_WAIT, args[0].c_str(), argv.data());
#else
    std::vector<char*> argv;
    argv.reserve(args.size() + 1);
    for (const auto& arg : args) {
        argv.push_back(const_cast<char*>(arg.c_str()));
    }
    argv.push_back(nullptr);

    pid_t pid = 0;
    const int spawn_rc =
        posix_spawnp(&pid, args[0].c_str(), nullptr, nullptr, argv.data(), environ);
    if (spawn_rc != 0) {
        return -1;
    }

    int status = 0;
    if (waitpid(pid, &status, 0) != pid) {
        return -1;
    }
    if (WIFEXITED(status)) {
        return WEXITSTATUS(status);
    }
    return -1;
#endif
}

// Runs a command silently (stdout/stderr discarded), used only for the
// `clang --version` availability probe. Still goes through exec, not a
// shell, and this particular call has no caller-controlled arguments.
int run_command_quiet(const std::vector<std::string>& args) {
    if (args.empty()) {
        return -1;
    }

#if defined(_WIN32)
    // No portable redirection via _spawnvp; the probe's stdout/stderr are
    // inherited, which is harmless noise for `clang --version`.
    return run_command(args);
#else
    std::vector<char*> argv;
    argv.reserve(args.size() + 1);
    for (const auto& arg : args) {
        argv.push_back(const_cast<char*>(arg.c_str()));
    }
    argv.push_back(nullptr);

    posix_spawn_file_actions_t actions;
    posix_spawn_file_actions_init(&actions);
    posix_spawn_file_actions_addopen(&actions, STDOUT_FILENO, "/dev/null", O_WRONLY, 0);
    posix_spawn_file_actions_addopen(&actions, STDERR_FILENO, "/dev/null", O_WRONLY, 0);

    pid_t pid = 0;
    const int spawn_rc =
        posix_spawnp(&pid, args[0].c_str(), &actions, nullptr, argv.data(), environ);
    posix_spawn_file_actions_destroy(&actions);
    if (spawn_rc != 0) {
        return -1;
    }

    int status = 0;
    if (waitpid(pid, &status, 0) != pid) {
        return -1;
    }
    if (WIFEXITED(status)) {
        return WEXITSTATUS(status);
    }
    return -1;
#endif
}

std::vector<std::string> platform_link_flags(NativeLinkKind kind) {
#if defined(_WIN32)
    if (kind == NativeLinkKind::SharedObject) {
        return {"-shared"};
    }
    return {};
#else
    if (kind == NativeLinkKind::SharedObject) {
        return {"-fPIC", "-shared"};
    }
    return {};
#endif
}

std::vector<std::string> platform_link_libraries() {
#if defined(_WIN32)
    return {};
#else
    return {"-ldl", "-pthread"};
#endif
}

std::vector<std::string> target_cpu_flags() {
#if defined(__x86_64__) || defined(_M_X64)
    const char* march = std::getenv("DRYAD_AOT_MARCH");
    if (march != nullptr && march[0] != '\0') {
        return {std::string("-march=") + march};
    }
    // Portable default: avoid x86-64-v3/v4 binaries that fail on common CPUs.
    return {"-march=x86-64-v2"};
#elif defined(__aarch64__)
    return {};
#else
    return {};
#endif
}

} // namespace

bool clang_available() {
    return run_command_quiet({"clang", "--version"}) == 0;
}

const char* default_shared_extension() {
#if defined(_WIN32)
    return ".dll";
#else
    return ".so";
#endif
}

const char* default_executable_extension() {
#if defined(_WIN32)
    return ".exe";
#else
    return "";
#endif
}

NativeBinaryFormat detect_binary_format(const std::vector<uint8_t>& binary) {
    if (binary.size() >= 4 && binary[0] == 0x7F && binary[1] == 'E' && binary[2] == 'L' &&
        binary[3] == 'F') {
        return NativeBinaryFormat::Elf;
    }
    if (binary.size() >= 2 && binary[0] == 'M' && binary[1] == 'Z') {
        return NativeBinaryFormat::Pe;
    }
    return NativeBinaryFormat::Unknown;
}

bool is_native_binary_image(const std::vector<uint8_t>& binary) {
    return detect_binary_format(binary) != NativeBinaryFormat::Unknown;
}

std::vector<uint8_t> read_binary_file(const std::string& path) {
    std::ifstream file(path, std::ios::binary);
    if (!file) {
        return {};
    }
    return std::vector<uint8_t>(std::istreambuf_iterator<char>(file), std::istreambuf_iterator<char>());
}

NativeLinkResult link_native_binary(NativeLinkKind kind,
                                    const std::string& llvm_ir_path,
                                    const std::string& output_path,
                                    const std::string& aot_runtime_source,
                                    const std::string& include_dir) {
    return link_native_binary(
        kind, std::vector<std::string>{llvm_ir_path}, output_path, aot_runtime_source, include_dir);
}

NativeLinkResult link_native_binary(NativeLinkKind kind,
                                    const std::vector<std::string>& llvm_ir_paths,
                                    const std::string& output_path,
                                    const std::string& aot_runtime_source,
                                    const std::string& include_dir) {
    NativeLinkResult result;
    result.output_path = output_path;

    if (!clang_available()) {
        result.message = "clang not found in PATH";
        return result;
    }
    if (llvm_ir_paths.empty()) {
        result.message = "No LLVM IR inputs provided for linking";
        return result;
    }

    // Build the argv vector directly instead of a shell command string:
    // output_path, llvm_ir_paths, aot_runtime_source and include_dir are all
    // ultimately derived from CLI-supplied file paths (see tools/main.cpp),
    // so they must never be interpolated into text a shell will re-parse.
    std::vector<std::string> args = {"clang", "-O2"};
    for (auto& flag : target_cpu_flags()) {
        args.push_back(std::move(flag));
    }
    for (auto& flag : platform_link_flags(kind)) {
        args.push_back(std::move(flag));
    }
    args.push_back("-I" + include_dir);
    args.push_back("-o");
    args.push_back(output_path);
    for (const auto& llvm_ir_path : llvm_ir_paths) {
        args.push_back(llvm_ir_path);
    }
    args.push_back(aot_runtime_source);
    for (auto& flag : platform_link_libraries()) {
        args.push_back(std::move(flag));
    }

    const int status = run_command(args);
    if (status != 0) {
        result.message = "clang linking failed with exit code " + std::to_string(status);
        return result;
    }

    result.binary = read_binary_file(output_path);
    if (result.binary.empty()) {
        result.message = "Linked binary is empty or unreadable: " + output_path;
        return result;
    }

    if (!is_native_binary_image(result.binary)) {
        result.message = "Linked output is not a recognized native binary image: " + output_path;
        return result;
    }

    result.success = true;
    if (kind == NativeLinkKind::Executable) {
        result.message = "Linked native executable to " + output_path + " (" +
                         std::to_string(result.binary.size()) + " bytes)";
    } else {
        result.message = "Linked native module to " + output_path + " (" +
                         std::to_string(result.binary.size()) + " bytes)";
    }
    return result;
}

NativeLinkResult link_native_shared_object(const std::string& llvm_ir_path,
                                           const std::string& output_path,
                                           const std::string& aot_runtime_source,
                                           const std::string& include_dir) {
    return link_native_binary(NativeLinkKind::SharedObject,
                              llvm_ir_path,
                              output_path,
                              aot_runtime_source,
                              include_dir);
}

} // namespace dryad
