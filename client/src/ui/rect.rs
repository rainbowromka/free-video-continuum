pub struct Rect {
    vao: Option<gl::types::GLuint>,
    vbo: Option<gl::types::GLuint>,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    dirty: bool,
}

impl Rect {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            vao: None,
            vbo: None,
            x,
            y,
            width,
            height,
            dirty: true,
        }
    }

    pub fn set_position(&mut self, x: u32, y: u32) {
        if self.x != x || self.y != y {
            self.x = x;
            self.y = y;
            self.dirty = true;
        }
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        if self.width != width || self.height != height {
            self.width = width;
            self.height = height;
            self.dirty = true;
        }
    }

    pub fn draw(&mut self, program: gl::types::GLuint, window_width: u32, window_height: u32) {
        // Ленивая инициализация — первый вызов
        if self.vao.is_none() {
            let mut vao = 0;
            let mut vbo = 0;
            unsafe {
                gl::GenVertexArrays(1, &mut vao);
                gl::GenBuffers(1, &mut vbo);
            }
            self.vao = Some(vao);
            self.vbo = Some(vbo);
        }

        // Если координаты изменились — обновляем буфер
        if self.dirty {
            self.update_buffer(window_width, window_height);
            self.dirty = false;
        }

        unsafe {
            gl::UseProgram(program);
            gl::BindVertexArray(self.vao.unwrap());
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            gl::BindVertexArray(0);
        }
    }

    fn update_buffer(&self, window_width: u32, window_height: u32) {
        let vertices = self.calculate_vertices(window_width, window_height);

        unsafe {
            gl::BindVertexArray(self.vao.unwrap());
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo.unwrap());
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                vertices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE,
                (2 * std::mem::size_of::<f32>()) as gl::types::GLsizei,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);
            gl::BindVertexArray(0);
        }
    }

    fn calculate_vertices(&self, window_width: u32, window_height: u32) -> [f32; 12] {
        let w = window_width as f32;
        let h = window_height as f32;
        let x = self.x as f32;
        let y = self.y as f32;
        let width = self.width as f32;
        let height = self.height as f32;

        let left = (x / w) * 2.0 - 1.0;
        let right = ((x + width) / w) * 2.0 - 1.0;
        let top = 1.0 - (y / h) * 2.0;
        let bottom = 1.0 - ((y + height) / h) * 2.0;

        [
            left, top,
            left, bottom,
            right, bottom,
            left, top,
            right, bottom,
            right, top,
        ]
    }
}