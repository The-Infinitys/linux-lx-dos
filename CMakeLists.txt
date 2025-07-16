cmake_minimum_required(VERSION 3.16)

project(linux_lx_dos VERSION 0.1 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(Qt6 REQUIRED COMPONENTS Widgets)
qt_standard_project_setup()

qt_add_executable(linux_lx_dos
        src/main.cpp
        ui/mainwindow.cpp
        ui/mainwindow.h
        ui/mainwindow.ui
)
target_link_libraries(linux_lx_dos PRIVATE Qt${QT_VERSION_MAJOR}::Widgets)
find_package(Qt6 COMPONENTS Widgets REQUIRED)
target_link_libraries(linux_lx_dos PRIVATE Qt6::Widgets)
set_target_properties(linux_lx_dos PROPERTIES
    MACOSX_BUNDLE_BUNDLE_VERSION ${PROJECT_VERSION}
    MACOSX_BUNDLE_SHORT_VERSION_STRING ${PROJECT_VERSION_MAJOR}.${PROJECT_VERSION_MINOR}
    MACOSX_BUNDLE TRUE
    WIN32_EXECUTABLE TRUE
)

include(GNUInstallDirs)
install(TARGETS linux_lx_dos
    BUNDLE DESTINATION .
    LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
    RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
)
