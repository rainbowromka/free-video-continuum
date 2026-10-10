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

    FontManager::instance().init("assets/DejaVuSansMono.ttf");

    Renderer& renderer = Renderer::instance();
    if (!renderer.init()) {
        std::cerr << "Failed to init renderer" << std::endl;
        return;
    }

    Ui ui(width_, height_);
    ui.setClientColor(client_r_, client_g_, client_b_);

    bool first_frame = true;

    while (!glfwWindowShouldClose(window_)) {
        glfwPollEvents();

        int w, h;
        glfwGetFramebufferSize(window_, &w, &h);

        if (first_frame ||  w != ui.clientWidth() || h != ui.clientHeight()) {
            ui.setSize(w, h);
            ui.resetChildCount();
            if (onDraw_) onDraw_(ui);
            ui.truncChildren();
            ui.calcAbs(0, 0);
            first_frame = false;
        }

        if (mouse_moved_) {
            ui.onMouseMove(mouse_x_, mouse_y_);
            mouse_moved_ = false;
        }

        if (ui.isDirty()) {
            glViewport(0, 0, w, h);
            ui.render();
            glfwSwapBuffers(window_);
        }
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

    glfwSetFramebufferSizeCallback(window_, [](GLFWwindow*, int w, int h) {
        glViewport(0, 0, w, h);
    });

    glfwSetWindowSizeLimits(window_, width_, height_, GLFW_DONT_CARE, GLFW_DONT_CARE);

    glfwSetWindowUserPointer(window_, this);
    glfwSetCursorPosCallback(window_, [](GLFWwindow* win, double x, double y) {
        Window* self = static_cast<Window*>(glfwGetWindowUserPointer(win));
        self->mouse_x_ = int(x);
        self->mouse_y_ = int(y);
        self->mouse_moved_ = true;
    });
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