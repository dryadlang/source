#include "dryad/runtime/package_manifest.h"
#include "dryad/common/utils.h"
#include <fstream>
#include <sstream>

namespace dryad {

namespace {

std::string unquote(const std::string& value) {
    if (value.size() >= 2 &&
        ((value.front() == '"' && value.back() == '"') ||
         (value.front() == '\'' && value.back() == '\''))) {
        return value.substr(1, value.size() - 2);
    }
    return value;
}

std::string trim_comment(const std::string& line) {
    const auto hash = line.find('#');
    if (hash == std::string::npos) {
        return line;
    }
    return line.substr(0, hash);
}

bool parse_key_value(const std::string& line, std::string& key, std::string& value) {
    const auto eq = line.find('=');
    if (eq == std::string::npos) {
        return false;
    }
    key = unquote(trim(line.substr(0, eq)));
    value = unquote(trim(line.substr(eq + 1)));
    return !key.empty();
}

std::string resolve_path(const std::string& base_dir, const std::string& path) {
    if (path.empty() || path.rfind("@", 0) == 0) {
        return path;
    }
    if (!base_dir.empty() && path[0] != '/') {
        return base_dir + "/" + path;
    }
    return path;
}

} // namespace

PackageManifest parse_package_manifest_content(const std::string& content,
                                               const std::string& base_dir) {
    PackageManifest manifest;
    std::string section;

    std::istringstream input(content);
    std::string line;
    while (std::getline(input, line)) {
        line = trim(trim_comment(line));
        if (line.empty()) {
            continue;
        }

        if (line.front() == '[' && line.back() == ']') {
            section = line.substr(1, line.size() - 2);
            continue;
        }

        std::string key;
        std::string value;
        if (!parse_key_value(line, key, value)) {
            continue;
        }

        if (section.empty()) {
            if (key == "name") {
                manifest.name = value;
            } else if (key == "version") {
                manifest.version = value;
            } else if (key == "entry") {
                manifest.entry = resolve_path(base_dir, value);
            }
            continue;
        }

        if (section == "package") {
            if (key == "entry") {
                manifest.entry = resolve_path(base_dir, value);
            }
            continue;
        }

        if (section == "dependencies") {
            PackageDependency dependency;
            dependency.name = key;
            dependency.path = resolve_path(base_dir, value);
            manifest.dependencies.push_back(std::move(dependency));
        }
    }

    if (manifest.name.empty()) {
        throw DryadException("dryad.toml: missing required field 'name'");
    }
    if (manifest.version.empty()) {
        manifest.version = "0.0.0";
    }

    return manifest;
}

PackageManifest parse_package_manifest(const std::string& path) {
    std::ifstream file(path);
    if (!file) {
        throw DryadException("Cannot open package manifest: " + path);
    }

    std::ostringstream buffer;
    buffer << file.rdbuf();

    std::string base_dir = path;
    const auto slash = base_dir.find_last_of("/\\");
    if (slash != std::string::npos) {
        base_dir = base_dir.substr(0, slash);
    } else {
        base_dir.clear();
    }

    return parse_package_manifest_content(buffer.str(), base_dir);
}

} // namespace dryad
