use crate::ui::{elements::{base::BaseElement, widget::Widget}, renderer::Renderer};
use std::{any::Any, ops::{Deref, DerefMut}};

pub struct Text {
    pub base: BaseElement,
    pub content: String,
    text_color: (f32, f32, f32),
}

impl Text {
    pub fn new(text: &str) -> Self {
        let x = 0;
        let y = 0;
        let width = text.len() as u32 * 8;  // временно: 8px на символ
        let height = 24;  // временно: высота строки

        Self {
            base: BaseElement::new(x, y, width, height),
            content: text.to_string(),
            text_color: (1.0, 1.0, 1.0),  // белый текст
        }
    }

    pub fn set_text_color(&mut self, r: u8, g: u8, b: u8) -> &mut Self {
        self.text_color = (
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
        );
        self.base.content_dirty = true;
        self
    }
}

impl Widget for Text {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn base(&self) -> &BaseElement {
        &self.base
    }
    
    fn update_from(&mut self, other: &dyn Widget) {
        if let Some(other_text) = other.as_any().downcast_ref::<Text>() {
            self.base.update_from(&other_text.base);
            
            if self.content != other_text.content {
                self.content = other_text.content.clone();
                self.base.content_dirty = true;
            }
        }
    }
    
    fn mark_quad_dirty(&mut self) {
        self.base.mark_quad_dirty();
    }
    
    fn draw(&mut self, texture_program: gl::types::GLuint, win_w: u32, win_h: u32) {
        self.base.draw(texture_program, win_w, win_h);
    }
}

impl Deref for Text {
    type Target = BaseElement;

    fn deref(&self) -> &BaseElement {
        &self.base
    }
}

impl DerefMut for Text {
    fn deref_mut(&mut self) -> &mut BaseElement {
        &mut self.base
    }
}