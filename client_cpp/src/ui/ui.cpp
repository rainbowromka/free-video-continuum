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
    // Пока ничего — заглушка
    // Позже: createTextures + draw
    dirty_ = false;
}