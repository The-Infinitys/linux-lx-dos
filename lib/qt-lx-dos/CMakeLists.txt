cmake_minimum_required(VERSION 3.16)

project(qt-lx-dos VERSION 0.1 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
find_package(Qt6 REQUIRED COMPONENTS Widgets)
find_package(Qt6 REQUIRED COMPONENTS Core)
qt_standard_project_setup()

add_library(qt-lx-dos STATIC
    ui/main/window.cpp
    ui/main/window.hpp
    ui/main/window.ui
    api/qt-lx-dos.cpp
    api/qt-lx-dos.hpp
    public/icon.svg
)

target_link_libraries(qt-lx-dos PRIVATE Qt${QT_VERSION_MAJOR}::Widgets)
target_link_libraries(qt-lx-dos PRIVATE Qt${QT_VERSION_MAJOR}::Core)

include(GNUInstallDirs)
install(TARGETS qt-lx-dos
    ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR}
    LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
    RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
)
