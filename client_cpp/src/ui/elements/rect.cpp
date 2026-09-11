#include "ui/elements/rect.h"
#include "ui/render/renderer.h"
#include <iostream>
#include <glad/gl.h>

Rect& Rect::setRect(int x, int y, int width, int height) {
    if (x_ != x || y_ != y || width_ != width || height_ != height) {
        x_ = x;
        y_ = y;
        width_ = width;
        height_ = height;
        dirty_ = true;
    }
    return *this;
}

Rect& Rect::setColor(uint8_t r, uint8_t g, uint8_t b) {
    if (bg_r_ != r || bg_g_ != g || bg_b_ != b) {
        bg_r_ = r;
        bg_g_ = g;
        bg_b_ = b;
        dirty_ = true;
    }
    return *this;
}

bool Rect::createTextures() {
    lazyInit();    
    bool was_dirty = dirty_;
    std::cout << "[Rect ENTER] dirty=" << dirty_ 
              << " " << width_ << "x" << height_ << std::endl;


    for (auto& child : children_) {
        was_dirty |= child->createTextures();
    }

    if (was_dirty) {
        glBindFramebuffer(GL_FRAMEBUFFER, fbo_);
        glViewport(0, 0, width_, height_);
        glClearColor(bg_r_ / 255.0f, bg_g_ / 255.0f, bg_b_ / 255.0f, 1.0f);
        glClear(GL_COLOR_BUFFER_BIT);

        glClear(GL_COLOR_BUFFER_BIT);
        GLenum err = glGetError();
        if (err != GL_NO_ERROR) {
            std::cerr << "[GL] error: " << err << std::endl;
        }

        for (auto& child : children_) {
            child->draw(width_, height_);
        }

        glBindFramebuffer(GL_FRAMEBUFFER, 0);
    }
    
    dirty_ = false;
    std::cout << "[Rect EXIT] was_dirty=" << was_dirty << std::endl;
    return was_dirty;
}

void Rect::draw(int parent_w, int parent_h) {
    std::cout << "[Rect] draw " << width_ << "x" << height_ << " parent=" << parent_w << "x" << parent_h << std::endl;
    lazyInit();

    // обновить квад
    float left = (x_ / (float)parent_w) * 2.0f - 1.0f;
    float right = ((x_ + width_) / (float)parent_w) * 2.0f - 1.0f;
    float top = 1.0f - (y_ / (float)parent_h) * 2.0f;
    float bottom = 1.0f - ((y_ + height_) / (float)parent_h) * 2.0f;

    float vertices[] = {
        left, top, 0.0f, 1.0f,
        left, bottom, 0.0f, 0.0f,
        right, bottom, 1.0f, 0.0f,
        left, top, 0.0f, 1.0f,
        right, bottom, 1.0f, 0.0f,
        right, top, 1.0f, 1.0f,
    };

    glBindVertexArray(quad_vao_);
    glBindBuffer(GL_ARRAY_BUFFER, quad_vbo_);
    glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_STATIC_DRAW);

    glActiveTexture(GL_TEXTURE0);
    glBindTexture(GL_TEXTURE_2D, texture_);
    glUseProgram(Renderer::instance().textureProgram());
    glBindVertexArray(quad_vao_);
    glDrawArrays(GL_TRIANGLES, 0, 6);
    glBindVertexArray(0);
}