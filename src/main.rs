use gl::types::{GLchar, GLint};
use glfw::Action;
use glfw::Context;
use glfw::Key;
use glfw::Window;
use glfw::WindowEvent;
use glfw::WindowHint;
use std::ffi::CString;
use std::ptr;
use std::sync::mpsc::Receiver;

const VERTEX_SHADER_SRC: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    void main() {
        gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
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

    unsafe {
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        let as_c_str = CString::new(VERTEX_SHADER_SRC.as_bytes()).unwrap();
        gl::ShaderSource(vertex_shader, 1, &as_c_str.as_ptr(), ptr::null());
        gl::CompileShader(vertex_shader);
        let mut size: GLint = 512;
        let mut info_log: Vec<GLchar> = Vec::with_capacity(512 as usize);
        info_log.resize(size as usize - 1, 0); // subtract 1 to skip the trailing null character
        gl::GetShaderInfoLog(vertex_shader, size, &mut size, info_log.as_mut_ptr());
        info_log.resize(size as usize, 0);
        println!(
            "Vertext Shader\n{}",
            String::from_utf8_unchecked(info_log.into_iter().map(|i| i as u8).collect())
        );
    }

    // render loop
    while !window.should_close() {
        // events
        process_events(&mut window, &events);

        //rendering
        unsafe {
            gl::ClearColor(0.2, 0.2, 0.3, 1.);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        window.swap_buffers();
        glfw.poll_events();
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
