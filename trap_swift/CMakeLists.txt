cmake_minimum_required(VERSION 3.28)
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

add_library(trap_swift STATIC rain_collected.swift)
install(TARGETS trap_swift DESTINATION .)
