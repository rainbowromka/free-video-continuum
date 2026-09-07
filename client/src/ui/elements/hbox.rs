use crate::ui::elements::base::BaseElement;
use crate::ui::elements::widget::Widget;
use crate::ui::render::shader::TEXTURE_PROGRAM;
use std::ops::{Deref, DerefMut};

pub struct HBox {
    pub base: BaseElement,
    children: Vec<Box<dyn Widget>>,
}

impl HBox {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            base: BaseElement::new(x, y, width, height),
            children: Vec::new(),
        }
    }

    pub fn set_rect(&mut self, x: u32, y: u32, width: u32, height: u32) -> &mut Self {
        self.base.x = x;
        self.base.y = y;
        self.base.width = width;
        self.base.height = height;
        self
    }

    pub fn set_color(&mut self, r: u8, g: u8, b: u8) -> &mut Self {
        self.base.set_color(r, g, b);
        self
    }
}

impl Widget for HBox {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn base(&mut self) -> &mut BaseElement {    
        &mut self.base
    }

    fn set_position(&mut self, x: u32, y: u32) {
        self.base.set_position(x, y);
    }
    
    fn create_textures(&mut self) -> bool {    
        let mut dirty = false;

        for element in &mut self.children {
            if element.create_textures() {
                dirty = true;
            }
        }

        if dirty {
            self.base.mark_dirty();
        }

        self.base.lazy_init();

        if self.base.dirty {
            unsafe {
                gl::BindFramebuffer(gl::FRAMEBUFFER, self.base.fbo.unwrap());
                gl::Viewport(0, 0, self.base.width as i32, self.base.height as i32);

                // Фон
                gl::ClearColor(
                    self.base.color().0,
                    self.base.color().1,
                    self.base.color().2,
                    1.0,
                );
                gl::Clear(gl::COLOR_BUFFER_BIT);

                // Дети поверх
                for element in &mut self.children {
                    gl::Viewport(0, 0, self.base.width as i32, self.base.height as i32);
                    element.draw(self.base.width, self.base.height);
                }

                gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            }
        }

        dirty
    }

    fn draw(&mut self, parent_w: u32, parent_h: u32) {
        if self.base.dirty {
            let vertices = self.base.update_quad_vertices(parent_w, parent_h);

            unsafe {
                gl::BindVertexArray(self.base.quad_vao.unwrap());
                gl::BindBuffer(gl::ARRAY_BUFFER, self.base.quad_vbo.unwrap());
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                    vertices.as_ptr() as *const gl::types::GLvoid,
                    gl::STATIC_DRAW,
                );
            }

            self.base.dirty = false;
        }

        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.base.texture.unwrap());
            gl::UseProgram(*TEXTURE_PROGRAM);
            gl::BindVertexArray(self.base.quad_vao.unwrap());
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            gl::BindVertexArray(0);
        }
    }

    fn mark_dirty_recursive(&mut self) {
        self.base.mark_dirty();
        for child in &mut self.children {
            child.mark_dirty_recursive();
        }        
    }

    fn diff(&mut self, other: &dyn Widget) {
        if let Some(other_rect) = other.as_any().downcast_ref::<Self>() {
            self.base.diff(&other_rect.base);
        }
    }

    fn layout(&mut self) {
        let mut x = 0;
        for child in &mut self.children {
            let y = (self.base.height - child.base().height) / 2;
            child.set_position(x, y);
            x += child.base().width;
        }        
    }

    fn push_child(&mut self, child: Box<dyn Widget>) {
        self.children.push(child);
    }
}

impl Deref for HBox {
    type Target = BaseElement;

    fn deref(&self) -> &BaseElement {
        &self.base
    }
}

impl DerefMut for HBox {
    fn deref_mut(&mut self) -> &mut BaseElement {
        &mut self.base
    }
}

impl Default for HBox {
    fn default() -> Self {
        Self::new(0, 0, 0, 0)
    }
}

// impl AddElement for HBox {
//     fn add<T>(&mut self, configure: impl FnOnce(&mut T)) -> &mut Self
//     where
//         T: Widget + Default + 'static,
//     {
//         let mut element = T::default();
//         configure(&mut element);
//         element.layout();
//         self.children.push(Box::new(element));
//         self
//     }
// }