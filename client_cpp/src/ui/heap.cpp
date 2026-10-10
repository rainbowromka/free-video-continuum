#include "heap.h"
#include "ui/elements/widget.h"

Widget* Heap::get(uint32_t id) {
    auto it = widgets_.find(id);
    return it != widgets_.end() ? it->second.get() : nullptr;
}

void Heap::remove(uint32_t id) {
    widgets_.erase(id);
    hoverable_ids_.erase(id);
}

bool Heap::contains(uint32_t id) const {
    return widgets_.find(id) != widgets_.end();
}

void Heap::addHoverable(uint32_t id) {  
    hoverable_ids_.insert(id);
}

Widget* Heap::hitTest(int mx, int my) {
    Widget* winner = nullptr;

    for (uint32_t id : hoverable_ids_) {
        Widget* w = get(id);
        if (!w) continue;

        if (mx >= w->absX() && mx < w->absX() + w->width() &&
            my >= w->absY() && my < w->absY() + w->height()) {
            if (!winner || id > winner->id()) {
                winner = w;
            }
        }
    }

    return winner;
}