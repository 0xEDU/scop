extern crate glfw;
extern crate image;

mod camera;
mod matrices;
mod parser;
mod scene;
mod shader;
mod tuple;

use c_str_macro::c_str;
use camera::Camera;
use gl::types::{GLfloat, GLsizei, GLsizeiptr};
use glfw::{Action, Context, Glfw, GlfwReceiver, Key, PWindow, WindowEvent};
use matrices::{perspective, Matrix};
use parser::Parser;
use scene::Scene;
use shader::Shader;
use std::mem;
use std::path::Path;
use std::{ffi::c_void, ptr};
use tuple::{normalize, vector};

const WINDOW_WIDTH: u32 = 1920;
const WINDOW_HEIGHT: u32 = 1080;

fn create_configured_window(glfw: &mut Glfw) -> (PWindow, GlfwReceiver<(f64, WindowEvent)>) {
    glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
    glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let (mut window, events) = glfw
        .create_window(
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
            "scop",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");
    window.set_key_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_cursor_mode(glfw::CursorMode::Disabled);
    window.set_scroll_polling(true);
    window.set_framebuffer_size_polling(true);
    window.set_framebuffer_size_callback(|_, width, height| unsafe {
        gl::Viewport(0, 0, width, height)
    });
    window.make_current();

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    (window, events)
}

fn main() {
    // Init OpenGL
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    // Create a window
    let (mut window, events) = create_configured_window(&mut glfw);
    let vertices: [f32; 288] = [
        // positions          // normals           // texture coords
        -0.5, -0.5, -0.5, 0.0, 0.0, -1.0, 0.0, 0.0, 0.5, -0.5, -0.5, 0.0, 0.0, -1.0, 1.0, 0.0, 0.5,
        0.5, -0.5, 0.0, 0.0, -1.0, 1.0, 1.0, 0.5, 0.5, -0.5, 0.0, 0.0, -1.0, 1.0, 1.0, -0.5, 0.5,
        -0.5, 0.0, 0.0, -1.0, 0.0, 1.0, -0.5, -0.5, -0.5, 0.0, 0.0, -1.0, 0.0, 0.0, -0.5, -0.5,
        0.5, 0.0, 0.0, 1.0, 0.0, 0.0, 0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 1.0, 0.0, 0.5, 0.5, 0.5, 0.0,
        0.0, 1.0, 1.0, 1.0, 0.5, 0.5, 0.5, 0.0, 0.0, 1.0, 1.0, 1.0, -0.5, 0.5, 0.5, 0.0, 0.0, 1.0,
        0.0, 1.0, -0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 0.0, -0.5, 0.5, 0.5, -1.0, 0.0, 0.0, 1.0,
        0.0, -0.5, 0.5, -0.5, -1.0, 0.0, 0.0, 1.0, 1.0, -0.5, -0.5, -0.5, -1.0, 0.0, 0.0, 0.0, 1.0,
        -0.5, -0.5, -0.5, -1.0, 0.0, 0.0, 0.0, 1.0, -0.5, -0.5, 0.5, -1.0, 0.0, 0.0, 0.0, 0.0,
        -0.5, 0.5, 0.5, -1.0, 0.0, 0.0, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, 0.0, 1.0, 0.0, 0.5, 0.5,
        -0.5, 1.0, 0.0, 0.0, 1.0, 1.0, 0.5, -0.5, -0.5, 1.0, 0.0, 0.0, 0.0, 1.0, 0.5, -0.5, -0.5,
        1.0, 0.0, 0.0, 0.0, 1.0, 0.5, -0.5, 0.5, 1.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0,
        0.0, 1.0, 0.0, -0.5, -0.5, -0.5, 0.0, -1.0, 0.0, 0.0, 1.0, 0.5, -0.5, -0.5, 0.0, -1.0, 0.0,
        1.0, 1.0, 0.5, -0.5, 0.5, 0.0, -1.0, 0.0, 1.0, 0.0, 0.5, -0.5, 0.5, 0.0, -1.0, 0.0, 1.0,
        0.0, -0.5, -0.5, 0.5, 0.0, -1.0, 0.0, 0.0, 0.0, -0.5, -0.5, -0.5, 0.0, -1.0, 0.0, 0.0, 1.0,
        -0.5, 0.5, -0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 0.5, 0.5, -0.5, 0.0, 1.0, 0.0, 1.0, 1.0, 0.5,
        0.5, 0.5, 0.0, 1.0, 0.0, 1.0, 0.0, 0.5, 0.5, 0.5, 0.0, 1.0, 0.0, 1.0, 0.0, -0.5, 0.5, 0.5,
        0.0, 1.0, 0.0, 0.0, 0.0, -0.5, 0.5, -0.5, 0.0, 1.0, 0.0, 0.0, 1.0,
    ];

    let cube_positions = [
        vector(0.0, 0.0, 0.0),
        vector(2.0, 5.0, -15.0),
        vector(-1.5, -2.2, -2.5),
        vector(-3.8, -2.0, -12.3),
        vector(2.4, -0.4, -3.5),
        vector(-1.7, 3.0, -7.5),
        vector(1.3, -2.0, -2.5),
        vector(1.5, 2.0, -2.5),
        vector(1.5, 0.2, -1.5),
        vector(-1.3, 1.0, -1.5),
    ];
    let point_light_positions = [
        vector(0.7, 0.2, 2.0),
        vector(2.3, -3.3, -4.0),
        vector(-4.0, 2.0, -12.0),
        vector(0.0, 0.0, -3.0),
    ];

    // We need to write manually at least 2 shaders: vertex shader and fragment shader
    let mut shader = Shader::new(
        "./src/shaders/vertex.shader",
        "./src/shaders/fragment.shader",
    );
    let mut light_shader = Shader::new(
        "./src/shaders/light_vertex.shader",
        "./src/shaders/light_fragment.shader",
    );
    let mut cam = Camera::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    let parser = Parser::new("./resources/42.obj").unwrap();
    let scene: Scene = parser.generate_scene().unwrap();
    for i in 0..scene.vertices.len() {
        println!(
            "Vertex -> x: {}, y: {}, z: {}, w: {}",
            scene.vertices[i].x, scene.vertices[i].y, scene.vertices[i].z, scene.vertices[i].w
        );
    }
    let (vbo, vao, light_vao, diffuse_map, specular_map) = unsafe {
        gl::Enable(gl::DEPTH_TEST);

        // Load vertex data
        let (mut vbo, mut vao) = (0, 0);
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
            &vertices[0] as *const f32 as *const std::os::raw::c_void,
            gl::STATIC_DRAW,
        );

        let stride = (8 * mem::size_of::<GLfloat>()) as GLsizei;
        // position attrib
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(0);
        // normal attrib
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (3 * mem::size_of::<GLfloat>()) as *const c_void,
        );
        gl::EnableVertexAttribArray(1);
        // texture coord attrib
        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (6 * mem::size_of::<GLfloat>()) as *const c_void,
        );
        gl::EnableVertexAttribArray(2);

        // Load light data
        let mut light_vao = 0;
        gl::GenVertexArrays(1, &mut light_vao);
        gl::BindVertexArray(light_vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(0);

        let mut diffuse_map = 0;

        // Generate diffuse_map texture
        gl::GenTextures(1, &mut diffuse_map);
        gl::BindTexture(gl::TEXTURE_2D, diffuse_map);

        // Texture wrapping
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

        // texture filtering
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        // load diffuse_map texture
        let img =
            image::open(&Path::new("./resources/container2.png")).expect("Failed to load texture");
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

        let mut specular_map = 0;

        // Generate diffuse_map texture
        gl::GenTextures(1, &mut specular_map);
        gl::BindTexture(gl::TEXTURE_2D, specular_map);

        // Texture wrapping
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

        // texture filtering
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        // load diffuse_map texture
        let img = image::open(&Path::new("./resources/container2_specular.png"))
            .expect("Failed to load texture");
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

        shader.use_program();
        shader.set_int(c_str!("material.diffuse"), 0);
        shader.set_int(c_str!("material.specular"), 1);

        (vbo, vao, light_vao, diffuse_map, specular_map)
    };

    while !window.should_close() {
        cam.update_delta_time(glfw.get_time() as f32);
        unsafe {
            gl::ClearColor(0.1 as f32, 0.1 as f32, 0.1 as f32, 1.0 as f32);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // object shader
            shader.use_program();
            shader.set_tuple(c_str!("viewPos"), cam.camera_position);
            shader.set_float(c_str!("material.shininess"), 32.);

            // directional light
            shader.set_vector(c_str!("dirLight.direction"), -0.2, -1.0, -0.3);
            shader.set_vector(c_str!("dirLight.ambient"), 0.05, 0.05, 0.05);
            shader.set_vector(c_str!("dirLight.diffuse"), 0.4, 0.4, 0.4);
            shader.set_vector(c_str!("dirLight.specular"), 0.5, 0.5, 0.5);

            // point light 1
            shader.set_tuple(c_str!("pointLights[0].position"), point_light_positions[0]);
            shader.set_vector(c_str!("pointLights[0].ambient"), 0.05, 0.05, 0.05);
            shader.set_vector(c_str!("pointLights[0].diffuse"), 0.8, 0.8, 0.8);
            shader.set_vector(c_str!("pointLights[0].specular"), 1.0, 1.0, 1.0);
            shader.set_float(c_str!("pointLights[0].constant"), 1.0);
            shader.set_float(c_str!("pointLights[0].linear"), 0.09);
            shader.set_float(c_str!("pointLights[0].quadratic"), 0.032);
            // point light 2
            shader.set_tuple(c_str!("pointLights[1].position"), point_light_positions[1]);
            shader.set_vector(c_str!("pointLights[1].ambient"), 0.05, 0.05, 0.05);
            shader.set_vector(c_str!("pointLights[1].diffuse"), 0.8, 0.8, 0.8);
            shader.set_vector(c_str!("pointLights[1].specular"), 1.0, 1.0, 1.0);
            shader.set_float(c_str!("pointLights[1].constant"), 1.0);
            shader.set_float(c_str!("pointLights[1].linear"), 0.09);
            shader.set_float(c_str!("pointLights[1].quadratic"), 0.032);
            // point light 3
            shader.set_tuple(c_str!("pointLights[2].position"), point_light_positions[2]);
            shader.set_vector(c_str!("pointLights[2].ambient"), 0.05, 0.05, 0.05);
            shader.set_vector(c_str!("pointLights[2].diffuse"), 0.8, 0.8, 0.8);
            shader.set_vector(c_str!("pointLights[2].specular"), 1.0, 1.0, 1.0);
            shader.set_float(c_str!("pointLights[2].constant"), 1.0);
            shader.set_float(c_str!("pointLights[2].linear"), 0.09);
            shader.set_float(c_str!("pointLights[2].quadratic"), 0.032);
            // point light 4
            shader.set_tuple(c_str!("pointLights[3].position"), point_light_positions[3]);
            shader.set_vector(c_str!("pointLights[3].ambient"), 0.05, 0.05, 0.05);
            shader.set_vector(c_str!("pointLights[3].diffuse"), 0.8, 0.8, 0.8);
            shader.set_vector(c_str!("pointLights[3].specular"), 1.0, 1.0, 1.0);
            shader.set_float(c_str!("pointLights[3].constant"), 1.0);
            shader.set_float(c_str!("pointLights[3].linear"), 0.09);
            shader.set_float(c_str!("pointLights[3].quadratic"), 0.032);

            // camera transformation
            shader.set_matrix(c_str!("view"), &cam.look_at());

            // projection transformation
            shader.set_matrix(
                c_str!("projection"),
                &perspective(
                    cam.fov,
                    WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32,
                    0.1,
                    100.,
                ),
            );

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, diffuse_map);

            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, specular_map);

            // model transformations
            gl::BindVertexArray(vao);
            for (i, cube_position) in cube_positions.iter().enumerate() {
                let model = Matrix::from_translation(*cube_position);
                let angle: f32 = 20. * i as f32;
                shader.set_matrix(
                    c_str!("model"),
                    &(Matrix::from_axis_angle(normalize(vector(1.0, 0.3, 0.5)), angle) * model),
                );
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
            // light shader
            light_shader.use_program();

            // light view
            light_shader.set_matrix(c_str!("view"), &cam.look_at());

            // light projection
            light_shader.set_matrix(
                c_str!("projection"),
                &perspective(
                    cam.fov,
                    WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32,
                    0.1,
                    100.,
                ),
            );

            // light model
            gl::BindVertexArray(light_vao);
            for (_, point_light) in point_light_positions.iter().enumerate() {
                let model = Matrix::from_translation(*point_light);
                let model = Matrix::from_scale(0.2) * model;
                light_shader.set_matrix(c_str!("model"), &model);
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }

            handle_keyboard_input(&mut window, &mut cam);
        }
        handle_window_events(&mut window, &events, &mut cam);
        window.swap_buffers();
        glfw.poll_events();
    }

    unsafe {
        gl::DeleteVertexArrays(1, &vao);
        gl::DeleteBuffers(1, &vbo);
    }
}

fn handle_window_events(
    window: &mut glfw::Window,
    events: &GlfwReceiver<(f64, WindowEvent)>,
    cam: &mut Camera,
) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                gl::Viewport(0, 0, width, height)
            },
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            }
            glfw::WindowEvent::CursorPos(xpos, ypos) => {
                cam.handle_cursor(xpos as f32, ypos as f32);
            }
            glfw::WindowEvent::Scroll(_, yoffset) => {
                cam.handle_scroll(yoffset as f32);
            }
            _ => {}
        }
    }
}

fn handle_keyboard_input(window: &mut glfw::Window, cam: &mut Camera) {
    if window.get_key(Key::Enter) == Action::Press {
        window.set_cursor_mode(glfw::CursorMode::Normal);
    }

    cam.update_camera_speed();
    if window.get_key(Key::W) == Action::Press {
        cam.handle_w();
    }
    if window.get_key(Key::S) == Action::Press {
        cam.handle_s();
    }
    if window.get_key(Key::A) == Action::Press {
        cam.handle_a();
    }
    if window.get_key(Key::D) == Action::Press {
        cam.handle_d();
    }
    if window.get_key(Key::P) == Action::Press {
        // Draw wireframe polygons
        unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE) };
    }
}
