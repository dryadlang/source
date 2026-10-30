#include "dryad/runtime/bytecode_cache.h"
#include "dryad/common/utils.h"

namespace dryad {

namespace {

std::string replace_extension(const std::string& path, const std::string& ext) {
    const auto slash = path.find_last_of("/\\");
    const auto dot = path.find_last_of('.');
    if (dot == std::string::npos || (slash != std::string::npos && dot < slash)) {
        return path + ext;
    }
    return path.substr(0, dot) + ext;
}

} // namespace

std::string companion_dryadc_path(const std::string& source_path) {
    return replace_extension(source_path, ".dryadc");
}

std::string companion_native_path(const std::string& source_path) {
    return replace_extension(source_path, ".so");
}

void BytecodeModuleCache::set_source_path(const std::string& source_path) {
    if (source_path_ != source_path) {
        clear();
        source_path_ = source_path;
    }
}

bool BytecodeModuleCache::try_load_companion() {
    if (source_path_.empty()) {
        return false;
    }
    return try_load_file(companion_dryadc_path(source_path_));
}

bool BytecodeModuleCache::try_load_file(const std::string& path) {
    CompiledProgram loaded;
    if (!load_compiled_program(path, loaded)) {
        return false;
    }
    program_ = std::move(loaded);
    loaded_ = true;
    return true;
}

const CompiledProgram& BytecodeModuleCache::program() const {
    if (!loaded_) {
        throw DryadException("BytecodeModuleCache: no program loaded");
    }
    return program_;
}

void BytecodeModuleCache::clear() {
    program_ = CompiledProgram{};
    loaded_ = false;
}

} // namespace dryad
