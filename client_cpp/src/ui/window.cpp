#include "ui/window.h"
#include "ui/ui.h"
#include <glad/gl.h>
#include <GLFW/glfw3.h>
#include <iostream>
#include "ui/render/renderer.h"
#include "ui/font/font_manager.h"

Window::Window() {}

Window::~Window() {
    cleanup();
}

Window& Window::setSize(int width, int height) {
    width_ = width;
    height_ = height;
    return *this;
}

Window& Window::setTitle(const std::string& title) {
    title_ = title;
    return *this;
}

Window& Window::setClientColor(uint8_t r, uint8_t g, uint8_t b) {
    client_r_ = r;
    client_g_ = g;
    client_b_ = b;
    return *this;
}

Window& Window::onDraw(std::function<void(Ui&)> callback) {
    onDraw_ = std::move(callback);
    return *this;
}

void Window::run() {
    initGlfw();
    createWindow();
    initOpenGL();

    FontManager::instance().init("assets/DejaVuSansMono.ttf", 16);

    Renderer& renderer = Renderer::instance();
    if (!renderer.init()) {
        std::cerr << "Failed to init renderer" << std::endl;
        return;
    }

    Ui ui(width_, height_);
    ui.setClientColor(client_r_, client_g_, client_b_);

    bool first_frame = true;

    while (!glfwWindowShouldClose(window_)) {
        int w, h;
        glfwGetFramebufferSize(window_, &w, &h);

        if (first_frame ||  w != ui.clientWidth() || h != ui.clientHeight()) {
            ui.setSize(w, h);
            ui.resetChildCount();
            if (onDraw_) onDraw_(ui);
            ui.truncChildren();
            first_frame = false;
        }

        if (ui.isDirty()) {
            glViewport(0, 0, w, h);
            ui.render();
            glfwSwapBuffers(window_);
        }

        glfwPollEvents();
    }
}

void Window::initGlfw() {
    if (!glfwInit()) {
        std::cerr << "Failed to init GLFW" << std::endl;
        std::exit(-1);
    }

    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
}

void Window::createWindow() {
    window_ = glfwCreateWindow(width_, height_, title_.c_str(), nullptr, nullptr);
    if (!window_) {
        std::cerr << "Failed to create window" << std::endl;
        glfwTerminate();
        std::exit(-1);
    }
    glfwMakeContextCurrent(window_);

    glfwSetWindowSizeLimits(window_, width_, height_, GLFW_DONT_CARE, GLFW_DONT_CARE);
}

void Window::initOpenGL() {
    int version = gladLoadGL(glfwGetProcAddress);
    if (version == 0) {
        std::cerr << "Failed to init GLAD" << std::endl;
        std::exit(-1);
    }
}

void Window::cleanup() {
    if (window_) {
        glfwDestroyWindow(window_);
        window_ = nullptr;
    }
    glfwTerminate();
}