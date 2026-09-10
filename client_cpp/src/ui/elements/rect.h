#pragma once

#include "ui/elements/widget.h"

class Rect : public Widget {
public:
    Rect() = default;
    ~Rect() = default;

    Rect& setRect(int x, int y, int width, int height);
    Rect& setColor(uint8_t r, uint8_t g, uint8_t b);

protected:
    int x_ = 0;
    int y_ = 0;
};