cmake_minimum_required (VERSION 2.6)
project (leap_motion_wrapper)

find_package (PkgConfig REQUIRED)
pkg_search_module (LEAP REQUIRED libleap)

set (CMAKE_POSITION_INDEPENDENT_CODE ON)

add_library (leap_motion_wrapper STATIC
    controller.cpp
    finger.cpp
    frame.cpp
    pointable.cpp
    vector.cpp
    interaction_box.cpp
    hand.cpp
    listener.cpp
    device.cpp
)

target_link_libraries(leap_motion_wrapper ${LEAP_LIBRARIES})
target_include_directories(leap_motion_wrapper PUBLIC ${LEAP_INCLUDE_DIRS})
target_compile_options(leap_motion_wrapper PUBLIC ${LEAP_CFLAGS_OTHER})

install (TARGETS leap_motion_wrapper ARCHIVE DESTINATION .)
