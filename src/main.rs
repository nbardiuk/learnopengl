mod shader;

use crate::shader::Shader;
use cgmath::perspective;
use cgmath::prelude::*;
use cgmath::vec3;
use cgmath::Deg;
use cgmath::Matrix4;
use cgmath::Point3;
use cgmath::Vector3;
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
    window.set_cursor_mode(glfw::CursorMode::Disabled);
    window.set_cursor_pos_polling(true);
    window.set_scroll_polling(true);

    // gl: load all OpenGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let shader = Shader::new("res/shaders/shader.vert", "res/shaders/shader.frag")
        .expect("Shader Program linkage failed");

    let vertices: [GLfloat; 180] = [
        // positions   // texture coords
        -0.5, -0.5, -0.5, 0.0, 0.0, //
        0.5, -0.5, -0.5, 1.0, 0.0, //
        0.5, 0.5, -0.5, 1.0, 1.0, //
        0.5, 0.5, -0.5, 1.0, 1.0, //
        -0.5, 0.5, -0.5, 0.0, 1.0, //
        -0.5, -0.5, -0.5, 0.0, 0.0, //
        // positions   // texture coords
        -0.5, -0.5, 0.5, 0.0, 0.0, //
        0.5, -0.5, 0.5, 1.0, 0.0, //
        0.5, 0.5, 0.5, 1.0, 1.0, //
        0.5, 0.5, 0.5, 1.0, 1.0, //
        -0.5, 0.5, 0.5, 0.0, 1.0, //
        -0.5, -0.5, 0.5, 0.0, 0.0, //
        // positions   // texture coords
        -0.5, 0.5, 0.5, 1.0, 0.0, //
        -0.5, 0.5, -0.5, 1.0, 1.0, //
        -0.5, -0.5, -0.5, 0.0, 1.0, //
        -0.5, -0.5, -0.5, 0.0, 1.0, //
        -0.5, -0.5, 0.5, 0.0, 0.0, //
        -0.5, 0.5, 0.5, 1.0, 0.0, //
        // positions   // texture coords
        0.5, 0.5, 0.5, 1.0, 0.0, //
        0.5, 0.5, -0.5, 1.0, 1.0, //
        0.5, -0.5, -0.5, 0.0, 1.0, //
        0.5, -0.5, -0.5, 0.0, 1.0, //
        0.5, -0.5, 0.5, 0.0, 0.0, //
        0.5, 0.5, 0.5, 1.0, 0.0, //
        // positions   // texture coords
        -0.5, -0.5, -0.5, 0.0, 1.0, //
        0.5, -0.5, -0.5, 1.0, 1.0, //
        0.5, -0.5, 0.5, 1.0, 0.0, //
        0.5, -0.5, 0.5, 1.0, 0.0, //
        -0.5, -0.5, 0.5, 0.0, 0.0, //
        -0.5, -0.5, -0.5, 0.0, 1.0, //
        // positions   // texture coords
        -0.5, 0.5, -0.5, 0.0, 1.0, //
        0.5, 0.5, -0.5, 1.0, 1.0, //
        0.5, 0.5, 0.5, 1.0, 0.0, //
        0.5, 0.5, 0.5, 1.0, 0.0, //
        -0.5, 0.5, 0.5, 0.0, 0.0, //
        -0.5, 0.5, -0.5, 0.0, 1.0, //
    ];

    let cube_positions = [
        vec3(0.0, 0.0, 0.0),
        vec3(2.0, 5.0, -15.0),
        vec3(-1.5, -2.2, -2.5),
        vec3(-3.8, -2.0, -12.3),
        vec3(2.4, -0.4, -3.5),
        vec3(-1.7, 3.0, -7.5),
        vec3(1.3, -2.0, -2.5),
        vec3(1.5, 2.0, -2.5),
        vec3(1.5, 0.2, -1.5),
        vec3(-1.3, 1.0, -1.5),
    ];

    let texture1 = load_texture("res/textures/container.jpg").unwrap();
    let texture2 = load_texture("res/textures/awesomeface.png").unwrap();

    // tell opengl for each sampler to which texture unit it belongs to
    shader.use_program();
    shader.set_int("texture1", 0);
    shader.set_int("texture2", 1);

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

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    let mut camera_pos = Point3::new(0., 0., 3.);
    let mut camera_front = vec3(0., 0., -1.);
    let camera_up = vec3(0., 1., 0.);

    let mut first_mouse = true;
    let mut last_x: f32 = 0.;
    let mut last_y: f32 = 0.;
    let mut yaw: f32 = -89.;
    let mut pitch: f32 = 0.;

    let mut field_of_view: f32 = 45.;

    let mut last_time = glfw.get_time() as f32;

    // render loop
    while !window.should_close() {
        let current_time = glfw.get_time() as f32;
        let delta_time = current_time - last_time;
        last_time = current_time;

        // events
        process_events(
            &events,
            &mut first_mouse,
            &mut last_x,
            &mut last_y,
            &mut yaw,
            &mut pitch,
            &mut camera_front,
            &mut field_of_view,
        );
        process_inputs(
            &mut window,
            &mut camera_pos,
            &camera_front,
            &camera_up,
            delta_time,
        );

        //rendering
        unsafe {
            // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE); // wireframe mode

            // clear the colorbuffer
            gl::ClearColor(0.2, 0.2, 0.3, 1.);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // activate the shader
            shader.use_program();

            //camera
            shader.set_matrix_4f(
                "view",
                Matrix4::look_at(camera_pos, camera_pos + camera_front, camera_up),
            );

            let (width, height) = window.get_size();
            shader.set_matrix_4f(
                "projection",
                perspective(Deg(field_of_view), width as f32 / height as f32, 0.1, 100.),
            );

            // render the shape
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture1);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, texture2);

            gl::BindVertexArray(vao);
            for (i, &position) in cube_positions.iter().enumerate() {
                shader.set_matrix_4f(
                    "model",
                    Matrix4::from_translation(position)
                        * Matrix4::from_axis_angle(
                            vec3(1.0, 0.3, 0.5).normalize(),
                            Deg((i as f32) * 20.),
                        ),
                );
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
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
        gl::DeleteTextures(1, &texture2);
    }
}

fn process_events(
    events: &Receiver<(f64, WindowEvent)>,
    first_mouse: &mut bool,
    last_x: &mut f32,
    last_y: &mut f32,
    yaw: &mut f32,
    pitch: &mut f32,
    camera_front: &mut Vector3<f32>,
    field_of_view: &mut f32,
) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            WindowEvent::CursorPos(xpos, ypos) => {
                if *first_mouse {
                    *last_x = xpos as f32;
                    *last_y = ypos as f32;
                    *first_mouse = false;
                }

                let sensitivity = 0.05;
                let xoffset = (xpos as f32 - *last_x) * sensitivity;
                let yoffset = (*last_y - ypos as f32) * sensitivity;

                *last_x = xpos as f32;
                *last_y = ypos as f32;

                *yaw += xoffset;
                *pitch = (*pitch + yoffset).min(89.).max(-89.);

                *camera_front = vec3(
                    yaw.to_radians().cos() * pitch.to_radians().cos(),
                    pitch.to_radians().sin(),
                    yaw.to_radians().sin() * pitch.to_radians().cos(),
                )
                .normalize();
            }
            WindowEvent::Scroll(_, yoffset) => {
                *field_of_view = (*field_of_view - yoffset as f32).min(45.).max(1.);
            }
            _ => {}
        }
    }
}

fn process_inputs(
    window: &mut Window,
    camera_pos: &mut Point3<f32>,
    camera_front: &Vector3<f32>,
    camera_up: &Vector3<f32>,
    delta_time: f32,
) {
    if window.get_key(Key::Escape) == Action::Press {
        window.set_should_close(true)
    }

    let camera_speed = 2.5 * delta_time;
    if window.get_key(Key::W) == Action::Press {
        *camera_pos += camera_speed * camera_front
    }
    if window.get_key(Key::S) == Action::Press {
        *camera_pos -= camera_speed * camera_front
    }
    if window.get_key(Key::A) == Action::Press {
        *camera_pos -= camera_front.cross(*camera_up).normalize() * camera_speed;
    }
    if window.get_key(Key::D) == Action::Press {
        *camera_pos += camera_front.cross(*camera_up).normalize() * camera_speed;
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
