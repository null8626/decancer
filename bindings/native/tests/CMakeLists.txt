cmake_minimum_required(VERSION 3.8.2)

project(
  decancer_native_test
  LANGUAGES C
  HOMEPAGE_URL "https://github.com/null8626/decancer"
  DESCRIPTION "Test suite for the native binding of the decancer library."
)

set(CMAKE_BUILD_TYPE Debug)

file(GLOB DECANCER_NATIVE_TEST_SOURCE_FILES *.c)

add_executable(decancer_native_test ${DECANCER_NATIVE_TEST_SOURCE_FILES})

if(WIN32)
find_file(
  DECANCER_DLL
  NAME "decancer.dll"
  HINTS "${CMAKE_SOURCE_DIR}/.." "${CMAKE_SOURCE_DIR}/../target/release" "${CMAKE_SOURCE_DIR}/../target/debug"
  REQUIRED
)
endif()

find_library(
  DECANCER_LIBRARY
  NAMES "decancer.dll.lib" "libdecancer.dylib" "libdecancer.so"
  HINTS "${CMAKE_SOURCE_DIR}/.." "${CMAKE_SOURCE_DIR}/../target/release" "${CMAKE_SOURCE_DIR}/../target/debug"
  REQUIRED
)

target_include_directories(decancer_native_test PUBLIC "${CMAKE_SOURCE_DIR}/..")
target_link_libraries(decancer_native_test ${DECANCER_LIBRARY})

if(WIN32)
add_custom_command(
  TARGET decancer_native_test POST_BUILD
  COMMAND ${CMAKE_COMMAND} -E copy ${DECANCER_DLL} $<TARGET_FILE_DIR:decancer_native_test>
  COMMAND_EXPAND_LISTS
)
endif()

add_custom_command(
  TARGET decancer_native_test POST_BUILD
  COMMAND decancer_native_test
)