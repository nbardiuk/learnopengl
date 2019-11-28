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

    let light_shader = Shader::new("res/shaders/light.vert", "res/shaders/light.frag")
        .expect("Shader Program linkage failed");
    let lamp_shader = Shader::new("res/shaders/lamp.vert", "res/shaders/lamp.frag")
        .expect("Shader Program linkage failed");

    let vertices: [f32; 288] = [
        // positions      // normals      // texture coords
        -0.5, -0.5, -0.5, 0.0, 0.0, -1.0, 0.0, 0.0, //
        0.5, -0.5, -0.5, 0.0, 0.0, -1.0, 1.0, 0.0, //
        0.5, 0.5, -0.5, 0.0, 0.0, -1.0, 1.0, 1.0, //
        0.5, 0.5, -0.5, 0.0, 0.0, -1.0, 1.0, 1.0, //
        -0.5, 0.5, -0.5, 0.0, 0.0, -1.0, 0.0, 1.0, //
        -0.5, -0.5, -0.5, 0.0, 0.0, -1.0, 0.0, 0.0, //
        //
        -0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 0.0, //
        0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 1.0, 0.0, //
        0.5, 0.5, 0.5, 0.0, 0.0, 1.0, 1.0, 1.0, //
        0.5, 0.5, 0.5, 0.0, 0.0, 1.0, 1.0, 1.0, //
        -0.5, 0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 1.0, //
        -0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 0.0, //
        //
        -0.5, 0.5, 0.5, -1.0, 0.0, 0.0, 1.0, 0.0, //
        -0.5, 0.5, -0.5, -1.0, 0.0, 0.0, 1.0, 1.0, //
        -0.5, -0.5, -0.5, -1.0, 0.0, 0.0, 0.0, 1.0, //
        -0.5, -0.5, -0.5, -1.0, 0.0, 0.0, 0.0, 1.0, //
        -0.5, -0.5, 0.5, -1.0, 0.0, 0.0, 0.0, 0.0, //
        -0.5, 0.5, 0.5, -1.0, 0.0, 0.0, 1.0, 0.0, //
        //
        0.5, 0.5, 0.5, 1.0, 0.0, 0.0, 1.0, 0.0, //
        0.5, 0.5, -0.5, 1.0, 0.0, 0.0, 1.0, 1.0, //
        0.5, -0.5, -0.5, 1.0, 0.0, 0.0, 0.0, 1.0, //
        0.5, -0.5, -0.5, 1.0, 0.0, 0.0, 0.0, 1.0, //
        0.5, -0.5, 0.5, 1.0, 0.0, 0.0, 0.0, 0.0, //
        0.5, 0.5, 0.5, 1.0, 0.0, 0.0, 1.0, 0.0, //
        //
        -0.5, -0.5, -0.5, 0.0, -1.0, 0.0, 0.0, 1.0, //
        0.5, -0.5, -0.5, 0.0, -1.0, 0.0, 1.0, 1.0, //
        0.5, -0.5, 0.5, 0.0, -1.0, 0.0, 1.0, 0.0, //
        0.5, -0.5, 0.5, 0.0, -1.0, 0.0, 1.0, 0.0, //
        -0.5, -0.5, 0.5, 0.0, -1.0, 0.0, 0.0, 0.0, //
        -0.5, -0.5, -0.5, 0.0, -1.0, 0.0, 0.0, 1.0, //
        //
        -0.5, 0.5, -0.5, 0.0, 1.0, 0.0, 0.0, 1.0, //
        0.5, 0.5, -0.5, 0.0, 1.0, 0.0, 1.0, 1.0, //
        0.5, 0.5, 0.5, 0.0, 1.0, 0.0, 1.0, 0.0, //
        0.5, 0.5, 0.5, 0.0, 1.0, 0.0, 1.0, 0.0, //
        -0.5, 0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 0.0, //
        -0.5, 0.5, -0.5, 0.0, 1.0, 0.0, 0.0, 1.0, //
    ];

    let cube_positions: [Vector3<f32>; 10] = [
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

    let point_light_positions: [Vector3<f32>; 4] = [
        vec3(0.7, 0.2, 2.0),
        vec3(2.3, -3.3, -4.0),
        vec3(-4.0, 2.0, -12.0),
        vec3(0.0, 0.0, -3.0),
    ];

    let diffuse_map = load_texture("res/textures/container2.png").unwrap();
    let specular_map = load_texture("res/textures/container2_specular.png").unwrap();

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

        let floats = |n: usize| (n * mem::size_of::<GLfloat>()) as GLint;
        let stride = floats(8);
        let void = |n| n as *const c_void;

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
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, void(floats(3)));
        gl::EnableVertexAttribArray(1);
        // aTexCoords
        gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, stride, void(floats(6)));
        gl::EnableVertexAttribArray(2);
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

            let as_vec3 = |p: Point3<f32>| vec3(p.x, p.y, p.z);

            light_shader.use_program();
            light_shader.set_vec3("dirLight.direction", vec3(-0.2, -1.0, -0.3));
            light_shader.set_vec3("dirLight.ambient", vec3(0.05, 0.05, 0.05));
            light_shader.set_vec3("dirLight.diffuse", vec3(0.4, 0.4, 0.4));
            light_shader.set_vec3("dirLight.specular", vec3(0.5, 0.5, 0.5));

            light_shader.set_vec3("spotLight.position", as_vec3(camera.position));
            light_shader.set_vec3("spotLight.direction", camera.front);
            light_shader.set_float("spotLight.cutOff", 12.5_f32.to_radians().cos());
            light_shader.set_float("spotLight.outerCutOff", 15_f32.to_radians().cos());
            light_shader.set_vec3("spotLight.ambient", vec3(0., 0., 0.));
            light_shader.set_vec3("spotLight.diffuse", vec3(1.0, 1.0, 1.0));
            light_shader.set_vec3("spotLight.specular", vec3(1.0, 1.0, 1.0));
            light_shader.set_float("spotLight.constant", 1.0);
            light_shader.set_float("spotLight.linear", 0.09);
            light_shader.set_float("spotLight.quadratic", 0.032);

            for (i, &position) in point_light_positions.iter().enumerate() {
                let light = format!("pointLights[{}]", i);
                light_shader.set_vec3(&format!("{}.{}", light, "position"), position);
                light_shader.set_vec3(&format!("{}.{}", light, "ambient"), vec3(0.05, 0.05, 0.05));
                light_shader.set_vec3(&format!("{}.{}", light, "diffuse"), vec3(0.8, 0.8, 0.8));
                light_shader.set_vec3(&format!("{}.{}", light, "specular"), vec3(1.0, 1.0, 1.0));
                light_shader.set_float(&format!("{}.{}", light, "constant"), 1.0);
                light_shader.set_float(&format!("{}.{}", light, "linear"), 0.09);
                light_shader.set_float(&format!("{}.{}", light, "quadratic"), 0.032);
            }

            light_shader.set_texture("material.diffuse", diffuse_map, gl::TEXTURE0);
            light_shader.set_texture("material.specular", specular_map, gl::TEXTURE1);
            light_shader.set_float("material.shininess", 32.);

            light_shader.set_vec3("viewPos", as_vec3(camera.position));
            light_shader.set_matrix4("projection", projection);
            light_shader.set_matrix4("view", camera.view());
            gl::BindVertexArray(cube_vao);
            for (i, &position) in cube_positions.iter().enumerate() {
                let axis = vec3(1., 0.3, 0.5).normalize();
                let rotation = Matrix4::from_axis_angle(axis, Deg(20. * i as f32));
                let translation = Matrix4::from_translation(position);
                light_shader.set_matrix4("model", translation * rotation);
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }

            lamp_shader.use_program();
            lamp_shader.set_matrix4("projection", projection);
            lamp_shader.set_matrix4("view", camera.view());
            gl::BindVertexArray(lamp_vao);
            for &position in point_light_positions.iter() {
                let model = Matrix4::from_translation(position) * Matrix4::from_scale(0.2);
                lamp_shader.set_matrix4("model", model);
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
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
        gl::DeleteTextures(1, &diffuse_map);
        gl::DeleteTextures(1, &specular_map);
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
