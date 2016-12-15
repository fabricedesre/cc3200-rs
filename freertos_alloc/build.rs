// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate common_build;
fn main() {
  common_build::gcc_config()
    .include("../cc3200-sys")
    .include("../cc3200-sys/sdk/third_party/FreeRTOS/source/include")
    .include("../cc3200-sys/sdk/third_party/FreeRTOS/source/portable/GCC/ARM_CM4")
    .file("realloc_helper.c")
    .file("../cc3200-sys/sdk/third_party/FreeRTOS/source/portable/MemMang/heap_4.c")
    .compile("libfreertos_alloc.a");

  println!("cargo:rustc-link-lib=freertos_alloc");
}
