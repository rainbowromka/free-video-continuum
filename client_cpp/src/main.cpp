#include <GLFW/glfw3.h>
#include <iostream>
#include <ui/window.h>

int main() {
    Window window;
    window.setSize(1024, 780)
        .setTitle("Free Video Continuum")
        .setClientColor(0x17, 0x1a, 0x21)
        .onDraw([](Ui& ui) {
            // ...
        })
        .run();
    
    return 0;
}