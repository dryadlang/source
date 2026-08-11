#include "dryad/runtime/intrinsics_registry.h"
#include "dryad/common/utils.h"
#include <unistd.h>
#include <sys/wait.h>
#include <pthread.h>
#include <cstdlib>

namespace dryad {

void IntrinsicsRegistry::register_process_intrinsics() {
    register_intrinsic("syscall.getpid", [](const std::vector<Value>& args) -> Value {
        (void)args;
        pid_t pid = ::getpid();
        return Value(static_cast<int64_t>(pid));
    });
    
    register_intrinsic("syscall.getenv", [](const std::vector<Value>& args) -> Value {
        if (args.empty()) {
            throw DryadException("syscall.getenv requires 1 argument");
        }
        
        const std::string& name = args[0].as_string();
        const char* value = ::getenv(name.c_str());
        
        if (value != nullptr) {
            return Value(std::string(value));
        }
        return Value("");
    });
    
    register_intrinsic("syscall.setenv", [](const std::vector<Value>& args) -> Value {
        if (args.size() < 2) {
            throw DryadException("syscall.setenv requires 2 arguments");
        }
        
        const std::string& name = args[0].as_string();
        const std::string& value = args[1].as_string();
        
        int result = ::setenv(name.c_str(), value.c_str(), 1);
        return Value(static_cast<int64_t>(result));
    });
    
    register_intrinsic("syscall.sleep", [](const std::vector<Value>& args) -> Value {
        if (args.empty()) {
            throw DryadException("syscall.sleep requires 1 argument");
        }
        
        int64_t seconds = args[0].as_integer();
        unsigned int result = ::sleep(static_cast<unsigned int>(seconds));
        return Value(static_cast<int64_t>(result));
    });
    
    register_intrinsic("syscall.exit", [](const std::vector<Value>& args) -> Value {
        int64_t code = 0;
        if (!args.empty()) {
            code = args[0].as_integer();
        }
        ::exit(static_cast<int>(code));
        return Value();
    });
}

} // namespace dryad