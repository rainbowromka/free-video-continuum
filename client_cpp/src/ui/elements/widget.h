#pragma once

#include <vector>
#include <memory>
#include <cstdint>
#include <functional>
#include <iostream>
#include "ui/heap.h"

class Heap;

class Widget {
public:
    Widget() = default;
    Widget(int width, int height)
        : width_(width), height_(height) {}
    virtual ~Widget() = default;

    virtual bool createTextures() { return false; }
    virtual void draw(int parent_w, int parent_h) { (void)parent_w; (void)parent_h; }
    virtual bool isDirty() const { return dirty_; }
    virtual void layout() {}

    template<typename T, typename F>
    Widget& add(F configure) {
        T* element = nullptr;
            // std::string action;

        if (child_count_ >= children_.size()) {
            T& new_elem = heap_->add<T>();
            new_elem.heap_ = heap_;
            new_elem.parent_id = id_;
            children_.push_back(new_elem.id());
            element = &new_elem;
            markDirty();
            // action = "CREATE";
        } else {
            Widget* existing = heap_->get(children_[child_count_]);
            T* casted = dynamic_cast<T*>(existing);
            if (casted) {
                element = casted; 
                // action = "EXIST";
            } else {
                heap_->remove(children_[child_count_]);
                T& new_elem = heap_->add<T>();                
                new_elem.heap_ = heap_;
                new_elem.parent_id = id_;
                children_[child_count_] = new_elem.id();
                element = &new_elem;

                markDirty();
                // action = "REPLACE";
            }
        }

        child_count_++;
        element->resetChildCount();
        element->setDefaultProp(true);        
        configure(*element);
        element->truncChildren();
        element->layout();

        return *this;
    }

    void resetChildCount() { child_count_ = 0; }
    int index() const { return index_; }
    void setIndex(int i) { index_ = i; }
    
    void truncChildren() {
        if (child_count_ < children_.size()) {
            for (size_t i = child_count_; i < children_.size(); ++i) {
                heap_->remove(children_[i]);
            }
            children_.resize(child_count_);
            markDirty();
        }
    }

    void setPosition(int x, int y) {
        if (x_ != x || y_ != y) {
            x_ = x;
            y_ = y;
            markDirty();
            default_prop_ = false;
        }
    }

    int x() const { return x_; }
    int y() const { return y_; }
    int width() const { return width_; }
    int height() const { return height_; }

    uint32_t id() const { return id_; }
    void setId(uint32_t id) { id_ = id; }
    void setHeap(Heap* h) { heap_ = h; }
    
    bool defaultProp() const { return default_prop_; }
    void setDefaultProp(bool v) {default_prop_ = v; }

    bool hovered() const { return hovered_; }
    void setHovered(bool v);

    void calcAbs(int parent_abs_x, int parent_abs_y);

    int absX() { return abs_x_; };
    int absY() { return abs_y_; };
    void markDirty();

protected:
    uint32_t id_ = 0;
    uint32_t parent_id = 0;

    int x_ = 0;
    int y_ = 0;
    int abs_x_ = 0;
    int abs_y_ = 0;
    int width_ = 0;
    int height_ = 0;
    uint8_t bg_r_ = 0x16, bg_g_ = 0x19, bg_b_ = 0x20;
    uint8_t hover_bg_r_ = bg_r_, hover_bg_g_ = bg_g_, hover_bg_b_ = bg_b_;
    bool dirty_ = true;
    int child_count_ = 0;
    int index_ = 0;
    bool default_prop_ = false;

    std::vector<uint32_t> children_;    

    unsigned int fbo_ = 0;
    unsigned int texture_ = 0;
    unsigned int quad_vao_ = 0;
    unsigned int quad_vbo_ = 0;

    Heap* heap_ = nullptr;

    bool hovered_ = false;

    void lazyInit();
    void initFbo();
    void initQuadBuffers();
    void recreateFbo();    
};