mod camera;
mod shader;

use crate::camera::Camera;
use crate::shader::Shader;
use cgmath::perspective;
use cgmath::prelude::*;
use cgmath::vec3;
use cgmath::Deg;
use cgmath::Matrix4;
use cgmath::Point2;
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
    window.set_cursor_mode(glfw::CursorMode::Disabled);
    window.set_cursor_pos_polling(true);
    window.set_scroll_polling(true);

    // gl: load all OpenGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let light_shader = Shader::new("res/shaders/light.vert", "res/shaders/light.frag")
        .expect("Shader Program linkage failed");
    let lamp_shader = Shader::new("res/shaders/lamp.vert", "res/shaders/lamp.frag")
        .expect("Shader Program linkage failed");

    let vertices: [GLfloat; 216] = [
        // position       // normal
        -0.5, -0.5, -0.5, 0.0, 0.0, -1.0, //
        0.5, -0.5, -0.5, 0.0, 0.0, -1.0, //
        0.5, 0.5, -0.5, 0.0, 0.0, -1.0, //
        0.5, 0.5, -0.5, 0.0, 0.0, -1.0, //
        -0.5, 0.5, -0.5, 0.0, 0.0, -1.0, //
        -0.5, -0.5, -0.5, 0.0, 0.0, -1.0, //
        -0.5, -0.5, 0.5, 0.0, 0.0, 1.0, //
        0.5, -0.5, 0.5, 0.0, 0.0, 1.0, //
        0.5, 0.5, 0.5, 0.0, 0.0, 1.0, //
        0.5, 0.5, 0.5, 0.0, 0.0, 1.0, //
        -0.5, 0.5, 0.5, 0.0, 0.0, 1.0, //
        -0.5, -0.5, 0.5, 0.0, 0.0, 1.0, //
        -0.5, 0.5, 0.5, -1.0, 0.0, 0.0, //
        -0.5, 0.5, -0.5, -1.0, 0.0, 0.0, //
        -0.5, -0.5, -0.5, -1.0, 0.0, 0.0, //
        -0.5, -0.5, -0.5, -1.0, 0.0, 0.0, //
        -0.5, -0.5, 0.5, -1.0, 0.0, 0.0, //
        -0.5, 0.5, 0.5, -1.0, 0.0, 0.0, //
        0.5, 0.5, 0.5, 1.0, 0.0, 0.0, //
        0.5, 0.5, -0.5, 1.0, 0.0, 0.0, //
        0.5, -0.5, -0.5, 1.0, 0.0, 0.0, //
        0.5, -0.5, -0.5, 1.0, 0.0, 0.0, //
        0.5, -0.5, 0.5, 1.0, 0.0, 0.0, //
        0.5, 0.5, 0.5, 1.0, 0.0, 0.0, //
        -0.5, -0.5, -0.5, 0.0, -1.0, 0.0, //
        0.5, -0.5, -0.5, 0.0, -1.0, 0.0, //
        0.5, -0.5, 0.5, 0.0, -1.0, 0.0, //
        0.5, -0.5, 0.5, 0.0, -1.0, 0.0, //
        -0.5, -0.5, 0.5, 0.0, -1.0, 0.0, //
        -0.5, -0.5, -0.5, 0.0, -1.0, 0.0, //
        -0.5, 0.5, -0.5, 0.0, 1.0, 0.0, //
        0.5, 0.5, -0.5, 0.0, 1.0, 0.0, //
        0.5, 0.5, 0.5, 0.0, 1.0, 0.0, //
        0.5, 0.5, 0.5, 0.0, 1.0, 0.0, //
        -0.5, 0.5, 0.5, 0.0, 1.0, 0.0, //
        -0.5, 0.5, -0.5, 0.0, 1.0, 0.0, //
    ];

    let mut cube_vao = 0;
    let mut lamp_vao = 0;
    let mut vbo = 0;
    unsafe {
        // 1. copy vertices array in a vertex buffer
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        let vert_ref = &vertices[0] as *const GLfloat as *const c_void;
        let vert_size = mem::size_of_val(&vertices) as GLsizeiptr;
        gl::BufferData(gl::ARRAY_BUFFER, vert_size, vert_ref, gl::STATIC_DRAW);

        let stride = 6 * mem::size_of::<GLfloat>() as GLint;

        // 2. bind Vertex Array Object for lamp
        gl::GenVertexArrays(1, &mut lamp_vao);
        gl::BindVertexArray(lamp_vao);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(0);

        // 2. bind Vertex Array Object for cube
        gl::GenVertexArrays(1, &mut cube_vao);
        gl::BindVertexArray(cube_vao);
        // aPos
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(0);
        // aNormal
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (3 * mem::size_of::<GLfloat>()) as *const c_void,
        );
        gl::EnableVertexAttribArray(1);
    }

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    let mut camera = Camera::new();

    let mut last_mouse: Option<Point2<f32>> = None;
    let mut last_time = glfw.get_time() as f32;

    // render loop
    while !window.should_close() {
        let current_time = glfw.get_time() as f32;
        let delta_time = current_time - last_time;
        last_time = current_time;

        // events
        process_events(&events, &mut last_mouse, &mut camera);
        process_inputs(&mut window, &mut camera, delta_time);

        let (width, height) = window.get_size();
        let projection = perspective(
            Deg(camera.field_of_view),
            width as f32 / height as f32,
            0.1,
            100.,
        );

        //rendering
        unsafe {
            // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE); // wireframe mode

            // clear the colorbuffer
            gl::ClearColor(0.2, 0.2, 0.3, 1.);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            let light_pos = vec3(1.2, 1.0, 2.0);

            light_shader.use_program();
            light_shader.set_vec3("lightColor", vec3(1., 1., 1.));
            light_shader.set_vec3("lightPos", light_pos);
            light_shader.set_vec3("material.ambient", vec3(1., 0.5, 0.31));
            light_shader.set_vec3("material.diffuse", vec3(1., 0.5, 0.31));
            light_shader.set_vec3("material.specular", vec3(0.5, 0.5, 0.5));
            light_shader.set_float("material.shininess", 32.);
            light_shader.set_vec3(
                "viewPos",
                vec3(camera.position.x, camera.position.y, camera.position.z),
            );
            light_shader.set_matrix4("projection", projection);
            light_shader.set_matrix4("view", camera.view());
            light_shader.set_matrix4("model", Matrix4::identity());
            gl::BindVertexArray(cube_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            let model = Matrix4::from_translation(light_pos) * Matrix4::from_scale(0.2);
            lamp_shader.use_program();
            lamp_shader.set_matrix4("projection", projection);
            lamp_shader.set_matrix4("view", camera.view());
            lamp_shader.set_matrix4("model", model);
            gl::BindVertexArray(lamp_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        window.swap_buffers();
        glfw.poll_events();
    }

    // dealocate resources
    unsafe {
        gl::DeleteVertexArrays(1, &cube_vao);
        gl::DeleteVertexArrays(1, &lamp_vao);
        gl::DeleteBuffers(1, &vbo);
    }
}

fn process_events(
    events: &Receiver<(f64, WindowEvent)>,
    last_mouse: &mut Option<Point2<f32>>,
    camera: &mut Camera,
) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            WindowEvent::CursorPos(xpos, ypos) => {
                let current = Point2::new(xpos as f32, ypos as f32);
                let last = last_mouse.replace(current).unwrap_or(current);

                let sensitivity = 0.05;
                let xoffset = (current.x - last.x) * sensitivity;
                let yoffset = (last.y - current.y) * sensitivity;

                camera.rotate(xoffset, yoffset);
            }
            WindowEvent::Scroll(_, yoffset) => {
                camera.zoom_in(yoffset as f32);
            }
            _ => {}
        }
    }
}

fn process_inputs(window: &mut Window, camera: &mut Camera, delta_time: f32) {
    if window.get_key(Key::Escape) == Action::Press {
        window.set_should_close(true)
    }

    let camera_speed = 2.5 * delta_time;
    if window.get_key(Key::W) == Action::Press {
        camera.move_forward(camera_speed);
    }
    if window.get_key(Key::S) == Action::Press {
        camera.move_forward(-camera_speed);
    }
    if window.get_key(Key::A) == Action::Press {
        camera.move_right(-camera_speed);
    }
    if window.get_key(Key::D) == Action::Press {
        camera.move_right(camera_speed);
    }
}
