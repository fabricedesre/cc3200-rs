use std::path::Path;
use std::process::Command;

fn get_lib(opt: &str) -> String {
    let libgcc = Command::new("arm-none-eabi-gcc")
      .arg("-mthumb")
      .arg("-mcpu=cortex-m4")
      .arg("-mfloat-abi=soft")
      .arg(opt)
      .output()
      .unwrap();

    String::from(Path::new(String::from_utf8_lossy(&libgcc.stdout).trim()).parent().unwrap().to_str().unwrap())
}

fn main() {
    // libm and libc are in the same directory, so we don't see to add the same path twice
    //println!("cargo:rustc-link-search=native={}", get_lib("-print-file-name=libm.a"));
    println!("cargo:rustc-link-search=native={}", get_lib("-print-file-name=libc.a"));
    println!("cargo:rustc-link-search=native={}", get_lib("-print-libgcc-file-name"));
}
