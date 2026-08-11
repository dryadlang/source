#ifndef DRYAD_RUNTIME_INTRINSICS_H
#define DRYAD_RUNTIME_INTRINSICS_H

#include "dryad/runtime/value.h"
#include <cstdint>
#include <mutex>

namespace dryad {

class MemoryManager {
private:
    struct AllocationMetadata {
        size_t size;
        bool is_freed;
        uint8_t* data;
        AllocationMetadata* next;
    };
    
    static AllocationMetadata* allocations;
    static std::mutex allocation_mutex;
    
    static AllocationMetadata* get_metadata(int64_t handle);
    static void validate_bounds(int64_t handle, int64_t index);
    
public:
    static Value alloc_bytes(int64_t size);
    static Value free_bytes(int64_t handle);
    static Value realloc(int64_t handle, int64_t new_size);
    
    static Value memcpy(int64_t dest_handle, int64_t src_handle, int64_t count);
    static Value memset(int64_t handle, int64_t value, int64_t count);
    static Value buffer_get(int64_t handle, int64_t index);
    static Value buffer_set(int64_t handle, int64_t index, int64_t value);
};

class Intrinsics {
public:
    static void initialize();
    
    static Value syscall_read(int fd, size_t count);
    static Value syscall_write(int fd, const std::string& data);
    
    // Memory management intrinsics
    Value alloc_bytes(int64_t size);
    Value free_bytes(int64_t handle);
    Value realloc(int64_t handle, int64_t new_size);
    Value memcpy(int64_t dest_handle, int64_t src_handle, int64_t count);
    Value memset(int64_t handle, int64_t value, int64_t count);
    Value buffer_get(int64_t handle, int64_t index);
    Value buffer_set(int64_t handle, int64_t index, int64_t value);
    
private:
    Intrinsics() = delete;
};

} // namespace dryad

#endif // DRYAD_RUNTIME_INTRINSICS_H
