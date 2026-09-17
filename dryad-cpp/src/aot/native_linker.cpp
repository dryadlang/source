#include "dryad/aot/native_linker.h"
#include <fstream>
#include <iterator>

namespace dryad {

namespace {

int run_command(const std::string& command) {
    return std::system(command.c_str());
}

std::string platform_link_flags(NativeLinkKind kind) {
#if defined(_WIN32)
    if (kind == NativeLinkKind::SharedObject) {
        return "-shared";
    }
    return "";
#else
    if (kind == NativeLinkKind::SharedObject) {
        return "-fPIC -shared";
    }
    return "";
#endif
}

std::string platform_link_libraries() {
#if defined(_WIN32)
    return "";
#else
    return "-ldl -pthread";
#endif
}

} // namespace

bool clang_available() {
    return run_command("clang --version >/dev/null 2>&1") == 0;
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

    const std::string kind_flags = platform_link_flags(kind);
    std::string command = "clang -O2";
    if (!kind_flags.empty()) {
        command += " " + kind_flags;
    }
    command += " -I\"" + include_dir + "\" -o \"" + output_path + "\"";
    for (const auto& llvm_ir_path : llvm_ir_paths) {
        command += " \"" + llvm_ir_path + "\"";
    }
    command += " \"" + aot_runtime_source + "\" " + platform_link_libraries() + " 2>&1";

    const int status = run_command(command);
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
