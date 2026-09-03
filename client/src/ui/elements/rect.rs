use crate::ui::elements::{self, base::BaseElement, common::{AddElement, Widget}};
use std::{any::Any, ops::{Deref, DerefMut}};
use crate::ui::render::shader::TEXTURE_PROGRAM;

pub struct Rect {
    pub base: BaseElement,
    children: Vec<Box<dyn Widget>>,
}

impl Rect {
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
}

impl Widget for Rect {    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn base(&self) -> &BaseElement {
        &self.base
    }    
    
    // fn update_from(&mut self, other: &dyn Widget) {
    //     if let Some(other_rect) = other.as_any().downcast_ref::<Rect>() {
    //         self.base.update_from(&other_rect.base);
    //     }    
    // }        

    fn update_from(&mut self, other: &dyn Widget) {
        if let Some(other_rect) = other.as_any().downcast_ref::<Rect>() {
            self.base.update_from(&other_rect.base);
            
            // Если размер Rect изменился — дети должны перерисоваться
            if self.base.width != other_rect.base.width || 
            self.base.height != other_rect.base.height {
                for child in &mut self.children {
                    child.mark_quad_dirty();
                }
            }
        }
    }    
    
    fn mark_quad_dirty(&mut self) {
        self.base.mark_quad_dirty();
    }      

    fn mark_content_dirty(&mut self) {
        self.base.mark_content_dirty();
    }      

    fn create_textures(&mut self) -> bool {
        let mut result = false;
        
        for element in &mut self.children {
            if element.create_textures() {
                result = true;
            }
        }

        if result {
            self.base.mark_quad_dirty();
            self.base.mark_content_dirty();
        }

        // Рисуем свой FBO
        self.base.lazy_init();

        println!("Rect create_textures: content_dirty={}, size={}x{}", 
            self.base.content_dirty, 
            self.base.width, 
            self.base.height
        );

        if self.base.content_dirty {
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

            self.base.content_dirty = false;

            result = true;
        }

        result
    }

    fn draw(&mut self, parent_w: u32, parent_h: u32) {
        println!("Rect draw: x={}, y={}, w={}, h={}, parent={}x{}",
            self.base.x, self.base.y, self.base.width, self.base.height, parent_w, parent_h);

        if self.base.quad_dirty {
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

            self.base.quad_dirty = false;
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

    fn is_content_dirty(&self) -> bool
    {
        self.base.content_dirty
    }

    fn is_quad_dirty(&self) -> bool
    {
        self.base.quad_dirty
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

impl Default for Rect {
    fn default() -> Self {
        Rect::new(0, 0, 0, 0)
    }
}

impl AddElement for Rect {
    fn add<T>(&mut self, configure: impl FnOnce(&mut T)) -> &mut Self
    where
        T: Widget + Default + 'static,
    {
        let mut element = T::default();
        configure(&mut element);
        self.children.push(Box::new(element));
        self
    }
}