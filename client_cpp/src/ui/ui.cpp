#include "ui/ui.h"
#include <GLFW/glfw3.h>

Ui::Ui(int width, int height)
    : Widget(width, height)
{
}

Ui::~Ui() = default;

int Ui::clientWidth() const {
    return width_;
}

int Ui::clientHeight() const {
    return height_;
}

void Ui::setSize(int width, int height) {
    if (width_ != width || height_ != height) {
        width_ = width;
        height_ = height;
        dirty_ = true;
    }
}

void Ui::setClientColor(uint8_t r, uint8_t g, uint8_t b) {
    bg_r_ = r;
    bg_g_ = g;
    bg_b_ = b;
    dirty_ = true;
}

void Ui::markDirty() {
    dirty_ = true;
}

bool Ui::isDirty() const {
    return dirty_;
}

void Ui::render() {
    createTextures();
    draw(width_, height_);
}

bool Ui::createTextures() {
    // bool any_dirty = false;

    for (auto& child : children_) {
        // if (child->createTextures()) {
        //     any_dirty = true;        
        // }
        // а если так
        dirty_ = child->createTextures();
    }

    // if (any_dirty) {
    //     dirty_ = true;
    // }

    // return any_dirty;
    return dirty_;
}

void Ui::draw(int parent_w, int parent_h) {
    for (auto& child : children_) {
        child->draw(parent_w, parent_h);
    }
}