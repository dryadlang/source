#ifndef DRYAD_COMMON_UTILS_H
#define DRYAD_COMMON_UTILS_H

#include <string>
#include <vector>
#include <memory>

namespace dryad {

// String utilities
std::string trim(const std::string& str);
std::vector<std::string> split(const std::string& str, char delimiter);
bool starts_with(const std::string& str, const std::string& prefix);
bool ends_with(const std::string& str, const std::string& suffix);

// Error handling
class DryadException : public std::exception {
public:
    explicit DryadException(std::string message) : message_(std::move(message)) {}
    const char* what() const noexcept override { return message_.c_str(); }
private:
    std::string message_;
};

} // namespace dryad

#endif // DRYAD_COMMON_UTILS_H
