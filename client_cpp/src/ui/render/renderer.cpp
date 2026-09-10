#include "ui/render/renderer.h"
#include <glad/gl.h>
#include <GLFW/glfw3.h>
#include <iostream>

static const char* RECT_VS = R"(
#version 330 core
layout (location = 0) in vec2 position;
void main() {
    gl_Position = vec4(position, 0.0, 1.0);
}
)";

static const char* RECT_FS = R"(
#version 330 core
uniform vec3 rect_color;
out vec4 FragColor;
void main() {
    FragColor = vec4(rect_color, 1.0);
}
)";

static const char* TEX_VS = R"(
#version 330 core
layout (location = 0) in vec2 position;
layout (location = 1) in vec2 texcoord;
out vec2 v_texcoord;
void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    v_texcoord = texcoord;
}
)";

static const char* TEX_FS = R"(
#version 330 core
uniform sampler2D tex;
in vec2 v_texcoord;
out vec4 FragColor;
void main() {
    FragColor = texture(tex, v_texcoord);
}
)";

Renderer::Renderer() {}
Renderer::~Renderer() {}

bool Renderer::init() {
    if (!initQuad()) return false;
    if (!initPrograms()) return false;
    return true;
}

void Renderer::clear(float r, float g, float b) {
    glClearColor(r, g, b, 1.0f);
    glClear(GL_COLOR_BUFFER_BIT);
}

unsigned int Renderer::compileShader(unsigned int type, const char* source) {
    unsigned int shader = glCreateShader(type);
    glShaderSource(shader, 1, &source, nullptr);
    glCompileShader(shader);

    int success;
    glGetShaderiv(shader, GL_COMPILE_STATUS, &success);
    if (!success) {
        char log[512];
        glGetShaderInfoLog(shader, 512, nullptr, log);
        std::cerr << "Shader error: " << log << std::endl;
        return 0;
    }
    return shader;
}

unsigned int Renderer::linkProgram(unsigned int vs, unsigned int fs) {
    unsigned int program = glCreateProgram();
    glAttachShader(program, vs);
    glAttachShader(program, fs);
    glLinkProgram(program);

    int success;
    glGetProgramiv(program, GL_LINK_STATUS, &success);
    if (!success) {
        char log[512];
        glGetProgramInfoLog(program, 512, nullptr, log);
        std::cerr << "Link error: " << log << std::endl;
        return 0;
    }

    glDeleteShader(vs);
    glDeleteShader(fs);
    return program;
}

bool Renderer::initPrograms() {
    unsigned int rect_vs = compileShader(GL_VERTEX_SHADER, RECT_VS);
    unsigned int rect_fs = compileShader(GL_FRAGMENT_SHADER, RECT_FS);
    rect_program_ = linkProgram(rect_vs, rect_fs);

    unsigned int tex_vs = compileShader(GL_VERTEX_SHADER, TEX_VS);
    unsigned int tex_fs = compileShader(GL_FRAGMENT_SHADER, TEX_FS);
    texture_program_ = linkProgram(tex_vs, tex_fs);

    return rect_program_ != 0 && texture_program_ != 0;
}

bool Renderer::initQuad() {
    // Полноэкранный квад -1..1 с UV
    float vertices[] = {
        -1.0f,  1.0f, 0.0f, 0.0f,
        -1.0f, -1.0f, 0.0f, 1.0f,
         1.0f, -1.0f, 1.0f, 1.0f,
        -1.0f,  1.0f, 0.0f, 0.0f,
         1.0f, -1.0f, 1.0f, 1.0f,
         1.0f,  1.0f, 1.0f, 0.0f,
    };

    glGenVertexArrays(1, &quad_vao_);
    glGenBuffers(1, &quad_vbo_);

    glBindVertexArray(quad_vao_);
    glBindBuffer(GL_ARRAY_BUFFER, quad_vbo_);
    glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_STATIC_DRAW);

    glVertexAttribPointer(0, 2, GL_FLOAT, GL_FALSE, 4 * sizeof(float), (void*)0);
    glEnableVertexAttribArray(0);

    glVertexAttribPointer(1, 2, GL_FLOAT, GL_FALSE, 4 * sizeof(float), (void*)(2 * sizeof(float)));
    glEnableVertexAttribArray(1);

    glBindVertexArray(0);
    return true;
}