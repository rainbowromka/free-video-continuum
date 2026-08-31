use crate::ui::renderer::Renderer;

pub trait Widget {
    // fn set_position(&mut self, x: u32, y: u32);
    // fn set_size(&mut self, width: u32, height: u32);
    fn set_color(&mut self, r: u8, g: u8, b: u8) -> &mut Self;
    // fn draw(&mut self, renderer: &Renderer, win_w: u32, win_h: u32);
    // fn is_dirty(&self) -> bool;
    // fn mark_clean(&mut self);
}