#include "ui/font/font_manager.h"
#include <glad/gl.h>
#include <iostream>
#include <vector>

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

std::vector<RasterGlyph> FontManager::rasterize(const std::string& text, unsigned int size) {
    std::vector<RasterGlyph> result;

    if (text.empty()) {
        return result;
    }

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

        FT_GlyphSlot slot = face_->glyph;
        int w = slot->bitmap.width;
        int h = slot->bitmap.rows;

        RasterGlyph g;
        g.bearing_x = slot->bitmap_left;
        g.bearing_y = slot->bitmap_top;
        g.width     = w;
        g.height    = h;
        g.advance   = static_cast<int>(slot->advance.x >> 6);

        // std::cout << "[Font] cp=0x" << std::hex << codepoint << std::dec
        //           << " w=" << w << " h=" << h
        //           << " left=" << g.bearing_x
        //           << " top=" << g.bearing_y
        //           << " advance=" << g.advance
        //           << " pitch=" << slot->bitmap.pitch
        //           << " pixel_mode=" << (int)slot->bitmap.pixel_mode << std::endl;

        if (w == 0 || h == 0) {
            result.push_back(g);
            continue;
        }

        unsigned int tex = 0;
        glGenTextures(1, &tex);
        glBindTexture(GL_TEXTURE_2D, tex);

        glTexImage2D(GL_TEXTURE_2D, 0, GL_RED,
                    w, h, 0,
                    GL_RED, GL_UNSIGNED_BYTE, slot->bitmap.buffer);

        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE);

        glBindTexture(GL_TEXTURE_2D, 0);

        g.texture = tex;
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

int FontManager::descender(int size) {
    if (!face_) {
        return 0;
    }
    FT_Set_Pixel_Sizes(face_, 0, size);
    return int(face_->size->metrics.descender >> 6);
}