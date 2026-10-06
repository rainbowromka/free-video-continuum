#include "ui/elements/button.h"

void Button::layout() {
    int max_x = 0;
    int max_y = 0;
    for (auto& child : children_) {
        int cx = child->x() + child->width();
        int cy = child->y() + child->height();
        if (cx > max_x) max_x = cx;
        if (cy > max_y) max_y = cy;
    }
    width_  = max_x;
    height_ = max_y;
}