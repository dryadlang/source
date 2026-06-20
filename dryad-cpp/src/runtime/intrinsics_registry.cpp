#include "dryad/runtime/intrinsics_registry.h"
#include "dryad/common/utils.h"
#include <fcntl.h>
#include <unistd.h>
#include <sys/stat.h>
#include <cerrno>
#include <cstring>
#include <ctime>

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
}

void IntrinsicsRegistry::register_network_intrinsics() {
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
}

void IntrinsicsRegistry::register_all() {
    register_file_io_intrinsics();
    register_network_intrinsics();
    register_filesystem_intrinsics();
    register_process_intrinsics();
    register_time_intrinsics();
    register_memory_intrinsics();
    register_misc_intrinsics();
}

} // namespace dryad