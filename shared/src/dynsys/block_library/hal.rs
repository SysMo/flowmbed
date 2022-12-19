pub use crate::cfg_device::{HalError, HalResult, esp32_hal};

use embedded_hal::digital::OutputPin;


pub type IOutputPin<'a> = &'a mut dyn OutputPin<Error = HalError>;

