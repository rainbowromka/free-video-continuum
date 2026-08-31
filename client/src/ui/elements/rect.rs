use crate::ui::elements::{base::{BaseElement}};
use std::ops::{Deref, DerefMut};

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

impl PartialEq for Rect {
    fn eq(&self, other: &Self) -> bool {
        self.base == other.base
    }
}

// impl Widget for Rect {
//     // fn set_color(&mut self, r: u8, g: u8, b: u8) -> &mut Self {
//     //     self.base.set_color(r, g, b);
//     //     self    
//     // }

//     // pub fn mark_quad_dirty(&mut self) {
//     //     self.base.mark_quad_dirty();
//     // }

//     // pub fn update_from(&mut self, new: BaseElement) {
//     //     self.base.update_from(new);
//     // }    
// }