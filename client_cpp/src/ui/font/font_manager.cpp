#include "ui/font/font_manager.h"
#include <glad/gl.h>
#include <iostream>
#include <vector>

FontManager& FontManager::instance() {
    static FontManager fm;
    return fm;
}

bool FontManager::init(const std::string& font_path) {
    if (FT_Init_FreeType(&library_) != 0) {
        std::cerr << "[Font] Failed to init FreeType" << std::endl;
        return false;
    }

    if (FT_New_Face(library_, font_path.c_str(), 0, &face_) != 0) {
        std::cerr << "[Font] Failed to load font: " << font_path << std::endl;
        return false;
    }

    std::cout << "[Font] Loaded " << font_path << std::endl;
    return true;
}

unsigned int FontManager::rasterizeY() {
    FT_Set_Pixel_Sizes(face_, 0, 20);

    if (FT_Load_Char(face_, 'y', FT_LOAD_RENDER) != 0) {
        std::cerr << "[Font] Failed to load glyph 'y'" << std::endl;
        return 0;
    }

    FT_GlyphSlot slot = face_->glyph;
    int w = slot->bitmap.width;
    int h = slot->bitmap.rows;

    std::cout << "[Font] glyph 'y' w=" << w << " h=" << h 
              << " pitch=" << slot->bitmap.pitch 
              << " pixel_mode=" << (int)slot->bitmap.pixel_mode << std::endl;

    // Создаём текстуру 20×20
    std::vector<uint8_t> pixels(20 * 20, 0);

    // Копируем bitmap (grayscale) в левый верхний угол
    for (int row = 0; row < h && row < 20; ++row) {
        for (int col = 0; col < w && col < 20; ++col) {
            pixels[row * 20 + col] = slot->bitmap.buffer[row * slot->bitmap.pitch + col];
        }
    }

    unsigned int tex = 0;
    glGenTextures(1, &tex);
    glBindTexture(GL_TEXTURE_2D, tex);

    glPixelStorei(GL_UNPACK_ALIGNMENT, 1);
    glTexImage2D(GL_TEXTURE_2D, 0, GL_RED,
                 20, 20, 0,
                 GL_RED, GL_UNSIGNED_BYTE, pixels.data());

    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE);

    glBindTexture(GL_TEXTURE_2D, 0);

    return tex;
}