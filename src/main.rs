mod shader;

use crate::shader::Shader;
use gl::types::GLfloat;
use gl::types::GLint;
use gl::types::GLsizeiptr;
use glfw::Action;
use glfw::Context;
use glfw::Key;
use glfw::Window;
use glfw::WindowEvent;
use glfw::WindowHint;
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::sync::mpsc::Receiver;

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

    let shader =
        Shader::new("res/setup.vert", "res/color.frag").expect("Shader Program linkage failed");

    let vertices: [GLfloat; 18] = [
        // positions    // colors
        0.5, -0.5, 0.0, 1.0, 0.0, 0.0, // bottom right
        -0.5, -0.5, 0.0, 0.0, 1.0, 0.0, // bottom left
        0.0, 0.5, 0.0, 0.0, 0.0, 1.0, // top
    ];

    let mut vao = 0;
    let mut vbo = 0;
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

        // 3. set vertex attribute pointers
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            6 * mem::size_of::<GLfloat>() as GLint,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            6 * mem::size_of::<GLfloat>() as GLint,
            (3 * mem::size_of::<GLfloat>() as GLint) as *const c_void,
        );
        gl::EnableVertexAttribArray(1);
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
            shader.use_program();

            shader.set_float("offset", glfw.get_time().sin() as f32);

            // render the triangle
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
