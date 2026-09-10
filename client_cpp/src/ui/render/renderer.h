#pragma once

#include <cstdint>

class Renderer {
public:
    Renderer();
    ~Renderer();

    bool init();

    // Шейдерные программы
    unsigned int rectProgram() const { return rect_program_; }
    unsigned int textureProgram() const { return texture_program_; }

    // VAO/VBO для полноэкранного квада (в FBO)
    unsigned int quadVao() const { return quad_vao_; }
    unsigned int quadVbo() const { return quad_vbo_; }

    void clear(float r, float g, float b);

private:
    unsigned int rect_program_ = 0;
    unsigned int texture_program_ = 0;
    unsigned int quad_vao_ = 0;
    unsigned int quad_vbo_ = 0;

    unsigned int compileShader(unsigned int type, const char* source);
    unsigned int linkProgram(unsigned int vs, unsigned int fs);
    bool initQuad();
    bool initPrograms();
};