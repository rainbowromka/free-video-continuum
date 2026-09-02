use std::any::Any;
use crate::ui::{elements::base::BaseElement};

pub trait Widget {
    fn as_any(&self) -> &dyn Any;
    fn base(&self) -> &BaseElement;
    fn update_from(&mut self, other: &dyn Widget);
    fn mark_quad_dirty(&mut self); 
    fn create_textures(&mut self);
    fn put_textures(&self, parent_w: u32, parent_h: u32);
}

pub trait AddElement {
    fn add<T>(&mut self, configure: impl FnOnce(&mut T)) -> &mut Self
    where
        T: Widget + Default + 'static;
}