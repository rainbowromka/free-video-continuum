mod ui;

use ui::window::Window;

use crate::ui::{elements::{rect::Rect, text::Text}, ui::Ui};
use crate::ui::elements::common::AddElement;

fn main() {
    Window::new()
        .with_size(1024, 780)
        .with_title("Free Video Continuum")
        .set_client_color(0x17, 0x1a, 0x21)                
        .on_draw(|ui: &mut Ui| {
            
            let w = ui.client_width();
            let h = ui.client_height();
            println!("w={}, h={}", w, h);

            ui.add::<Rect>(|rect| {
                rect.set_rect(0, 0, 390, h);
                rect.set_color(0x27, 0x2a, 0x31);
                rect.add::<Text>(|text| {                
                    text.set_content("[d]Mamay");
                    text.set_position(10, 20);
                    text.set_height(16);
                    text.set_text_color(0xff, 0xff, 0xff);       // чёрный фон
                    text.set_color(0x37, 0x3a, 0x41);  // белый текст
                });
            })
            .add::<Rect>(|rect| {
                rect.set_rect(w - 300, 0, 300, h, );
                rect.set_color(0x27, 0x2a, 0x31);
            })
            // .add::<Text>(|text| {                
            //     text.set_content("[d]Mamay");
            //     text.set_position(10, 20);
            //     text.set_height(16);
            //     text.set_text_color(0xff, 0xff, 0xff);       // чёрный фон
            //     text.set_color(0x37, 0x3a, 0x41);  // белый текст
            // })
            ;
        })
        .run();
}