add_subdirectory(googletest)

include(GoogleTest)

cmake_minimum_required(VERSION 3.13)

project(provola-googletest-example)

set(CMAKE_CXX_STANDARD 17)

add_executable(example
    src/tests.cpp
)

target_link_libraries(example
    gtest
    gtest_main
)

target_include_directories(example
    PRIVATE ${PROJECT_SOURCE_DIR}/lib
)

gtest_discover_tests(example
    WORKING_DIRECTORY ${PROJECT_DIR}
    PROPERTIES VS_DEBUGGER_WORKING_DIRECTORY "${PROJECT_DIR}"
)

set_target_properties(example
    PROPERTIES FOLDER tests
)

enable_testing()
