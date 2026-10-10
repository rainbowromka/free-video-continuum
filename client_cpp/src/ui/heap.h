#pragma once

#include <unordered_map>
#include <unordered_set>
#include <memory>
#include <cstdint>

class Widget;

class Heap {
public:
    template<typename T>
    T& add() {
        auto w = std::make_unique<T>();
        uint32_t id = next_id_++;
        T& ref = *w;
        ref.setId(id);
        widgets_[id] = std::move(w);
        return ref;
    }

    Widget* get(uint32_t id);
    void remove(uint32_t id);
    bool contains(uint32_t id) const;
    void addHoverable(uint32_t id);
    Widget* hitTest(int mx, int my);

private:
    std::unordered_map<uint32_t, std::unique_ptr<Widget>> widgets_;
    std::unordered_set<uint32_t> hoverable_ids_;
    uint32_t next_id_ = 1;
};