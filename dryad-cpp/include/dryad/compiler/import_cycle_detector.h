#ifndef DRYAD_COMPILER_IMPORT_CYCLE_DETECTOR_H
#define DRYAD_COMPILER_IMPORT_CYCLE_DETECTOR_H

#include <string>
#include <vector>

namespace dryad {

class ModuleLoader;

struct ImportCycleReport {
    bool has_cycle = false;
    std::vector<std::string> cycle;
};

ImportCycleReport detect_import_cycles(const std::string& entry_path, ModuleLoader& loader);

} // namespace dryad

#endif // DRYAD_COMPILER_IMPORT_CYCLE_DETECTOR_H
