cmake_minimum_required(VERSION 3.28)
project(trap CXX)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

add_subdirectory(trap_swift)
add_subdirectory(trap_cpp)
add_subdirectory(cpp)
