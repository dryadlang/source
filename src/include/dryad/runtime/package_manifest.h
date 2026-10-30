#ifndef DRYAD_RUNTIME_PACKAGE_MANIFEST_H
#define DRYAD_RUNTIME_PACKAGE_MANIFEST_H

#include <string>
#include <vector>

namespace dryad {

struct PackageDependency {
    std::string name;
    std::string version;
    std::string path;
};

struct PackageManifest {
    std::string name;
    std::string version;
    std::string entry;
    std::vector<PackageDependency> dependencies;
};

PackageManifest parse_package_manifest(const std::string& path);
PackageManifest parse_package_manifest_content(const std::string& content, const std::string& base_dir);

} // namespace dryad

#endif // DRYAD_RUNTIME_PACKAGE_MANIFEST_H
