use std::any::Any;
use crate::ui::{elements::base::BaseElement};

pub trait Widget {
    fn as_any(&self) -> &dyn Any;
    fn base(&self) -> &BaseElement;
    fn update_from(&mut self, other: &dyn Widget);
    fn mark_quad_dirty(&mut self); 
    fn draw(&mut self, win_w: u32, win_h: u32);
    // fn set_position(&mut self, x: u32, y: u32);
    // fn set_size(&mut self, width: u32, height: u32);
    // fn set_color(&mut self, r: u8, g: u8, b: u8);
    // fn draw(&mut self, renderer: &Renderer, win_w: u32, win_h: u32);
    // fn is_dirty(&self) -> bool;
    // fn mark_clean(&mut self);
}