use libc::{c_char, c_void};

#[repr(C)]
pub struct MlxT {
    window: *mut c_void,
    context: *mut c_void,
    width: i32,
    height: i32,
    delta_time: f64,
}

extern "C" {
    pub fn mlx_init(width: i32, height: i32, title: *const c_char, resize: bool) -> *mut MlxT;
    pub fn mlx_loop(mlx: *mut MlxT) -> c_void;
    pub fn mlx_terminate(mlx: *mut MlxT) -> c_void;
}

