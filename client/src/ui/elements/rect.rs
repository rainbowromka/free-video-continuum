use crate::ui::{elements::{base::BaseElement, widget::Widget}};
use std::{any::Any, ops::{Deref, DerefMut}};

pub struct Rect {
    pub base: BaseElement,
}

impl Rect {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            base: BaseElement::new(x, y, width, height)
        }
    }
}

impl Widget for Rect {    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn base(&self) -> &BaseElement {
        &self.base
    }    
    
    fn update_from(&mut self, other: &dyn Widget) {
        if let Some(other_rect) = other.as_any().downcast_ref::<Rect>() {
            self.base.update_from(&other_rect.base);
        }    
    }            
    
    fn mark_quad_dirty(&mut self) {
        self.base.mark_quad_dirty();
    }        
    
    fn draw(&mut self, win_w: u32, win_h: u32) {
        self.base.draw(win_w, win_h);
    }
}

impl Deref for Rect {
    type Target = BaseElement;

    fn deref(&self) -> &BaseElement {
        &self.base
    }
}

impl DerefMut for Rect {
    fn deref_mut(&mut self) -> &mut BaseElement {
        &mut self.base
    }
}