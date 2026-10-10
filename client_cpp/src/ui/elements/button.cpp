#include <algorithm>
#include "ui/elements/button.h"

Button::Button() {
    hover_bg_r_ = std::min(255, bg_r_ + 30);
    hover_bg_g_ = std::min(255, bg_g_ + 30);
    hover_bg_b_ = std::min(255, bg_b_ + 30);
}

void Button::layout() {

    if (default_prop_) {
        int max_x = 0;
        int max_y = 0;    
        for (uint32_t id : children_) {
            Widget* child = heap_->get(id);
            if (!child) continue;

            int cx = child->x() + child->width();
            int cy = child->y() + child->height();
            if (cx > max_x) max_x = cx;
            if (cy > max_y) max_y = cy;
        }
        width_  = max_x;
        height_ = max_y;
    }

    heap_->addHoverable(id_);
}