extern crate glfw;

mod shader;

use std::{
    ffi::c_void,
    ptr,
};
use std::mem;
use gl::types::{GLfloat, GLsizei, GLsizeiptr};
use glfw::{Action, Context, GlfwReceiver, Key, WindowEvent};
use shader::Shader;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

fn main() {
    // Init OpenGL
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    // Window options
    glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
    glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    // Create a window
    let (mut window, events) = glfw
        .create_window(
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
            "scop",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    window.set_framebuffer_size_callback(|_, width, height| unsafe {
        gl::Viewport(0, 0, width, height)
    });
    window.make_current();

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let vertices1: [f32; 18] = [
        0.5, -0.5, 0.0, 1.0, 0.0, 0.0,
        -0.5, -0.5, 0.0, 0.0, 1.0, 0.0,
        0.0, 0.5, 0.0, 0.0, 1.0, 0.0,
    ];
    // let vertices2: [f32; 18] = [
    //     -0.14, 0.24, 0.0, 0.0, 0.0, 1.0,
    //     0.14, 0.24, 0.0, 0.0, 1.0, 0.0,
    //     0.0, 0.14, 0.0, 1.0, 0.0, 0.0,
    // ];
    // let indices = [0, 1, 3, 1, 2, 3];

    // We need to write manually at least 2 shaders: vertex shader and fragment shader
    let mut shader = Shader::new("./src/shaders/vertex.shader", "./src/shaders/fragment.shader");
    let (mut vbos, mut vaos) = unsafe {

        // Load vertex data
        let (mut vbos, mut vaos, mut ebo) = ([0, 0], [0, 0], 0);
        gl::GenVertexArrays(2, vaos.as_mut_ptr());
        gl::GenBuffers(2, vbos.as_mut_ptr());
        gl::GenBuffers(1, &mut ebo);

        gl::BindVertexArray(vaos[0]);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbos[0]);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices1.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
            &vertices1[0] as *const f32 as *const std::os::raw::c_void,
            gl::STATIC_DRAW,
        );
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            6 * mem::size_of::<GLfloat>() as GLsizei,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            6 * mem::size_of::<GLfloat>() as GLsizei,
            (3 * mem::size_of::<GLfloat>()) as *const c_void,
        );
        gl::EnableVertexAttribArray(1);

        // gl::BindVertexArray(vaos[1]);
        // gl::BindBuffer(gl::ARRAY_BUFFER, vbos[1]);
        // gl::BufferData(
        //     gl::ARRAY_BUFFER,
        //     (vertices2.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
        //     &vertices2[0] as *const f32 as *const std::os::raw::c_void,
        //     gl::STATIC_DRAW,
        // );
        // gl::VertexAttribPointer(
        //     0,
        //     3,
        //     gl::FLOAT,
        //     gl::FALSE,
        //     6 * mem::size_of::<GLfloat>() as GLsizei,
        //     ptr::null(),
        // );
        // gl::EnableVertexAttribArray(0);
        // gl::VertexAttribPointer(
        //     1,
        //     3,
        //     gl::FLOAT,
        //     gl::FALSE,
        //     6 * mem::size_of::<GLfloat>() as GLsizei,
        //     (3 * mem::size_of::<GLfloat>()) as *const c_void,
        // );
        // gl::EnableVertexAttribArray(1);
        // gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());
        // gl::EnableVertexAttribArray(0);

        // gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        // gl::BufferData(
        //     gl::ELEMENT_ARRAY_BUFFER,
        //     (indices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
        //     &indices[0] as *const i32 as *const std::os::raw::c_void,
        //     gl::STATIC_DRAW,
        // );
        // gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        // gl::BindVertexArray(0);

        // Draw wireframe polygons
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        (vbos, vaos)
    };

    while !window.should_close() {
        handle_window_events(&mut window, &events);
        unsafe {
            gl::ClearColor(0.2 as f32, 0.3 as f32, 0.3 as f32, 1.0 as f32);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // draw triangle, finally
            shader.use_program();
            // let time_value = glfw.get_time() as f32;
            // let green_value = time_value.sin() / 2.0 + 0.5;
            // let our_color = CString::new("ourColor").unwrap();
            // let vertex_color_location = gl::GetUniformLocation(shader_program, our_color.as_ptr());
            // gl::Uniform4f(vertex_color_location, 0.0, green_value, 0.0, 1.0);
            gl::BindVertexArray(vaos[0]);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            // gl::BindVertexArray(vaos[1]);
            // gl::DrawArrays(gl::TRIANGLES, 0, 3);
            // gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }
        window.swap_buffers();
        glfw.poll_events();
    }

    unsafe {
        gl::DeleteVertexArrays(2, vaos.as_mut_ptr());
        gl::DeleteBuffers(2, vbos.as_mut_ptr());
    }
}

fn handle_window_events(window: &mut glfw::Window, events: &GlfwReceiver<(f64, WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                gl::Viewport(0, 0, width, height)
            },
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            }
            _ => {}
        }
    }
}
