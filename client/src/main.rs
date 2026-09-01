mod ui;

use ui::window::Window;

use crate::ui::ui::Ui;

fn main() {
    Window::new()
        .with_size(1024, 780)
        .with_title("Free Video Continuum")
        .set_client_color(0x17, 0x1a, 0x21)                
        .on_draw(|ui: &mut Ui| {

            let w = ui.client_width();
            let h = ui.client_height();


            ui.add_rect(0, 0, 390, h, |rect| {
                rect.set_color(0x27, 0x2a, 0x31);
            })
            .add_rect(w - 300, 0, 300, h, |rect| {
                rect.set_color(0x27, 0x2a, 0x31);
            })
            .add_text("[d] Mamay", |text| {                
                text.set_position(200, 20);
                text.set_color(0x00, 0x00, 0x00);
            })
            ;
        })
        .run();
}