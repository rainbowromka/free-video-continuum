use crate::ui::{
    elements::{base::BaseElement, widget::Widget},
    render::shader::{TEXTURE_PROGRAM,TEXT_PROGRAM}
};
use std::{any::Any, ops::{Deref, DerefMut}};
use crate::ui::font::FONT_MANAGER;


pub struct Text {
    pub base: BaseElement,
    pub content: String,
    text_color: (f32, f32, f32),
}

impl Text {
    pub fn new(content: &str) -> Self {
        let mut text = Self {
            base: BaseElement::new(0, 0, content.len() as u32 * 16, 32),
            content: content.to_string(),
            text_color: (1.0, 1.0, 1.0),
        };
        text.set_color(0xff, 0xff, 0xff);
        text
    }

    pub fn set_text_color(&mut self, r: u8, g: u8, b: u8) -> &mut Self {
        self.text_color = (
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
        );
        self
    }

    pub fn set_height(&mut self, height: u32) -> &mut Self {
        if self.base.height != height {
            self.base.height = height;
            
            // Пересчитываем ширину из текста
            let (width, _) = FONT_MANAGER.measure_text(&self.content, height as f32);
            self.base.width = width;
        }
        self
    }
}

impl Text {
    pub fn set_content(&mut self, content: &str) -> &mut Self {
        self.content = content.to_string();
        self
    }

    pub fn set_position(&mut self, x: u32, y: u32) -> &mut Self{
        self.base.set_position(x, y);
        self
    }

    fn render_text(&self) {
        let font_manager = &*FONT_MANAGER;

        let size = self.base.height as f32;
        let (bitmap, width, height) = font_manager.rasterize_text(&self.content, size);
        
        // println!("Text render: width={}, height={}, content='{}'", width, height, self.content);
        if width == 0 || height == 0 {
            return;
        }

        let mut texture = 0;
        unsafe {
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexImage2D(
                gl::TEXTURE_2D, 0, gl::RGBA as i32,
                width as i32, height as i32, 0,
                gl::RGBA, gl::UNSIGNED_BYTE,
                bitmap.as_ptr() as *const _,
            );
            // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            // Включаем альфа-смешивание
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            // Используем TEXT_PROGRAM
            gl::UseProgram(*TEXT_PROGRAM);

            // Передаём цвет текста в шейдер
            let color_location = gl::GetUniformLocation(
                *TEXT_PROGRAM,
                b"text_color\0".as_ptr() as *const gl::types::GLchar,
            );
            gl::Uniform3f(
                color_location,
                self.text_color.0,
                self.text_color.1,
                self.text_color.2,
            );

            // Привязываем текстуру
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture);

            // Рисуем quad на весь FBO
            gl::BindVertexArray(self.base.quad_vao.unwrap());

            let vertices: [f32; 24] = [
                -1.0, 1.0, 0.0, 0.0,    // верхний левый → V=0
                -1.0, -1.0, 0.0, 1.0,   // нижний левый → V=1
                1.0, -1.0, 1.0, 1.0,    // нижний правый → V=1
                -1.0, 1.0, 0.0, 0.0,    // верхний левый → V=0
                1.0, -1.0, 1.0, 1.0,    // нижний правый → V=1
                1.0, 1.0, 1.0, 0.0,     // верхний правый → V=0
            ];            

            gl::BindVertexArray(self.base.quad_vao.unwrap());
            gl::BindBuffer(gl::ARRAY_BUFFER, self.base.quad_vbo.unwrap());
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                vertices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );

            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            gl::BindVertexArray(0);

            // Удаляем временную текстуру
            gl::DeleteTextures(1, &texture);
        }
    }
}

impl Widget for Text {
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
        if let Some(other_text) = other.as_any().downcast_ref::<Text>() {
            self.base.diff(&other_text.base);
            
            if self.content != other_text.content {
                self.content = other_text.content.clone();
                self.base.dirty = true;
            }
        }
    }

    fn create_textures(&mut self) -> bool {
        self.base.lazy_init();

        if self.base.dirty {
            // Рисуем в свой FBO
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

                // Буквы
                self.render_text();

                gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            }

            true
        } else {
            false
        }
    }

    fn draw(&mut self, parent_w: u32, parent_h: u32) {
        // Если позиция изменилась — обновить quad
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

        // Нарисовать свою текстуру в текущий FBO родителя
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
    }

    fn layout(&mut self) {        
    }
    
    fn push_child(&mut self, child: Box<dyn Widget>) {
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

impl Default for Text {
    fn default() -> Self {
        Text::new("")
    }
}