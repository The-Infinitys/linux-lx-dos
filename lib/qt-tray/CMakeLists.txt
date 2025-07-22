cmake_minimum_required(VERSION 3.16)

project(qt-tray VERSION 0.1 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

option(ENABLE_LINTER "Enable clang-tidy linter" ON)
option(ENABLE_FORMATTER "Enable clang-format formatter" ON)

set(CMAKE_AUTOMOC ON)

if(ENABLE_LINTER)
    find_program(CLANG_TIDY clang-tidy)
    if(CLANG_TIDY)
        set(CMAKE_CXX_CLANG_TIDY "${CLANG_TIDY}" PARENT_SCOPE)
        message(STATUS "Clang-tidy enabled: ${CLANG_TIDY}")
    else()
        message(WARNING "Clang-tidy not found. Linter will not be used.")
    endif()
endif()

if(ENABLE_FORMATTER)
    find_program(CLANG_FORMAT clang-format)
    if(CLANG_FORMAT)
        set(CMAKE_CXX_CLANG_FORMAT "${CLANG_FORMAT}" PARENT_SCOPE)
        set(CMAKE_CXX_CLANG_FORMAT_ARGS "-style=file" PARENT_SCOPE)
        message(STATUS "Clang-format enabled: ${CLANG_FORMAT}")
    else()
        message(WARNING "Clang-format not found. Formatter will not be used.")
    endif()
endif()

find_package(Qt6 REQUIRED COMPONENTS Widgets)
qt_standard_project_setup()

add_library(qt-tray STATIC
        src/lib.cpp
)
target_include_directories(qt-tray PUBLIC src)
target_link_libraries(qt-tray PRIVATE Qt${QT_VERSION_MAJOR}::Widgets)

include(GNUInstallDirs)
install(TARGETS qt-tray
    LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
    ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR}
)
