#ifndef DRYAD_RUNTIME_MODULE_LOADER_H
#define DRYAD_RUNTIME_MODULE_LOADER_H

#include <string>

namespace dryad {

class ModuleLoader {
public:
    ModuleLoader();
    ~ModuleLoader();
    
    void load_module(const std::string& path);
    
private:
};

} // namespace dryad

#endif // DRYAD_RUNTIME_MODULE_LOADER_H
