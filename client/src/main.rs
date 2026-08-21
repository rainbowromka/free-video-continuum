mod ui;

use ui::rect::Rect;
use ui::window::Window;

fn main() {
    let mut rect = Rect::new(100.0, 100.0, 200.0, 150.0);

    Window::new()
        .with_size(1024.0, 780.0)
        .with_title("Free Video Continuum")
        .on_draw(move |renderer| {
            rect.draw(renderer.program(), renderer.window_width(), renderer.window_height());
        })
        .run();
}