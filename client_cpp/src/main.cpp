#include <GLFW/glfw3.h>
#include <iostream>
#include <ui/window.h>
#include <ui/ui.h>
#include <ui/elements/rect.h>

int main() {
    Window window;
    window.setSize(1024, 780)
        .setTitle("Free Video Continuum")
        .setClientColor(0x17, 0x1a, 0x21)
        .onDraw([](Ui& ui) {
            // ...
            int w = ui.clientWidth();
            int h = ui.clientHeight();

            ui.add<Rect>([w, h](Rect& r) {
                r.setRect(0, 0, 390, h);
                r.setColor(0x27, 0x2a, 0x31);
            });
        })
        .run();    
    return 0;
}