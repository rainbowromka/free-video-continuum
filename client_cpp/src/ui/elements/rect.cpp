#include "ui/elements/rect.h"
#include <iostream>

Rect& Rect::setRect(int x, int y, int width, int height) {
    if (x_ != x || y_ != y || width_ != width || height_ != height) {
        x_ = x;
        y_ = y;
        width_ = width;
        height_ = height;
        dirty_ = true;
    }
    return *this;
}

Rect& Rect::setColor(uint8_t r, uint8_t g, uint8_t b) {
    if (bg_r_ != r || bg_g_ != g || bg_b_ != b) {
        bg_r_ = r;
        bg_g_ = g;
        bg_b_ = b;
        dirty_ = true;
    }
    return *this;
}

bool Rect::createTextures() {
    std::cout << "[TEXTURE] Rect at " << x_ << "," << y_ << " " << width_ << "x" << height_ << std::endl;
    return false;
}

void Rect::draw(int parent_w, int parent_h) {
    std::cout << "[DRAW] Rect " << width_ << "x" << height_ << " parent=" << parent_w << "x" << parent_h << std::endl;
}