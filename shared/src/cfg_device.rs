use cfg_if::cfg_if;

cfg_if!{
  if #[cfg(feature = "esp32")] {
    pub type HalError = esp_idf_sys::EspError;
    pub use esp_idf_hal as esp32_hal;  
  } else {
    pub type HalError = anyhow::Error;
    pub use esp32_virtual_hal as esp32_hal;  
  }
}

pub type HalResult<T> = core::result::Result<T, HalError>;

cfg_if!{
  if #[cfg(feature = "esp32")] {
    pub fn config_logger() {
      // Bind the log crate to the ESP Logging facilities
      esp_idf_svc::log::EspLogger::initialize_default();
    }
  } else {
    pub fn config_logger() {
        // Bind the log crate to the simple logger facilities
      simple_logger::SimpleLogger::new().env().init().unwrap();
    }
  }
}