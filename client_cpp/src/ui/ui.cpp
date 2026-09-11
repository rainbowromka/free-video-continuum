#include "ui/ui.h"
#include <GLFW/glfw3.h>
#include <iostream>

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
    bool was_dirty = dirty_;
    std::cout << "[Ui ENTER] ui.dirty=" << dirty_ << std::endl;

    for (auto& child : children_) {
        dirty_ |= child->createTextures();
    }

    std::cout << "[Ui EXIT] was_dirty=" << was_dirty << " now=" << dirty_ << std::endl;
    return was_dirty;
}

void Ui::draw(int parent_w, int parent_h) {
    glViewport(0, 0, width_, height_);
    glClearColor(bg_r_ / 255.0f, bg_g_ / 255.0f, bg_b_ / 255.0f, 1.0f);
    glClear(GL_COLOR_BUFFER_BIT);    

    for (auto& child : children_) {        
        child->draw(parent_w, parent_h);
    }

    dirty_ = false;
}