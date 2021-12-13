#include <catch2/catch.hpp>

TEST_CASE("Foo") {
  REQUIRE(false);
}

struct Bar { };

TEST_CASE_METHOD(Bar, "Bar1") {
  REQUIRE(true);
}

TEST_CASE_METHOD(Bar, "Bar2") {
  REQUIRE(true);
}

TEST_CASE_METHOD(Bar, "Bar3", "[.disabled]") {
  REQUIRE(true);
}
