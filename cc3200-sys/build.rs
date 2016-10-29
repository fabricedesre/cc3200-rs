extern crate gcc;
fn main() {
  gcc::Config::new()
    .compiler("arm-none-eabi-gcc")
    .define("gcc", None)
    .include("sdk")
    .include("sdk/inc")
    .include("sdk/driverlib")
    .include("sdk/example/common")
    .file("board.c")
    .file("sdk/driverlib/cpu.c")
    .file("sdk/driverlib/gpio.c")
    .file("sdk/driverlib/interrupt.c")
    .file("sdk/driverlib/pin.c")
    .file("sdk/driverlib/prcm.c")
    .file("sdk/driverlib/uart.c")
    .file("sdk/driverlib/utils.c")
    .file("sdk/example/common/gpio_if.c")
    .file("sdk/example/common/uart_if.c")
    .compile("libboard.a");

  println!("cargo:rustc-link-lib=board");
}
