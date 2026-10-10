#pragma once
#include "ui/elements/rect.h"

class Button : public Rect {
public:
    Button();
    ~Button() = default;
    void layout() override;
};