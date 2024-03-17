cmake_minimum_required(VERSION 3.28)
project(trap_swift Swift)
add_library(trap_swift rain_collected.swift)
install(TARGETS trap_swift DESTINATION .)
