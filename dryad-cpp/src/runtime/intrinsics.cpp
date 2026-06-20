#include "dryad/runtime/intrinsics.h"
#include <unistd.h>
#include <vector>

namespace dryad {

void Intrinsics::initialize() {
}

Value Intrinsics::syscall_read(int fd, size_t count) {
    std::vector<char> buffer(count);
    ssize_t bytes_read = ::read(fd, buffer.data(), count);
    
    if (bytes_read < 0) {
        return Value();
    }
    
    return Value(std::string(buffer.data(), bytes_read));
}

Value Intrinsics::syscall_write(int fd, const std::string& data) {
    ssize_t bytes_written = ::write(fd, data.c_str(), data.size());
    
    if (bytes_written < 0) {
        return Value(static_cast<int64_t>(-1));
    }
    
    return Value(static_cast<int64_t>(bytes_written));
}

} // namespace dryad
