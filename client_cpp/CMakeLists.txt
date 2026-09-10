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

# glad (локально)
add_library(glad STATIC
    third_party/glad/src/gl.c
)
target_include_directories(glad PUBLIC
    third_party/glad/include/
)

# OpenGL
set(OpenGL_GL_PREFERENCE GLVND)
find_package(OpenGL REQUIRED)

# Все .cpp файлы в src/ рекурсивно
file(GLOB_RECURSE SOURCES
    "src/*.cpp"
)

add_executable(continuum_client ${SOURCES})

# Заголовки — в src/
target_include_directories(continuum_client PRIVATE src)

target_link_libraries(continuum_client
    glfw
    glad
    OpenGL::GL
)

# Windows — дополнительно opengl32
if (WIN32)
    target_link_libraries(continuum_client opengl32)
endif()