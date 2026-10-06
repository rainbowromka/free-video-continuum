#pragma once
#include "ui/elements/rect.h"

class Button : public Rect {
public:
    Button() = default;
    ~Button() = default;
    void layout() override;
};