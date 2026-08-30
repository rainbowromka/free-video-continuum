pub struct Renderer {
    program: gl::types::GLuint,
    texture_program: gl::types::GLuint,
    window_width: u32,
    window_height: u32,
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Result<Self, String> {
        let program = create_program()?;
        let texture_program = create_texture_program()?;
        Ok(Self {
            program,
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

    pub fn program(&self) -> gl::types::GLuint {
        self.program
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
        uniform vec3 rect_color;
        out vec4 FragColor;
        void main() {
            FragColor = vec4(rect_color, 1.0);
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

fn create_texture_program() -> Result<gl::types::GLuint, String> {
    let vertex_shader = compile_shader(
        gl::VERTEX_SHADER,
        r#"
        #version 330 core
        layout (location = 0) in vec2 position;
        layout (location = 1) in vec2 texcoord;
        out vec2 v_texcoord;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
            v_texcoord = texcoord;
        }
        "#,
    )?;

    let fragment_shader = compile_shader(
        gl::FRAGMENT_SHADER,
        r#"
        #version 330 core
        uniform sampler2D tex;
        in vec2 v_texcoord;
        out vec4 FragColor;
        void main() {
            FragColor = texture(tex, v_texcoord);
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
            return Err("Texture program linking failed".to_string());
        }

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        Ok(program)
    }
}