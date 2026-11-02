#include "dryad/runtime/intrinsics.h"
#include <unistd.h>
#include <algorithm>
#include <vector>
#include <cstring>
#include <new>
#include <stdexcept>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>

namespace dryad {

MemoryManager::AllocationMetadata* MemoryManager::allocations = nullptr;
std::mutex MemoryManager::allocation_mutex;

MemoryManager::AllocationMetadata* MemoryManager::get_metadata(int64_t handle) {
    AllocationMetadata* current = allocations;
    while (current) {
        if (reinterpret_cast<int64_t>(current) == handle) {
            return current;
        }
        current = current->next;
    }
    return nullptr;
}

void MemoryManager::validate_bounds(int64_t handle, int64_t index) {
    AllocationMetadata* meta = get_metadata(handle);
    if (!meta || meta->is_freed || index < 0 || index >= static_cast<int64_t>(meta->size)) {
        throw std::runtime_error("Buffer bounds check failed");
    }
}

Value MemoryManager::alloc_bytes(int64_t size) {
    if (size <= 0) {
        return Value();
    }
    
    std::lock_guard<std::mutex> lock(allocation_mutex);
    
    AllocationMetadata* meta = new AllocationMetadata();
    try {
        meta->data = new uint8_t[static_cast<size_t>(size)]{};
    } catch (const std::bad_alloc&) {
        delete meta;
        return Value();
    }
    meta->size = static_cast<size_t>(size);
    meta->is_freed = false;
    meta->next = allocations;
    allocations = meta;
    
    return Value(reinterpret_cast<int64_t>(meta));
}

Value MemoryManager::free_bytes(int64_t handle) {
    std::lock_guard<std::mutex> lock(allocation_mutex);
    
    AllocationMetadata* meta = get_metadata(handle);
    if (!meta || meta->is_freed) {
        return Value();
    }
    
    meta->is_freed = true;
    delete[] meta->data;
    meta->data = nullptr;
    
    return Value();
}

Value MemoryManager::realloc(int64_t handle, int64_t new_size) {
    if (new_size <= 0) {
        return Value();
    }
    
    std::lock_guard<std::mutex> lock(allocation_mutex);
    
    AllocationMetadata* meta = get_metadata(handle);
    if (!meta || meta->is_freed) {
        return Value();
    }
    
    uint8_t* new_data = nullptr;
    try {
        new_data = new uint8_t[static_cast<size_t>(new_size)]{};
    } catch (const std::bad_alloc&) {
        return Value();
    }
    size_t copy_size = std::min(meta->size, static_cast<size_t>(new_size));
    std::memcpy(new_data, meta->data, copy_size);
    
    delete[] meta->data;
    meta->data = new_data;
    meta->size = static_cast<size_t>(new_size);
    
    return Value(handle);
}

Value MemoryManager::memmove(int64_t dest_handle, int64_t dest_offset,
                              int64_t src_handle, int64_t src_offset, int64_t count) {
    if (count <= 0) {
        return Value();
    }

    std::lock_guard<std::mutex> lock(allocation_mutex);

    AllocationMetadata* dest_meta = get_metadata(dest_handle);
    AllocationMetadata* src_meta = get_metadata(src_handle);

    if (!dest_meta || dest_meta->is_freed || !src_meta || src_meta->is_freed) {
        return Value();
    }

    if (dest_offset < 0 || src_offset < 0) {
        return Value();
    }

    const int64_t dest_size = static_cast<int64_t>(dest_meta->size);
    const int64_t src_size = static_cast<int64_t>(src_meta->size);
    if (dest_offset > dest_size || src_offset > src_size ||
        count > dest_size - dest_offset || count > src_size - src_offset) {
        return Value();
    }

    std::memmove(dest_meta->data + dest_offset, src_meta->data + src_offset,
                 static_cast<size_t>(count));

    return Value();
}

Value MemoryManager::memcpy(int64_t dest_handle, int64_t src_handle, int64_t count) {
    if (count <= 0) {
        return Value();
    }
    
    std::lock_guard<std::mutex> lock(allocation_mutex);
    
    AllocationMetadata* dest_meta = get_metadata(dest_handle);
    AllocationMetadata* src_meta = get_metadata(src_handle);
    
    if (!dest_meta || dest_meta->is_freed || !src_meta || src_meta->is_freed) {
        return Value();
    }
    
    if (count > static_cast<int64_t>(dest_meta->size) || 
        count > static_cast<int64_t>(src_meta->size)) {
        return Value();
    }
    
    std::memcpy(dest_meta->data, src_meta->data, static_cast<size_t>(count));
    
    return Value();
}

Value MemoryManager::memset(int64_t handle, int64_t value, int64_t count) {
    if (count <= 0) {
        return Value();
    }
    
    std::lock_guard<std::mutex> lock(allocation_mutex);
    
    AllocationMetadata* meta = get_metadata(handle);
    if (!meta || meta->is_freed) {
        return Value();
    }
    
    if (count > static_cast<int64_t>(meta->size)) {
        return Value();
    }
    
    std::memset(meta->data, static_cast<int>(value & 0xFF), static_cast<size_t>(count));
    
    return Value();
}

Value MemoryManager::buffer_get(int64_t handle, int64_t index) {
    try {
        std::lock_guard<std::mutex> lock(allocation_mutex);
        validate_bounds(handle, index);
        AllocationMetadata* meta = get_metadata(handle);
        return Value(static_cast<int64_t>(meta->data[index]));
    } catch (...) {
        return Value();
    }
}

Value MemoryManager::buffer_set(int64_t handle, int64_t index, int64_t value) {
    try {
        std::lock_guard<std::mutex> lock(allocation_mutex);
        validate_bounds(handle, index);
        AllocationMetadata* meta = get_metadata(handle);
        meta->data[index] = static_cast<uint8_t>(value & 0xFF);
        return Value();
    } catch (...) {
        return Value();
    }
}

Value MemoryManager::string_to_bytes(int64_t handle, int64_t offset, const std::string& data) {
    if (data.empty()) {
        return Value(static_cast<int64_t>(0));
    }
    
    std::lock_guard<std::mutex> lock(allocation_mutex);
    
    AllocationMetadata* meta = get_metadata(handle);
    if (!meta || meta->is_freed) {
        return Value();
    }
    
    if (offset < 0 || offset + static_cast<int64_t>(data.size()) > static_cast<int64_t>(meta->size)) {
        return Value();
    }
    
    std::memcpy(meta->data + offset, data.data(), data.size());
    
    return Value(static_cast<int64_t>(data.size()));
}

Value MemoryManager::read_fd_to_buffer(int64_t fd, int64_t handle, int64_t offset, int64_t count) {
    if (count < 0 || offset < 0) {
        return Value(static_cast<int64_t>(-1));
    }

    AllocationMetadata* meta = nullptr;
    {
        std::lock_guard<std::mutex> lock(allocation_mutex);
        meta = get_metadata(handle);
        if (!meta || meta->is_freed || !meta->data) {
            return Value(static_cast<int64_t>(-1));
        }
        if (offset + count > static_cast<int64_t>(meta->size)) {
            return Value(static_cast<int64_t>(-1));
        }
    }

    std::vector<uint8_t> staging(static_cast<size_t>(count), 0);
    ssize_t nbytes = ::read(static_cast<int>(fd), staging.data(), staging.size());
    if (nbytes > 0) {
        std::lock_guard<std::mutex> lock(allocation_mutex);
        // The buffer may have been freed or resized while the call was
        // unlocked. The bytes are already consumed from the descriptor, so
        // report the call as failed rather than writing into a stale region.
        // realloc keeps this node but may have shrunk the allocation.
        if (meta->is_freed || !meta->data ||
            offset + static_cast<int64_t>(nbytes) > static_cast<int64_t>(meta->size)) {
            return Value(static_cast<int64_t>(-1));
        }
        std::memcpy(meta->data + offset, staging.data(),
                    std::min(static_cast<size_t>(nbytes), staging.size()));
    }
    return Value(static_cast<int64_t>(nbytes));
}

Value MemoryManager::write_fd_from_buffer(int64_t fd, int64_t handle, int64_t offset, int64_t count) {
    if (count < 0 || offset < 0) {
        return Value(static_cast<int64_t>(-1));
    }

    std::vector<uint8_t> staging;
    {
        std::lock_guard<std::mutex> lock(allocation_mutex);
        AllocationMetadata* meta = get_metadata(handle);
        if (!meta || meta->is_freed || !meta->data) {
            return Value(static_cast<int64_t>(-1));
        }
        if (offset + count > static_cast<int64_t>(meta->size)) {
            return Value(static_cast<int64_t>(-1));
        }
        staging.assign(meta->data + offset, meta->data + offset + count);
    }

    ssize_t nbytes = ::write(static_cast<int>(fd), staging.data(), staging.size());
    return Value(static_cast<int64_t>(nbytes));
}

Value MemoryManager::io_buffer(int64_t handle, int64_t offset, int64_t count,
                             const std::function<ssize_t(uint8_t*, size_t)>& io) {
    if (count < 0 || offset < 0) {
        return Value(static_cast<int64_t>(-1));
    }

    AllocationMetadata* meta = nullptr;
    std::vector<uint8_t> staging;
    {
        std::lock_guard<std::mutex> lock(allocation_mutex);
        meta = get_metadata(handle);
        if (!meta || meta->is_freed || !meta->data) {
            return Value(static_cast<int64_t>(-1));
        }
        if (offset + count > static_cast<int64_t>(meta->size)) {
            return Value(static_cast<int64_t>(-1));
        }
        staging.assign(meta->data + offset, meta->data + offset + count);
    }

    ssize_t nbytes = io(staging.data(), staging.size());
    if (nbytes > 0) {
        std::lock_guard<std::mutex> lock(allocation_mutex);
        // The buffer may have been freed or resized while the call was
        // unlocked. The bytes are already consumed from the descriptor, so
        // report the call as failed rather than writing into a stale region.
        // realloc keeps this node but may have shrunk the allocation.
        if (meta->is_freed || !meta->data ||
            offset + static_cast<int64_t>(nbytes) > static_cast<int64_t>(meta->size)) {
            return Value(static_cast<int64_t>(-1));
        }
        std::memcpy(meta->data + offset, staging.data(),
                    std::min(static_cast<size_t>(nbytes), staging.size()));
    }
    return Value(static_cast<int64_t>(nbytes));
}

Value MemoryManager::sendto_buffer(int64_t fd, int64_t handle, int64_t offset, int64_t count,
                                   const std::string& host, int64_t port) {
    if (count < 0 || offset < 0) {
        return Value(static_cast<int64_t>(-1));
    }

    struct sockaddr_in addr;
    std::memset(&addr, 0, sizeof(addr));
    addr.sin_family = AF_INET;
    addr.sin_port = htons(static_cast<uint16_t>(port));
    if (::inet_pton(AF_INET, host.c_str(), &addr.sin_addr) <= 0) {
        return Value(static_cast<int64_t>(-1));
    }

    std::vector<uint8_t> staging;
    {
        std::lock_guard<std::mutex> lock(allocation_mutex);
        AllocationMetadata* meta = get_metadata(handle);
        if (!meta || meta->is_freed || !meta->data) {
            return Value(static_cast<int64_t>(-1));
        }
        if (offset + count > static_cast<int64_t>(meta->size)) {
            return Value(static_cast<int64_t>(-1));
        }
        staging.assign(meta->data + offset, meta->data + offset + count);
    }

    ssize_t nbytes = ::sendto(static_cast<int>(fd), staging.data(),
                              staging.size(), 0,
                              reinterpret_cast<struct sockaddr*>(&addr), sizeof(addr));
    return Value(static_cast<int64_t>(nbytes));
}

Value MemoryManager::recvfrom_buffer(int64_t fd, int64_t handle, int64_t offset, int64_t count) {
    auto make_result = [](int64_t nbytes, const std::string& host, int64_t port) {
        Value result = Value::create_object();
        result.object_set("nbytes", Value(nbytes));
        result.object_set("host", Value(host));
        result.object_set("port", Value(port));
        return result;
    };

    if (count < 0 || offset < 0) {
        return make_result(-1, "", 0);
    }

    struct sockaddr_in addr;
    socklen_t addrlen = sizeof(addr);
    std::memset(&addr, 0, sizeof(addr));

    AllocationMetadata* meta = nullptr;
    {
        std::lock_guard<std::mutex> lock(allocation_mutex);
        meta = get_metadata(handle);
        if (!meta || meta->is_freed || !meta->data) {
            return make_result(-1, "", 0);
        }
        if (offset + count > static_cast<int64_t>(meta->size)) {
            return make_result(-1, "", 0);
        }
    }

    std::vector<uint8_t> staging(static_cast<size_t>(count), 0);
    ssize_t nbytes = ::recvfrom(static_cast<int>(fd), staging.data(),
                               staging.size(), 0,
                               reinterpret_cast<struct sockaddr*>(&addr), &addrlen);
    if (nbytes < 0) {
        return make_result(-1, "", 0);
    }
    if (nbytes > 0) {
        std::lock_guard<std::mutex> lock(allocation_mutex);
        // The buffer may have been freed or resized while the call was
        // unlocked. The bytes are already consumed from the descriptor, so
        // report the call as failed rather than writing into a stale region.
        // realloc keeps this node but may have shrunk the allocation.
        if (meta->is_freed || !meta->data ||
            offset + static_cast<int64_t>(nbytes) > static_cast<int64_t>(meta->size)) {
            return make_result(-1, "", 0);
        }
        std::memcpy(meta->data + offset, staging.data(),
                    std::min(static_cast<size_t>(nbytes), staging.size()));
    }

    char host_buf[INET_ADDRSTRLEN] = {};
    ::inet_ntop(AF_INET, &addr.sin_addr, host_buf, sizeof(host_buf));
    return make_result(static_cast<int64_t>(nbytes), std::string(host_buf),
                       static_cast<int64_t>(ntohs(addr.sin_port)));
}

Value MemoryManager::bytes_to_string(int64_t handle, int64_t offset, int64_t count) {
    if (count == 0) {
        return Value(std::string());
    }
    
    std::lock_guard<std::mutex> lock(allocation_mutex);
    
    AllocationMetadata* meta = get_metadata(handle);
    if (!meta || meta->is_freed) {
        return Value();
    }
    
    if (offset < 0 || count < 0 || offset + count > static_cast<int64_t>(meta->size)) {
        return Value();
    }
    
    return Value(std::string(reinterpret_cast<const char*>(meta->data + offset), static_cast<size_t>(count)));
}

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

Value Intrinsics::alloc_bytes(int64_t size) {
    return MemoryManager::alloc_bytes(size);
}

Value Intrinsics::free_bytes(int64_t handle) {
    return MemoryManager::free_bytes(handle);
}

Value Intrinsics::realloc(int64_t handle, int64_t new_size) {
    return MemoryManager::realloc(handle, new_size);
}

Value Intrinsics::memcpy(int64_t dest_handle, int64_t src_handle, int64_t count) {
    return MemoryManager::memcpy(dest_handle, src_handle, count);
}

Value Intrinsics::memmove(int64_t dest_handle, int64_t dest_offset,
                          int64_t src_handle, int64_t src_offset, int64_t count) {
    return MemoryManager::memmove(dest_handle, dest_offset, src_handle, src_offset, count);
}

Value Intrinsics::memset(int64_t handle, int64_t value, int64_t count) {
    return MemoryManager::memset(handle, value, count);
}

Value Intrinsics::buffer_get(int64_t handle, int64_t index) {
    return MemoryManager::buffer_get(handle, index);
}

Value Intrinsics::buffer_set(int64_t handle, int64_t index, int64_t value) {
    return MemoryManager::buffer_set(handle, index, value);
}

} // namespace dryad
