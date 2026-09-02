use crate::ui::render::shader;

pub struct Renderer {
    texture_program: gl::types::GLuint,
    window_width: u32,
    window_height: u32,
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Result<Self, String> {
        let texture_program = shader::create_texture_program()?;
        Ok(Self {
            texture_program,
            window_width: width,
            window_height: height,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.window_width = width;
        self.window_height = height;
        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
        }
    }

    pub fn texture_program(&self) -> gl::types::GLuint {
        self.texture_program
    }

    pub fn window_width(&self) -> u32 {
        self.window_width
    }

    pub fn window_height(&self) -> u32 {
        self.window_height
    }

    pub fn clear(&self, r: f32, g: f32, b: f32) {
        unsafe {
            gl::ClearColor(r, g, b, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}