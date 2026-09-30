#pragma once

#include <ft2build.h>
#include FT_FREETYPE_H
#include <string>
#include <vector>
#include <unordered_map>
#include <cstdint>


struct RasterGlyph {
    unsigned int texture = 0;
    int bearing_x = 0;
    int bearing_y = 0;
    int width = 0;
    int height = 0;
    int advance = 0;
    float u0 = 0.0f, v0 = 0.0f, u1 = 0.0f, v1 = 0.0f;
};

struct FontAtlas {
    unsigned int texture = 0;                        
    int width = 0;
    int height = 0;
    int cursor_x = 0;           // точка нового глифа в атласе
    int cursor_y = 0;           // точка нового глифа в атласе
    int row_height = 0;         // размер глифа по высоте
};

class FontManager {
public:
    static FontManager& instance();

    bool init(const std::string& font_path);
    
    std::vector<std::reference_wrapper<RasterGlyph>> getGlyphs(const std::string& text, unsigned int size);
    int measureText(const std::string& text, int size);
    int max_height(int size);
    void shutdown();

private:
    FontManager() = default;
    ~FontManager() = default;
    FontManager(const FontManager&) = delete;
    FontManager& operator=(const FontManager&) = delete;

    FT_Library library_ = nullptr;
    FT_Face face_ = nullptr;

    std::unordered_map<uint64_t, RasterGlyph> glyphs_;
    std::unordered_map<int, FontAtlas> active_atlases_;
    std::vector<unsigned int> atlas_textures_;    

    void createAtlas(FontAtlas& atlas, unsigned int size);
    FontAtlas& getAtlas(unsigned int size);
    
    // ВНИМАНИЕ: getGlyph вызывается ТОЛЬКО после того,
    // как снаружи (в getGlyphs) подтверждено:
    //   FT_Get_Char_Index(face_, codepoint) != 0 
    //   FT_Load_Glyph(face_, glyph_index, FT_LOAD_RENDER) == 0
    // Внутри getGlyph ошибки НЕ обрабатываются.
    RasterGlyph& getGlyph(uint32_t codepoint, unsigned int size);
};