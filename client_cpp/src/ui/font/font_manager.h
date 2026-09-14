#pragma once

#include <ft2build.h>
#include FT_FREETYPE_H

#include <string>
#include <unordered_map>
#include <cstdint>

struct Glyph {
    unsigned int texture = 0;   // GL-текстура
    int width = 0;
    int height = 0;
    int bearing_x = 0;
    int bearing_y = 0;
    int advance = 0;            // сдвиг к следующему символу
};

class FontManager {
public:
    static FontManager& instance();

    bool init(const std::string& font_path, int font_size);

    Glyph* getGlyph(char c);
    int lineHeight() const { return line_height_; }

private:
    FontManager() = default;
    ~FontManager() = default;
    FontManager(const FontManager&) = delete;
    FontManager& operator=(const FontManager&) = delete;

    FT_Library library_ = nullptr;
    FT_Face face_ = nullptr;
    int font_size_ = 0;
    int line_height_ = 0;

    // TODO (future): заменить на атлас глифов.
    // Сейчас каждый глиф — отдельная текстура. При большом количестве
    // символов это неэффективно (много draw calls, много текстур).
    // Атлас объединит все глифы в одну текстуру — один draw call,
    // меньше переключений состояния.
    std::unordered_map<char, Glyph> glyphs_;
};