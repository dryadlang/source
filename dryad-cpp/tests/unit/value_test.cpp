#include <gtest/gtest.h>
#include "dryad/runtime/value.h"
#include "dryad/common/utils.h"

using namespace dryad;

TEST(ValueTest, DefaultConstructorCreatesNull) {
    Value v;
    EXPECT_TRUE(v.is_null());
    EXPECT_EQ(v.type(), ValueType::Null);
}

TEST(ValueTest, BooleanConstructor) {
    Value v_true(true);
    Value v_false(false);
    
    EXPECT_TRUE(v_true.is_boolean());
    EXPECT_TRUE(v_true.as_boolean());
    EXPECT_FALSE(v_false.as_boolean());
}

TEST(ValueTest, IntegerConstructor) {
    Value v(static_cast<int64_t>(42));
    
    EXPECT_TRUE(v.is_integer());
    EXPECT_EQ(v.as_integer(), 42);
}

TEST(ValueTest, FloatConstructor) {
    Value v(3.14);
    
    EXPECT_TRUE(v.is_float());
    EXPECT_DOUBLE_EQ(v.as_float(), 3.14);
}

TEST(ValueTest, StringConstructor) {
    Value v1("hello");
    Value v2(std::string("world"));
    
    EXPECT_TRUE(v1.is_string());
    EXPECT_TRUE(v2.is_string());
    EXPECT_EQ(v1.as_string(), "hello");
    EXPECT_EQ(v2.as_string(), "world");
}

TEST(ValueTest, CopyConstructor) {
    Value original(static_cast<int64_t>(42));
    Value copy(original);
    
    EXPECT_TRUE(copy.is_integer());
    EXPECT_EQ(copy.as_integer(), 42);
}

TEST(ValueTest, MoveConstructor) {
    Value original(std::string("test"));
    Value moved(std::move(original));
    
    EXPECT_TRUE(moved.is_string());
    EXPECT_EQ(moved.as_string(), "test");
    EXPECT_TRUE(original.is_null());
}

TEST(ValueTest, CopyAssignment) {
    Value original(static_cast<int64_t>(42));
    Value copy;
    copy = original;
    
    EXPECT_TRUE(copy.is_integer());
    EXPECT_EQ(copy.as_integer(), 42);
}

TEST(ValueTest, MoveAssignment) {
    Value original(std::string("test"));
    Value moved;
    moved = std::move(original);
    
    EXPECT_TRUE(moved.is_string());
    EXPECT_EQ(moved.as_string(), "test");
    EXPECT_TRUE(original.is_null());
}

TEST(ValueTest, IsTruthy) {
    EXPECT_FALSE(Value().is_truthy());
    EXPECT_FALSE(Value(false).is_truthy());
    EXPECT_TRUE(Value(true).is_truthy());
    EXPECT_FALSE(Value(static_cast<int64_t>(0)).is_truthy());
    EXPECT_TRUE(Value(static_cast<int64_t>(1)).is_truthy());
    EXPECT_FALSE(Value(0.0).is_truthy());
    EXPECT_TRUE(Value(1.0).is_truthy());
    EXPECT_FALSE(Value("").is_truthy());
    EXPECT_TRUE(Value("hello").is_truthy());
}

TEST(ValueTest, ToString) {
    EXPECT_EQ(Value().to_string(), "null");
    EXPECT_EQ(Value(true).to_string(), "true");
    EXPECT_EQ(Value(false).to_string(), "false");
    EXPECT_EQ(Value(static_cast<int64_t>(42)).to_string(), "42");
    EXPECT_EQ(Value("hello").to_string(), "hello");
}

TEST(ValueTest, TypeChecking) {
    Value null_val;
    Value bool_val(true);
    Value int_val(static_cast<int64_t>(42));
    Value float_val(3.14);
    Value str_val("hello");
    
    EXPECT_TRUE(null_val.is_null());
    EXPECT_TRUE(bool_val.is_boolean());
    EXPECT_TRUE(int_val.is_integer());
    EXPECT_TRUE(float_val.is_float());
    EXPECT_TRUE(str_val.is_string());
    
    EXPECT_TRUE(int_val.is_number());
    EXPECT_TRUE(float_val.is_number());
    EXPECT_FALSE(str_val.is_number());
}

TEST(ValueTest, ThrowsOnWrongTypeAccess) {
    Value int_val(static_cast<int64_t>(42));
    
    EXPECT_THROW(int_val.as_boolean(), DryadException);
    EXPECT_THROW(int_val.as_string(), DryadException);
}

TEST(ValueTest, CreateArray) {
    Value arr = Value::create_array();
    EXPECT_TRUE(arr.is_array());
}

TEST(ValueTest, CreateObject) {
    Value obj = Value::create_object();
    EXPECT_TRUE(obj.is_object());
}
