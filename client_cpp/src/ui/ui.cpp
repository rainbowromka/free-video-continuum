#include "ui/ui.h"
#include <GLFW/glfw3.h>
#include "ui/heap.h"

Ui::Ui(int width, int height)
    : Widget(width, height)
{
    heap_ = &widgets_heap_;
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
        default_prop_ = false;
        markDirty();        
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

    for (uint32_t id : children_) {
        Widget* child = heap_->get(id);
        if (!child) continue;
                        
        dirty_ |= child->createTextures();
    }

    return was_dirty;
}

void Ui::draw(int parent_w, int parent_h) {
    glViewport(0, 0, width_, height_);
    glClearColor(bg_r_ / 255.0f, bg_g_ / 255.0f, bg_b_ / 255.0f, 1.0f);
    glClear(GL_COLOR_BUFFER_BIT);    

    for (uint32_t id : children_) {
        Widget* child = heap_->get(id);
        if (!child) continue;

        child->draw(parent_w, parent_h);
    }

    dirty_ = false;
}

void Ui::onMouseMove(int mx, int my) {
    Widget* hit = heap_->hitTest(mx, my);

    if (hit == hovered_widget_ ) return;

    if (hovered_widget_) hovered_widget_ ->setHovered(false);
    hovered_widget_ = hit;
    if (hovered_widget_) hovered_widget_ ->setHovered(true);
    dirty_ = true;
    std::cout << "[hover] hit id=" << (hit ? hit->id() : 0) << std::endl;
}