#include <chrono>
#include <cinttypes>
#include <cstdint>
#include <cstdio>
#include "dryad/runtime/intrinsics.h"

using namespace dryad;

namespace {

using Clock = std::chrono::steady_clock;

double elapsed_ms(Clock::time_point from, Clock::time_point to) {
    return std::chrono::duration<double, std::milli>(to - from).count();
}

}  // namespace

int main() {
    constexpr int64_t kSize = 1 << 16;
    constexpr int kRuns = 3;

    Intrinsics intr;
    Value src = intr.alloc_bytes(kSize);
    Value dst = intr.alloc_bytes(kSize);
    const int64_t sh = src.as_integer();
    const int64_t dh = dst.as_integer();
    for (int64_t i = 0; i < kSize; ++i) {
        intr.buffer_set(sh, i, i & 0xFF);
    }

    const auto t0 = Clock::now();
    for (int r = 0; r < kRuns; ++r) {
        intr.memmove(dh, 0, sh, 0, kSize);
    }
    const auto t1 = Clock::now();
    for (int r = 0; r < kRuns; ++r) {
        for (int64_t i = 0; i < kSize; ++i) {
            Value v = intr.buffer_get(sh, i);
            intr.buffer_set(dh, i, v.as_integer());
        }
    }
    const auto t2 = Clock::now();

    std::printf("memmove: %.3f ms\nbyte loop: %.3f ms\n", elapsed_ms(t0, t1), elapsed_ms(t1, t2));

    intr.free_bytes(sh);
    intr.free_bytes(dh);
    return 0;
}
