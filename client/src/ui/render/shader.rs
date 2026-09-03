use gl::types::{GLchar, GLenum, GLint, GLuint};
use lazy_static::lazy_static;

pub fn create_texture_program() -> Result<GLuint, String> {
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

        let mut success = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success == gl::FALSE as GLint {
            return Err("Texture program linking failed".to_string());
        }

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        Ok(program)
    }
}

pub fn compile_shader(shader_type: GLenum, source: &str) -> Result<GLuint, String> {
    unsafe {
        let shader = gl::CreateShader(shader_type);
        gl::ShaderSource(
            shader,
            1,
            &(source.as_ptr() as *const GLchar),
            &(source.len() as GLint),
        );
        gl::CompileShader(shader);

        let mut success = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success == gl::FALSE as GLint {
            return Err("Shader compilation failed".to_string());
        }

        Ok(shader)
    }
}

pub fn create_text_program() -> Result<GLuint, String> {
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
        uniform vec3 text_color;
        in vec2 v_texcoord;
        out vec4 FragColor;
        void main() {
            float alpha = texture(tex, v_texcoord).a;
            FragColor = vec4(text_color, alpha);
        }
        "#,
    )?;

    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);

        let mut success = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success == gl::FALSE as GLint {
            return Err("Text program linking failed".to_string());
        }

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        Ok(program)
    }
}

lazy_static! {
    pub static ref TEXTURE_PROGRAM: gl::types::GLuint = 
        create_texture_program().expect("Failed to create texture program");

        pub static ref TEXT_PROGRAM: GLuint = 
        create_text_program().expect("Failed to create text program");
}