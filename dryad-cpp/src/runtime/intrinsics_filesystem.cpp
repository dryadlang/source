#include "dryad/runtime/intrinsics_registry.h"
#include "dryad/common/utils.h"
#include <sys/stat.h>
#include <dirent.h>
#include <unistd.h>
#include <cstdio>

namespace dryad {

void IntrinsicsRegistry::register_filesystem_intrinsics() {
    register_intrinsic("syscall.mkdir", [](const std::vector<Value>& args) -> Value {
        if (args.empty()) {
            throw DryadException("syscall.mkdir requires 1 argument");
        }
        
        const std::string& path = args[0].as_string();
        int result = ::mkdir(path.c_str(), 0755);
        return Value(static_cast<int64_t>(result));
    });
    
    register_intrinsic("syscall.rmdir", [](const std::vector<Value>& args) -> Value {
        if (args.empty()) {
            throw DryadException("syscall.rmdir requires 1 argument");
        }
        
        const std::string& path = args[0].as_string();
        int result = ::rmdir(path.c_str());
        return Value(static_cast<int64_t>(result));
    });
    
    register_intrinsic("syscall.lseek", [](const std::vector<Value>& args) -> Value {
        if (args.size() < 3) {
            throw DryadException("syscall.lseek requires 3 arguments");
        }
        
        int64_t fd = args[0].as_integer();
        int64_t offset = args[1].as_integer();
        int64_t whence = args[2].as_integer();
        
        off_t result = ::lseek(static_cast<int>(fd), 
                              static_cast<off_t>(offset),
                              static_cast<int>(whence));
        return Value(static_cast<int64_t>(result));
    });
    
    register_intrinsic("syscall.rename", [](const std::vector<Value>& args) -> Value {
        if (args.size() < 2) {
            throw DryadException("syscall.rename requires 2 arguments");
        }
        
        const std::string& oldpath = args[0].as_string();
        const std::string& newpath = args[1].as_string();
        
        int result = ::rename(oldpath.c_str(), newpath.c_str());
        return Value(static_cast<int64_t>(result));
    });
    
    register_intrinsic("syscall.getcwd", [](const std::vector<Value>& args) -> Value {
        (void)args;
        char buffer[4096];
        if (::getcwd(buffer, sizeof(buffer)) != nullptr) {
            return Value(std::string(buffer));
        }
        return Value("");
    });
    
    register_intrinsic("syscall.chdir", [](const std::vector<Value>& args) -> Value {
        if (args.empty()) {
            throw DryadException("syscall.chdir requires 1 argument");
        }
        
        const std::string& path = args[0].as_string();
        int result = ::chdir(path.c_str());
        return Value(static_cast<int64_t>(result));
    });
}

} // namespace dryad