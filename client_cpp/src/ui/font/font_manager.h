#pragma once

#include <ft2build.h>
#include FT_FREETYPE_H
#include <string>
#include <vector>
#include <unordered_map>
#include <cstdint>


struct RasterGlyph {
    unsigned int texture = 0;   // GL-текстура (GL_RED), 0 = ошибка
    int bearing_x = 0;          // bitmap_left
    int bearing_y = 0;          // bitmap_top (это и есть ascent)
    int width = 0;              // bitmap.width
    int height = 0;             // bitmap.rows
    int advance = 0;            // advance.x >> 6

    float u0 = 0.0,
        v0 = 0.0,
        u1 = 0.0,
        v1 = 0.0; // координаты текстуры в атласе.
};

struct FontAtlas {
    int cursor_x = 0;           // точка нового глифа в атласе
    int cursor_y = 0;           // точка нового глифа в атласе
    int width = 1024;           // размер атласа
    int height = 1024;          // размер атласа
    int row_height = 0;         // размер глифа по высоте
    unsigned int texture = 0;                        
    std::unordered_map<uint32_t, RasterGlyph> glyphs;
};

class FontManager {
public:
    static FontManager& instance();

    bool init(const std::string& font_path);
    
    std::vector<std::reference_wrapper<RasterGlyph>> getGlyphs(const std::string& text, unsigned int size);
    int measureText(const std::string& text, int size);
    int max_height(int size);

private:
    FontManager() = default;
    ~FontManager() = default;
    FontManager(const FontManager&) = delete;
    FontManager& operator=(const FontManager&) = delete;

    FT_Library library_ = nullptr;
    FT_Face face_ = nullptr;

    std::unordered_map<int, FontAtlas> atlases_; 

    FontAtlas& getAtlas(unsigned int size);
    RasterGlyph& getGlyph(FontAtlas& atlas, uint32_t codepoint, unsigned int size);
};