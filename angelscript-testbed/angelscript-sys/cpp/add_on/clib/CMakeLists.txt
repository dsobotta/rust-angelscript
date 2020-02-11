cmake_minimum_required(VERSION 3.0)
project(libangelscript_c CXX)

include_directories(../../angelscript/include)

add_library(angelscript_c STATIC as_c.cpp)

install(TARGETS angelscript_c DESTINATION .)
