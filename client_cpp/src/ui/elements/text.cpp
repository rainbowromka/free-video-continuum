#include "ui/elements/text.h"
#include "ui/font/font_manager.h"
#include "ui/render/renderer.h"
#include <glad/gl.h>
#include <iostream>

Text& Text::setContent(const std::string& content) {
    if (content_ != content) {
        content_ = content;
        dirty_ = true;
    }
    return *this;
}

Text& Text::setTextColor(uint8_t r, uint8_t g, uint8_t b) {
    if (text_r_ != r || text_g_ != g || text_b_ != b) {
        text_r_ = r;
        text_g_ = g;
        text_b_ = b;
        dirty_ = true;
    }
    return *this;
}

Text& Text::setHeight(int height) {
    if (height_ != height) {
        height_ = height;

        FontManager& fm = FontManager::instance();
        int w = 0;
        for (char c : content_) {
            Glyph* g = fm.getGlyph(c);
            if (g) w += g->advance;
        }
        width_ = w;

        recreateFbo();
        dirty_ = true;
    }
    return *this;
}

bool Text::createTextures() {
    lazyInit();

    bool was_dirty = dirty_;

    for (auto& child : children_) {
        was_dirty |= child->createTextures();
    }

    if (was_dirty) {
        glBindFramebuffer(GL_FRAMEBUFFER, fbo_);
        glViewport(0, 0, width_, height_);

        // Фон
        glClearColor(bg_r_ / 255.0f, bg_g_ / 255.0f, bg_b_ / 255.0f, 1.0f);
        glClear(GL_COLOR_BUFFER_BIT);

        // Рисуем строку
        FontManager& fm = FontManager::instance();
        Renderer& r = Renderer::instance();

        glUseProgram(r.textProgram());

        // Uniform для цвета текста
        int color_loc = glGetUniformLocation(r.textProgram(), "text_color");
        glUniform3f(color_loc, text_r_ / 255.0f, text_g_ / 255.0f, text_b_ / 255.0f);

        // Сэмплер на TEXTURE0
        int tex_loc = glGetUniformLocation(r.textProgram(), "tex");
        glUniform1i(tex_loc, 0);
        glActiveTexture(GL_TEXTURE0);

        float x = 0.0f;
        float y = (float)fm.lineHeight();

        for (char c : content_) {
            if (c == ' ') {
                Glyph* space = fm.getGlyph(' ');
                if (space) x += space->advance;
                continue;
            }

            Glyph* g = fm.getGlyph(c);
            if (!g) continue;

            // Позиция глифа в пикселях
            float xpos = x + g->bearing_x;
            float ypos = y - g->bearing_y;

            // В NDC (относительно FBO Text'а)
            float x0 = (xpos / (float)width_) * 2.0f - 1.0f;
            float y0 = 1.0f - (ypos / (float)height_) * 2.0f;
            float x1 = ((xpos + g->width) / (float)width_) * 2.0f - 1.0f;
            float y1 = 1.0f - ((ypos + g->height) / (float)height_) * 2.0f;

            // Квад
            float vertices[] = {
                x0, y0, 0.0f, 0.0f,
                x0, y1, 0.0f, 1.0f,
                x1, y1, 1.0f, 1.0f,
                x0, y0, 0.0f, 0.0f,
                x1, y1, 1.0f, 1.0f,
                x1, y0, 1.0f, 0.0f,
            };

            glBindVertexArray(quad_vao_);
            glBindBuffer(GL_ARRAY_BUFFER, quad_vbo_);
            glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_STATIC_DRAW);

            glBindTexture(GL_TEXTURE_2D, g->texture);
            glDrawArrays(GL_TRIANGLES, 0, 6);
            glBindVertexArray(0);

            x += g->advance;
        }

        glBindFramebuffer(GL_FRAMEBUFFER, 0);
    }

    dirty_ = false;
    return was_dirty;
}

void Text::draw(int parent_w, int parent_h) {
    // Как у Rect
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