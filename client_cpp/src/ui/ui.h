#pragma once

#include "ui/elements/widget.h"

class Ui : public Widget {
public:
    Ui(int width, int height);
    ~Ui();

    int clientWidth() const;
    int clientHeight() const;

    void setSize(int width, int height);
    void setClientColor(uint8_t r, uint8_t g, uint8_t b);

    void markDirty();
    bool isDirty() const;

    void render();
};