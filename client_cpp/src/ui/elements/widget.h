#pragma once

#include <vector>
#include <memory>
#include <cstdint>
#include <functional>
#include <iostream>

class Widget {
public:
    Widget() = default;
    Widget(int width, int height)
        : width_(width), height_(height) {}
    virtual ~Widget() = default;

    virtual bool createTextures() { return false; }
    virtual void draw(int parent_w, int parent_h) { (void)parent_w; (void)parent_h; }
    virtual bool isDirty() const { return dirty_; }

    template<typename T, typename F>
    Widget& add(F configure) {
        T* element = nullptr;
        // std::string action;

        if (child_count_ >= children_.size()) {
            auto new_elem = std::make_unique<T>();
            element = new_elem.get();
            children_.push_back(std::move(new_elem));
            dirty_ = true;
            // action = "CREATE";
        } else {
            auto& existing = children_[child_count_];
            T* casted = dynamic_cast<T*>(existing.get()); 
            if (casted) {
                element = casted; 
                // action = "EXIST";
            } else {
                auto new_elem = std::make_unique<T>();
                element = new_elem.get();
                existing = std::move(new_elem);
                dirty_ = true;
                // action = "REPLACE";
            }
        }

        // std::cout << "[ADD] " << action << " index=" << child_count_ << std::endl;

        child_count_++;
        element->resetChildCount();
        configure(*element);
        element->truncChildren();

        std::cout << "[ADD] before return: element->dirty=" << element->isDirty() 
            << " self->dirty=" << dirty_ << std::endl;


        return *this;
    }

    void resetChildCount() { child_count_ = 0; }
    int index() const { return index_; }
    void setIndex(int i) { index_ = i; }
    void truncChildren() {
        if (child_count_ < children_.size()) {
            children_.resize(child_count_);
            dirty_ = true;
        }
    }

protected:
    int x_ = 0;
    int y_ = 0;
    int width_ = 0;
    int height_ = 0;
    uint8_t bg_r_ = 0x16, bg_g_ = 0x19, bg_b_ = 0x20;
    bool dirty_ = true;
    int child_count_ = 0;
    int index_ = 0;

    std::vector<std::unique_ptr<Widget>> children_;

    unsigned int fbo_ = 0;
    unsigned int texture_ = 0;
    unsigned int quad_vao_ = 0;
    unsigned int quad_vbo_ = 0;

    void lazyInit();
    void initFbo();
    void initQuadBuffers();
    void recreateFbo();    
};