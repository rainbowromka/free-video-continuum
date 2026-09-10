#pragma once

#include <string>
#include <functional>
#include <cstdint>

struct GLFWwindow;

class Ui;

class Window {
public:
    Window();
    ~Window();

    Window& setSize(int width, int height);
    Window& setTitle(const std::string& title);
    Window& setClientColor(uint8_t r, uint8_t g, uint8_t b);
    Window& onDraw(std::function<void(Ui&)> callback);
    void run();

private:
    int width_ = 1024;
    int height_ = 780;
    uint8_t client_r_ = 0x16;
    uint8_t client_g_ = 0x19;
    uint8_t client_b_ = 0x20;

    std::string title_ = "Free Video Continuum";
    std::function<void(Ui&)> onDraw_;

    GLFWwindow* window_ = nullptr;

    void initGlfw();
    void createWindow();
    void initOpenGL();
    void cleanup();
};