#pragma once

#include <cstdint>

namespace dryad {

// Dryad integers are 64-bit two's complement and wrap on overflow (like `<<`,
// which already wraps modulo 2^64). Signed overflow is undefined behaviour in
// C++, so every integer + - * / % and unary minus in the interpreter and the
// VM goes through these helpers, which compute in uint64_t.

inline int64_t wrapping_add(int64_t a, int64_t b) {
    return static_cast<int64_t>(static_cast<uint64_t>(a) + static_cast<uint64_t>(b));
}

inline int64_t wrapping_sub(int64_t a, int64_t b) {
    return static_cast<int64_t>(static_cast<uint64_t>(a) - static_cast<uint64_t>(b));
}

inline int64_t wrapping_mul(int64_t a, int64_t b) {
    return static_cast<int64_t>(static_cast<uint64_t>(a) * static_cast<uint64_t>(b));
}

inline int64_t wrapping_neg(int64_t a) {
    return wrapping_sub(0, a);
}

// Truncating division; the caller rejects b == 0. INT64_MIN / -1 wraps to
// INT64_MIN instead of trapping (SIGFPE on x86).
inline int64_t wrapping_div(int64_t a, int64_t b) {
    if (b == -1) {
        return wrapping_neg(a);
    }
    return a / b;
}

// Remainder with the sign of the dividend; the caller rejects b == 0.
// INT64_MIN % -1 is 0 instead of trapping.
inline int64_t wrapping_rem(int64_t a, int64_t b) {
    if (b == -1) {
        return 0;
    }
    return a % b;
}

} // namespace dryad
