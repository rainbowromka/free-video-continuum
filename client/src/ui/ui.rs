use crate::ui::rect::Rect;
use crate::ui::renderer::Renderer;

pub struct Ui {
    rects: Vec<Rect>,
    new_rects: Vec<Rect>,
    client_width: u32,
    client_height: u32,
    bg_color: (f32, f32, f32),
}

impl Ui {
    pub fn new(client_width: u32, client_height: u32) -> Self {
        Self {
            rects: Vec::new(),
            new_rects: Vec::new(),
            client_width,
            client_height,
            bg_color: hex_to_rgb(0x16, 0x19, 0x20),
        }
    }

    pub fn resize(&mut self, client_width: u32, client_height: u32) {
        self.client_width = client_width;
        self.client_height = client_height;
    }

    pub fn client_width(&self) -> u32 {
        self.client_width
    }

    pub fn client_height(&self) -> u32 {
        self.client_height
    }

    pub fn add_rect(&mut self, x: u32, y: u32, width: u32, height: u32) -> &mut Self {
        self.new_rects.push(Rect::new(x, y, width, height));
        self
    }

    pub fn render(&mut self, renderer: &Renderer) {
        // Очищаем старые
        self.rects.clear();

        // Переносим новые
        for rect in self.new_rects.drain(..) {
            // todo: вот тут надо сравнивать ректы между собой и если они разные, то помечать как грязный (требующий обновления)
            // т.е. если поменялось 1. состояние, 2 координаты, 3 размеры, то помечаем как грязный. Появился новый или старый.
            // что считать обновлением, ui должен рисоваться в текстуру, текстура уже рисуется в rect и запоминается. Если элемент 
            // чистый, то в момент перерисовки перерисовывается старая тектура, если элемент грязный, то перерисовывается сначала
            // текстура, потом элемент с новой текстурой.
            self.rects.push(rect);
        }

        // Рендерим
        for rect in &mut self.rects {
            rect.draw(
                renderer.program(),
                self.client_width,
                self.client_height,
            );
        }

        self.new_rects.clear();
    }

    pub fn set_client_color(&mut self, r: u8, g: u8, b: u8) -> &mut Self {
        self.bg_color = hex_to_rgb(r, g, b);
        self
    }

    pub fn bg_color(&self) -> (f32, f32, f32) {
        self.bg_color
    }
}

fn hex_to_rgb(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    (r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
}
