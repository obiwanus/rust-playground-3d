extern crate gl;
extern crate sdl2;

pub mod render_gl;

use gl::types::{GLint, GLsizeiptr, GLuint, GLvoid};
use std::ffi::CString;

use render_gl::{Program, Shader};

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);

    let window = video_subsystem
        .window("Game", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let gl_context = window.gl_create_context().unwrap();
    let gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl::Viewport(0, 0, 900, 700);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let vertices: Vec<f32> = vec![-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];
    let mut vbo: GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,                                            // target
            (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const GLvoid,                          // pointer to data
            gl::STATIC_DRAW,                                             // usage
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind
    }
    let mut vao: GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::EnableVertexAttribArray(0); // layout (location = 0) in vertex shader
        gl::VertexAttribPointer(
            0,                                         // index of the generic vertex attribute
            3,         // number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalised (int-to-float conversion)
            (3 * std::mem::size_of::<f32>()) as GLint, // stride
            std::ptr::null(), // offset of the first component
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    let vert_shader =
        Shader::vertex_from_source(&CString::new(include_str!("triangle.vert")).unwrap()).unwrap();
    let frag_shader =
        Shader::fragment_from_source(&CString::new(include_str!("triangle.frag")).unwrap())
            .unwrap();
    let shader_program = Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        shader_program.set_used();

        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawArrays(
                gl::TRIANGLES,
                0, // starting index in the enabled arrays
                3, // number of indices to be rendered
            );
        }

        window.gl_swap_window();
    }
}
