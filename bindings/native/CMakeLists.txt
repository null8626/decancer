cmake_minimum_required(VERSION 3.8.2)

project(
  decancer
  LANGUAGES CXX
  HOMEPAGE_URL "https://github.com/null8626/decancer"
  DESCRIPTION "A library that removes common unicode confusables/homoglyphs from strings."
)

set(CMAKE_BUILD_TYPE Release)
set(CMAKE_WINDOWS_EXPORT_ALL_SYMBOLS ON)

add_library(decancer SHARED decancer.cpp)

set_target_properties(decancer PROPERTIES
  OUTPUT_NAME           decancer_cxx
  CXX_STANDARD          17
  CXX_STANDARD_REQUIRED ON
)

if(MSVC)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} /O2 /Oi /Oy /Gy")
else()
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -O3 -Wall -Wextra -Wpedantic -Wformat=2 -Wnull-dereference -Wuninitialized -Wdeprecated")
endif()

if(NOT EXISTS "${CMAKE_SOURCE_DIR}/target/release/decancer.lib"
   AND NOT EXISTS "${CMAKE_SOURCE_DIR}/target/release/libdecancer.a")
execute_process(COMMAND cargo build "--release" WORKING_DIRECTORY ${CMAKE_SOURCE_DIR})
endif()

find_library(DECANCER_LIBRARY NAMES "decancer.lib" "libdecancer.a" HINTS "${CMAKE_SOURCE_DIR}/target/release" "${CMAKE_SOURCE_DIR}/target/debug" REQUIRED)

target_compile_definitions(decancer PRIVATE __DECANCER_CXX_BUILDING__)
target_include_directories(decancer PUBLIC ${CMAKE_SOURCE_DIR})
target_link_libraries(decancer ${DECANCER_LIBRARY})