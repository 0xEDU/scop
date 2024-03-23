use scop::*;
use scop::KeysT::*;
use libc::{c_char, c_void};

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

extern "C" fn ft_hook(mlx_v: *mut c_void) -> () {
    let mlx = mlx_v as *mut MlxT;
    unsafe {
        if mlx_is_key_down(mlx, MlxKeyEscape) {
            mlx_close_window(mlx);
        }
    }
}

fn main() {
    let window_name: &str = "dale";
    unsafe {
        let mlx = mlx_init(WIDTH, HEIGHT, window_name.as_ptr() as *const c_char, true);
        let image = mlx_new_image(mlx, WIDTH, HEIGHT);
        for i in 0..WIDTH {
            mlx_put_pixel(image, i, 300, 0xff0000ff);
        }
        mlx_image_to_window(mlx, image, 0, 0);
        mlx_loop_hook(mlx, ft_hook, mlx as *mut c_void);
        mlx_loop(mlx);
        mlx_terminate(mlx);
    }
}
