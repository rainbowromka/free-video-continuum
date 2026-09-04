use std::any::Any;

use crate::ui::elements::base::BaseElement;

pub trait Widget {
    fn as_any(&self) -> &dyn Any;    
    fn base(&self) -> &BaseElement;
    fn set_position(&mut self, x: u32, y: u32);
    fn update_from(&mut self, other: &dyn Widget);
    fn mark_dirty_recursive(&mut self);
    fn create_textures(&mut self) -> bool;
    fn draw(&mut self, parent_w: u32, parent_h: u32);
    fn layout(&mut self);
}

pub trait AddElement {
    fn add<T>(&mut self, configure: impl FnOnce(&mut T)) -> &mut Self
    where
        T: Widget + Default + 'static;
}