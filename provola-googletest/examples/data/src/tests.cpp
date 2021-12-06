#include <gtest/gtest.h>

TEST(Foo, Foo1) {
  ASSERT_FALSE(true);
}

TEST(Foo, Foo2) {
  ASSERT_TRUE(true);
}

TEST(Bar, Bar1) {
  ASSERT_TRUE(true);
}

TEST(Bar, Bar2) {
  ASSERT_TRUE(true);
}
