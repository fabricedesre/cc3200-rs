// We won't use the standard library because it requires OS abstractions like threads and files and
// those are not available in this platform.
#![no_std]

#[allow(non_camel_case_types)]
pub enum LedEnum {
  NO_LED  = 0x0,
  LED1 = 0x1, // RED LED D7/GP9/Pin64
  LED2 = 0x2, // ORANGE LED D6/GP10/Pin1
  LED3 = 0x4  // GREEN LED D5/GP11/Pin2
}


#[allow(non_camel_case_types)]
pub enum LedName {
  NO_LED_IND = 0,
  MCU_SENDING_DATA_IND,
  MCU_ASSOCIATED_IND, /* Device associated to an AP */
  MCU_IP_ALLOC_IND, /* Device acquired an IP */
  MCU_SERVER_INIT_IND, /* Device connected to remote server */
  MCU_CLIENT_CONNECTED_IND, /* Any client connects to device */
  MCU_ON_IND,              /* Device SLHost invoked successfully */
  MCU_EXECUTE_SUCCESS_IND, /* Task executed sucessfully */
  MCU_EXECUTE_FAIL_IND, /* Task execution failed */
  MCU_RED_LED_GPIO,	/* GP09 for LED RED as per LP 3.0 */
  MCU_ORANGE_LED_GPIO,/* GP10 for LED ORANGE as per LP 3.0 */
  MCU_GREEN_LED_GPIO, /* GP11 for LED GREEN as per LP 3.0 */
  MCU_ALL_LED_IND
}

pub use self::{
  UtilsDelay as MAP_UtilsDelay,
};

extern {
  pub fn board_init();

  pub fn GPIO_IF_LedConfigure(pins: u8);
  pub fn GPIO_IF_LedOff(ledNum: i8);
  pub fn GPIO_IF_LedOn(ledNum: i8);

  pub fn UtilsDelay(loops: u32);
}

