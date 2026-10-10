#include <iostream>
#include "ui/elements/hbox.h"

void HBox::layout() {
    int x = 0;
    for (uint32_t id : children_) {
        Widget* child = heap_->get(id);
        if (!child) continue;
        
        // кастомные (defaultProp=false) пропускаются: координаты заданы пользователем,
        // x не сдвигается — они не участвуют в потоке раскладки HBox        
        if (child->defaultProp()){
            int y = (height() - child->height()) / 2;
            child->setPosition(x, y);
            x += child->width();
        }
    }
}