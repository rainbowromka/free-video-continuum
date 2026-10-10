#pragma once

#include "ui/elements/widget.h"
#include "ui/heap.h"

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
    
    bool createTextures() override;
    void draw(int parent_w, int parent_h) override;    
    void onMouseMove(int mx, int my);

private:
    Heap widgets_heap_;
    Widget* hovered_widget_ = nullptr;
};