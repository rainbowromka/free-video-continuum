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

# FreeType — отключаем лишнее
set(FT_DISABLE_HARFBUZZ ON CACHE BOOL "" FORCE)
set(FT_DISABLE_BZIP2   ON CACHE BOOL "" FORCE)
set(FT_DISABLE_PNG     ON CACHE BOOL "" FORCE)
set(FT_DISABLE_BROTLI  ON CACHE BOOL "" FORCE)
set(FT_DISABLE_ZLIB    ON CACHE BOOL "" FORCE)

FetchContent_Declare(
    freetype
    GIT_REPOSITORY https://github.com/freetype/freetype.git
    GIT_TAG VER-2-13-3
)
FetchContent_MakeAvailable(freetype)

# glad (локально)
add_library(glad STATIC
    third_party/glad/src/gl.c
)
target_include_directories(glad PUBLIC
    third_party/glad/include
)

# OpenGL
set(OpenGL_GL_PREFERENCE GLVND)
find_package(OpenGL REQUIRED)

# Все .cpp из src/
file(GLOB_RECURSE SOURCES "src/*.cpp")

add_executable(continuum_client ${SOURCES})

# Заголовки — в src/
target_include_directories(continuum_client PRIVATE src)

target_link_libraries(continuum_client
    glfw
    glad
    OpenGL::GL
    freetype
)

# Windows — дополнительно opengl32
if (WIN32)
    target_link_libraries(continuum_client opengl32)
endif()