mod shader;

use crate::shader::Shader;
use cgmath::prelude::*;
use cgmath::Deg;
use cgmath::Matrix4;
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
use image::ColorType;
use image::GenericImageView;
use std::mem;
use std::os::raw::c_void;
use std::path::Path;
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

    let shader = Shader::new("res/shaders/shader.vert", "res/shaders/shader.frag")
        .expect("Shader Program linkage failed");

    let vertices: [GLfloat; 20] = [
        // positions   // texture coords
        0.5, 0.5, 0.0, 1.0, 1.0, // top right
        0.5, -0.5, 0.0, 1.0, 0.0, // bottom right
        -0.5, -0.5, 0.0, 0.0, 0.0, // bottom left
        -0.5, 0.5, 0.0, 0.0, 1.0, // top left
    ];
    let indices: [GLuint; 6] = [
        // start from 0
        0, 1, 3, // first triangle
        1, 2, 3, // second triangle
    ];

    let texture1 = load_texture("res/textures/container.jpg").unwrap();
    let texture2 = load_texture("res/textures/awesomeface.png").unwrap();

    // tell opengl for each sampler to which texture unit it belongs to
    shader.use_program();
    shader.set_int("texture1", 0);
    shader.set_int("texture2", 1);

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
        // positions
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            5 * mem::size_of::<GLfloat>() as GLint,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        // texture coords
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            5 * mem::size_of::<GLfloat>() as GLint,
            (3 * mem::size_of::<GLfloat>() as GLint) as *const c_void,
        );
        gl::EnableVertexAttribArray(1);
    }

    let trans = Matrix4::identity();
    let trans = trans * Matrix4::from_angle_z(Deg(90.));
    let trans = trans * Matrix4::from_nonuniform_scale(0.5, 0.5, 0.5);
    shader.set_matrix_4f("transform", trans);

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

            // render the shape
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture1);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, texture2);

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
        gl::DeleteTextures(1, &texture1);
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

fn load_texture(path: &str) -> Result<GLuint, String> {
    // read image data
    let img = image::open(&Path::new(path)).map_err(|e| format!("Could not load texture {}", e))?;

    let format = match img.color() {
        ColorType::RGB(_) => gl::RGB,
        ColorType::RGBA(_) => gl::RGBA,
        ColorType::Gray(_) => gl::DEPTH_COMPONENT,
        ColorType::GrayA(_) => gl::DEPTH_STENCIL,
        ColorType::BGR(_) => gl::BGR,
        ColorType::BGRA(_) => gl::BGRA,
        ColorType::Palette(_) => gl::DEPTH_COMPONENT,
    };

    unsafe {
        // initialize texture
        let mut texture = 0;
        gl::GenTextures(1, &mut texture);

        // set the texture wrapping parameters
        // set texture wrapping to GL_REPEAT (default wrapping method)
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
        // set texture filtering parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);

        // transfer image data
        gl::BindTexture(gl::TEXTURE_2D, texture);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            format as GLint,
            img.width() as GLint,
            img.height() as GLint,
            0,
            format,
            gl::UNSIGNED_BYTE,
            &img.raw_pixels()[0] as *const u8 as *const c_void,
        );

        // generate all mip map images for us
        gl::GenerateMipmap(gl::TEXTURE_2D);
        Ok(texture)
    }
}
