use crate::dsl::device::{PeripheralConfig, PeripheralConfigEnum};
use super::super::device::PeripheryGenerator;
use genco::prelude::{rust, quote};

#[allow(non_snake_case)]
pub struct ESP32PeripheryGenerator {
  gpio: rust::Import,
  ledc: rust::Import,
  Peripherals: rust::Import,
}

impl ESP32PeripheryGenerator {
  pub fn new() -> ESP32PeripheryGenerator {
    ESP32PeripheryGenerator {
      gpio: rust::import("esp32_hal", "gpio"),
      ledc: rust::import("esp32_hal", "ledc"),
      Peripherals: rust::import("esp32_hal::peripherals", "Peripherals")
    }
  }
}

impl PeripheryGenerator for ESP32PeripheryGenerator {

  fn generate_imports(&self) -> anyhow::Result<rust::Tokens> {
    Ok(quote!(
      use esp_idf_hal::prelude::*;
    ))
  }

  fn take_peripherals(&self) -> anyhow::Result<rust::Tokens> {
    Ok(quote!(
      match $(&self.Peripherals)::take() {
        Some(x) => x,
        None => anyhow::bail!("Peripherals already taken!")
      }
    ))
  }

  fn generate_declare(&self, peripheral: &PeripheralConfig) -> anyhow::Result<rust::Tokens> {
    let gpio = &self.gpio;
    let output = match &peripheral.conf {
      PeripheralConfigEnum::DigitalInput(input) => 
        quote! {$(gpio)::PinDriver::<'a, $(gpio)::Gpio$(input.pin), $(gpio)::Input>},
      PeripheralConfigEnum::DigitalOutput(output) => 
        quote! {$(gpio)::PinDriver::<'a, $(gpio)::Gpio$(output.pin), $(gpio)::Output>},
      PeripheralConfigEnum::PWMOutput(output) => 
        quote! {$(&self.ledc)::LedcDriver::<'a>},
    };

    Ok(output)
  }

  fn generate_initialize(&self, peripheral: &PeripheralConfig, device_var: &str) -> anyhow::Result<rust::Tokens> {
    let gpio = &self.gpio;
    let output = match &peripheral.conf {
      PeripheralConfigEnum::DigitalInput(input) => 
        quote! {
          $(gpio)::PinDriver::input($(device_var).pins.gpio$(input.pin))
        },
      PeripheralConfigEnum::DigitalOutput(output) => 
        quote! {
          $(gpio)::PinDriver::output($(device_var).pins.gpio$(output.pin))
        },
      PeripheralConfigEnum::PWMOutput(output) => 
        quote! {
          $(&self.ledc)::LedcDriver::new(
            $(device_var).ledc.$(&output.channel),
            $(&self.ledc)::LedcTimerDriver::new(
              $(device_var).ledc.$(&output.timer),
                &$(&self.ledc)::config::TimerConfig::new().frequency($(output.freq.to_string())_u32.kHz().into()),
            )?,
            $(device_var).pins.gpio$(&output.pin.to_string()),
          )
        },
      };

    Ok(output)
  }

}