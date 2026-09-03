use std::any::Any;
use crate::ui::{elements::base::BaseElement};

pub trait Widget {
    fn as_any(&self) -> &dyn Any;
    fn base(&self) -> &BaseElement;
    fn update_from(&mut self, other: &dyn Widget);
    fn mark_dirty(&mut self); 
    fn mark_dirty_recursive(&mut self);
    fn is_dirty(&self) -> bool;
    fn create_textures(&mut self) -> bool;
    fn draw(&mut self, parent_w: u32, parent_h: u32);
}

pub trait AddElement {
    fn add<T>(&mut self, configure: impl FnOnce(&mut T)) -> &mut Self
    where
        T: Widget + Default + 'static;
}