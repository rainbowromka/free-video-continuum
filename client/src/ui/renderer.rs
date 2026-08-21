pub struct Renderer {
    program: gl::types::GLuint,
    window_width: f32,
    window_height: f32,
}

impl Renderer {
    pub fn new(width: f32, height: f32) -> Result<Self, String> {
        let program = create_program()?;
        Ok(Self {
            program,
            window_width: width,
            window_height: height,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.window_width = width as f32;
        self.window_height = height as f32;
        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
        }
    }

    pub fn program(&self) -> gl::types::GLuint {
        self.program
    }

    pub fn window_width(&self) -> f32 {
        self.window_width
    }

    pub fn window_height(&self) -> f32 {
        self.window_height
    }

    pub fn clear(&self) {
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}

fn create_program() -> Result<gl::types::GLuint, String> {
    let vertex_shader = compile_shader(
        gl::VERTEX_SHADER,
        r#"
        #version 330 core
        layout (location = 0) in vec2 position;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
        "#,
    )?;

    let fragment_shader = compile_shader(
        gl::FRAGMENT_SHADER,
        r#"
        #version 330 core
        out vec4 FragColor;
        void main() {
            FragColor = vec4(0.2, 0.6, 1.0, 1.0);
        }
        "#,
    )?;

    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);

        let mut success = gl::FALSE as gl::types::GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success == gl::FALSE as gl::types::GLint {
            return Err("Program linking failed".to_string());
        }

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        Ok(program)
    }
}

fn compile_shader(shader_type: gl::types::GLenum, source: &str) -> Result<gl::types::GLuint, String> {
    unsafe {
        let shader = gl::CreateShader(shader_type);
        gl::ShaderSource(
            shader,
            1,
            &(source.as_ptr() as *const gl::types::GLchar),
            &(source.len() as gl::types::GLint),
        );
        gl::CompileShader(shader);

        let mut success = gl::FALSE as gl::types::GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success == gl::FALSE as gl::types::GLint {
            return Err("Shader compilation failed".to_string());
        }

        Ok(shader)
    }
}

fn create_rectangle() -> Result<(gl::types::GLuint, gl::types::GLuint), String> {
    let vertices: [f32; 12] = [
        -0.5, 0.5,
        -0.5, -0.5,
        0.5, -0.5,
        -0.5, 0.5,
        0.5, -0.5,
        0.5, 0.5,
    ];

    let mut vao = 0;
    let mut vbo = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
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

    Ok((vao, vbo))
}