use scop::*;
use libc::c_char;

fn main() {
    let window_name: &str = "dale";
    unsafe {
        let mlx = mlx_init(800, 600, window_name.as_ptr() as *const c_char, true);
        mlx_loop(mlx);
        mlx_terminate(mlx);
    }
}
