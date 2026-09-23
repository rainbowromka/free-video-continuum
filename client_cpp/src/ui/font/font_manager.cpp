#include "ui/font/font_manager.h"
#include <glad/gl.h>
#include <iostream>
#include <vector>
#include <functional>

namespace {
    // Читает один Unicode-кодпоинт из UTF-8 строки, начиная с pos.
    // Сдвигает pos на число прочитанных байт.
    // Возвращает 0 при пустой строке или некорректной последовательности.
    uint32_t utf8DecodeFirst(const std::string& s, size_t& pos) {
        if (pos >= s.size()) {
            return 0;
        }

        const unsigned char* p = reinterpret_cast<const unsigned char*>(s.data() + pos);
        unsigned char b0 = p[0];

        // ASCII: 0xxxxxxx
        if (b0 < 0x80) {
            pos += 1;
            return b0;
        }

        // 2 байта: 110xxxxx 10xxxxxx
        if ((b0 & 0xE0) == 0xC0) {
            if (pos + 1 >= s.size()) { pos = s.size(); return 0; }
            unsigned char b1 = p[1];
            if ((b1 & 0xC0) != 0x80) { pos += 1; return 0; }
            uint32_t cp = ((b0 & 0x1F) << 6) | (b1 & 0x3F);
            pos += 2;
            return cp;
        }

        // 3 байта: 1110xxxx 10xxxxxx 10xxxxxx
        if ((b0 & 0xF0) == 0xE0) {
            if (pos + 2 >= s.size()) { pos = s.size(); return 0; }
            unsigned char b1 = p[1];
            unsigned char b2 = p[2];
            if ((b1 & 0xC0) != 0x80 || (b2 & 0xC0) != 0x80) { pos += 1; return 0; }
            uint32_t cp = ((b0 & 0x0F) << 12) | ((b1 & 0x3F) << 6) | (b2 & 0x3F);
            pos += 3;
            return cp;
        }

        // 4 байта: 11110xxx 10xxxxxx 10xxxxxx 10xxxxxx
        if ((b0 & 0xF8) == 0xF0) {
            if (pos + 3 >= s.size()) { pos = s.size(); return 0; }
            unsigned char b1 = p[1];
            unsigned char b2 = p[2];
            unsigned char b3 = p[3];
            if ((b1 & 0xC0) != 0x80 || (b2 & 0xC0) != 0x80 || (b3 & 0xC0) != 0x80) {
                pos += 1; return 0;
            }
            uint32_t cp = ((b0 & 0x07) << 18) | ((b1 & 0x3F) << 12)
                        | ((b2 & 0x3F) << 6)  | (b3 & 0x3F);
            pos += 4;
            return cp;
        }

        // Некорректный ведущий байт — пропускаем один байт
        pos += 1;
        return 0;
    }

}

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

std::vector<std::reference_wrapper<RasterGlyph>> FontManager::getGlyphs(
    const std::string& text,
     unsigned int size)
{
    std::vector<std::reference_wrapper<RasterGlyph>> result;

    if (text.empty()) {
        return result;        
    }

    FontAtlas& atlas = getAtlas(size);

    // Размер задаётся один раз для всей строки
    FT_Set_Pixel_Sizes(face_, 0, size);
    glPixelStorei(GL_UNPACK_ALIGNMENT, 1);

    size_t pos = 0;
    while (pos < text.size()) {
        uint32_t codepoint = utf8DecodeFirst(text, pos);

        if (codepoint == 0) {
            continue;   // некорректный байт — пропускаем
        }

        FT_UInt glyph_index = FT_Get_Char_Index(face_, codepoint);
        if (glyph_index == 0) {
            std::cerr << "[Font] No glyph for codepoint 0x"
                    << std::hex << codepoint << std::dec << std::endl;
            continue;
        }

        if (FT_Load_Glyph(face_, glyph_index, FT_LOAD_RENDER) != 0) {
            std::cerr << "[Font] Failed to load/render glyph 0x"
                    << std::hex << codepoint << std::dec << std::endl;
            continue;
        }

        RasterGlyph& g = getGlyph(atlas, codepoint, size);
        result.push_back(g);
    }

    return result;
}

int FontManager::measureText(const std::string& text, int size) {
    if (text.empty()) {
        return 0;
    }

    FT_Set_Pixel_Sizes(face_, 0, size);

    int total = 0;
    size_t pos = 0;
    while (pos < text.size()) {
        uint32_t codepoint = utf8DecodeFirst(text, pos);
        if (codepoint == 0) {
            continue;
        }

        FT_UInt glyph_index = FT_Get_Char_Index(face_, codepoint);
        if (glyph_index == 0) {
            continue;
        }

        if (FT_Load_Glyph(face_, glyph_index, FT_LOAD_DEFAULT) != 0) {
            continue;
        }

        total += static_cast<int>(face_->glyph->advance.x >> 6);
    }

    return total;
}

int FontManager::max_height(int size) {
    if (!face_) {
        return 0;
    }
    FT_Set_Pixel_Sizes(face_, 0, size);

    int asc = face_->size->metrics.ascender>>6;
    int desc = face_->size->metrics.descender>>6;


    return int(asc-desc);
}

FontAtlas& FontManager::getAtlas(unsigned int size) {    
    FontAtlas& atlas = atlases_[size];

    if (atlas.texture == 0) {
        glGenTextures(1, &atlas.texture);
        glBindTexture(GL_TEXTURE_2D, atlas.texture);

        const int ATLAS_W = 1024;
        const int ATLAS_H = 1024;
        std::vector<uint8_t> empty(ATLAS_W * ATLAS_H, 0);

        glPixelStorei(GL_UNPACK_ALIGNMENT, 1);
        glTexImage2D(GL_TEXTURE_2D, 0, GL_RED,
                     ATLAS_W, ATLAS_H, 0,
                     GL_RED, GL_UNSIGNED_BYTE, empty.data());

        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE);

        glBindTexture(GL_TEXTURE_2D, 0);

        std::cout << "[Font] Created atlas for size " << size
                  << " texture=" << atlas.texture << std::endl;
    }

    return atlas;
}

RasterGlyph& FontManager::getGlyph(FontAtlas& atlas, uint32_t codepoint, unsigned int size)                        
{
    auto it = atlas.glyphs.find(codepoint);
    if (it != atlas.glyphs.end()) {
        return it->second;
    }

    // RasterGlyph& g = atlas.glyphs[codepoint];

    FT_GlyphSlot slot = face_->glyph;
    int w = slot->bitmap.width;
    int h = slot->bitmap.rows;

    RasterGlyph g;
    g.bearing_x = slot->bitmap_left;
    g.bearing_y = slot->bitmap_top;
    g.width     = w;
    g.height    = h;
    g.advance   = static_cast<int>(slot->advance.x >> 6);

    if (w == 0 || h == 0) {
        atlas.glyphs[codepoint] = g;
        return atlas.glyphs[codepoint];
    }

    if (atlas.cursor_x + w > atlas.width) {
        atlas.cursor_x = 0;
        atlas.cursor_y += atlas.row_height;
        atlas.row_height = 0;
    }

    if (atlas.cursor_y + h > atlas.height) {
        growAtlas(atlas, size);
        // после пересоздания cursor_x = 0, cursor_y = 0, row_height = 0
        // и проверка по X тоже может сработать заново
        if (atlas.cursor_x + w > atlas.width) {
            atlas.cursor_x = 0;
            atlas.cursor_y += atlas.row_height;
            atlas.row_height = 0;
        }        
    }

    // atlas.glyphs[codepoint] = g;
    // RasterGlyph& g = atlas.glyphs[codepoint];

    int last_x = atlas.cursor_x;
    int last_y = atlas.cursor_y;

    g.u0 = float(last_x)     / float(atlas.width);
    g.v0 = float(last_y)     / float(atlas.height);
    g.u1 = float(last_x + w) / float(atlas.width);
    g.v1 = float(last_y + h) / float(atlas.height);

    glBindTexture(GL_TEXTURE_2D, atlas.texture);
    glPixelStorei(GL_UNPACK_ALIGNMENT, 1);
    glTexSubImage2D(GL_TEXTURE_2D, 0,
                    last_x, last_y,
                    w, h,
                    GL_RED, GL_UNSIGNED_BYTE,
                    slot->bitmap.buffer);

    glBindTexture(GL_TEXTURE_2D, 0);

    atlas.cursor_x += w;
    if (h > atlas.row_height) {
        atlas.row_height = h;
    }

    g.texture = atlas.texture;

    atlas.glyphs[codepoint] = g;
    return atlas.glyphs[codepoint];
}


void FontManager::shutdown() {
    // 1. Удаляем GL-текстуры атласов
    for (auto& [size, atlas] : atlases_) {
        if (atlas.texture != 0) {
            glDeleteTextures(1, &atlas.texture);
            atlas.texture = 0;
        }
    }
    atlases_.clear();

    // 2. Освобождаем FreeType
    if (face_) {
        FT_Done_Face(face_);
        face_ = nullptr;
    }
    if (library_) {
        FT_Done_FreeType(library_);
        library_ = nullptr;
    }

    std::cout << "[Font] Shutdown complete" << std::endl;
}