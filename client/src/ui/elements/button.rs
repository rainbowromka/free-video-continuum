use crate::ui::{elements::{base::BaseElement, widget::Widget}};
use std::{any::Any};
use crate::ui::render::shader::TEXTURE_PROGRAM;

pub struct Button {
    pub base: BaseElement,
    min_width: u32,
    min_height: u32,
}

impl Button {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            base: BaseElement::new(x, y, width, height),
            min_width: width,
            min_height: height,
        }
    }

    pub fn set_rect(&mut self, x: u32, y: u32, width: u32, height: u32) -> &mut Self {
        self.base.x = x;
        self.base.y = y;
        self.min_width = width;
        self.min_height = height;
        self
    }

    pub fn set_color(&mut self, r: u8, g: u8, b: u8) -> &mut Self {
        self.base.set_color(r, g, b);
        self
    }
}

impl Widget for Button {    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn base(&mut self) -> &mut BaseElement {
        &mut self.base
    }

    fn set_position(&mut self, x: u32, y: u32) {
        self.base.set_position(x, y);
    }

    fn diff(&mut self, other: &dyn Widget) {
        if let Some(other_rect) = other.as_any().downcast_ref::<Button>() {
            self.base.diff(&other_rect.base);
        }
    }    
    
    fn create_textures(&mut self) -> bool {
        let mut dirty = false;
        
        for element in &mut self.base.children {
            if element.create_textures() {
                dirty = true;
            }
        }

        if dirty {
            self.base.mark_dirty();
        }

        self.base.lazy_init();

        if self.base.dirty {
            let w = self.base.width;
            let h = self.base.height;

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

                for element in &mut self.base.children {
                    gl::Viewport(0, 0, w as i32, h as i32);
                    element.draw(w, h);
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

    fn layout(&mut self) {
        // Шаг 1: читаем размеры ребёнка (immutable borrow)
        let child_size = {
            if let Some(child) = self.base.children.first() {
                let b = child;
                Some((b.width, b.height))
            } else {
                None
            }
        };

        // Шаг 2: если есть ребёнок — обновляем размеры кнопки
        if let Some((child_w, child_h)) = child_size {
            let new_width = self.min_width.max(child_w);
            let new_height = self.min_height.max(child_h);

            if self.base.width != new_width || self.base.height != new_height {
                self.base.width = new_width;
                self.base.height = new_height;
                self.base.recreate_fbo();
                self.base.mark_dirty();
            }

            // Шаг 3: позиционируем ребёнка (отдельный mutable borrow)
            let x = (self.base.width - child_w) / 2;
            let y = (self.base.height - child_h) / 2;

            if let Some(child) = self.base.children.first_mut() {
                child.set_position(x, y);
            }
        }
    }

    fn push_child(&mut self, child: Box<dyn Widget>) {
        self.base.children.push(child);
    }
}

impl Default for Button {
    fn default() -> Self {
        Button::new(0, 0, 0, 0)
    }
}
