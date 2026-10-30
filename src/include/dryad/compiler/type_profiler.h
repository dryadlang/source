#ifndef DRYAD_COMPILER_TYPE_PROFILER_H
#define DRYAD_COMPILER_TYPE_PROFILER_H

#include "dryad/compiler/ast.h"
#include "dryad/runtime/value.h"
#include <cstdint>
#include <string>
#include <unordered_map>
#include <vector>

namespace dryad {

enum class TypeConfidence {
    STATIC,
    OBSERVED,
    UNKNOWN
};

enum class ObservedType {
    Null,
    Bool,
    I32,
    F64,
    String,
    Array,
    Object,
    Function,
    Class,
    Instance,
    Any
};

struct SlotProfile {
    ObservedType type = ObservedType::Any;
    TypeConfidence confidence = TypeConfidence::UNKNOWN;
    uint64_t observation_count = 0;
};

struct BranchProfile {
    uint64_t taken = 0;
    uint64_t not_taken = 0;
};

struct CallSiteProfile {
    std::string callee_name;
    uint64_t call_count = 0;
};

struct FunctionProfile {
    std::string name;
    std::unordered_map<std::string, SlotProfile> slots;
    std::unordered_map<std::string, BranchProfile> branches;
    std::unordered_map<std::string, CallSiteProfile> call_sites;
    uint64_t invocation_count = 0;
};

class TypeProfiler {
public:
    void set_enabled(bool enabled) { enabled_ = enabled; }
    bool enabled() const { return enabled_; }

    void reset();

    void begin_function(FunctionDeclaration* fn);
    void end_function();

    void observe_slot(const std::string& name, const Value& value,
                      const std::string& type_annotation = "");
    void observe_branch(const std::string& site_id, bool taken);
    void observe_call(const std::string& callee_name);

    const FunctionProfile* current_function_profile() const;
    const FunctionProfile* get_function_profile(const std::string& name) const;
    const std::unordered_map<std::string, SlotProfile>& script_slots() const { return script_slots_; }
    FunctionProfile build_script_function_profile(const std::string& name) const;
    const std::unordered_map<FunctionDeclaration*, FunctionProfile>& profiles() const {
        return profiles_;
    }

    static ObservedType from_value(const Value& value);
    static ObservedType from_annotation(const std::string& annotation);
    static std::string observed_type_name(ObservedType type);
    static std::string confidence_name(TypeConfidence confidence);
    static std::string function_key(FunctionDeclaration* fn);

private:
    bool enabled_ = true;
    FunctionDeclaration* current_fn_ = nullptr;
    std::unordered_map<std::string, SlotProfile> script_slots_;
    std::unordered_map<FunctionDeclaration*, FunctionProfile> profiles_;

    FunctionProfile& profile_for(FunctionDeclaration* fn);
    void merge_slot(SlotProfile& slot, ObservedType new_type, TypeConfidence confidence);
};

} // namespace dryad

#endif // DRYAD_COMPILER_TYPE_PROFILER_H
