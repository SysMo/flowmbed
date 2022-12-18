use embedded_hal::digital::OutputPin;
// use crate::util::config_compile::{cascade, __items, __apply};
// See https://gist.github.com/alexcrichton/29b618d75cde5b57d797 for a nicer way

// cascade! {
//   if #[cfg(esp32)] {
//     pub type HalError = esp_idf_sys::EspError;
//   } else {
//     pub type HalError = anyhow::Error;
//   }
// }

#[cfg(feature = "esp32")]
pub type HalError = esp_idf_sys::EspError;
#[cfg(not(feature = "esp32"))]
pub type HalError = anyhow::Error;

pub type HalResult<T> = core::result::Result<T, HalError>;


//struct 
// pub type IOutputPin = dyn OutputPin<Error = HalError>;
pub trait TOutputPin : OutputPin<Error = HalError> {}
pub type IOutputPin<'a> = &'a mut dyn OutputPin<Error = HalError>;

// pub type IOutputPin = Box<dyn OutputPin<Error = HalError>>;

