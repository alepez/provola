cmake_minimum_required(VERSION 3.13)

project(provola-catch2-example)

set(CATCH_BUILD_STATIC_LIBRARY ON CACHE BOOL "Enable catch build static lib")

add_subdirectory(catch2)

add_executable(example
    src/tests.cpp
)

target_link_libraries(example
  PRIVATE Catch2WithMain
)

target_include_directories(example
  PRIVATE catch2/single_include
)
