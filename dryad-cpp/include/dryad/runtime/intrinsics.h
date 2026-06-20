#ifndef DRYAD_RUNTIME_INTRINSICS_H
#define DRYAD_RUNTIME_INTRINSICS_H

#include "dryad/runtime/value.h"

namespace dryad {

class Intrinsics {
public:
    static void initialize();
    
    static Value syscall_read(int fd, size_t count);
    static Value syscall_write(int fd, const std::string& data);
    
private:
    Intrinsics() = delete;
};

} // namespace dryad

#endif // DRYAD_RUNTIME_INTRINSICS_H
