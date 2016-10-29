#include <stdint.h>

#include "board.h"

// Driverlib includes
#include "hw_types.h"
#include "hw_ints.h"
#include "hw_memmap.h"
#include "hw_common_reg.h"
#include "interrupt.h"
#include "hw_apps_rcm.h"
#include "prcm.h"
#include "rom.h"
#include "rom_map.h"
#include "prcm.h"
#include "gpio.h"
#include "pin.h"
#include "uart.h"
#include "uart_if.h"
#include "utils.h"

extern void (* const ISR_VECTORS[])(void);
extern uint32_t _bss;
extern uint32_t _ebss;

void board_init(void) {

    // On the cc3200 there is no internal flash, so we don't need to copy
    // initialized data from ROM into RAM, the bootloader does it for us
    // when it loads the program into RAM.

    //
    // Zero fill the bss segment.
    //
    __asm("    ldr     r0, =_bss\n"
          "    ldr     r1, =_ebss\n"
          "    mov     r2, #0\n"
          "    .thumb_func\n"
          "zero_loop:\n"
          "    cmp     r0, r1\n"
          "    it      lt\n"
          "    strlt   r2, [r0], #4\n"
          "    blt     zero_loop");


    // Initialize the vector table
    MAP_IntVTableBaseSet((unsigned long)&ISR_VECTORS[0]);
    MAP_IntMasterEnable();
    MAP_IntEnable(FAULT_SYSTICK);

    // Initialize the MCU
    PRCMCC3200MCUInit();

    // Setup the pinmux.
    //
    // See http://www.ti.com/product/CC3200MOD/datasheet/terminal_configuration_and_functions
    // for mapping of Module pin numbers to Device pin numbers.

    //
    // Enable Peripheral Clocks 
    //
    MAP_PRCMPeripheralClkEnable(PRCM_GPIOA1, PRCM_RUN_MODE_CLK);
    MAP_PRCMPeripheralClkEnable(PRCM_UARTA0, PRCM_RUN_MODE_CLK);

    //
    // Configure PIN_64 (GPIO9) for GPIOOutput - RED LED
    //
    MAP_PinTypeGPIO(PIN_64, PIN_MODE_0, false);
    MAP_GPIODirModeSet(GPIOA1_BASE, 0x2, GPIO_DIR_MODE_OUT);

    //
    // Configure PIN_01 (GPIO10) for GPIOOutput - ORANGE LED
    //
    MAP_PinTypeGPIO(PIN_01, PIN_MODE_0, false);
    MAP_GPIODirModeSet(GPIOA1_BASE, 0x4, GPIO_DIR_MODE_OUT);

    //
    // Configure PIN_02 (GPIO11) for GPIOOutput - GREEN LED
    //
    MAP_PinTypeGPIO(PIN_02, PIN_MODE_0, false);
    MAP_GPIODirModeSet(GPIOA1_BASE, 0x8, GPIO_DIR_MODE_OUT);

    //
    // Configure PIN_55 (GPIO1) for UART0 UART0_TX
    //
    MAP_PinTypeUART(PIN_55, PIN_MODE_3);

    //
    // Configure PIN_57 (GPIO2) for UART0 UART0_RX
    //
    MAP_PinTypeUART(PIN_57, PIN_MODE_3);
}

void console_putchar(char ch) {
    if (ch == '\n') {
        MAP_UARTCharPut(CONSOLE, '\r');
    }
    MAP_UARTCharPut(CONSOLE, ch);
}
