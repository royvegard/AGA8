cmake_minimum_required (VERSION 3.12)
project (AGA8
    VERSION 0.1.0
    LANGUAGES Fortran)

add_library(detail SHARED DETAIL.FOR)
add_library(gerg SHARED GERG2008.FOR)
add_library(gross SHARED GROSS.FOR)
