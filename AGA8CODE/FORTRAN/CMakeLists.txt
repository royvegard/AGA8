cmake_minimum_required (VERSION 3.12)
project (AGA8
    VERSION 0.1.0
    LANGUAGES Fortran)

add_executable(detailtest DETAILTEST.FOR DETAIL.FOR)
add_executable(gergtest GERGTEST.FOR GERG2008.FOR)
add_executable(grosstest GROSSTEST.FOR GROSS.FOR)

enable_testing()

add_test(NAME Detail COMMAND detailtest)
add_test(NAME Gerg COMMAND gergtest)
add_test(NAME Gross COMMAND grosstest)
