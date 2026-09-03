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

            ui.add::<Rect>(|r| {
                r.set_rect(0, 0, 390, h);
                r.set_color(0x27, 0x2a, 0x31);

                r.add::<Rect>(|r|{
                    r.set_rect(10,10, 200, 22);
                    r.set_color(0x87, 0x2a, 0x31);
                    r.add::<Text>(|t| {                
                        t.set_content("Медиа:");
                        t.set_position(1, 1);
                        t.set_height(20);
                        t.set_text_color(0xff, 0xff, 0xff);       // чёрный фон
                        t.set_color(0x37, 0x3a, 0x41);  // белый текст
                    });
                });

                r.add::<Rect>(|r|{
                    r.set_rect(10,36, 370, 20);
                    r.set_color(0x87, 0x2a, 0x31);
                    r.add::<Text>(|t| {                
                        t.set_content("[d]Mamay");
                        t.set_position(1, 2);
                        t.set_height(16);
                        t.set_text_color(0xff, 0xff, 0xff);       
                        t.set_color(0x37, 0x3a, 0x41);  
                    });
                    r.add::<Text>(|t| {
                        t.set_content("[r]MyVideo");
                        t.set_position(80, 2);
                        t.set_height(16);
                        t.set_text_color(0xef, 0xef, 0xef);
                        t.set_color(0x37, 0x3a, 0x41);  
                    });
                });
            })
            .add::<Rect>(|r| {
                r.set_rect(w - 300, 0, 300, h, );
                r.set_color(0x27, 0x2a, 0x31);
            });
        })
        .run();
}