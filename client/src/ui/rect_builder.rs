use crate::ui::rect::Rect;
use crate::ui::ui::Ui;

pub struct RectBuilder<'a> {
    ui: &'a mut Ui,
    rect: Rect,
}

impl<'a> RectBuilder<'a> {
    pub fn new(ui: &'a mut Ui, x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            ui,
            rect: Rect::new(x, y, width, height),
        }
    }

    pub fn set_color(mut self, r: u8, g: u8, b: u8) -> Self {
        self.rect.set_color(r, g, b);
        self
    }

    pub fn build(self) -> &'a mut Ui {
        self.ui.push_rect(self.rect);
        self.ui
    }
}