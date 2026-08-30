pub struct Rect {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    color: (f32, f32, f32),
    content_dirty: bool,
    quad_dirty: bool,


    fbo: Option<gl::types::GLuint>,
    texture: Option<gl::types::GLuint>,
    quad_vao: Option<gl::types::GLuint>,
    quad_vbo: Option<gl::types::GLuint>,
}

impl Rect {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            color: (0.2, 0.6, 1.0),  // голубой по умолчанию
            content_dirty: true,
            quad_dirty: true,
            fbo: None,
            texture: None,
            quad_vao: None,
            quad_vbo: None,
        }
    }

    pub fn set_color(&mut self, r: u8, g: u8, b: u8) -> &mut Self {
        let new_color = (
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
        );
        if self.color != new_color {
            self.color = new_color;
            self.content_dirty = true;
        }
        self
    }

    pub fn color(&self) -> (f32, f32, f32) {
        self.color
    }
    
    pub fn set_position(&mut self, x: u32, y: u32) {
        if self.x != x || self.y != y {
            self.x = x;
            self.y = y;
            self.quad_dirty = true;
        }
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        if self.width != width || self.height != height {
            self.width = width;
            self.height = height;
            self.recreate_fbo();
            self.content_dirty = true;
            self.quad_dirty = true;
        }
    }

    pub fn mark_quad_dirty(&mut self) {
        self.quad_dirty = true;
    }

    pub fn draw(&mut self, texture_program: gl::types::GLuint, window_width: u32, window_height: u32) {
        // Ленивая инициализация
        self.lazy_init();

        // Полная перерисовка (содержимое + позиция)
        if self.content_dirty {
            self.full_redraw(window_width, window_height);
        }
        // Только позиция
        else if self.quad_dirty {
            self.redraw_position(window_width, window_height);
        }

        // Всегда отображаем готовую текстуру
        self.redraw(texture_program);
    }

    fn lazy_init(&mut self) {
        if self.fbo.is_none() {
            self.init_fbo();
        }
        if self.quad_vao.is_none() {
            self.init_quad_buffers();
        }
    }

    fn full_redraw(&mut self, window_width: u32, window_height: u32) {
        // Перерисовать содержимое в FBO
        self.render_to_fbo();

        // Восстановить viewport
        unsafe {
            gl::Viewport(0, 0, window_width as i32, window_height as i32);
        }

        // Обновить позицию на экране
        self.redraw_position(window_width, window_height);

        self.content_dirty = false;
    }

    fn redraw_position(&mut self, window_width: u32, window_height: u32) {
        let vertices = self.update_quad_vertices(window_width, window_height);

        unsafe {
            gl::BindVertexArray(self.quad_vao.unwrap());
            gl::BindBuffer(gl::ARRAY_BUFFER, self.quad_vbo.unwrap());
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                vertices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
        }

        self.quad_dirty = false;
    }

    fn redraw(&self, texture_program: gl::types::GLuint) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture.unwrap());

            gl::UseProgram(texture_program);
            gl::BindVertexArray(self.quad_vao.unwrap());
            gl::DrawArrays(gl::TRIANGLES, 0, 6);

            gl::BindVertexArray(0);
        }
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

    fn recreate_fbo(&mut self) {
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

    fn render_to_fbo(&self) {
        unsafe {
            // Рисуем в FBO вместо экрана
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo.unwrap());

            gl::Viewport(0, 0, self.width as i32, self.height as i32);

            // Очищаем цветом Rect
            gl::ClearColor(self.color.0, self.color.1, self.color.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // Возвращаемся к обычному экрану
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
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

    fn update_quad_vertices(&self, window_width: u32, window_height: u32) -> [f32; 24] {
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

        // x, y, u, v
        [
            left, top, 0.0, 0.0,       // верхний левый
            left, bottom, 0.0, 1.0,    // нижний левый
            right, bottom, 1.0, 1.0,   // нижний правый
            left, top, 0.0, 0.0,       // верхний левый
            right, bottom, 1.0, 1.0,   // нижний правый
            right, top, 1.0, 0.0,      // верхний правый
        ]
    }

    pub fn update_from(&mut self, new: Rect) {
        // Позиция изменилась
        if self.x != new.x || self.y != new.y {
            self.x = new.x;
            self.y = new.y;
            self.quad_dirty = true;
        }

        // Размер изменился
        if self.width != new.width || self.height != new.height {
            self.width = new.width;
            self.height = new.height;
            self.recreate_fbo();
            self.content_dirty = true;
            self.quad_dirty = true;
        }

        // Цвет изменился
        if self.color != new.color {
            self.color = new.color;
            self.content_dirty = true;
        }
    }
}

impl Drop for Rect {
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

impl PartialEq for Rect {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
            && self.y == other.y
            && self.width == other.width
            && self.height == other.height
            && self.color == other.color
    }
}