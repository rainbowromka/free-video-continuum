use crate::ui::rect::Rect;
use crate::ui::renderer::Renderer;

pub struct Ui {
    rects: Vec<Rect>,
    new_rects: Vec<Rect>,
    client_width: u32,
    client_height: u32,
}

impl Ui {
    pub fn new(client_width: u32, client_height: u32) -> Self {
        Self {
            rects: Vec::new(),
            new_rects: Vec::new(),
            client_width,
            client_height,
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

    pub fn add_rect(&mut self, x: u32, y: u32, width: u32, height: u32) {
        self.new_rects.push(Rect::new(x, y, width, height));
    }

    pub fn render(&mut self, renderer: &Renderer) {
        // Очищаем старые
        self.rects.clear();

        // Переносим новые
        for rect in self.new_rects.drain(..) {
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
}