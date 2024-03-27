extern crate glfw;
extern crate image;

mod shader;
mod tuple;
mod matrices;

use cgmath::Deg;
// use cgmath::{vec3, Matrix, Matrix4, Rad, SquareMatrix};
use gl::types::{GLfloat, GLsizei, GLsizeiptr};
use glfw::{Action, Context, GlfwReceiver, Key, WindowEvent};
use matrices::{perspective, Matrix};
use shader::Shader;
use tuple::vector;
use c_str_macro::c_str;
use std::f32::consts::PI;
use std::mem;
use std::path::Path;
use std::{ffi::c_void, ptr};

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

    let vertices1: [f32; 32] = [
        0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0,
        0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0,
        -0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
        -0.5, 0.5, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0,
    ];
    // let vertices2: [f32; 18] = [
    //     -0.14, 0.24, 0.0, 0.0, 0.0, 1.0,
    //     0.14, 0.24, 0.0, 0.0, 1.0, 0.0,
    //     0.0, 0.14, 0.0, 1.0, 0.0, 0.0,
    // ];
    let indices = [0, 1, 3, 1, 2, 3];

    // We need to write manually at least 2 shaders: vertex shader and fragment shader
    let mut shader = Shader::new(
        "./src/shaders/vertex.shader",
        "./src/shaders/fragment.shader",
    );
    let (mut vbos, mut vaos, texture1, texture2) = unsafe {
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

        let stride = 8 * mem::size_of::<GLfloat>() as GLsizei;

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (3 * mem::size_of::<GLfloat>()) as *const c_void,
        );
        gl::EnableVertexAttribArray(1);

        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (6 * mem::size_of::<GLfloat>()) as *const c_void,
        );
        gl::EnableVertexAttribArray(2);

        let (mut texture1, mut texture2) = (0, 0);

        // Generate texture 1
        gl::GenTextures(1, &mut texture1);
        gl::BindTexture(gl::TEXTURE_2D, texture1);

        // texture wrapping
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);

        // texture filtering
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        // load texture 1
        let img = image::open(&Path::new("./resources/container.jpg")).expect("Failed to load texture");
        let data = img.as_bytes();
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            img.width() as i32,
            img.height() as i32,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            &data[0] as *const u8 as *const c_void,
        );

        // Generate texture 2
        gl::GenTextures(1, &mut texture2);
        gl::BindTexture(gl::TEXTURE_2D, texture2);

        // texture wrapping
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

        // texture filtering
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        // load texture 2
        let img = image::open(&Path::new("./resources/awesomeface.png")).expect("Failed to load texture");
        let img = img.flipv();
        let data = img.as_bytes();
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            img.width() as i32,
            img.height() as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            &data[0] as *const u8 as *const c_void,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);

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

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
            &indices[0] as *const i32 as *const std::os::raw::c_void,
            gl::STATIC_DRAW,
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);

        // Draw wireframe polygons
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        shader.use_program();
        shader.set_int(&mut String::from("texture1"), 0);
        shader.set_int(&mut String::from("texture2"), 1);

        (vbos, vaos, texture1, texture2)
    };

    while !window.should_close() {
        handle_window_events(&mut window, &events);
        unsafe {
            gl::ClearColor(0.2 as f32, 0.3 as f32, 0.3 as f32, 1.0 as f32);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // draw triangle, finally
            // shader.set_float(String::from("xOffset").as_mut_str(), 1.0);
            // let time_value = glfw.get_time() as f32;
            // let green_value = time_value.sin() / 2.0 + 0.5;
            // let our_color = CString::new("ourColor").unwrap();
            // let vertex_color_location = gl::GetUniformLocation(shader_program, our_color.as_ptr());
            // gl::Uniform4f(vertex_color_location, 0.0, green_value, 0.0, 1.0);

            // bind texture on corresponding units
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture1);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, texture2);

            // create transformations
            // let mut transform = Matrix::identity();
            // transform = transform * Matrix::from_translation(vector(0.0, 0.0, 0.0));
            // transform = Matrix::from_angle_z(glfw.get_time() as f32) * transform;
            let model = Matrix::from_angle_x(-55. * PI / 180.);
            let view = Matrix::from_translation(vector(0., 0., -3.));
            let projection = perspective(45. * PI / 180., WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32, 0.1, 100.);

            shader.use_program();
            let model_loc = gl::GetUniformLocation(shader.id, c_str!("model").as_ptr());
            gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, model.as_ptr());

            let view_loc = gl::GetUniformLocation(shader.id, c_str!("view").as_ptr());
            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, view.as_ptr());

            let projection_loc = gl::GetUniformLocation(shader.id, c_str!("projection").as_ptr());
            gl::UniformMatrix4fv(projection_loc, 1, gl::FALSE, projection.as_ptr());

            gl::BindVertexArray(vaos[0]);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
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
