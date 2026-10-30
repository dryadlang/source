#include <gtest/gtest.h>

TEST(Sample, AlwaysPasses) {
    EXPECT_TRUE(true);
}

TEST(Sample, StringComparison) {
    std::string expected = "hello";
    std::string actual = "hello";
    EXPECT_EQ(expected, actual);
}
