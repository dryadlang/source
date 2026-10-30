#ifndef DRYAD_RUNTIME_PACKAGE_INSTALLER_H
#define DRYAD_RUNTIME_PACKAGE_INSTALLER_H

#include <string>

namespace dryad {

struct PackageInstallResult {
    bool success = false;
    std::string message;
    std::string modules_dir;
};

PackageInstallResult install_package(const std::string& manifest_path,
                                     const std::string& modules_dir = "dryad_modules");

} // namespace dryad

#endif // DRYAD_RUNTIME_PACKAGE_INSTALLER_H
