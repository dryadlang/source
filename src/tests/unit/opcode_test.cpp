#include <gtest/gtest.h>
#include "dryad/runtime/opcode.h"
#include <string>

using namespace dryad;

TEST(OpcodeTest, OpcodeNameCoversRepresentativeOpcodes) {
    // opcode_name() is only ever called from error/diagnostic formatting
    // (vm.cpp's "Unimplemented opcode" message, analyze_pass.cpp's
    // diagnostics). Exercise it directly so the mapping itself is checked
    // without needing to provoke those error paths end to end.
    EXPECT_EQ(std::string(opcode_name(Opcode::PUSH_NULL)), "PUSH_NULL");
    EXPECT_EQ(std::string(opcode_name(Opcode::ADD)), "ADD");
    EXPECT_EQ(std::string(opcode_name(Opcode::JUMP_IF_FALSE)), "JUMP_IF_FALSE");
    EXPECT_EQ(std::string(opcode_name(Opcode::CALL_VALUE)), "CALL_VALUE");
    EXPECT_EQ(std::string(opcode_name(Opcode::MAKE_CLOSURE)), "MAKE_CLOSURE");
    EXPECT_EQ(std::string(opcode_name(Opcode::LOAD_UPVALUE_REF)), "LOAD_UPVALUE_REF");
    EXPECT_EQ(std::string(opcode_name(Opcode::INTRINSIC_SYSCALL)), "INTRINSIC_SYSCALL");
}

TEST(OpcodeTest, OpcodeSizeMatchesEncodingWidth) {
    // 9-byte operand (1 tag + 8-byte immediate)
    EXPECT_EQ(opcode_size(Opcode::PUSH_INT), 9u);
    EXPECT_EQ(opcode_size(Opcode::PUSH_FLOAT), 9u);
    // 3-byte operand (1 tag + 2-byte index)
    EXPECT_EQ(opcode_size(Opcode::PUSH_CONST), 3u);
    EXPECT_EQ(opcode_size(Opcode::LOAD_FAST), 3u);
    EXPECT_EQ(opcode_size(Opcode::JUMP), 3u);
    // 4-byte operand
    EXPECT_EQ(opcode_size(Opcode::CALL), 4u);
    EXPECT_EQ(opcode_size(Opcode::MAKE_CLOSURE), 4u);
    // 2-byte operand
    EXPECT_EQ(opcode_size(Opcode::CALL_VALUE), 2u);
    // no operand (default case)
    EXPECT_EQ(opcode_size(Opcode::POP), 1u);
    EXPECT_EQ(opcode_size(Opcode::ADD), 1u);
    EXPECT_EQ(opcode_size(Opcode::HALT), 1u);
}
