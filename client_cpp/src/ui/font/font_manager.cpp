#include "ui/font/font_manager.h"
#include <glad/gl.h>
#include <iostream>

FontManager& FontManager::instance() {
    static FontManager fm;
    return fm;
}

bool FontManager::init(const std::string& font_path, int font_size) {
    if (FT_Init_FreeType(&library_) != 0) {
        std::cerr << "[Font] Failed to init FreeType" << std::endl;
        return false;
    }

    if (FT_New_Face(library_, font_path.c_str(), 0, &face_) != 0) {
        std::cerr << "[Font] Failed to load font: " << font_path << std::endl;
        return false;
    }

    FT_Set_Pixel_Sizes(face_, 0, font_size);

    // Высота строки
    line_height_ = face_->size->metrics.height >> 6;
    font_size_ = font_size;

    std::cout << "[Font] Loaded " << font_path 
              << " size=" << font_size 
              << " line_height=" << line_height_ << std::endl;
    return true;
}

Glyph* FontManager::getGlyph(char c) {
    auto it = glyphs_.find(c);
    if (it != glyphs_.end()) {
        return &it->second;
    }

    // Загружаем глиф с LCD-сглаживанием
    if (FT_Load_Char(face_, c, FT_LOAD_RENDER | FT_LOAD_TARGET_LCD) != 0) {
        return nullptr;
    }

    FT_GlyphSlot slot = face_->glyph;
    int width = slot->bitmap.width;
    int height = slot->bitmap.rows;

    Glyph glyph;
    glyph.width = width;
    glyph.height = height;
    glyph.bearing_x = slot->bitmap_left;
    glyph.bearing_y = slot->bitmap_top;
    glyph.advance = slot->advance.x >> 6;

    // Создаём текстуру
    glGenTextures(1, &glyph.texture);
    glBindTexture(GL_TEXTURE_2D, glyph.texture);

    // LCD — 3 байта на пиксель (RGB)
    glPixelStorei(GL_UNPACK_ALIGNMENT, 1);
    glTexImage2D(GL_TEXTURE_2D, 0, GL_RGB,
                 width, height, 0,
                 GL_RGB, GL_UNSIGNED_BYTE, slot->bitmap.buffer);

    // NEAREST — для чёткости субпикселей
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE);

    glBindTexture(GL_TEXTURE_2D, 0);

    auto result = glyphs_.emplace(c, glyph);
    return &result.first->second;
}