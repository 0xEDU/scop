fn main() {
    println!("cargo:rustc-link-search=native=./mlx");
    println!("cargo:rustc-link-lib=static=mlx42");

    // Link the additional libraries
    println!("cargo:rustc-link-lib=dl");
    println!("cargo:rustc-link-lib=glfw");
    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=m"); 
}
