#include <gtest/gtest.h>
#include "dryad/common/utils.h"
#include "dryad/runtime/vm.h"
#include "dryad/runtime/bytecode_builder.h"
#include "dryad/runtime/intrinsics_registry.h"

using namespace dryad;

class VMTest : public ::testing::Test {
protected:
    VM vm;

    void SetUp() override {
        IntrinsicsRegistry::instance().register_all();
        vm.reset();
    }
};

TEST_F(VMTest, StackOperations) {
    BytecodeBuilder builder;
    builder.emit_push_int(42);
    builder.emit(Opcode::DUP);
    builder.emit(Opcode::POP);
    builder.emit_halt();
    vm.load(builder.build());
    vm.execute();

    EXPECT_EQ(vm.stack_size(), 1u);
    EXPECT_TRUE(vm.get_stack_top().is_integer());
    EXPECT_EQ(vm.get_stack_top().as_integer(), 42);
}

TEST_F(VMTest, PushFloatAndAddF64) {
    BytecodeBuilder builder;
    builder.emit_push_float(1.5);
    builder.emit_push_float(2.5);
    builder.emit(Opcode::ADD_F64);
    builder.emit_halt();
    vm.load(builder.build());
    vm.execute();

    EXPECT_DOUBLE_EQ(vm.get_stack_top().as_float(), 4.0);
}

TEST_F(VMTest, AddI32Specialized) {
    BytecodeBuilder builder;
    builder.emit_push_int(10);
    builder.emit_push_int(32);
    builder.emit(Opcode::ADD_I32);
    builder.emit_halt();
    vm.load(builder.build());
    vm.execute();

    EXPECT_EQ(vm.get_stack_top().as_integer(), 42);
}

TEST_F(VMTest, FastLocalsLoop) {
    BytecodeBuilder builder;

    builder.emit_push_int(0);
    builder.emit_store_fast(0);
    builder.emit_push_int(0);
    builder.emit_store_fast(1);

    const size_t loop_start = builder.current_pc();
    builder.emit_load_fast(1);
    builder.emit_push_int(3);
    builder.emit(Opcode::LT);
    const size_t jump_false_pc = builder.current_pc();
    builder.emit_jump(Opcode::JUMP_IF_FALSE, 0);

    builder.emit_load_fast(0);
    builder.emit_load_fast(1);
    builder.emit(Opcode::ADD_I32);
    builder.emit_store_fast(0);

    builder.emit_load_fast(1);
    builder.emit_push_int(1);
    builder.emit(Opcode::ADD_I32);
    builder.emit_store_fast(1);

    builder.emit_jump(Opcode::JUMP, loop_start);
    const size_t end_pc = builder.current_pc();
    builder.emit_halt();

    BytecodeModule module = builder.build();
    module.code[jump_false_pc + 1] = static_cast<uint8_t>(end_pc & 0xFF);
    module.code[jump_false_pc + 2] = static_cast<uint8_t>((end_pc >> 8) & 0xFF);

    vm.load(module);
    vm.execute();

    EXPECT_EQ(vm.slots().size(), 2u);
    EXPECT_EQ(vm.slots()[0].as_integer(), 3);
    EXPECT_EQ(vm.slots()[1].as_integer(), 3);
}

TEST_F(VMTest, GetElementF64) {
    Value arr = Value::create_array();
    arr.array_push(Value(1.5));
    arr.array_push(Value(2.5));

    BytecodeBuilder builder;
    uint16_t arr_idx = builder.add_constant(arr);
    builder.emit_push_const(arr_idx);
    builder.emit_push_int(1);
    builder.emit(Opcode::GET_ELEMENT_F64);
    builder.emit_halt();

    vm.load(builder.build());
    vm.execute();

    EXPECT_DOUBLE_EQ(vm.get_stack_top().as_float(), 2.5);
}

TEST_F(VMTest, IntrinsicSyscallTime) {
    auto& registry = IntrinsicsRegistry::instance();
    const uint16_t time_id = registry.get_id("syscall.time");

    BytecodeBuilder builder;
    builder.emit_intrinsic_syscall(time_id, 0);
    builder.emit_halt();
    vm.load(builder.build());
    vm.execute();

    EXPECT_EQ(vm.stack_size(), 1u);
    EXPECT_TRUE(vm.get_stack_top().is_integer());
    EXPECT_GT(vm.get_stack_top().as_integer(), 0);
}

TEST_F(VMTest, IntrinsicSyscallStackUnderflow) {
    auto& registry = IntrinsicsRegistry::instance();
    const uint16_t read_id = registry.get_id("syscall.read");

    BytecodeBuilder builder;
    builder.emit_intrinsic_syscall(read_id, 2);
    builder.emit_halt();
    vm.load(builder.build());

    EXPECT_THROW(vm.execute(), DryadException);
}

TEST_F(VMTest, IntrinsicSyscallUnknownId) {
    BytecodeBuilder builder;
    builder.emit_intrinsic_syscall(9999, 0);
    builder.emit_halt();
    vm.load(builder.build());

    EXPECT_THROW(vm.execute(), DryadException);
}

TEST_F(VMTest, SequentialIntrinsicCalls) {
    auto& registry = IntrinsicsRegistry::instance();
    const uint16_t time_id = registry.get_id("syscall.time");

    BytecodeBuilder builder;
    builder.emit_intrinsic_syscall(time_id, 0);
    builder.emit_intrinsic_syscall(time_id, 0);
    builder.emit_halt();
    vm.load(builder.build());
    vm.execute();

    EXPECT_EQ(vm.stack_size(), 2u);
}

TEST_F(VMTest, GenericAddStringConcat) {
    BytecodeBuilder builder;
    uint16_t a = builder.add_constant(Value("hello"));
    uint16_t b = builder.add_constant(Value(" world"));
    builder.emit_push_const(a);
    builder.emit_push_const(b);
    builder.emit(Opcode::ADD);
    builder.emit_halt();
    vm.load(builder.build());
    vm.execute();

    EXPECT_EQ(vm.get_stack_top().as_string(), "hello world");
}
