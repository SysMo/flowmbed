pub use crate::cfg_device::{HalError, HalResult, esp32_hal};

use embedded_hal::digital;
use embedded_hal_0_2 as eh_02;


pub type OutputPin<'a> = &'a mut dyn digital::OutputPin<Error = HalError>;

pub type PwmPin<'a> = &'a mut dyn eh_02::PwmPin<Duty = f64>;

