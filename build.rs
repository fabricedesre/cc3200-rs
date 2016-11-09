extern crate common_build;

use std::path::Path;

fn get_lib(opt: &str) -> String {
    let libgcc = common_build::gcc_config().get_compiler().to_command()
        .arg(opt)
        .output()
        .unwrap();

    String::from(Path::new(String::from_utf8_lossy(&libgcc.stdout).trim())
        .parent()
        .unwrap()
        .to_str()
        .unwrap())
}

fn main() {
    // libm and libc are in the same directory, so we don't see to add the same path twice
    println!("cargo:rustc-link-search=native={}",
             get_lib("-print-file-name=libc.a"));
    println!("cargo:rustc-link-search=native={}",
             get_lib("-print-libgcc-file-name"));
}
