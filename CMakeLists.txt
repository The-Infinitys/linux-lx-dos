cmake_minimum_required(VERSION 3.16)

project(linux_lx_dos VERSION 0.1 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(Qt6 REQUIRED COMPONENTS Widgets)
qt_standard_project_setup()

# --- Rust Backend Integration --- 
include(ExternalProject)

set(RUST_BACKEND_DIR ${CMAKE_CURRENT_SOURCE_DIR}/rust_backend)
set(RUST_TARGET_DIR ${RUST_BACKEND_DIR}/target)

ExternalProject_Add(
    rust_backend_build
    SOURCE_DIR ${RUST_BACKEND_DIR}
    # Disable configure and install steps
    CONFIGURE_COMMAND ""
    INSTALL_COMMAND ""
    # Use cargo to build the rust library
    BUILD_COMMAND cargo build --release
    # Build in the source directory
    BUILD_IN_SOURCE 1
    # Define build output
    BYPRODUCTS ${RUST_TARGET_DIR}/release/librust_backend.a
)

# Location of the compiled static rust library
set(RUST_BACKEND_LIB ${RUST_TARGET_DIR}/release/librust_backend.a)

# --- C++/Qt Frontend --- 
qt_add_executable(linux_lx_dos
        src/main.cpp
        ui/mainwindow.cpp
        ui/mainwindow.h
        ui/mainwindow.ui
        ui/welcomedialog.cpp
        ui/welcomedialog.h
        ui/welcomedialog.ui
)

# Add dependency to ensure rust lib is built before C++ app
add_dependencies(linux_lx_dos rust_backend_build)

# Link the C++ application with the Rust library
target_link_libraries(linux_lx_dos 
    PRIVATE Qt${QT_VERSION_MAJOR}::Widgets
    PRIVATE ${RUST_BACKEND_LIB}
    PRIVATE Threads::Threads # For std::thread
    PRIVATE -ldl # Rust often needs the dynamic linking loader library
)

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