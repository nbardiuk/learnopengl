use gl::types::{GLchar, GLenum, GLfloat, GLint, GLsizeiptr, GLuint};
use glfw::Action;
use glfw::Context;
use glfw::Key;
use glfw::Window;
use glfw::WindowEvent;
use glfw::WindowHint;
use std::ffi::CString;
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::sync::mpsc::Receiver;

const VERTEX_SHADER_SRC: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    void main() {
        gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;

const FRAGMENT_SHADER_SRC: &str = r#"
    #version 330 core
    out vec4 FragColor;
    void main() {
        FragColor = vec4(1.0, 0.5, 0.2, 1.0);
    }
"#;

fn main() {
    // Initialize glfw for OpenGL 3.3
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(WindowHint::ContextVersion(3, 3));
    glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    let (mut window, events) = glfw
        .create_window(800, 600, "LearnOpenGL", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_framebuffer_size_polling(true);
    window.set_key_polling(true);

    // gl: load all OpenGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let vertex_shader = setup_shader(VERTEX_SHADER_SRC, gl::VERTEX_SHADER)
        .expect("Vertex Shader compilation failed");
    let fragment_shader = setup_shader(FRAGMENT_SHADER_SRC, gl::FRAGMENT_SHADER)
        .expect("Fragment Shader compilation failed");
    let shader_program = setup_shader_program(&[vertex_shader, fragment_shader])
        .expect("Shader Program likage failed");

    let vertices: [GLfloat; 9] = [
        -0.5, -0.5, 0., // left
        0.5, -0.5, 0., // right
        0., 0.5, 0., // top
    ];

    let mut vao = 0;
    let mut vbo = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            mem::size_of_val(&vertices) as GLsizeiptr,
            &vertices[0] as *const GLfloat as *const c_void,
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());
        gl::EnableVertexAttribArray(0);
    }

    // render loop
    while !window.should_close() {
        // events
        process_events(&mut window, &events);

        //rendering
        unsafe {
            gl::ClearColor(0.2, 0.2, 0.3, 1.);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(shader_program);
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        window.swap_buffers();
        glfw.poll_events();
    }

    // dealocate resources
    unsafe {
        gl::DeleteVertexArrays(1, &vao);
        gl::DeleteBuffers(1, &vbo);
    }
}

fn process_events(window: &mut Window, events: &Receiver<(f64, WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            _ => {}
        }
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
