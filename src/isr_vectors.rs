// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

extern "C" {
    fn isr_nmi();
    fn isr_mmfault();
    fn isr_busfault();
    fn isr_usagefault();

    fn isr_debugmon();
    fn isr_default();

    // FreeRTOS handlers
    fn vPortSVCHandler();
    fn xPortPendSVHandler();
    fn xPortSysTickHandler();

    // From board.c
    fn isr_reset();
    fn isr_hardfault();
}

#[no_mangle]
pub unsafe extern "C" fn isr_handler_wrapper() {
    asm!(".weak isr_nmi, isr_hardfault, isr_mmfault, isr_busfault
      .weak isr_usagefault, isr_svcall, isr_pendsv, isr_systick
      .weak isr_debugmon
      .weak isr_reserved_1

      .thumb_func
      isr_nmi:

      .thumb_func
      isr_mmfault:

      .thumb_func
      isr_busfault:

      .thumb_func
      isr_usagefault:

      .thumb_func
      isr_svcall:

      .thumb_func
      isr_pendsv:

      .thumb_func
      isr_systick:

      b isr_default

      .thumb_func
      isr_default:
      mrs r0, psp
      mrs r1, msp
      ldr r2, [r0, 0x18]
      ldr r3, [r1, 0x18]
      bkpt" :::: "volatile");
}

const ISR_COUNT: usize = 255;

#[link_section=".reset"]
#[no_mangle]
#[cfg_attr(rustfmt, rustfmt_skip)]
pub static ISR_VECTORS: [Option<unsafe extern fn()>; ISR_COUNT] = [
  Some(isr_reset),              // Reset
  Some(isr_nmi),                // NMI
  Some(isr_hardfault),          // Hard Fault
  Some(isr_mmfault),            // Memory Management Fault
  Some(isr_busfault),           // Bus Fault
  Some(isr_usagefault),         // Usage Fault
  None,                         // Reserved
  None,                         // Reserved
  None,                         // Reserved
  None,                         // Reserved

  Some(vPortSVCHandler),        // SVC Handler
  Some(isr_debugmon),           // Reserved for debug
  None,                         // Reserved
  Some(xPortPendSVHandler),     // The PendSV handler
  Some(xPortSysTickHandler),    // The SysTick handler

  Some(isr_default),            // GPIO Port A0
  Some(isr_default),            // GPIO Port A1
  Some(isr_default),            // GPIO Port A2
  Some(isr_default),            // GPIO Port A3
  None,                         // Reserved
  Some(isr_default),            // UART0 Rx and Tx
  Some(isr_default),            // UART1 Rx and Tx
  None,                         // Reserved
  Some(isr_default),            // I2C0 Master and Slave
  None, None, None, None, None, // Reserved
  Some(isr_default),            // ADC Channel 0
  Some(isr_default),            // ADC Channel 1
  Some(isr_default),            // ADC Channel 2
  Some(isr_default),            // ADC Channel 3
  Some(isr_default),            // Watchdog Timer
  Some(isr_default),            // Timer 0 subtimer A
  Some(isr_default),            // Timer 0 subtimer B
  Some(isr_default),            // Timer 1 subtimer A
  Some(isr_default),            // Timer 1 subtimer B
  Some(isr_default),            // Timer 2 subtimer A
  Some(isr_default),            // Timer 2 subtimer B
  None, None, None, None,       // Reserved
  Some(isr_default),            // Flash
  None, None, None, None, None, // Reserved
  Some(isr_default),            // Timer 3 subtimer A
  Some(isr_default),            // Timer 3 subtimer B
  None, None, None, None, None, // Reserved
  None, None, None, None,       // Reserved
  Some(isr_default),            // uDMA Software Transfer
  Some(isr_default),            // uDMA Error
  None, None, None, None, None, // Reserved
  None, None, None, None, None, // Reserved
  None, None, None, None, None, // Reserved
  None, None, None, None, None, // Reserved
  None, None, None, None, None, // Reserved
  None, None, None, None, None, // Reserved
  None, None, None, None, None, // Reserved
  None, None, None, None, None, // Reserved
  None, None, None, None, None, // Reserved
  None, None, None, None, None, // Reserved
  None, None, None, None, None, // Reserved
  None, None, None, None, None, // Reserved
  None, None, None, None, None, // Reserved
  None, None, None, None, None, // Reserved
  None, None, None, None, None, // Reserved
  None, None, None, None, None, // Reserved
  None, None, None, None, None, // Reserved
  None, None, None, None, None, // Reserved
  None, None, None, None, None, // Reserved
  None, None, None, None, None, // Reserved
  Some(isr_default),            // SHA
  None, None,                   // Reserved
  Some(isr_default),            // AES
  None,                         // Reserved
  Some(isr_default),            // DES
  None, None, None, None, None, // Reserved
  Some(isr_default),            // SDHost
  None,                         // Reserved
  Some(isr_default),            // I2S
  None,                         // Reserved
  Some(isr_default),            // Camera
  None, None, None, None, None, // Reserved
  None, None,                   // Reserved
  Some(isr_default),            // NWP to APPS Interrupt
  Some(isr_default),            // Power, Reset and Clock module
  None, None,                   // Reserved
  Some(isr_default),            // Shared SPI
  Some(isr_default),            // Generic SPI
  Some(isr_default),            // Link SPI
  None, None, None, None, None, // Reserved
  None, None, None, None, None, // Reserved
  None, None, None, None, None, // Reserved
  None, None, None, None, None, // Reserved
  None, None, None, None, None, // Reserved
  None, None, None, None, None, // Reserved
  None, None, None, None, None, // Reserved
  None, None, None, None, None, // Reserved
  None, None, None, None, None, // Reserved
  None, None, None, None, None, // Reserved
  None, None, None, None, None, // Reserved
  None, None, None, None, None, // Reserved
  None, None                    // Reserved
];
