cmake_minimum_required(VERSION 3.0)
project(LibHello C)

add_library(hello STATIC hello.c)

install(TARGETS hello DESTINATION .)