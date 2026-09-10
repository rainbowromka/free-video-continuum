#pragma once

#include <vector>
#include <memory>
#include <cstdint>
#include <functional>

class Widget {
public:
    Widget() = default;
    Widget(int width, int height)
        : width_(width), height_(height) {}
    virtual ~Widget() = default;

    template<typename T, typename F>
    T& add(F configure) {
        auto element = std::make_unique<T>();
        configure(*element);
        T& ref = *element;
        children_.push_back(std::move(element));
        return ref;
    }

protected:
    int width_ = 0;
    int height_ = 0;
    uint8_t bg_r_ = 0x16, bg_g_ = 0x19, bg_b_ = 0x20;
    bool dirty_ = true;

    std::vector<std::unique_ptr<Widget>> children_;
};