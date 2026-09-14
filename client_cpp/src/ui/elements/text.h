#pragma once

#include "ui/elements/widget.h"
#include <string>

class Text : public Widget {
public:
    Text() {
        width_ = 20;
        height_ = 20;
    }
    ~Text() = default;

    Text& setContent(const std::string& content);
    Text& setTextColor(uint8_t r, uint8_t g, uint8_t b);

    bool createTextures() override;
    void draw(int parent_w, int parent_h) override;

protected:
    std::string content_;
    uint8_t text_r_ = 0xff, text_g_ = 0xff, text_b_ = 0xff;
};