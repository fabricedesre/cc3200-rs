extern crate gcc;

// Various crates need to ensure that they all use the same compiler options,
// so this crate provides a function that returns a gcc::Config object with
// all of the "must have" options already set.

pub fn gcc_config() -> gcc::Config {
    let mut config = gcc::Config::new();
    config.compiler("arm-none-eabi-gcc")
        .define("gcc", None)
        .define("USE_FREERTOS", None)
        .define("SL_PLATFORM_MULTI_THREADED", None)
        .flag("-std=c99")
        .flag("-mthumb")
        .flag("-mcpu=cortex-m4")
        .flag("-mfloat-abi=soft");
    config
}
