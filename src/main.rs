use gl::types::GLchar;
use gl::types::GLenum;
use gl::types::GLfloat;
use gl::types::GLint;
use gl::types::GLsizeiptr;
use gl::types::GLuint;
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

const VERTEX_SHADER_SRC: &str = include_str!("../res/simple.vert");
const FRAGMENT_SHADER_SRC: &str = include_str!("../res/color.frag");

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

    let vertices: [GLfloat; 12] = [
        0.5, 0.5, 0.0, // top right
        0.5, -0.5, 0.0, // bottom right
        -0.5, -0.5, 0.0, // bottom left
        -0.5, 0.5, 0.0, // top left
    ];

    let indices: [GLuint; 6] = [
        // start from 0
        0, 1, 3, // first triangle
        1, 2, 3, // second triangle
    ];

    let mut vao = 0;
    let mut vbo = 0;
    let mut ebo = 0;
    unsafe {
        // 1. bind Vertex Array Object
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // 2. copy vertices array in a vertex buffer
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            mem::size_of_val(&vertices) as GLsizeiptr,
            &vertices[0] as *const GLfloat as *const c_void,
            gl::STATIC_DRAW,
        );

        // 3. copy indices array in a element buffer
        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            mem::size_of_val(&indices) as GLsizeiptr,
            &indices[0] as *const GLuint as *const c_void,
            gl::STATIC_DRAW,
        );

        // 4. set vertex attribute pointers
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());
        gl::EnableVertexAttribArray(0);
    }

    // render loop
    while !window.should_close() {
        // events
        process_events(&mut window, &events);

        //rendering
        unsafe {
            // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE); // wireframe mode

            // clear the colorbuffer
            gl::ClearColor(0.2, 0.2, 0.3, 1.);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // activate the shader
            gl::UseProgram(shader_program);

            // unpdate uniform color
            let our_color = CString::new("ourColor").unwrap();
            let vertex_color_location = gl::GetUniformLocation(shader_program, our_color.as_ptr());
            let green_value = glfw.get_time().sin() as f32 / 2. + 0.5;
            gl::Uniform4f(vertex_color_location, 0., green_value, 0., 1.);

            // render the triangle
            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        window.swap_buffers();
        glfw.poll_events();
    }

    // dealocate resources
    unsafe {
        gl::DeleteVertexArrays(1, &vao);
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteBuffers(1, &ebo);
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
