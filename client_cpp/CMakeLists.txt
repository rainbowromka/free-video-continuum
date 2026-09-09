cmake_minimum_required(VERSION 3.15)
project(continuum_client_cpp)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# FetchContent для GLFW
include(FetchContent)
FetchContent_Declare(
    glfw
    GIT_REPOSITORY https://github.com/glfw/glfw.git
    GIT_TAG 3.4
)
FetchContent_MakeAvailable(glfw)

# OpenGL
find_package(OpenGL REQUIRED)

add_executable(continuum_client
    src/main.cpp
)

target_link_libraries(continuum_client
    glfw
    OpenGL::GL
)

# Windows — дополнительно opengl32
if (WIN32)
    target_link_libraries(continuum_client opengl32)
endif()