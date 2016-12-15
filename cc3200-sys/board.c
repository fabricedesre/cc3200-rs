// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

#include <stdint.h>
#include <stddef.h>

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

#include "StrPrintf.h"

extern void (* const ISR_VECTORS[])(void);
extern uint32_t _bss;
extern uint32_t _ebss;

// start is written in rust
void start(void);

typedef struct
{
  volatile uint32_t DHCSR;  /*!< Offset: 0x000 (R/W)  Debug Halting Control and Status Register */
  volatile uint32_t DCRSR;  /*!< Offset: 0x004 ( /W)  Debug Core Register Selector Register */
  volatile uint32_t DCRDR;  /*!< Offset: 0x008 (R/W)  Debug Core Register Data Register */
  volatile uint32_t DEMCR;  /*!< Offset: 0x00C (R/W)  Debug Exception and Monitor Control Register */
} CoreDebug_Type;

#define CoreDebug_BASE      (0xE000EDF0UL)                            /*!< Core Debug Base Address */
#define CoreDebug           ((CoreDebug_Type *)     CoreDebug_BASE)   /*!< Core Debug configuration struct */

int is_debugger_running(void) {
    return (CoreDebug->DHCSR & 1) != 0; // Bit 0 is DEBUGEN
}

void reset(void)
{
    if (is_debugger_running()) {
        __asm volatile ("bkpt #0");
    }
    PRCMMCUReset(1);
}

__attribute__((naked))
void isr_reset(void)
{
    // On the cc3200 there is no internal flash, so we don't need to copy
    // initialized data from ROM into RAM, the bootloader does it for us
    // when it loads the program into RAM.

    //
    // Zero fill the bss segment. Note that since our stack is also inside
    // the .bss section, we have to clear the stack before calling any
    // functions that might need a return address to be stored on the stack.
    //
    // Note: Not sure why yet, but FreeRTOS doesn't seem to like the stack
    //       being at the end of RAM. It might be because the stack isn't
    //       cleared, or it may be because its accessing the word at the
    //       end of the stack.
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

    //
    // Call the application's entry point.
    //
    start();
}

#define USE_I2C     1

void board_init(void) {

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
    MAP_PRCMPeripheralClkEnable(PRCM_I2CA0, PRCM_RUN_MODE_CLK);

    //
    // Configure PIN_64 (GPIO9) for GPIOOutput - RED LED
    //
    MAP_PinTypeGPIO(PIN_64, PIN_MODE_0, false);
    MAP_GPIODirModeSet(GPIOA1_BASE, 0x2, GPIO_DIR_MODE_OUT);

#if USE_I2C
    //
    // Configure PIN_01 for I2C0 I2C_SCL
    //
    MAP_PinTypeI2C(PIN_01, PIN_MODE_1);

    //
    // Configure PIN_02 for I2C0 I2C_SDA
    //
    MAP_PinTypeI2C(PIN_02, PIN_MODE_1);
#else
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
#endif

    //
    // Configure PIN_55 (GPIO1) for UART0 UART0_TX
    //
    MAP_PinTypeUART(PIN_55, PIN_MODE_3);

    //
    // Configure PIN_57 (GPIO2) for UART0 UART0_RX
    //
    MAP_PinTypeUART(PIN_57, PIN_MODE_3);

    InitTerm();
    ClearTerm();
}

void console_putchar(char ch) {
    if (ch == '\n') {
        MAP_UARTCharPut(CONSOLE, '\r');
    }
    MAP_UARTCharPut(CONSOLE, ch);
}

void console_puts(const char *s) {
    while (*s != '\0') {
        console_putchar(*s++);
    }
}

static int strxprintf_func(void *param, int ch) {
    console_putchar(ch);
    return 1;
}

int Report(const char *pcFormat, ...) __attribute__((alias ("console_printf")));

int console_printf(const char *fmt, ...) {
    va_list  args;
    va_start(args, fmt);
    int rc = vStrXPrintf(strxprintf_func, NULL, fmt, args);
    va_end(args);
    return rc;
}

void board_test(void) {
    console_printf("Test: %d, %s\n", 42, "string");
}

void vApplicationMallocFailedHook()
{
    console_puts("vApplicationMallocFailedHook\n");

    //Handle Memory Allocation Errors
    while(1)
    {
    }
}

void vApplicationStackOverflowHook( void *pxTask,
                                   signed char *pcTaskName)
{
    console_printf("vApplicationStackOverflowHook for task '%s'\n", pcTaskName);

    //Handle FreeRTOS Stack Overflow
    while(1)
    {
    }
}

void
vApplicationIdleHook( void)
{
    //Handle Idle Hook for Profiling, Power Management etc
}

void *memset(void *s, int c, size_t n) {
    if (c == 0 && ((uintptr_t)s & 3) == 0) {
        // aligned store of 0
        uint32_t *s32 = s;
        for (size_t i = n >> 2; i > 0; i--) {
            *s32++ = 0;
        }
        if (n & 2) {
            *((uint16_t*)s32) = 0;
            s32 = (uint32_t*)((uint16_t*)s32 + 1);
        }
        if (n & 1) {
            *((uint8_t*)s32) = 0;
        }
    } else {
        uint8_t *s2 = s;
        for (; n > 0; n--) {
            *s2++ = c;
        }
    }
    return s;
}

static char *fmt_hex(uint32_t val, char *buf) {
    const char *hexDig = "0123456789abcdef";

    buf[0] = hexDig[(val >> 28) & 0x0f];
    buf[1] = hexDig[(val >> 24) & 0x0f];
    buf[2] = hexDig[(val >> 20) & 0x0f];
    buf[3] = hexDig[(val >> 16) & 0x0f];
    buf[4] = hexDig[(val >> 12) & 0x0f];
    buf[5] = hexDig[(val >>  8) & 0x0f];
    buf[6] = hexDig[(val >>  4) & 0x0f];
    buf[7] = hexDig[(val >>  0) & 0x0f];
    buf[8] = '\0';

    return buf;
}

void print_reg(const char *label, uint32_t val) {
    char hexStr[9];
    console_puts(label);
    console_puts(fmt_hex(val, hexStr));
    console_putchar('\n');
}

// From ARM CMSIS
typedef struct
{
  volatile const    uint32_t CPUID;         /*!< Offset: 0x000 (R/ )  CPUID Base Register */
  volatile          uint32_t ICSR;          /*!< Offset: 0x004 (R/W)  Interrupt Control and State Register */
  volatile          uint32_t VTOR;          /*!< Offset: 0x008 (R/W)  Vector Table Offset Register */
  volatile          uint32_t AIRCR;         /*!< Offset: 0x00C (R/W)  Application Interrupt and Reset Control Register */
  volatile          uint32_t SCR;           /*!< Offset: 0x010 (R/W)  System Control Register */
  volatile          uint32_t CCR;           /*!< Offset: 0x014 (R/W)  Configuration Control Register */
  volatile          uint8_t  SHP[12U];      /*!< Offset: 0x018 (R/W)  System Handlers Priority Registers (4-7, 8-11, 12-15) */
  volatile          uint32_t SHCSR;         /*!< Offset: 0x024 (R/W)  System Handler Control and State Register */
  volatile          uint32_t CFSR;          /*!< Offset: 0x028 (R/W)  Configurable Fault Status Register */
  volatile          uint32_t HFSR;          /*!< Offset: 0x02C (R/W)  HardFault Status Register */
  volatile          uint32_t DFSR;          /*!< Offset: 0x030 (R/W)  Debug Fault Status Register */
  volatile          uint32_t MMFAR;         /*!< Offset: 0x034 (R/W)  MemManage Fault Address Register */
  volatile          uint32_t BFAR;          /*!< Offset: 0x038 (R/W)  BusFault Address Register */
  volatile          uint32_t AFSR;          /*!< Offset: 0x03C (R/W)  Auxiliary Fault Status Register */
  volatile const    uint32_t PFR[2U];       /*!< Offset: 0x040 (R/ )  Processor Feature Register */
  volatile const    uint32_t DFR;           /*!< Offset: 0x048 (R/ )  Debug Feature Register */
  volatile const    uint32_t ADR;           /*!< Offset: 0x04C (R/ )  Auxiliary Feature Register */
  volatile const    uint32_t MMFR[4U];      /*!< Offset: 0x050 (R/ )  Memory Model Feature Register */
  volatile const    uint32_t ISAR[5U];      /*!< Offset: 0x060 (R/ )  Instruction Set Attributes Register */
                    uint32_t RESERVED0[5U];
  volatile          uint32_t CPACR;         /*!< Offset: 0x088 (R/W)  Coprocessor Access Control Register */
} SCB_Type;

#define SCS_BASE    (0xE000E000UL)          /*!< System Control Space Base Address */
#define SCB_BASE    (SCS_BASE + 0x0D00UL)   /*!< System Control Block Base Address */
#define SCB         ((SCB_Type *)SCB_BASE)  /*!< SCB configuration struct */

// The ARMv7M Architecture manual (section B.1.5.6) says that upon entry
// to an exception, that the registers will be in the following order on the
// // stack: R0, R1, R2, R3, R12, LR, PC, XPSR

typedef struct {
    uint32_t    r0, r1, r2, r3, r12, lr, pc, xpsr;
} ExceptionRegisters_t;

void isr_hardfault_c_handler(ExceptionRegisters_t *regs) {
    console_puts("HardFault\n");
    print_reg("R0    ", regs->r0);
    print_reg("R1    ", regs->r1);
    print_reg("R2    ", regs->r2);
    print_reg("R3    ", regs->r3);
    print_reg("R12   ", regs->r12);
    print_reg("LR    ", regs->lr);
    print_reg("PC    ", regs->pc);
    print_reg("XPSR  ", regs->xpsr);

    uint32_t cfsr = SCB->CFSR;

    print_reg("HFSR  ", SCB->HFSR);
    print_reg("CFSR  ", cfsr);
    if (cfsr & 0x80) {
        print_reg("MMFAR ", SCB->MMFAR);
    }
    if (cfsr & 0x8000) {
        print_reg("BFAR  ", SCB->BFAR);
    }
    /* Go to infinite loop when Hard Fault exception occurs */
    while (1) {
        ;
    }
}

// Naked functions have no compiler generated gunk, so are the best thing to
// use for asm functions.
__attribute__((naked))
void isr_hardfault(void) {

    // From the ARMv7M Architecture Reference Manual, section B.1.5.6
    // on entry to the Exception, the LR register contains, amongst other
    // things, the value of CONTROL.SPSEL. This can be found in bit 3.
    //
    // If CONTROL.SPSEL is 0, then the exception was stacked up using the
    // main stack pointer (aka MSP). If CONTROL.SPSEL is 1, then the exception
    // was stacked up using the process stack pointer (aka PSP).

    __asm volatile(
    " tst lr, #4    \n"             // Test Bit 3 to see which stack pointer we should use.
    " ite eq        \n"             // Tell the assembler that the nest 2 instructions are if-then-else
    " mrseq r0, msp \n"             // Make R0 point to main stack pointer
    " mrsne r0, psp \n"             // Make R0 point to process stack pointer
    " b isr_hardfault_c_handler\n"  // Off to C land
    );
}
