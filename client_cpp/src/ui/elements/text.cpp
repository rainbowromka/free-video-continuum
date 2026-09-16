#include "ui/elements/text.h"
#include "ui/font/font_manager.h"
#include "ui/render/renderer.h"
#include <glad/gl.h>
#include <iostream>

Text& Text::setContent(const std::string& content) {
    content_ = content;
    dirty_ = true;
    return *this;
}

Text& Text::setTextColor(uint8_t r, uint8_t g, uint8_t b) {
    text_r_ = r;
    text_g_ = g;
    text_b_ = b;
    dirty_ = true;
    return *this;
}

bool Text::createTextures() {
    lazyInit();

    bool was_dirty = dirty_;

    if (was_dirty) {

        glBindFramebuffer(GL_FRAMEBUFFER, fbo_);
        glViewport(0, 0, width_, height_);

        // Фон — чёрный
        glClearColor(0.0f, 0.0f, 0.0f, 1.0f);
        glClear(GL_COLOR_BUFFER_BIT);

        std::vector<RasterGlyph> glyphs = FontManager::instance().rasterize(content_);
        for (const RasterGlyph& g : glyphs) {
            if (g.texture) {
                drawGlyph(g.texture, g.bearing_x, g.bearing_y, g.width, g.height);
                glDeleteTextures(1, &g.texture);
            }
        }
        // RasterGlyph glyph = FontManager::instance().rasterize(content_);
        // if (glyph.texture) {
        //     drawGlyph(glyph.texture, glyph.bearing_x, glyph.bearing_y, glyph.width, glyph.height);
        // }


        glBindFramebuffer(GL_FRAMEBUFFER, 0);
    }

    dirty_ = false;
    return was_dirty;
}

void Text::drawGlyph(unsigned int tex,  int bearing_x, int bearing_y, int width, int height) {
    float scale = float(width_) / 2.0f;
    float x1 = float(bearing_x) / scale - 1.0f;
    float x2 = float(bearing_x + width) / scale - 1.0f;
    float y1 = float(baseLine + bearing_y - height) / scale - 1.0f;
    float y2 = float(baseLine + bearing_y) / scale - 1.0f;

    float vertices[] = {                
        x1, y2, 0.0f, 0.0f,
        x1, y1, 0.0f, 1.0f,
        x2, y1, 1.0f, 1.0f,
        x1, y2, 0.0f, 0.0f,
        x2, y1, 1.0f, 1.0f,
        x2, y2, 1.0f, 0.0f,
    };

    glBindVertexArray(quad_vao_);
    glBindBuffer(GL_ARRAY_BUFFER, quad_vbo_);
    glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_STATIC_DRAW);

    glActiveTexture(GL_TEXTURE0);
    glBindTexture(GL_TEXTURE_2D, tex);

    glUseProgram(Renderer::instance().textProgram());

    int color_loc = glGetUniformLocation(Renderer::instance().textProgram(), "text_color");
    glUniform3f(color_loc, text_r_ / 255.0f, text_g_ / 255.0f, text_b_ / 255.0f);

    int tex_loc = glGetUniformLocation(Renderer::instance().textProgram(), "tex");
    glUniform1i(tex_loc, 0);

    glDrawArrays(GL_TRIANGLES, 0, 6);
    glBindVertexArray(0);
    glDeleteTextures(1, &tex);
}

void Text::draw(int parent_w, int parent_h) {
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
    glDrawArrays(GL_TRIANGLES, 0, 6);
    glBindVertexArray(0);
}