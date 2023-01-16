use crate::dsl::device::{DeviceConfig};
use crate::dsl::devices::esp32;
use crate::gen::device::{IDeviceGenerator, IPeripheralGenerator};
use genco::prelude::{rust, quote};
use lazy_static::lazy_static;


#[allow(non_snake_case)]
struct Imports {
  pub gpio: rust::Import,
  ledc: rust::Import,
  Peripherals: rust::Import,
}

lazy_static! {
  static ref IMPORTS: Imports = Imports {
    gpio: rust::import("esp32_hal", "gpio"),
    ledc: rust::import("esp32_hal", "ledc"),
    Peripherals: rust::import("esp32_hal::peripherals", "Peripherals"),
  };  
}


pub struct ESP32DeviceGenerator<'a> {
  pub device: &'a esp32::ESP32Device,
}

impl<'a> ESP32DeviceGenerator<'a> {
  pub fn new(device: &'a esp32::ESP32Device) -> ESP32DeviceGenerator<'a> {
    ESP32DeviceGenerator {
      device,
    }
  }
}

impl<'a> IDeviceGenerator<'a> for ESP32DeviceGenerator<'a> {
  fn generate_imports(&self) -> anyhow::Result<rust::Tokens> {
    Ok(quote!(
      use esp_idf_hal::prelude::*;
    ))
  }

  fn take_peripherals(&self) -> anyhow::Result<rust::Tokens> {
    let peripherals = &IMPORTS.Peripherals;
    Ok(quote!(
      match $(peripherals)::take() {
        Some(x) => x,
        None => anyhow::bail!("Peripherals already taken!")
      }
    ))
  }

  fn peripheral_generators(&self) -> Vec<(&'a str, &'a dyn IPeripheralGenerator)> {
    self.device.peripherals.iter().map(|p| {
      let peripheral_gen: &dyn IPeripheralGenerator= match p.config {
        esp32::PeripheralConfigEnum::DigitalInputPin(ref x) 
          => x,
        esp32::PeripheralConfigEnum::DigitalOutputPin(ref x) 
          => x,
        esp32::PeripheralConfigEnum::PwmPin(ref x) 
          => x,        
      };
      (&p.id as &str, peripheral_gen)
    }
    ).collect()
  }
}

impl IPeripheralGenerator for esp32::DigitalInputPinConfig {
  fn generate_declare(&self) -> anyhow::Result<rust::Tokens> {
    let gpio = &IMPORTS.gpio;
    Ok(quote! {
      $(gpio)::PinDriver::<'a, $(gpio)::Gpio$(self.pin), $(gpio)::Input>
    })
  }

  fn generate_initialize(&self, device_var: &str) -> anyhow::Result<rust::Tokens> {
    let gpio = &IMPORTS.gpio;
    Ok(quote! {
      $(gpio)::PinDriver::input($(device_var).pins.gpio$(self.pin))
    })
  }
}

impl IPeripheralGenerator for esp32::DigitalOutputPinConfig {
  fn generate_declare(&self) -> anyhow::Result<rust::Tokens> {
    let gpio = &IMPORTS.gpio;
    Ok(quote! {
      $(gpio)::PinDriver::<'a, $(gpio)::Gpio$(self.pin), $(gpio)::Output>
    })
  }

  fn generate_initialize(&self, device_var: &str) -> anyhow::Result<rust::Tokens> {
    let gpio = &IMPORTS.gpio;
    Ok(quote! {
      $(gpio)::PinDriver::output($(device_var).pins.gpio$(self.pin))
    })
  }
}

impl IPeripheralGenerator for esp32::PwmPinConfig {
  fn generate_declare(&self) -> anyhow::Result<rust::Tokens> {
    let gpio = &IMPORTS.gpio;
    let ledc = &IMPORTS.ledc;
    Ok(quote! {
      $(ledc)::LedcDriver::<'a>
    })
  }

  fn generate_initialize(&self, device_var: &str) -> anyhow::Result<rust::Tokens> {
    let gpio = &IMPORTS.gpio;
    let ledc = &IMPORTS.ledc;
    Ok(quote! {
      $(ledc)::LedcDriver::new(
        $(device_var).ledc.$(&self.channel),
        $(ledc)::LedcTimerDriver::new(
          $(device_var).ledc.$(&self.timer),
            &$(ledc)::config::TimerConfig::new().frequency($(self.freq.to_string())_u32.Hz().into()),
        )?,
        $(device_var).pins.gpio$(&self.pin.to_string()),
      )
    })
  }
}