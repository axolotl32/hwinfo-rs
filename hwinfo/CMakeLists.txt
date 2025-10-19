cmake_minimum_required(VERSION 3.22)

project(hwinfo_c_static CXX C)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(COMMON_SOURCES
        src/battery.cpp
        src/cpu.cpp
        src/disk.cpp
        src/gpu.cpp
        src/mainboard.cpp
        src/network.cpp
        src/os.cpp
        src/ram.cpp
        src/hwinfo.cpp
        src/hwinfo_c.cpp 
)

if(WIN32)
    message(STATUS "Configuring for Windows")
    list(APPEND PLATFORM_SOURCES
            src/windows/battery.cpp
            src/windows/cpu.cpp
            src/windows/disk.cpp
            src/windows/gpu.cpp
            src/windows/mainboard.cpp
            src/windows/network.cpp
            src/windows/os.cpp
            src/windows/ram.cpp
            src/windows/utils/filesystem.cpp
            src/windows/utils/wmi_wrapper.cpp
    )
elseif(APPLE)
    message(STATUS "Configuring for Apple/macOS")
    list(APPEND PLATFORM_SOURCES
            src/apple/battery.cpp
            src/apple/cpu.cpp
            src/apple/disk.cpp
            src/apple/gpu.cpp
            src/apple/mainboard.cpp
            src/apple/network.cpp
            src/apple/os.cpp
            src/apple/ram.cpp
            src/apple/utils/filesystem.cpp
            src/PCIMapper.cpp # PCIMapper is used on UNIX-like systems
    )
elseif(UNIX)
    message(STATUS "Configuring for Linux")
    list(APPEND PLATFORM_SOURCES
            src/linux/battery.cpp
            src/linux/cpu.cpp
            src/linux/disk.cpp
            src/linux/gpu.cpp
            src/linux/mainboard.cpp
            src/linux/network.cpp
            src/linux/os.cpp
            src/linux/ram.cpp
            src/linux/utils/filesystem.cpp
            src/PCIMapper.cpp # PCIMapper is used on UNIX-like systems
    )
endif()

add_library(hwinfo_static STATIC
        ${COMMON_SOURCES}
        ${PLATFORM_SOURCES}
)

set_target_properties(hwinfo_static PROPERTIES OUTPUT_NAME "hwinfo")

target_include_directories(hwinfo_static PUBLIC
        "${CMAKE_CURRENT_SOURCE_DIR}/include"
)

if(WIN32)
    target_compile_definitions(hwinfo_static PRIVATE -DWIN32)
    target_link_libraries(hwinfo_static PRIVATE wbemuuid.lib ole32.lib oleaut32.lib)
elseif(APPLE)
    target_link_libraries(hwinfo_static PRIVATE "-framework IOKit" "-framework CoreFoundation")
endif()

message(STATUS "Configuration complete. The static library 'hwinfo' will be built.")

install(
    TARGETS hwinfo_static
    ARCHIVE DESTINATION lib
)

install(
    FILES include/hwinfo/hwinfo_c.h
    DESTINATION include/hwinfo
)