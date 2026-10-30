#include "dryad/runtime/package_installer.h"
#include "dryad/aot/aot_compiler.h"
#include "dryad/aot/library_linker.h"
#include "dryad/aot/native_linker.h"
#include "dryad/runtime/package_manifest.h"
#include "dryad/common/utils.h"
#include <cstdlib>
#include <filesystem>

namespace dryad {

namespace {

std::filesystem::path resolve_dependency_source(const PackageDependency& dependency) {
    const std::filesystem::path source_root = dependency.path;
    if (std::filesystem::is_regular_file(source_root)) {
        return source_root;
    }

    const std::filesystem::path sibling_dyd =
        std::filesystem::path(source_root.string() + ".dyd");
    const std::filesystem::path sibling_dryad =
        std::filesystem::path(source_root.string() + ".dryad");
    if (std::filesystem::exists(sibling_dyd)) {
        return sibling_dyd;
    }
    if (std::filesystem::exists(sibling_dryad)) {
        return sibling_dryad;
    }

    const std::string base_name =
        dependency.name.substr(dependency.name.find_last_of('/') + 1);
    const std::filesystem::path dyd = source_root / (base_name + ".dyd");
    const std::filesystem::path dryad = source_root / (base_name + ".dryad");
    const std::filesystem::path index_dyd = source_root / "index.dyd";
    const std::filesystem::path index_dryad = source_root / "index.dryad";

    if (std::filesystem::exists(dyd)) {
        return dyd;
    }
    if (std::filesystem::exists(dryad)) {
        return dryad;
    }
    if (std::filesystem::exists(index_dyd)) {
        return index_dyd;
    }
    if (std::filesystem::exists(index_dryad)) {
        return index_dryad;
    }

    throw DryadException("Cannot find source module for dependency: " + dependency.name);
}

void install_dependency(const PackageDependency& dependency,
                        const std::filesystem::path& modules_root,
                        const std::string& stdlib_root) {
    const std::filesystem::path source = resolve_dependency_source(dependency);
    const std::filesystem::path target_source =
        modules_root / (dependency.name + source.extension().string());
    std::filesystem::create_directories(target_source.parent_path());
    std::filesystem::copy_file(source, target_source,
                               std::filesystem::copy_options::overwrite_existing);

    CompileOptions options;
    options.stdlib_root = stdlib_root;
    options.run_profiler = false;
    options.library_paths = {modules_root.string()};
    const std::string dyl_path = (modules_root / (dependency.name + ".dyl")).string();
    auto compile_result = compile_to_dyl(source.string(), dyl_path, options);
    if (!compile_result.success) {
        throw DryadException("Failed to compile dependency '" + dependency.name +
                             "': " + compile_result.message);
    }

    const char* runtime_env = std::getenv("DRYAD_AOT_RUNTIME_SRC");
    const char* include_env = std::getenv("DRYAD_INCLUDE_DIR");
    if (clang_available() && runtime_env && include_env) {
        const std::string native_path = library_native_companion_path(dyl_path);
        compile_library_to_native(dyl_path, native_path, runtime_env, include_env);
    }
}

} // namespace

PackageInstallResult install_package(const std::string& manifest_path,
                                     const std::string& modules_dir) {
    PackageInstallResult result;
    result.modules_dir = modules_dir;

    try {
        const PackageManifest manifest = parse_package_manifest(manifest_path);
        const std::filesystem::path modules_root = modules_dir;
        std::filesystem::create_directories(modules_root);

        std::string stdlib_root = "stdlib";
        if (const char* env = std::getenv("DRYAD_STDLIB_DIR")) {
            stdlib_root = env;
        }

        for (const auto& dependency : manifest.dependencies) {
            install_dependency(dependency, modules_root, stdlib_root);
        }

        if (!manifest.entry.empty()) {
            const std::filesystem::path entry_source = manifest.entry;
            const std::filesystem::path entry_target =
                modules_root / (manifest.name + entry_source.extension().string());
            std::filesystem::copy_file(entry_source, entry_target,
                                     std::filesystem::copy_options::overwrite_existing);

            CompileOptions options;
            options.stdlib_root = stdlib_root;
            options.run_profiler = false;
            options.library_paths = {modules_root.string()};
            const std::string dyl_path = (modules_root / (manifest.name + ".dyl")).string();
            auto compile_result = compile_to_dyl(entry_source.string(), dyl_path, options);
            if (!compile_result.success) {
                throw DryadException("Failed to compile package entry: " + compile_result.message);
            }
        }

        result.success = true;
        result.message = "Installed " + std::to_string(manifest.dependencies.size()) +
                         " dependency(ies) into " + modules_dir;
    } catch (const std::exception& ex) {
        result.message = ex.what();
    }

    return result;
}

} // namespace dryad
