cmake_minimum_required(VERSION 3.16)

project(qt-tray VERSION 0.1 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(Qt6 REQUIRED COMPONENTS Widgets)
qt_standard_project_setup()

add_library(qt-tray STATIC
        src/lib.cpp
        src/modules/tray.cpp
)
target_include_directories(qt-tray PUBLIC src)
target_link_libraries(qt-tray PRIVATE Qt${QT_VERSION_MAJOR}::Widgets)

include(GNUInstallDirs)
install(TARGETS qt-tray
    LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
    ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR}
)
