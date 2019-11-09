use gl::types::GLchar;
use gl::types::GLenum;
use gl::types::GLint;
use gl::types::GLuint;
use std::ffi::CString;
use std::fs;
use std::ptr;

pub struct Shader {
    pub id: GLuint,
}

impl Shader {
    pub fn new(vertex_path: &str, fragment_path: &str) -> Result<Self, String> {
        let vertex_src = fs::read_to_string(vertex_path)
            .map_err(|e| format!("Failed to read {}: {}", vertex_path, e))?;
        let vertex_shader = setup_shader(&vertex_src, gl::VERTEX_SHADER)?;

        let fragment_src = fs::read_to_string(fragment_path)
            .map_err(|e| format!("Failed to read {}: {}", fragment_path, e))?;
        let fragment_shader = setup_shader(&fragment_src, gl::FRAGMENT_SHADER)?;

        let id = setup_shader_program(&[vertex_shader, fragment_shader])?;

        Ok(Shader { id })
    }
}

fn setup_shader(src: &str, shader_type: GLenum) -> Result<GLuint, String> {
    unsafe {
        // init
        let shader = gl::CreateShader(shader_type);

        // load source
        let as_c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &as_c_str.as_ptr(), ptr::null());

        // compile
        gl::CompileShader(shader);

        // collect logs
        let mut size: GLint = 512;
        let mut info_log: Vec<GLchar> = Vec::with_capacity(size as usize);
        info_log.resize(size as usize - 1, 0); // subtract 1 to skip the trailing null character
        gl::GetShaderInfoLog(shader, size, &mut size, info_log.as_mut_ptr());
        info_log.resize(size as usize, 0);

        if info_log.is_empty() {
            Ok(shader)
        } else {
            Err(String::from_utf8_unchecked(
                info_log.into_iter().map(|i| i as u8).collect(),
            ))
        }
    }
}

fn setup_shader_program(shaders: &[GLuint]) -> Result<GLuint, String> {
    unsafe {
        // init
        let shader_program = gl::CreateProgram();

        // add shaders
        for shader in shaders {
            gl::AttachShader(shader_program, *shader);
        }

        // link program
        gl::LinkProgram(shader_program);

        // cleanup shaders
        for shader in shaders {
            gl::DeleteShader(*shader);
        }

        // collect logs
        let mut size: GLint = 512;
        let mut info_log: Vec<GLchar> = Vec::with_capacity(size as usize);
        info_log.resize(size as usize - 1, 0); // subtract 1 to skip the trailing null character
        gl::GetProgramInfoLog(shader_program, size, &mut size, info_log.as_mut_ptr());
        info_log.resize(size as usize, 0);

        if info_log.is_empty() {
            Ok(shader_program)
        } else {
            Err(String::from_utf8_unchecked(
                info_log.into_iter().map(|i| i as u8).collect(),
            ))
        }
    }
}
