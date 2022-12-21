use crate::dsl::device::{
  DeviceConfig, DeviceKind,
  PeripheralConfig
};
use super::traits::CodeGenerator;
use genco::prelude::{rust, quote};

use super::devices::esp32::ESP32PeripheryGenerator;

pub struct DeviceGenerator<'a> {
  pub device: &'a DeviceConfig,
}

impl<'a> DeviceGenerator<'a> {
  pub fn new(device: &'a DeviceConfig) -> DeviceGenerator<'a> {
    DeviceGenerator { 
      device
    }
  }
}

impl<'a> CodeGenerator for DeviceGenerator<'a> {
  fn generate(&self) -> anyhow::Result<rust::Tokens> {
    let peripheral_gen = match self.device.kind {
      DeviceKind::esp32 => ESP32PeripheryGenerator::new(),
    };

    let device_name = "LedSystem";
    let device_peripherals = "device_peripherals";

    Ok(quote! {
      #[doc = $(format!("\"Device {}\"", self.device.id))]$['\r']
      struct $(device_name)Peripherals<'a> {$['\r']
        $(for peripheral in &self.device.peripherals => 
          $(&peripheral.id): $(peripheral_gen.generate_declare(&peripheral)?),$['\r']
        )
      }

      impl<'a> $(device_name)Peripherals<'a> {
        pub fn new() -> $(device_name)Peripherals<'a> {
          let $(device_peripherals) = $(peripheral_gen.take_peripherals()?);
          $(device_name)Peripherals {
            $(for peripheral in &self.device.peripherals => 
              $(&peripheral.id): $(peripheral_gen.generate_initialize(&peripheral, device_peripherals)?).unwrap(),$['\r']
            )  
          }
        }
      }
    })
  }
}


pub trait PeripheryGenerator {
  fn take_peripherals(&self) -> anyhow::Result<rust::Tokens>;
  fn generate_declare(&self, peripheral: &PeripheralConfig) -> anyhow::Result<rust::Tokens>;
  fn generate_initialize(&self, peripheral: &PeripheralConfig, device_peripherals: &str) -> anyhow::Result<rust::Tokens>;
}