cmake_minimum_required(VERSION 3.16)

project(linux-lx-dos VERSION 0.1 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(Qt6 6.4 REQUIRED COMPONENTS Quick)

qt_standard_project_setup()

qt_add_executable(applinux-lx-dos
    main.cpp
)

qt_add_qml_module(applinux-lx-dos
    URI linux-lx-dos
    VERSION 1.0
    QML_FILES
        Main.qml
)

set_target_properties(applinux-lx-dos PROPERTIES
    MACOSX_BUNDLE_BUNDLE_VERSION ${PROJECT_VERSION}
    MACOSX_BUNDLE_SHORT_VERSION_STRING ${PROJECT_VERSION_MAJOR}.${PROJECT_VERSION_MINOR}
    MACOSX_BUNDLE TRUE
    WIN32_EXECUTABLE TRUE
)

target_link_libraries(applinux-lx-dos
    PRIVATE Qt6::Quick
)

include(GNUInstallDirs)
install(TARGETS applinux-lx-dos
    BUNDLE DESTINATION .
    LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
    RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
)
