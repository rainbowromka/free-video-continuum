#pragma once

class Renderer {
public:
    static Renderer& instance();

    bool init();

    unsigned int rectProgram() const { return rect_program_; }
    unsigned int textureProgram() const { return texture_program_; }

    void clear(float r, float g, float b);

private:
    Renderer() = default;
    ~Renderer() = default;
    Renderer(const Renderer&) = delete;
    Renderer& operator=(const Renderer&) = delete;

    unsigned int rect_program_ = 0;
    unsigned int texture_program_ = 0;

    unsigned int compileShader(unsigned int type, const char* source);
    unsigned int linkProgram(unsigned int vs, unsigned int fs);
    bool initPrograms();
};