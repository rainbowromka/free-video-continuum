#pragma once

#include <ft2build.h>
#include FT_FREETYPE_H

#include <string>

class FontManager {
public:
    static FontManager& instance();

    bool init(const std::string& font_path);

    // Хардкод: растеризует букву 'y' 20px, возвращает GL-текстуру 20×20
    unsigned int rasterizeY();

private:
    FontManager() = default;
    ~FontManager() = default;
    FontManager(const FontManager&) = delete;
    FontManager& operator=(const FontManager&) = delete;

    FT_Library library_ = nullptr;
    FT_Face face_ = nullptr;
};