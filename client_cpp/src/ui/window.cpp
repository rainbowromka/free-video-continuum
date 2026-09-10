#include "ui/window.h"
#include "ui/ui.h"
#include <GLFW/glfw3.h>
#include <iostream>

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

    // glClearColor(0.09f, 0.10f, 0.13f, 1.0f);

    Ui ui(width_, height_);
    ui.setClientColor(0x16, 0x19, 0x20);

    if (onDraw_) onDraw_(ui);

    while (!glfwWindowShouldClose(window_)) {
        int w, h;
        glfwGetFramebufferSize(window_, &w, &h);

        if (w != ui.clientWidth() || h != ui.clientHeight()) {
            ui.setSize(w, h);
            if (onDraw_) onDraw_(ui);
        }

        if (ui.isDirty()) {
            glViewport(0, 0, w, h);

            // Очистка экрана
            glClearColor(0.09f, 0.10f, 0.13f, 1.0f);
            glClear(GL_COLOR_BUFFER_BIT);

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
}

void Window::initOpenGL() {
    // пока ничего
}

void Window::cleanup() {
    if (window_) {
        glfwDestroyWindow(window_);
        window_ = nullptr;
    }
    glfwTerminate();
}