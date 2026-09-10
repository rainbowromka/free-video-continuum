#include "ui/elements/rect.h"

Rect& Rect::setRect(int x, int y, int width, int height) {
    x_ = x;
    y_ = y;
    width_ = width;
    height_ = height;
    return *this;
}

Rect& Rect::setColor(uint8_t r, uint8_t g, uint8_t b) {
    bg_r_ = r;
    bg_g_ = g;
    bg_b_ = b;
    return *this;
}