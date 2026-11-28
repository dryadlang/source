#ifndef DRYAD_RUNTIME_ELEMENT_ACCESS_H
#define DRYAD_RUNTIME_ELEMENT_ACCESS_H

#include "dryad/runtime/value.h"

namespace dryad {

// `container[key]`, shared by the interpreter and the bytecode VM so both
// tiers accept the same operands and report the same errors:
// - arrays take an integer index (bounds-checked);
// - objects and class instances take a string key (a missing key reads null).
// Anything else throws.
Value get_element(const Value& container, const Value& key);

// `container[key] = value`, with the same rules as get_element.
void set_element(const Value& container, const Value& key, const Value& value);

} // namespace dryad

#endif // DRYAD_RUNTIME_ELEMENT_ACCESS_H
