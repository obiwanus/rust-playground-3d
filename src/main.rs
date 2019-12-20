use std::f32::consts::PI;
use std::time::SystemTime;

extern crate gl;
extern crate nalgebra_glm as glm;
extern crate sdl2;
extern crate stb_image;

use sdl2::keyboard::Scancode;

#[macro_use]
extern crate failure;

mod shader;
use shader::Program;

mod texture;
use texture::Texture;

mod buffers;
use buffers::{VertexArray, VertexBuffer};

mod camera;
use camera::Camera;
use camera::Movement::*;

fn main() {
    if let Err(error) = run() {
        eprintln!("{}", error_into_string(error));
        std::process::exit(1);
    }
}

fn run() -> Result<(), failure::Error> {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);
    gl_attr.set_depth_size(16);
    gl_attr.set_double_buffer(true);

    const SCREEN_WIDTH: f32 = 1024.0;
    const SCREEN_HEIGHT: f32 = 768.0;

    let window = video_subsystem
        .window("Boulder Dash", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    println!(
        "Swap interval: {:?}",
        video_subsystem.gl_get_swap_interval()
    );
    sdl.mouse().set_relative_mouse_mode(true);

    unsafe {
        gl::Viewport(0, 0, 1024, 768);
        gl::ClearColor(0.05, 0.05, 0.05, 1.0);
        gl::Enable(gl::DEPTH_TEST);
    }

    #[rustfmt::skip]
    let vertices: Vec<f32> = vec![
        // positions        // tex coords   // normals
        0.5, 0.5, 0.5,      0.5, 0.5,       0.0, 0.0, 1.0,      // 0
        0.5, -0.5, 0.5,     0.5, -0.5,      0.0, 0.0, 1.0,      // 1
       -0.5, 0.5, 0.5,     -0.5, 0.5,       0.0, 0.0, 1.0,      // 3
        0.5, -0.5, 0.5,     0.5, -0.5,      0.0, 0.0, 1.0,      // 1
       -0.5, -0.5, 0.5,    -0.5, -0.5,      0.0, 0.0, 1.0,      // 2
       -0.5, 0.5, 0.5,     -0.5, 0.5,       0.0, 0.0, 1.0,      // 3

       -0.5, 0.5, -0.5,     0.5, 0.5,       0.0, 0.0, -1.0,     // 7
        0.5, 0.5, -0.5,     0.5, -0.5,      0.0, 0.0, -1.0,     // 4
       -0.5, -0.5, -0.5,   -0.5, 0.5,       0.0, 0.0, -1.0,     // 6
       -0.5, -0.5, -0.5,    0.5, -0.5,      0.0, 0.0, -1.0,     // 6
        0.5, -0.5, -0.5,   -0.5, -0.5,      0.0, 0.0, -1.0,     // 5
        0.5, 0.5, -0.5,    -0.5, 0.5,       0.0, 0.0, -1.0,     // 4

        0.5, 0.5, -0.5,     0.5, 0.5,       1.0, 0.0, 0.0,      // 4
        0.5, -0.5, -0.5,    0.5, -0.5,      1.0, 0.0, 0.0,      // 5
        0.5, 0.5, 0.5,     -0.5, 0.5,       1.0, 0.0, 0.0,      // 0
        0.5, -0.5, -0.5,    0.5, -0.5,      1.0, 0.0, 0.0,      // 5
        0.5, -0.5, 0.5,    -0.5, -0.5,      1.0, 0.0, 0.0,      // 1
        0.5, 0.5, 0.5,     -0.5, 0.5,       1.0, 0.0, 0.0,      // 0

       -0.5, 0.5, 0.5,      0.5, 0.5,      -1.0, 0.0, 0.0,      // 3
       -0.5, -0.5, 0.5,     0.5, -0.5,     -1.0, 0.0, 0.0,      // 2
       -0.5, 0.5, -0.5,    -0.5, 0.5,      -1.0, 0.0, 0.0,      // 7
       -0.5, -0.5, 0.5,     0.5, -0.5,     -1.0, 0.0, 0.0,      // 2
       -0.5, -0.5, -0.5,   -0.5, -0.5,     -1.0, 0.0, 0.0,      // 6
       -0.5, 0.5, -0.5,    -0.5, 0.5,      -1.0, 0.0, 0.0,      // 7

        0.5, 0.5, -0.5,     0.5, 0.5,       0.0, 1.0, 0.0,      // 4
        0.5, 0.5, 0.5,      0.5, -0.5,      0.0, 1.0, 0.0,      // 0
       -0.5, 0.5, -0.5,    -0.5, 0.5,       0.0, 1.0, 0.0,      // 7
        0.5, 0.5, 0.5,      0.5, -0.5,      0.0, 1.0, 0.0,      // 0
       -0.5, 0.5, 0.5,     -0.5, -0.5,      0.0, 1.0, 0.0,      // 3
       -0.5, 0.5, -0.5,    -0.5, 0.5,       0.0, 1.0, 0.0,      // 7

        0.5, -0.5, 0.5,     0.5, 0.5,       0.0, -1.0, 0.0,     // 1
        0.5, -0.5, -0.5,    0.5, -0.5,      0.0, -1.0, 0.0,     // 5
       -0.5, -0.5, 0.5,    -0.5, 0.5,       0.0, -1.0, 0.0,     // 2
        0.5, -0.5, -0.5,    0.5, -0.5,      0.0, -1.0, 0.0,     // 5
       -0.5, -0.5, -0.5,   -0.5, -0.5,      0.0, -1.0, 0.0,     // 6
       -0.5, -0.5, 0.5,    -0.5, 0.5,       0.0, -1.0, 0.0,     // 2
    ];

    let cube_model = glm::rotation(-0.25 * PI, &glm::vec3(0.0, 0.0, 1.0));

    let cube_positions = vec![
        glm::vec3(0.0, 0.0, 0.0),
        glm::vec3(2.0, 5.0, -15.0),
        glm::vec3(-1.5, -2.2, -2.5),
        glm::vec3(-3.8, -2.0, -12.3),
        glm::vec3(2.4, -0.4, -3.5),
        glm::vec3(-1.7, 3.0, -7.5),
        glm::vec3(1.3, -2.0, -2.5),
        glm::vec3(1.5, 2.0, -2.5),
        glm::vec3(1.5, 0.2, -1.5),
        glm::vec3(-1.3, 1.0, -1.5),
    ];

    let light_position = glm::vec3(1.2, 1.0, 2.0);
    let light_model = glm::translation(&light_position);
    let light_model = glm::scale(&light_model, &glm::vec3(0.2, 0.2, 0.2));

    // Buffers
    let stride = 8;
    let cube = VertexBuffer::new().set_static_data(&vertices, stride);
    cube.bind();
    let cube_vao = VertexArray::new()
        .set_attrib(0, 3, stride, 0) // Positions
        .set_attrib(1, 2, stride, 3) // Texture coords
        .set_attrib(2, 3, stride, 5); // Normals
    cube_vao.unbind();

    let light_vao = VertexArray::new().set_attrib(0, 3, stride, 0);
    cube.unbind();

    let wall_texture = Texture::new()
        .set_default_parameters()
        .load_image("assets/textures/wall.jpg")?;
    let face_texture = Texture::new()
        .set_default_parameters()
        .load_image("assets/textures/awesomeface.png")?;

    // Cube shader
    let cube_shader = Program::new()
        .vertex_shader("assets/shaders/cube/cube.vert")?
        .fragment_shader("assets/shaders/cube/cube.frag")?
        .link()?;
    cube_shader.set_used();
    cube_shader.set_texture_unit("wall", 0)?;
    cube_shader.set_texture_unit("face", 1)?;
    cube_shader.set_vec3("light_color", glm::vec3(1.0, 1.0, 1.0))?;
    cube_shader.set_vec3("light_pos", light_position)?;

    // Light shader
    let light_shader = Program::new()
        .vertex_shader("assets/shaders/light/light.vert")?
        .fragment_shader("assets/shaders/light/light.frag")?
        .link()?;
    light_shader.set_used();
    light_shader.set_mat4("model", &light_model)?;

    let mut camera = Camera::new()
        .set_position(glm::vec3(0.0, 0.0, 5.0))
        .set_aspect_ratio(SCREEN_WIDTH / SCREEN_HEIGHT)
        .look_at(glm::vec3(0.0, 0.0, 0.0));

    let start_timestamp = SystemTime::now();
    let mut frame_start = SystemTime::now();

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        let now = SystemTime::now();
        let delta_time = now.duration_since(frame_start).unwrap().as_secs_f32();
        frame_start = now;

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::MouseWheel { y, .. } => camera.adjust_zoom(y),
                _ => {}
            }
        }

        // Look around
        let mouse_state = event_pump.relative_mouse_state();
        camera.rotate(mouse_state.x(), mouse_state.y());

        // Move camera
        let keyboard = event_pump.keyboard_state();
        if keyboard.is_scancode_pressed(Scancode::W) {
            camera.go(Forward, delta_time);
        }
        if keyboard.is_scancode_pressed(Scancode::S) {
            camera.go(Backward, delta_time);
        }
        if keyboard.is_scancode_pressed(Scancode::A) {
            camera.go(Left, delta_time);
        }
        if keyboard.is_scancode_pressed(Scancode::D) {
            camera.go(Right, delta_time);
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // Transformations
        let proj = camera.get_projection_matrix();
        let view = camera.get_view_matrix();

        // Draw rotating cubes
        cube_shader.set_used();
        cube_shader.set_mat4("proj", &proj)?;
        cube_shader.set_mat4("view", &view)?;

        wall_texture.bind(0);
        face_texture.bind(1);
        let seconds_elapsed = SystemTime::now()
            .duration_since(start_timestamp)
            .unwrap()
            .as_secs_f32();
        let angle = seconds_elapsed * PI / 5.0;
        for pos in cube_positions.iter() {
            let cube_model = glm::translate(&cube_model, pos);
            let cube_model = glm::rotate(&cube_model, angle, pos); // rotate around position to get different directions
            cube_shader.set_mat4("model", &cube_model)?;

            cube.draw_triangles(&cube_vao);
        }

        // Draw light cube
        light_shader.set_used();
        light_shader.set_mat4("proj", &proj)?;
        light_shader.set_mat4("view", &view)?;
        cube.draw_triangles(&light_vao);

        // // Rendering time
        // let render_ms = SystemTime::now()
        //     .duration_since(frame_start)
        //     .unwrap()
        //     .as_micros() as f32
        //     / 1000.0;
        // println!("rendering time: {} ms", render_ms);

        window.gl_swap_window();
    }

    Ok(())
}

fn error_into_string(err: failure::Error) -> String {
    let mut pretty = err.to_string();
    let mut prev = err.as_fail();
    while let Some(next) = prev.cause() {
        pretty.push_str(": ");
        pretty.push_str(&next.to_string());
        if let Some(backtrace) = next.backtrace() {
            pretty.push_str(&backtrace.to_string());
        }
        prev = next;
    }
    pretty
}
