#ifndef DRYAD_RUNTIME_LOOP_ANALYZER_H
#define DRYAD_RUNTIME_LOOP_ANALYZER_H

#include <cstdint>
#include <vector>

namespace dryad {

struct LoopInfo {
    uint16_t header_pc = 0;
    uint16_t back_edge_pc = 0;
    uint16_t body_start = 0;
    uint16_t body_end = 0;
};

std::vector<LoopInfo> analyze_loops(const std::vector<uint8_t>& code);

} // namespace dryad

#endif // DRYAD_RUNTIME_LOOP_ANALYZER_H
