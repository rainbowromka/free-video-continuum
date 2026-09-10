#pragma once

#include "ui/elements/widget.h"

class Rect : public Widget {
public:
    Rect() = default;
    ~Rect() = default;

    Rect& setRect(int x, int y, int width, int height);
    Rect& setColor(uint8_t r, uint8_t g, uint8_t b);

    bool createTextures() override;
    void draw(int parent_w, int parent_h) override;

protected:
    int x_ = 0;
    int y_ = 0;
};