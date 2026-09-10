use crate::ui::elements::widget::Widget;

pub struct BaseElement {
    pub abs_x: u32,
    pub abs_y: u32,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub color: (f32, f32, f32),
    pub dirty: bool,
    
    pub z: u32,

    pub children: Vec<Box<dyn Widget>>,

    pub fbo: Option<gl::types::GLuint>,
    pub texture: Option<gl::types::GLuint>,
    pub quad_vao: Option<gl::types::GLuint>,
    pub quad_vbo: Option<gl::types::GLuint>,
}

impl BaseElement {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            abs_x: x,
            abs_y: y,
            x,
            y,
            width,
            height,
            color: (0.2, 0.6, 1.0),
            dirty: true,            
            z: 0,
            children: Vec::new(),
            fbo: None,
            texture: None,
            quad_vao: None,
            quad_vbo: None,
        }
    }

    pub fn set_rect(&mut self, x: u32, y: u32, width: u32, height: u32) {
        self.set_position(x, y);
        // self.x = x;
        // self.y = y;
        self.width = width;
        self.height = height;
    }

    pub fn set_position(&mut self, x: u32, y: u32) {
        // if self.x != x || self.y != y {
        self.abs_x = self.abs_x - self.x + x;
        self.abs_y = self.abs_y - self.y + y;
        self.x = x;
        self.y = y;
            // self.dirty = true;
        // }
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn recreate_fbo(&mut self) {
        if let Some(fbo) = self.fbo {
            unsafe {
                gl::DeleteFramebuffers(1, &fbo);
            }
        }
        if let Some(texture) = self.texture {
            unsafe {
                gl::DeleteTextures(1, &texture);
            }
        }

        self.init_fbo();
    }

    fn init_fbo(&mut self) {
        let mut fbo = 0;
        let mut texture = 0;

        unsafe {
            // 1. Создаём FBO
            gl::GenFramebuffers(1, &mut fbo);

            // 2. Создаём текстуру
            gl::GenTextures(1, &mut texture);

            // 3. Привязываем текстуру
            gl::BindTexture(gl::TEXTURE_2D, texture);

            // 4. Выделяем память в GPU — width × height, RGBA8
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                self.width as i32,
                self.height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                std::ptr::null(),
            );

            // 5. Фильтрация — линейная (при масштабировании сглаживать)
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            // 6. Привязываем FBO
            gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);

            // 7. Привязываем текстуру к FBO — теперь рисование в FBO попадёт в текстуру
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                texture,
                0,
            );

            // 8. Отвязываем FBO (возвращаемся к обычному экрану)
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }

        self.fbo = Some(fbo);
        self.texture = Some(texture);
    }

    pub fn update_quad_vertices(&self, window_width: u32, window_height: u32) -> [f32; 24] {
        let w = window_width as f32;
        let h = window_height as f32;
        let x = self.x as f32;
        let y = self.y as f32;
        let width = self.width as f32;
        let height = self.height as f32;

        // Позиция на экране в NDC
        let left = (x / w) * 2.0 - 1.0;
        let right = ((x + width) / w) * 2.0 - 1.0;
        let top = 1.0 - (y / h) * 2.0;
        let bottom = 1.0 - ((y + height) / h) * 2.0;

        [
            left, top, 0.0, 1.0,      // V=1
            left, bottom, 0.0, 0.0,   // V=0
            right, bottom, 1.0, 0.0,  // V=0
            left, top, 0.0, 1.0,      // V=1
            right, bottom, 1.0, 0.0,  // V=0
            right, top, 1.0, 1.0,     // V=1
        ]

    }

    pub fn diff(&mut self, new: &BaseElement) {
        // Позиция изменилась
        if self.x != new.x || self.y != new.y {
            self.x = new.x;
            self.y = new.y;
            self.dirty = true;
        }

        // Размер изменился
        if self.width != new.width || self.height != new.height {
            self.width = new.width;
            self.height = new.height;
            self.recreate_fbo();  // ← пересоздать FBO с новым размером
            self.dirty = true;
        }

        // Цвет изменился
        if self.color != new.color {
            self.color = new.color;
            self.dirty = true;
        }
    }

    pub fn lazy_init(&mut self) {
        if self.fbo.is_none() {
            self.init_fbo();
        }
        if self.quad_vao.is_none() {
            self.init_quad_buffers();
        }
    }

    fn init_quad_buffers(&mut self) {
        let mut quad_vao = 0;
        let mut quad_vbo = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut quad_vao);
            gl::GenBuffers(1, &mut quad_vbo);

            gl::BindVertexArray(quad_vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, quad_vbo);

            // Атрибут 0: позиция (x, y) — 2 float
            gl::VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE,
                (4 * std::mem::size_of::<f32>()) as gl::types::GLsizei,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            // Атрибут 1: UV-координаты (u, v) — 2 float, смещение 2 float от начала
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                (4 * std::mem::size_of::<f32>()) as gl::types::GLsizei,
                (2 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );
            gl::EnableVertexAttribArray(1);

            gl::BindVertexArray(0);
        }

        self.quad_vao = Some(quad_vao);
        self.quad_vbo = Some(quad_vbo);
    }    

    pub fn set_color(&mut self, r: u8, g: u8, b: u8) {
        let new_color = (
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
        );
        if self.color != new_color {
            self.color = new_color;
            self.dirty = true;
        }
    }

    pub fn color(&self) -> (f32, f32, f32) {
        self.color
    }

    pub fn set_color_raw(&mut self, r: f32, g: f32, b: f32) {
        if self.color != (r, g, b) {
            self.color = (r, g, b);
            self.dirty = true;
        }
    }

    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        x >= self.abs_x as f32
            && x <= (self.abs_x + self.width) as f32
            && y >= self.abs_y as f32
            && y <= (self.abs_y + self.height) as f32
    }
}

impl Drop for BaseElement {
    fn drop(&mut self) {
        unsafe {
            if let Some(fbo) = self.fbo {
                gl::DeleteFramebuffers(1, &fbo);
            }
            if let Some(texture) = self.texture {
                gl::DeleteTextures(1, &texture);
            }
            if let Some(vao) = self.quad_vao {
                gl::DeleteVertexArrays(1, &vao);
            }
            if let Some(vbo) = self.quad_vbo {
                gl::DeleteBuffers(1, &vbo);
            }
        }
    }
}
