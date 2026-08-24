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

            ui
                .add_rect(0, 0, 200, h)
                .add_rect(w - 300,0,w,h);
        })
        .run();
}