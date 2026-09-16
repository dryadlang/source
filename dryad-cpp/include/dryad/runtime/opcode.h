#ifndef DRYAD_RUNTIME_OPCODE_H
#define DRYAD_RUNTIME_OPCODE_H

#include <cstdint>

namespace dryad {

enum class Opcode : uint8_t {
    // Stack operations (0x00–0x0F)
    PUSH_NULL   = 0x00,
    PUSH_TRUE   = 0x01,
    PUSH_FALSE  = 0x02,
    PUSH_INT    = 0x03,
    PUSH_FLOAT  = 0x04,
    PUSH_CONST  = 0x05,
    POP         = 0x06,
    DUP         = 0x07,
    SWAP        = 0x08,

    // Arithmetic — generic (0x10–0x14)
    ADD         = 0x10,
    SUB         = 0x11,
    MUL         = 0x12,
    DIV         = 0x13,
    MOD         = 0x14,

    // Arithmetic — type-specialized (0x15–0x1F)
    ADD_F64     = 0x15,
    SUB_F64     = 0x16,
    MUL_F64     = 0x17,
    DIV_F64     = 0x18,
    ADD_I32     = 0x19,
    SUB_I32     = 0x1A,
    MUL_I32     = 0x1B,

    // Comparison (0x40–0x45)
    EQ          = 0x40,
    NEQ         = 0x41,
    LT          = 0x42,
    LTE         = 0x43,
    GT          = 0x44,
    GTE         = 0x45,

    // Control flow (0x50–0x57)
    JUMP        = 0x50,
    JUMP_IF_FALSE = 0x51,
    JUMP_IF_TRUE  = 0x52,
    HALT        = 0x53,

    // Variables — fast locals (0x60–0x63)
    LOAD_FAST   = 0x60,
    STORE_FAST  = 0x61,

    // Array / element access (0x80–0x87)
    GET_ELEMENT         = 0x85,
    GET_ELEMENT_F64     = 0x86,

    // Intrinsic dispatch (0xFF)
    INTRINSIC_SYSCALL = 0xFF,
};

const char* opcode_name(Opcode op);

} // namespace dryad

#endif // DRYAD_RUNTIME_OPCODE_H
