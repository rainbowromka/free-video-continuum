#pragma once

#include <ft2build.h>
#include FT_FREETYPE_H
#include <string>
#include <vector>

struct RasterGlyph {
    unsigned int texture = 0;   // GL-текстура (GL_RED), 0 = ошибка
    int bearing_x = 0;          // bitmap_left
    int bearing_y = 0;          // bitmap_top (это и есть ascent)
    int width = 0;              // bitmap.width
    int height = 0;             // bitmap.rows
    int advance = 0;            // advance.x >> 6
};

class FontManager {
public:
    static FontManager& instance();

    bool init(const std::string& font_path);

    std::vector<RasterGlyph> rasterize(const std::string& text, unsigned int size);
    int measureText(const std::string& text, int size);
    int descender(int size);

private:
    FontManager() = default;
    ~FontManager() = default;
    FontManager(const FontManager&) = delete;
    FontManager& operator=(const FontManager&) = delete;

    FT_Library library_ = nullptr;
    FT_Face face_ = nullptr;
};