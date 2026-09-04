mod ui;

use ui::window::Window;

use crate::ui::{elements::{hbox::HBox, rect::Rect, text::Text}, ui::Ui};
use crate::ui::elements::common::AddElement;

fn main() {
    Window::new()
        .with_size(1024, 780)
        .with_title("Free Video Continuum")
        .set_client_color(0x17, 0x1a, 0x21)                
        .on_draw(|ui: &mut Ui| {
            
            let w = ui.client_width();
            let h = ui.client_height();

            ui.add::<Rect>(|r| { r
                .set_rect(0, 0, 390, h)
                .set_color(0x27, 0x2a, 0x31)
                .add::<Rect>(|r|{ r
                    .set_rect(10,10, 200, 22)
                    .set_color(0x87, 0x2a, 0x31)
                    .add::<Text>(|t| { t
                        .set_content("Медиа:")
                        .set_position(1, 1)
                        .set_height(20)
                        .set_text_color(0xff, 0xff, 0xff)
                        .set_color(0x37, 0x3a, 0x41)
                    });
                })
                .add::<Rect>(|r|{ r
                    .set_rect(10,36, 370, 20)
                    .set_color(0x87, 0x2a, 0x31)
                    .add::<Text>(|t| { t
                        .set_content("[d]Mamay")
                        .set_position(1, 2)
                        .set_height(16)
                        .set_text_color(0xff, 0xff, 0xff)
                        .set_color(0x37, 0x3a, 0x41)
                    });
                })
                // .add<HBoxText>(|h|{});
                // .add::<HBox>(|h|{h
                //     .set_rect(10,36, 370, 20)
                //     .set_color(0x87, 0x2a, 0x31)
                //     .add::<Text>(|t| {t
                //         .set_content("[d]Mamay")
                //         .set_height(16)
                //         .set_text_color(0xff, 0xff, 0xff)
                //         .set_color(0x37, 0x3a, 0x41)
                //     })
                //     .add::<Text>(|t| { t
                //         .set_content("[r]MyVideo")
                //         .set_height(16)
                //         .set_text_color(0xef, 0xef, 0xef)
                //         .set_color(0x37, 0x3a, 0x41)
                //     })
                //     .add::<Text>(|t| { t
                //         .set_content("[e]Новый год")
                //         .set_height(16)
                //         .set_text_color(0xef, 0xef, 0xef)
                //         .set_color(0x37, 0x3a, 0x41)
                //     });
                });
            })
            .add::<Rect>(|r| {
                r.set_rect(w - 300, 0, 300, h, );
                r.set_color(0x27, 0x2a, 0x31);
            });
        })
        .run();
}