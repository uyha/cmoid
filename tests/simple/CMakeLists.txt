cmake_minimum_required(VERSION 3.21)
project(simple VERSION 0.1.0 LANGUAGES CXX)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/cmake")
set(CMAKE_SYSROOT_LINK a)

add_library(lib main.cpp)
target_link_libraries(lib PRIVATE m)
set_property(
  TARGET lib
  PROPERTY INTERPROCEDURAL_OPTIMIZATION TRUE
)

add_executable(main main.cpp)
target_link_libraries(main PRIVATE m lib)
target_compile_features(main PRIVATE cxx_std_20)
target_include_directories(main PRIVATE include)
target_compile_definitions(main PRIVATE STUFF=1)
target_precompile_headers(main
  PRIVATE
    <iostream>
)
set_property(
  TARGET main
  PROPERTY INTERPROCEDURAL_OPTIMIZATION TRUE
)
