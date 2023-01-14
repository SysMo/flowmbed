use crate::dsl::device::{
  DeviceConfig, DeviceKind,
  PeripheralConfig
};
use super::traits::CodeGenerator;
use genco::prelude::{rust, quote};
use super::comments::{Comment, DocComment};
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

    let device_name = &self.device.id;
    let peripherals_type = &format!("{}Peripherals", device_name);
    let device_var = "device_peripherals";    
    Ok(quote! {
      $(peripheral_gen.generate_imports()?)
      $(DocComment([format!("Device {}", self.device.id)]))$['\r']
      struct $(peripherals_type)<'a> {$['\r']
        __marker: std::marker::PhantomData<&'a ()>,
        $(for peripheral in &self.device.peripherals => 
          $(&peripheral.id): $(peripheral_gen.generate_declare(&peripheral)?),$['\r']
        )
      }

      impl<'a> $(peripherals_type)<'a> {
        pub fn new() -> anyhow::Result<$(peripherals_type)<'a>> {
          let $(device_var) = $(peripheral_gen.take_peripherals()?);
          Ok($(peripherals_type) {
            __marker: std::marker::PhantomData,
            $(for peripheral in &self.device.peripherals => 
              $(&peripheral.id): $(peripheral_gen.generate_initialize(&peripheral, device_var)?)?,$['\r']
            )  
          })
        }
      }
    })
  }
}


pub trait PeripheryGenerator {
  fn take_peripherals(&self) -> anyhow::Result<rust::Tokens>;
  fn generate_imports(&self) -> anyhow::Result<rust::Tokens>;
  fn generate_declare(&self, peripheral: &PeripheralConfig) -> anyhow::Result<rust::Tokens>;
  fn generate_initialize(&self, peripheral: &PeripheralConfig, device_var: &str) -> anyhow::Result<rust::Tokens>;
}