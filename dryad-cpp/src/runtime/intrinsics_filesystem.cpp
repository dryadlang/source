#include "dryad/runtime/intrinsics_registry.h"
#include "dryad/common/utils.h"
#include <sys/stat.h>
#include <dirent.h>
#include <fcntl.h>
#include <unistd.h>
#ifdef _WIN32
#include <direct.h>
#include <io.h>
#endif
#include <cstdio>

namespace dryad {

void IntrinsicsRegistry::register_filesystem_intrinsics() {
    register_intrinsic("syscall.mkdir", [](const std::vector<Value>& args) -> Value {
        if (args.empty()) {
            throw DryadException("syscall.mkdir requires 1 argument");
        }
        
        const std::string& path = args[0].as_string();
#ifdef _WIN32
        int result = ::_mkdir(path.c_str());
#else
        int result = ::mkdir(path.c_str(), 0755);
#endif
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

    register_intrinsic("syscall.exists", [](const std::vector<Value>& args) -> Value {
        if (args.empty() || !args[0].is_string()) {
            throw DryadException("syscall.exists requires 1 string argument");
        }
        const std::string& path = args[0].as_string();
#ifdef _WIN32
        int result = ::_access(path.c_str(), 0);
#else
        int result = ::access(path.c_str(), F_OK);
#endif
        return Value(result == 0);
    });

    register_intrinsic("syscall.filesize", [](const std::vector<Value>& args) -> Value {
        if (args.empty() || !args[0].is_string()) {
            throw DryadException("syscall.filesize requires 1 string argument");
        }
        struct stat st;
        if (::stat(args[0].as_string().c_str(), &st) < 0) {
            return Value(static_cast<int64_t>(-1));
        }
        return Value(static_cast<int64_t>(st.st_size));
    });

    register_intrinsic("syscall.readdir", [](const std::vector<Value>& args) -> Value {
        if (args.empty() || !args[0].is_string()) {
            throw DryadException("syscall.readdir requires 1 string argument");
        }
        DIR* dir = ::opendir(args[0].as_string().c_str());
        if (dir == nullptr) {
            return Value::create_array();
        }
        Value result = Value::create_array();
        while (struct dirent* entry = ::readdir(dir)) {
            std::string name = entry->d_name;
            if (name == "." || name == "..") {
                continue;
            }
            result.array_push(Value(name));
        }
        ::closedir(dir);
        return result;
    });

    auto open_with_flags = [](const std::string& path, int flags) -> Value {
        int fd = ::open(path.c_str(), flags, 0644);
        return Value(static_cast<int64_t>(fd));
    };

    register_intrinsic("syscall.open_read", [open_with_flags](const std::vector<Value>& args) -> Value {
        if (args.empty() || !args[0].is_string()) {
            throw DryadException("syscall.open_read requires 1 string argument");
        }
        return open_with_flags(args[0].as_string(), O_RDONLY);
    });

    register_intrinsic("syscall.open_write", [open_with_flags](const std::vector<Value>& args) -> Value {
        if (args.empty() || !args[0].is_string()) {
            throw DryadException("syscall.open_write requires 1 string argument");
        }
        return open_with_flags(args[0].as_string(), O_WRONLY | O_CREAT | O_TRUNC);
    });

    register_intrinsic("syscall.open_append", [open_with_flags](const std::vector<Value>& args) -> Value {
        if (args.empty() || !args[0].is_string()) {
            throw DryadException("syscall.open_append requires 1 string argument");
        }
        return open_with_flags(args[0].as_string(), O_WRONLY | O_CREAT | O_APPEND);
    });

    register_intrinsic("syscall.normalize_path", [](const std::vector<Value>& args) -> Value {
        if (args.empty() || !args[0].is_string()) {
            throw DryadException("syscall.normalize_path requires 1 string argument");
        }
        std::string normalized = args[0].as_string();
        for (char& ch : normalized) {
            if (ch == '\\') {
                ch = '/';
            }
        }
        return Value(normalized);
    });
}

} // namespace dryad