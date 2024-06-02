cmake_minimum_required(VERSION 3.22)
project(trap_swift Swift)

if (CMAKE_SYSTEM_NAME)
    if(CMAKE_SYSTEM_NAME STREQUAL "Darwin")
        if(CMAKE_SYSTEM_PROCESSOR)
            if(CMAKE_SYSTEM_PROCESSOR STREQUAL "arm64")
                set(CMAKE_Swift_FLAGS "${CMAKE_Swift_FLAGS} -target arm64-apple-macosx14")
            elseif(CMAKE_SYSTEM_PROCESSOR STREQUAL "x86_64")
                set(CMAKE_Swift_FLAGS "${CMAKE_Swift_FLAGS} -target x86_64-apple-macosx14")
            endif()
        endif(CMAKE_SYSTEM_PROCESSOR)
    endif()
endif()

add_custom_command(OUTPUT ${CMAKE_CURRENT_BINARY_DIR}/trap_swift.h
    DEPENDS trap_swift
    COMMENT "Generating Swift header"
)
# this is for c++ code to depend on the swift header
add_custom_target(trap_swift_header DEPENDS ${CMAKE_CURRENT_BINARY_DIR}/trap_swift.h)
add_library(trap_swift STATIC rain_collected.swift)
target_compile_options(trap_swift PRIVATE "SHELL:-cxx-interoperability-mode=default -emit-objc-header-path ${CMAKE_CURRENT_BINARY_DIR}/trap_swift.h -module-name trap_swift")
target_include_directories(trap_swift PUBLIC ${CMAKE_CURRENT_BINARY_DIR})
# this is for rust to find the swift library
install(TARGETS trap_swift DESTINATION .)
install(FILES ${CMAKE_CURRENT_BINARY_DIR}/trap_swift.h DESTINATION .)
