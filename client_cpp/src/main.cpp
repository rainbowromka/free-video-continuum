#include <GLFW/glfw3.h>
#include <ui/window.h>
#include <ui/ui.h>
#include <ui/elements/rect.h>
#include <ui/elements/text.h>
#include <ui/elements/hbox.h>
#include <ui/elements/button.h>

int main() {
    Window window;
    window.setSize(1024, 780)
        .setTitle("Free Video Continuum")
        .setClientColor(0x17, 0x1a, 0x21)
        .onDraw([](Ui& ui) {
            
            int w = ui.clientWidth();
            int h = ui.clientHeight();

            ui.add<Rect>([h](Rect& r) { r
                .setRect(0, 0, 390, h)
                .setColor(0x27, 0x2a, 0x31)
                .add<Rect>([](Rect& r){ r
                    .setRect(10,10, 200, 22)
                    .add<Text>([](Text& t){ t
                        .setContent(21, "Медиа:")
                        .setTextColor(0xff, 0xff, 0xff);
                    });
                })
                .add<HBox>([](HBox& h){ h
                    .setRect(10,36, 370, 22)
                    .setColor(0x17, 0x1a, 0x21)
                    .add<Button>([](Button& b) {b
                        .add<Text>([](Text& t) {t
                            .setContent(16, "[d]Mamay")                                
                            .setTextColor(0xff, 0xff, 0xff);
                        });
                    })
                    .add<Text>([](Text& t) {t
                        .setContent(16, "[r]MyVideo")
                        .setTextColor(0xef, 0xef, 0xef)
                        // .set_color(0x17, 0x1a, 0x21)
                        ;
                    })
                    .add<Text>([](Text& t) { t
                        .setContent(16, "[e]Новый год")
                        .setTextColor(0xef, 0xef, 0xef);
                    //         .set_color(0x17, 0x1a, 0x21)
                    })
                    ;
                });
            })
            .add<Rect>([w, h](Rect& r) { r
                .setRect(w - 300, 0, 300, h)
                .setColor(0x27, 0x2a, 0x31);
            });})
        .run();


    FontManager::instance().shutdown();
    return 0;
}