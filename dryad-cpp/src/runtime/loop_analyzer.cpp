#include "dryad/runtime/loop_analyzer.h"
#include "dryad/runtime/opcode.h"

namespace dryad {

namespace {

uint16_t read_u16(const std::vector<uint8_t>& code, size_t offset) {
    return static_cast<uint16_t>(code[offset]) |
           (static_cast<uint16_t>(code[offset + 1]) << 8);
}

size_t instruction_size(Opcode op) {
    switch (op) {
        case Opcode::PUSH_INT:
        case Opcode::PUSH_FLOAT:
            return 9;
        case Opcode::LOAD_FAST:
        case Opcode::STORE_FAST:
        case Opcode::LOAD_GLOBAL:
        case Opcode::JUMP:
        case Opcode::JUMP_IF_FALSE:
        case Opcode::JUMP_IF_TRUE:
            return 3;
        case Opcode::CALL:
        case Opcode::INTRINSIC_SYSCALL:
        case Opcode::METHOD_CALL:
        case Opcode::NEW_INSTANCE:
        case Opcode::CALL_GLOBAL:
            return 4;
        case Opcode::GET_PROPERTY:
        case Opcode::SET_PROPERTY:
            return 3;
        case Opcode::NEW_ARRAY:
        case Opcode::ARRAY_PUSH:
        case Opcode::SET_ELEMENT:
        case Opcode::NOT:
        case Opcode::DIV_I32:
            return 1;
        default:
            return 1;
    }
}

bool find_while_body(const std::vector<uint8_t>& code,
                     uint16_t header_pc,
                     uint16_t back_edge_pc,
                     uint16_t& body_start,
                     uint16_t& body_end) {
    const uint16_t exit_pc = static_cast<uint16_t>(back_edge_pc + 3);
    size_t pc = header_pc;
    while (pc < back_edge_pc) {
        Opcode op = static_cast<Opcode>(code[pc]);
        if (op == Opcode::JUMP_IF_FALSE) {
            const uint16_t target = read_u16(code, pc + 1);
            if (target == exit_pc) {
                body_start = static_cast<uint16_t>(pc + 3);
                body_end = back_edge_pc;
                return body_start < body_end;
            }
        }
        pc += instruction_size(op);
    }
    return false;
}

} // namespace

std::vector<LoopInfo> analyze_loops(const std::vector<uint8_t>& code) {
    std::vector<LoopInfo> loops;
    if (code.empty()) {
        return loops;
    }

    size_t pc = 0;
    while (pc < code.size()) {
        Opcode op = static_cast<Opcode>(code[pc]);
        if (op == Opcode::JUMP) {
            const uint16_t target = read_u16(code, pc + 1);
            if (target < pc) {
                LoopInfo loop;
                loop.header_pc = target;
                loop.back_edge_pc = static_cast<uint16_t>(pc);
                if (find_while_body(code, loop.header_pc, loop.back_edge_pc,
                                    loop.body_start, loop.body_end)) {
                    loops.push_back(loop);
                }
            }
        }
        pc += instruction_size(op);
    }

    return loops;
}

} // namespace dryad
