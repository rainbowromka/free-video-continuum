mod ui;

use ui::window::Window;

use crate::ui::ui::Ui;

fn main() {
    Window::new()
        .with_size(1024, 780)
        .with_title("Free Video Continuum")
        .on_draw(|ui: &mut Ui| {
            ui.add_rect(
                0,
                0,
                200,
                ui.client_height(),
            );
// Идея такая, что ширина левой панели должна быть всегда 200 пикселей в координатах экрана (окна), 
// и должна пересчитываться с учетом изменения окна. Есть подозрение что при ресайзе renderer не обновляет ширину/высоту.
        })
        .run();
}