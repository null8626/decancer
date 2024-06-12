cmake_minimum_required(VERSION 3.8.2)

project(
  decancer_native_test
  LANGUAGES CXX
  HOMEPAGE_URL "https://github.com/null8626/decancer"
  DESCRIPTION "Test suite for the native binding of the decancer library."
)

set(CMAKE_BUILD_TYPE Debug)

file(GLOB DECANCER_NATIVE_TEST_SOURCE_C_FILES *.c)
file(GLOB DECANCER_NATIVE_TEST_SOURCE_CXX_FILES *.cpp)

add_executable(decancer_native_test ${DECANCER_NATIVE_TEST_SOURCE_C_FILES} ${DECANCER_NATIVE_TEST_SOURCE_CXX_FILES})

set_target_properties(decancer_native_test PROPERTIES
  OUTPUT_NAME           decancer_native_test
  CXX_STANDARD          17
  CXX_STANDARD_REQUIRED ON
)

find_library(
  DECANCER_LIBRARY NAMES "decancer.lib" "libdecancer.a" HINTS
  "${CMAKE_SOURCE_DIR}/.."
  "${CMAKE_SOURCE_DIR}/../target/release"
  "${CMAKE_SOURCE_DIR}/../target/debug"
  REQUIRED
)

find_library(
  DECANCER_CXX_LIBRARY NAMES "decancer_cxx.lib" "libdecancer_cxx.so" HINTS
  "${CMAKE_SOURCE_DIR}/../build"
  "${CMAKE_SOURCE_DIR}/../build/Release"
  "${CMAKE_SOURCE_DIR}/../build/Debug"
  REQUIRED
)

target_compile_definitions(decancer_native_test PRIVATE __DECANCER_TEST__)
target_include_directories(decancer_native_test PUBLIC "${CMAKE_SOURCE_DIR}/..")
target_link_libraries(decancer_native_test ${DECANCER_LIBRARY} ${DECANCER_CXX_LIBRARY})

add_custom_command(
  TARGET decancer_native_test
  POST_BUILD
  COMMAND decancer_native_test
)