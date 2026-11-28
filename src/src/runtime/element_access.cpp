#include "dryad/runtime/element_access.h"
#include "dryad/common/utils.h"
#include "dryad/compiler/ast.h"
#include "dryad/runtime/class.h"
#include <cstdint>

namespace dryad {

namespace {

size_t array_index(const Value& key) {
    if (!key.is_integer()) {
        throw DryadException("Array index must be an integer");
    }
    const int64_t index = key.as_integer();
    if (index < 0) {
        throw DryadException("Array index out of bounds");
    }
    return static_cast<size_t>(index);
}

const std::string& object_key(const Value& key) {
    if (!key.is_string()) {
        throw DryadException("Object key must be a string");
    }
    return key.as_string();
}

} // namespace

Value get_element(const Value& container, const Value& key) {
    if (container.is_array()) {
        return container.array_get(array_index(key));
    }
    if (container.is_object()) {
        return container.object_get(object_key(key));
    }
    if (container.is_instance()) {
        return container.as_instance()->get(object_key(key));
    }
    throw DryadException("Cannot access member of non-object/array value");
}

void set_element(const Value& container, const Value& key, const Value& value) {
    if (container.is_array()) {
        // Value shares the underlying array, so writing through a copy is
        // visible to every holder.
        Value(container).array_set(array_index(key), value);
        return;
    }
    if (container.is_object()) {
        Value(container).object_set(object_key(key), value);
        return;
    }
    if (container.is_instance()) {
        container.as_instance()->set(object_key(key), value);
        return;
    }
    throw DryadException("Cannot assign to member of non-object/array value");
}

} // namespace dryad
