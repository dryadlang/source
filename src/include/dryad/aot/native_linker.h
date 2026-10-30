#ifndef DRYAD_AOT_NATIVE_LINKER_H
#define DRYAD_AOT_NATIVE_LINKER_H

#include <cstdint>
#include <string>
#include <vector>

namespace dryad {

enum class NativeLinkKind {
    SharedObject,
    Executable,
};

enum class NativeBinaryFormat {
    Unknown,
    Elf,
    Pe,
};

struct NativeLinkResult {
    bool success = false;
    std::string output_path;
    std::string message;
    std::vector<uint8_t> binary;
};

NativeLinkResult link_native_binary(NativeLinkKind kind,
                                  const std::string& llvm_ir_path,
                                  const std::string& output_path,
                                  const std::string& aot_runtime_source,
                                  const std::string& include_dir);

NativeLinkResult link_native_binary(NativeLinkKind kind,
                                  const std::vector<std::string>& llvm_ir_paths,
                                  const std::string& output_path,
                                  const std::string& aot_runtime_source,
                                  const std::string& include_dir);

NativeLinkResult link_native_shared_object(const std::string& llvm_ir_path,
                                           const std::string& output_path,
                                           const std::string& aot_runtime_source,
                                           const std::string& include_dir);

std::vector<uint8_t> read_binary_file(const std::string& path);

bool clang_available();
bool is_native_binary_image(const std::vector<uint8_t>& binary);
NativeBinaryFormat detect_binary_format(const std::vector<uint8_t>& binary);
const char* default_shared_extension();
const char* default_executable_extension();

} // namespace dryad

#endif // DRYAD_AOT_NATIVE_LINKER_H
