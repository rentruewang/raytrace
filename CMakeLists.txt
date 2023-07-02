cmake_minimum_required(VERSION 3.10)
project(raytrace)

set(CMAKE_CXX_STANDARD 20)

find_package(fmt CONFIG REQUIRED)

file(GLOB files RELATIVE ${CMAKE_CURRENT_SOURCE_DIR} "*.cxx")
add_executable(raytrace ${files})

target_include_directories(raytrace PRIVATE "include" "templates")
add_subdirectory("src")

target_link_libraries(raytrace PRIVATE fmt::fmt)
