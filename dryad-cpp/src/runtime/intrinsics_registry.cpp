#include "dryad/runtime/intrinsics_registry.h"
#include "dryad/runtime/intrinsics.h"
#include "dryad/common/utils.h"
#include <fcntl.h>
#include <unistd.h>
#include <sys/stat.h>
#include <cerrno>
#include <ctime>
#include <string>

namespace dryad {

IntrinsicsRegistry& IntrinsicsRegistry::instance() {
    static IntrinsicsRegistry registry;
    return registry;
}

void IntrinsicsRegistry::register_intrinsic(const std::string& name, IntrinsicFunction func) {
    intrinsics_[name] = std::move(func);
}

Value IntrinsicsRegistry::call(const std::string& name, const std::vector<Value>& args) {
    auto it = intrinsics_.find(name);
    if (it == intrinsics_.end()) {
        throw DryadException("Unknown intrinsic: " + name);
    }
    return it->second(args);
}

bool IntrinsicsRegistry::has(const std::string& name) const {
    return intrinsics_.find(name) != intrinsics_.end();
}

void IntrinsicsRegistry::require_network_access() const {
    if (!network_allowed_) {
        throw DryadException(
            "Network access denied. Pass --allow-net to dryad or set DRYAD_ALLOW_NET=1");
    }
}

void IntrinsicsRegistry::register_file_io_intrinsics() {
    register_intrinsic("syscall.open", [](const std::vector<Value>& args) -> Value {
        if (args.size() < 2) {
            throw DryadException("syscall.open requires 2 arguments");
        }
        
        const std::string& path = args[0].as_string();
        int64_t flags = args[1].as_integer();
        
        int fd = ::open(path.c_str(), static_cast<int>(flags), 0644);
        return Value(static_cast<int64_t>(fd));
    });
    
    register_intrinsic("syscall.read", [](const std::vector<Value>& args) -> Value {
        if (args.size() < 2) {
            throw DryadException("syscall.read requires 2 arguments");
        }
        
        int64_t fd = args[0].as_integer();
        int64_t size = args[1].as_integer();
        
        std::string buffer(size, '\0');
        ssize_t n = ::read(static_cast<int>(fd), &buffer[0], size);
        
        if (n < 0) {
            return Value(static_cast<int64_t>(n));
        }
        
        buffer.resize(n);
        return Value(buffer);
    });
    
    register_intrinsic("syscall.write", [](const std::vector<Value>& args) -> Value {
        if (args.size() < 2) {
            throw DryadException("syscall.write requires 2 arguments");
        }
        
        int64_t fd = args[0].as_integer();
        const std::string& data = args[1].as_string();
        
        ssize_t n = ::write(static_cast<int>(fd), data.c_str(), data.size());
        return Value(static_cast<int64_t>(n));
    });
    
    register_intrinsic("syscall.close", [](const std::vector<Value>& args) -> Value {
        if (args.empty()) {
            throw DryadException("syscall.close requires 1 argument");
        }
        
        int64_t fd = args[0].as_integer();
        int result = ::close(static_cast<int>(fd));
        return Value(static_cast<int64_t>(result));
    });

    register_intrinsic("syscall.read_buffer", [](const std::vector<Value>& args) -> Value {
        if (args.size() < 4 || !args[0].is_integer() || !args[1].is_integer() ||
            !args[2].is_integer() || !args[3].is_integer()) {
            throw DryadException("syscall.read_buffer requires 4 integer arguments");
        }
        return MemoryManager::read_fd_to_buffer(
            args[0].as_integer(), args[1].as_integer(), args[2].as_integer(), args[3].as_integer());
    });

    register_intrinsic("syscall.write_buffer", [](const std::vector<Value>& args) -> Value {
        if (args.size() < 4 || !args[0].is_integer() || !args[1].is_integer() ||
            !args[2].is_integer() || !args[3].is_integer()) {
            throw DryadException("syscall.write_buffer requires 4 integer arguments");
        }
        return MemoryManager::write_fd_from_buffer(
            args[0].as_integer(), args[1].as_integer(), args[2].as_integer(), args[3].as_integer());
    });

    register_intrinsic("syscall.fstat", [](const std::vector<Value>& args) -> Value {
        if (args.empty() || !args[0].is_integer()) {
            throw DryadException("syscall.fstat requires 1 integer argument");
        }
        struct stat st;
        if (::fstat(static_cast<int>(args[0].as_integer()), &st) < 0) {
            return Value(static_cast<int64_t>(-1));
        }
        return Value(static_cast<int64_t>(st.st_size));
    });

    register_intrinsic("syscall.fsync", [](const std::vector<Value>& args) -> Value {
        if (args.empty() || !args[0].is_integer()) {
            throw DryadException("syscall.fsync requires 1 integer argument");
        }
#ifdef _WIN32
        int result = ::_commit(static_cast<int>(args[0].as_integer()));
#else
        int result = ::fsync(static_cast<int>(args[0].as_integer()));
#endif
        return Value(static_cast<int64_t>(result));
    });
    
    register_intrinsic("syscall.unlink", [](const std::vector<Value>& args) -> Value {
        if (args.empty()) {
            throw DryadException("syscall.unlink requires 1 argument");
        }
        
        const std::string& path = args[0].as_string();
        int result = ::unlink(path.c_str());
        return Value(static_cast<int64_t>(result));
    });
    
    register_intrinsic("syscall.stat", [](const std::vector<Value>& args) -> Value {
        if (args.empty()) {
            throw DryadException("syscall.stat requires 1 argument");
        }
        
        const std::string& path = args[0].as_string();
        struct stat st;
        int result = ::stat(path.c_str(), &st);
        
        if (result < 0) {
            return Value(static_cast<int64_t>(result));
        }
        
        Value obj = Value::create_object();
        return obj;
    });
}

void IntrinsicsRegistry::register_time_intrinsics() {
    register_intrinsic("syscall.time", [](const std::vector<Value>& args) -> Value {
        (void)args;
        time_t now = ::time(nullptr);
        return Value(static_cast<int64_t>(now));
    });
    
    register_intrinsic("syscall.clock_gettime", [](const std::vector<Value>& args) -> Value {
        (void)args;
        struct timespec ts;
        ::clock_gettime(CLOCK_REALTIME, &ts);
        return Value(static_cast<double>(ts.tv_sec) + static_cast<double>(ts.tv_nsec) / 1e9);
    });
}

void IntrinsicsRegistry::register_memory_intrinsics() {
    register_intrinsic("syscall.malloc", [](const std::vector<Value>& args) -> Value {
        if (args.empty()) {
            throw DryadException("syscall.malloc requires 1 argument");
        }
        
        int64_t size = args[0].as_integer();
        void* ptr = ::malloc(static_cast<size_t>(size));
        return Value(reinterpret_cast<int64_t>(ptr));
    });
    
    register_intrinsic("syscall.free", [](const std::vector<Value>& args) -> Value {
        if (args.empty()) {
            throw DryadException("syscall.free requires 1 argument");
        }
        
        int64_t ptr_val = args[0].as_integer();
        ::free(reinterpret_cast<void*>(ptr_val));
        return Value();
    });
    
    register_intrinsic("syscall.realloc", [](const std::vector<Value>& args) -> Value {
        if (args.size() < 2 || !args[0].is_integer() || !args[1].is_integer()) {
            throw DryadException("syscall.realloc requires 2 integer arguments");
        }
        
        return MemoryManager::realloc(args[0].as_integer(), args[1].as_integer());
    });
    
    register_intrinsic("syscall.memcpy", [](const std::vector<Value>& args) -> Value {
        if (args.size() < 3 || !args[0].is_integer() || 
            !args[1].is_integer() || !args[2].is_integer()) {
            throw DryadException("syscall.memcpy requires 3 integer arguments");
        }
        
        return MemoryManager::memcpy(args[0].as_integer(), args[1].as_integer(), args[2].as_integer());
    });
    
    register_intrinsic("syscall.memset", [](const std::vector<Value>& args) -> Value {
        if (args.size() < 3 || !args[0].is_integer() || 
            !args[1].is_integer() || !args[2].is_integer()) {
            throw DryadException("syscall.memset requires 3 integer arguments");
        }
        
        return MemoryManager::memset(args[0].as_integer(), args[1].as_integer(), args[2].as_integer());
    });
    
    register_intrinsic("syscall.alloc_bytes", [](const std::vector<Value>& args) -> Value {
        if (args.empty() || !args[0].is_integer()) {
            throw DryadException("syscall.alloc_bytes requires 1 integer argument");
        }
        
        return MemoryManager::alloc_bytes(args[0].as_integer());
    });
    
    register_intrinsic("syscall.free_bytes", [](const std::vector<Value>& args) -> Value {
        if (args.empty() || !args[0].is_integer()) {
            throw DryadException("syscall.free_bytes requires 1 integer argument");
        }
        
        return MemoryManager::free_bytes(args[0].as_integer());
    });
    
    register_intrinsic("syscall.buffer_get", [](const std::vector<Value>& args) -> Value {
        if (args.size() < 2 || !args[0].is_integer() || !args[1].is_integer()) {
            throw DryadException("syscall.buffer_get requires 2 integer arguments");
        }
        
        return MemoryManager::buffer_get(args[0].as_integer(), args[1].as_integer());
    });
    
    register_intrinsic("syscall.buffer_set", [](const std::vector<Value>& args) -> Value {
        if (args.size() < 3 || !args[0].is_integer() || 
            !args[1].is_integer() || !args[2].is_integer()) {
            throw DryadException("syscall.buffer_set requires 3 integer arguments");
        }
        
        return MemoryManager::buffer_set(args[0].as_integer(), args[1].as_integer(), args[2].as_integer());
    });
}

void IntrinsicsRegistry::register_string_intrinsics() {
    register_intrinsic("syscall.string_length", [](const std::vector<Value>& args) -> Value {
        if (args.empty() || !args[0].is_string()) {
            throw DryadException("syscall.string_length requires 1 string argument");
        }
        
        return Value(static_cast<int64_t>(args[0].as_string().size()));
    });

    register_intrinsic("syscall.int_to_string", [](const std::vector<Value>& args) -> Value {
        if (args.empty() || !args[0].is_integer()) {
            throw DryadException("syscall.int_to_string requires 1 integer argument");
        }
        return Value(std::to_string(args[0].as_integer()));
    });
    
    register_intrinsic("syscall.string_to_bytes", [](const std::vector<Value>& args) -> Value {
        if (args.size() < 3 || !args[0].is_string() || 
            !args[1].is_integer() || !args[2].is_integer()) {
            throw DryadException("syscall.string_to_bytes requires 3 arguments (string, handle, offset)");
        }
        
        return MemoryManager::string_to_bytes(args[1].as_integer(), args[2].as_integer(), args[0].as_string());
    });
    
    register_intrinsic("syscall.bytes_to_string", [](const std::vector<Value>& args) -> Value {
        if (args.size() < 3 || !args[0].is_integer() || 
            !args[1].is_integer() || !args[2].is_integer()) {
            throw DryadException("syscall.bytes_to_string requires 3 integer arguments (handle, offset, count)");
        }
        
        return MemoryManager::bytes_to_string(args[0].as_integer(), args[1].as_integer(), args[2].as_integer());
    });

    register_intrinsic("syscall.string_starts_with", [](const std::vector<Value>& args) -> Value {
        if (args.size() < 2 || !args[0].is_string() || !args[1].is_string()) {
            throw DryadException("syscall.string_starts_with requires 2 string arguments");
        }
        const std::string& value = args[0].as_string();
        const std::string& prefix = args[1].as_string();
        if (prefix.size() > value.size()) {
            return Value(false);
        }
        return Value(value.compare(0, prefix.size(), prefix) == 0);
    });
}

void IntrinsicsRegistry::register_misc_intrinsics() {
    register_intrinsic("runtime.get_stack_trace", [](const std::vector<Value>& args) -> Value {
        (void)args;
        return Value("[Stack trace placeholder - to be implemented with proper unwinding]");
    });
    
    register_intrinsic("runtime.create_object", [](const std::vector<Value>& args) -> Value {
        (void)args;
        return Value::create_object();
    });

    register_intrinsic("runtime.value_kind", [](const std::vector<Value>& args) -> Value {
        if (args.empty()) {
            return Value("null");
        }
        switch (args[0].type()) {
            case ValueType::Null: return Value("null");
            case ValueType::Boolean: return Value("boolean");
            case ValueType::Integer: return Value("integer");
            case ValueType::Float: return Value("float");
            case ValueType::String: return Value("string");
            case ValueType::Array: return Value("array");
            case ValueType::Object: return Value("object");
            case ValueType::Function: return Value("function");
            case ValueType::Class: return Value("class");
            case ValueType::Instance: return Value("instance");
            default: return Value("unknown");
        }
    });
    
    register_intrinsic("runtime.object_set", [](const std::vector<Value>& args) -> Value {
        if (args.size() < 3) {
            throw DryadException("runtime.object_set requires 3 arguments (object, key, value)");
        }
        
        if (!args[0].is_object()) {
            throw DryadException("First argument must be an object");
        }
        if (!args[1].is_string()) {
            throw DryadException("Second argument (key) must be a string");
        }
        
        const_cast<Value&>(args[0]).object_set(args[1].as_string(), args[2]);
        return Value();
    });
    
    register_intrinsic("runtime.object_get", [](const std::vector<Value>& args) -> Value {
        if (args.size() < 2) {
            throw DryadException("runtime.object_get requires 2 arguments (object, key)");
        }
        
        if (!args[0].is_object()) {
            throw DryadException("First argument must be an object");
        }
        if (!args[1].is_string()) {
            throw DryadException("Second argument (key) must be a string");
        }
        
        return args[0].object_get(args[1].as_string());
    });
    
    register_intrinsic("runtime.array_length", [](const std::vector<Value>& args) -> Value {
        if (args.empty()) {
            throw DryadException("runtime.array_length requires 1 argument (array)");
        }
        
        if (!args[0].is_array()) {
            throw DryadException("Argument must be an array");
        }
        
        return Value(static_cast<int64_t>(args[0].array_length()));
    });

    register_intrinsic("runtime.object_keys", [](const std::vector<Value>& args) -> Value {
        if (args.empty() || !args[0].is_object()) {
            throw DryadException("runtime.object_keys requires 1 object argument");
        }
        Value result = Value::create_array();
        for (const auto& key : args[0].object_keys()) {
            result.array_push(Value(key));
        }
        return result;
    });

    register_intrinsic("runtime.array_push", [](const std::vector<Value>& args) -> Value {
        if (args.size() < 2 || !args[0].is_array()) {
            throw DryadException("runtime.array_push requires (array, value)");
        }
        const_cast<Value&>(args[0]).array_push(args[1]);
        return args[0];
    });
}

void IntrinsicsRegistry::register_all() {
    register_file_io_intrinsics();
    register_network_intrinsics();
    register_tls_intrinsics();
    register_filesystem_intrinsics();
    register_process_intrinsics();
    register_time_intrinsics();
    register_memory_intrinsics();
    register_string_intrinsics();
    register_misc_intrinsics();
    register_thread_intrinsics();
    register_event_loop_intrinsics();
    register_poller_intrinsics();
}

} // namespace dryad