#include <iostream>
#include "ui/elements/hbox.h"

void HBox::layout() {
    int x = 0;
    for (auto& child : children_) {
        int y = (height() - child->height()) / 2;
        child->setPosition(x, y);
        x += child->width();
    }
}