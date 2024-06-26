use crate::{matrices::Matrix, tuple::Tuple};
use gl::types::{GLchar, GLint};
use std::{
    ffi::{CStr, CString},
    fs::File,
    io::Read,
    ptr, str,
};

pub struct Shader {
    pub id: u32,
}

impl Shader {
    pub fn new(vertex_path: &str, fragment_path: &str) -> Self {
        let mut vertex_shader_file = File::open(vertex_path).unwrap();
        let mut fragment_shader_file = File::open(fragment_path).unwrap();

        let mut vertex_shader_contents = String::new();
        vertex_shader_file
            .read_to_string(&mut vertex_shader_contents)
            .unwrap();

        let mut fragment_shader_contents = String::new();
        fragment_shader_file
            .read_to_string(&mut fragment_shader_contents)
            .unwrap();

        let shader_program_id = unsafe {
            // Vertex shader
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            let c_str_vert = CString::new(vertex_shader_contents.as_bytes()).unwrap();
            gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
            gl::CompileShader(vertex_shader);
            let mut success = gl::FALSE as GLint;
            let mut info_log = Vec::with_capacity(512);
            info_log.set_len(512 - 1);
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(
                    vertex_shader,
                    512,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                println!(
                    "ERROR COMPILATION FAILED!\n{}",
                    str::from_utf8(&info_log).unwrap()
                );
            }
            // Fragment shader
            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            let c_str_frag = CString::new(fragment_shader_contents.as_bytes()).unwrap();
            gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
            gl::CompileShader(fragment_shader);
            gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(
                    fragment_shader,
                    512,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                for i in 0..info_log.len() {
                    print!("{}", info_log[i] as char);
                }
                println!();
            }

            // Link shaders
            let shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);
            gl::GetProgramiv(shader_program, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(
                    vertex_shader,
                    512,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                println!(
                    "ERROR LINKING FAILED!\n{}",
                    str::from_utf8(&info_log).unwrap()
                );
            }

            // Clean up
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            shader_program
        };

        Self {
            id: shader_program_id,
        }
    }

    pub fn use_program(&mut self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    // pub fn set_bool(&mut self, name: &mut str, value: bool) {
    //     unsafe {
    //         gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_mut_ptr() as *mut i8), value as i32);
    //     }
    // }

    pub fn set_int(&self, name: &CStr, value: i32) {
        unsafe {
            gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_ptr()), value);
        }
    }

    pub fn set_float(&self, name: &CStr, value: f32) {
        unsafe {
            gl::Uniform1f(gl::GetUniformLocation(self.id, name.as_ptr()), value);
        }
    }

    pub fn set_matrix(&self, name: &CStr, m: &Matrix) {
        unsafe {
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.id, name.as_ptr()),
                1,
                gl::FALSE,
                m.as_ptr(),
            );
        }
    }

    pub fn set_vector(&self, name: &CStr, x: f32, y: f32, z: f32) {
        unsafe {
            gl::Uniform3f(gl::GetUniformLocation(self.id, name.as_ptr()), x, y, z);
        }
    }

    pub fn set_tuple(&self, name: &CStr, t: Tuple) {
        unsafe {
            gl::Uniform3f(
                gl::GetUniformLocation(self.id, name.as_ptr()),
                t.x,
                t.y,
                t.z,
            );
        }
    }
}
