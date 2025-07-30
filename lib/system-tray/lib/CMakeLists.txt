cmake_minimum_required(VERSION 3.16)
project(SystemTray)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(Qt6 COMPONENTS Widgets REQUIRED)

add_library(SystemTray STATIC src/lib.cpp)

target_include_directories(SystemTray PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/src)

target_link_libraries(SystemTray PRIVATE Qt6::Widgets)

install(TARGETS SystemTray DESTINATION lib)
