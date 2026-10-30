#ifndef DRYAD_AOT_ANALYZE_PASS_H
#define DRYAD_AOT_ANALYZE_PASS_H

#include "dryad/compiler/compile_pipeline.h"
#include <string>
#include <vector>

namespace dryad {

enum class AnalyzePathKind {
    FastPath,
    SlowPath
};

struct AnalyzeFinding {
    std::string function_name;
    std::string site;
    AnalyzePathKind path_kind;
    std::string reason;
    std::string suggestion;
};

struct AnalyzeReport {
    std::string source_file;
    std::vector<AnalyzeFinding> findings;
    size_t fast_path_sites = 0;
    size_t slow_path_sites = 0;

    std::string format() const;
};

AnalyzeReport run_analyze_pass(const std::string& path, const CompileOptions& options);

} // namespace dryad

#endif // DRYAD_AOT_ANALYZE_PASS_H
