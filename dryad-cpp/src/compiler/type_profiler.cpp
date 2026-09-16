#include "dryad/compiler/type_profiler.h"
#include <algorithm>
#include <cctype>

namespace dryad {

namespace {

std::string to_lower(std::string s) {
    std::transform(s.begin(), s.end(), s.begin(),
                   [](unsigned char c) { return static_cast<char>(std::tolower(c)); });
    return s;
}

} // namespace

void TypeProfiler::reset() {
    current_fn_ = nullptr;
    profiles_.clear();
}

std::string TypeProfiler::function_key(FunctionDeclaration* fn) {
    if (!fn) {
        return "<unknown>";
    }
    if (!fn->name.empty()) {
        return fn->name;
    }
    return "<lambda>@" + std::to_string(fn->location.line) + ":" +
           std::to_string(fn->location.column);
}

FunctionProfile& TypeProfiler::profile_for(FunctionDeclaration* fn) {
    auto& profile = profiles_[fn];
    if (profile.name.empty()) {
        profile.name = function_key(fn);
    }
    return profile;
}

void TypeProfiler::begin_function(FunctionDeclaration* fn) {
    if (!enabled_ || !fn) {
        return;
    }
    current_fn_ = fn;
    auto& profile = profile_for(fn);
    profile.invocation_count++;

    for (const auto& param : fn->parameters) {
        if (!param.type_annotation.empty()) {
            auto& slot = profile.slots[param.name];
            merge_slot(slot, from_annotation(param.type_annotation), TypeConfidence::STATIC);
        }
    }
}

void TypeProfiler::end_function() {
    current_fn_ = nullptr;
}

void TypeProfiler::observe_slot(const std::string& name, const Value& value,
                                const std::string& type_annotation) {
    if (!enabled_ || !current_fn_ || name.empty()) {
        return;
    }

    auto& profile = profile_for(current_fn_);
    auto& slot = profile.slots[name];

    if (!type_annotation.empty()) {
        merge_slot(slot, from_annotation(type_annotation), TypeConfidence::STATIC);
        return;
    }

    merge_slot(slot, from_value(value), TypeConfidence::OBSERVED);
}

void TypeProfiler::observe_branch(const std::string& site_id, bool taken) {
    if (!enabled_ || !current_fn_) {
        return;
    }

    auto& profile = profile_for(current_fn_);
    auto& branch = profile.branches[site_id];
    if (taken) {
        branch.taken++;
    } else {
        branch.not_taken++;
    }
}

void TypeProfiler::observe_call(const std::string& callee_name) {
    if (!enabled_ || !current_fn_ || callee_name.empty()) {
        return;
    }

    auto& profile = profile_for(current_fn_);
    profile.call_sites[callee_name].callee_name = callee_name;
    profile.call_sites[callee_name].call_count++;
}

const FunctionProfile* TypeProfiler::current_function_profile() const {
    if (!current_fn_) {
        return nullptr;
    }
    auto it = profiles_.find(current_fn_);
    return it != profiles_.end() ? &it->second : nullptr;
}

const FunctionProfile* TypeProfiler::get_function_profile(const std::string& name) const {
    for (const auto& entry : profiles_) {
        if (entry.second.name == name) {
            return &entry.second;
        }
    }
    return nullptr;
}

ObservedType TypeProfiler::from_value(const Value& value) {
    switch (value.type()) {
        case ValueType::Null:     return ObservedType::Null;
        case ValueType::Boolean:  return ObservedType::Bool;
        case ValueType::Integer:  return ObservedType::I32;
        case ValueType::Float:    return ObservedType::F64;
        case ValueType::String:   return ObservedType::String;
        case ValueType::Array:    return ObservedType::Array;
        case ValueType::Object:   return ObservedType::Object;
        case ValueType::Function: return ObservedType::Function;
        case ValueType::Class:    return ObservedType::Class;
        case ValueType::Instance: return ObservedType::Instance;
        default:                  return ObservedType::Any;
    }
}

ObservedType TypeProfiler::from_annotation(const std::string& annotation) {
    const std::string lower = to_lower(annotation);

    if (lower == "bool" || lower == "boolean") {
        return ObservedType::Bool;
    }
    if (lower == "i32" || lower == "int" || lower == "integer") {
        return ObservedType::I32;
    }
    if (lower == "f64" || lower == "number" || lower == "float" || lower == "double") {
        return ObservedType::F64;
    }
    if (lower == "string") {
        return ObservedType::String;
    }
    if (lower == "null") {
        return ObservedType::Null;
    }
    if (lower == "any") {
        return ObservedType::Any;
    }
    if (lower.size() > 2 && lower.back() == ']' && lower[lower.size() - 2] == '[') {
        return ObservedType::Array;
    }
    if (lower == "object") {
        return ObservedType::Object;
    }
    if (lower == "function" || lower == "fn") {
        return ObservedType::Function;
    }

    return ObservedType::Any;
}

void TypeProfiler::merge_slot(SlotProfile& slot, ObservedType new_type,
                              TypeConfidence confidence) {
    if (confidence == TypeConfidence::STATIC) {
        if (slot.confidence == TypeConfidence::UNKNOWN ||
            slot.confidence == TypeConfidence::STATIC) {
            slot.type = new_type;
            slot.confidence = TypeConfidence::STATIC;
            slot.observation_count++;
        } else if (slot.type != new_type) {
            slot.type = ObservedType::Any;
            slot.confidence = TypeConfidence::UNKNOWN;
        }
        return;
    }

    if (slot.confidence == TypeConfidence::STATIC) {
        if (slot.type != new_type && new_type != ObservedType::Null) {
            slot.type = ObservedType::Any;
            slot.confidence = TypeConfidence::UNKNOWN;
        }
        slot.observation_count++;
        return;
    }

    if (slot.observation_count == 0) {
        slot.type = new_type;
        slot.confidence = TypeConfidence::OBSERVED;
        slot.observation_count = 1;
        return;
    }

    if (slot.type == new_type || new_type == ObservedType::Null) {
        slot.observation_count++;
        if (slot.confidence != TypeConfidence::STATIC) {
            slot.confidence = TypeConfidence::OBSERVED;
        }
        return;
    }

    if ((slot.type == ObservedType::I32 && new_type == ObservedType::F64) ||
        (slot.type == ObservedType::F64 && new_type == ObservedType::I32)) {
        slot.type = ObservedType::F64;
        slot.observation_count++;
        return;
    }

    slot.type = ObservedType::Any;
    slot.confidence = TypeConfidence::UNKNOWN;
    slot.observation_count++;
}

std::string TypeProfiler::observed_type_name(ObservedType type) {
    switch (type) {
        case ObservedType::Null:     return "null";
        case ObservedType::Bool:     return "bool";
        case ObservedType::I32:      return "i32";
        case ObservedType::F64:      return "f64";
        case ObservedType::String:   return "string";
        case ObservedType::Array:    return "array";
        case ObservedType::Object:   return "object";
        case ObservedType::Function: return "function";
        case ObservedType::Class:    return "class";
        case ObservedType::Instance: return "instance";
        case ObservedType::Any:      return "any";
    }
    return "any";
}

std::string TypeProfiler::confidence_name(TypeConfidence confidence) {
    switch (confidence) {
        case TypeConfidence::STATIC:   return "STATIC";
        case TypeConfidence::OBSERVED: return "OBSERVED";
        case TypeConfidence::UNKNOWN:  return "UNKNOWN";
    }
    return "UNKNOWN";
}

} // namespace dryad
