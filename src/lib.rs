use libc::{c_char, c_void};

#[repr(C)]
pub struct MlxT {
    window: *mut c_void,
    context: *mut c_void,
    width: i32,
    height: i32,
    delta_time: f64,
}

#[repr(C)]
pub struct MlxInstanceT {
    x: i32,
    y: i32,
    z: i32,
    enabled: bool,
}

#[repr(C)]
pub struct MlxImageT {
    width: i32,
    height: i32,
    pixels: *mut i8,
    instances: *mut MlxInstanceT,
    count: usize,
    enabled: bool,
    context: *mut c_void,
}

extern "C" {
    pub fn mlx_init(width: i32, height: i32, title: *const c_char, resize: bool) -> *mut MlxT;
    pub fn mlx_loop(mlx: *mut MlxT) -> c_void;
    pub fn mlx_terminate(mlx: *mut MlxT) -> c_void;
    pub fn mlx_new_image(mlx: *mut MlxT, width: i32, height: i32) -> *mut MlxImageT;
    pub fn mlx_put_pixel(image: *mut MlxImageT, x: i32, y: i32, color: u32);
    pub fn mlx_image_to_window(mlx: *mut MlxT, image: *mut MlxImageT, x: i32, y: i32);
}

