#include <GLFW/glfw3.h>
#include <ui/window.h>
#include <ui/ui.h>
#include <ui/elements/rect.h>

int main() {
    Window window;
    window.setSize(1024, 780)
        .setTitle("Free Video Continuum")
        .setClientColor(0x17, 0x1a, 0x21)
        // .onDraw([](Ui& ui) {
        //     // ...
        //     int w = ui.clientWidth();
        //     int h = ui.clientHeight();

        //     ui.add<Rect>([w, h](Rect& r) {
        //         r.setRect(0, 0, 390, h);
        //         r.setColor(0x27, 0x2a, 0x31);
        //     });
        // })
        // .run();    
        .onDraw([](Ui& ui) {
            
            int w = ui.clientWidth();
            int h = ui.clientHeight();

            ui.add<Rect>([h](Rect& r) { r
                .setRect(0, 0, 390, h)
                .setColor(0x27, 0x2a, 0x31)
                .add<Rect>([](Rect& r){ r
                    .setRect(10,10, 200, 22)
                    // .add::<Text>(|t| { t
                    //     .set_content("Медиа:")
                    //     .set_position(1, 1)
                    //     .set_height(20)
                    //     .set_text_color(0xff, 0xff, 0xff);
                    // });
                // })
                // .add::<HBox>(|h|{h
                //     .set_rect(10,36, 370, 22)
                //     .set_color(0x17, 0x1a, 0x21)
                //     .add::<Button>(|b|{b
                //         .add::<Text>(|t| {t
                //             .set_content("[d]Mamay")
                //             .set_height(16)
                //             .set_text_color(0xff, 0xff, 0xff);
                //         });
                //     })
                //     .add::<Text>(|t| { t
                //         .set_content("[r]MyVideo")
                //         .set_height(16)
                //         .set_text_color(0xef, 0xef, 0xef)
                //         .set_color(0x17, 0x1a, 0x21)
                //     })
                //     .add::<Text>(|t| { t
                //         .set_content("[e]Новый год")
                //         .set_height(16)
                //         .set_text_color(0xef, 0xef, 0xef)
                //         .set_color(0x17, 0x1a, 0x21)
                //     });
                ;
                });
            })
            .add<Rect>([w, h](Rect& r) { r
                .setRect(w - 300, 0, 300, h)
                .setColor(0x27, 0x2a, 0x31);
            });
        })
        .run();        
    return 0;
}