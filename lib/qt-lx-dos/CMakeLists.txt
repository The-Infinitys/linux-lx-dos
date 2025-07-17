cmake_minimum_required(VERSION 3.16)

project(qt-lx-dos VERSION 0.1 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
find_package(Qt6 REQUIRED COMPONENTS Widgets)
find_package(Qt6 REQUIRED COMPONENTS Core)
qt_standard_project_setup()

add_library(qt-lx-dos STATIC
    api/qt-lx-dos.cpp
    api/qt-lx-dos.hpp
    src/tray.qrc
    ui/main/main.cpp
    ui/main/main.hpp
    ui/main/main.ui
    ui/task/tray.cpp
    ui/task/tray.hpp
    ui/settings/settings.cpp
    ui/settings/settings.hpp
    ui/settings/settings.ui
    ui/welcome/welcome.cpp
    ui/welcome/welcome.hpp
    ui/welcome/welcome.ui
)

qt_add_resources(RESOURCES src/tray.qrc)

target_link_libraries(qt-lx-dos PRIVATE Qt${QT_VERSION_MAJOR}::Widgets)
target_link_libraries(qt-lx-dos PRIVATE Qt${QT_VERSION_MAJOR}::Core)
target_link_libraries(qt-lx-dos PRIVATE RESOURCES)
include(GNUInstallDirs)
install(TARGETS qt-lx-dos
    ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR}
    LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
    RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
)
