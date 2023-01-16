use crate::dsl::device::{
  DeviceConfig, DeviceConfigEnum
};
use super::traits::CodeGenerator;
use genco::prelude::{rust, quote};
use super::comments::{Comment, DocComment};
use super::devices::esp32::ESP32DeviceGenerator;

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
    let device_gen: &dyn IDeviceGenerator = &match &self.device.config {
      DeviceConfigEnum::ESP32(device) => ESP32DeviceGenerator::new(device),
    };    
    let device_name = &self.device.id;

    let peripherals_type = &format!("{}Peripherals", device_name);
    let device_var = "device_peripherals";    

    let peripheral_gens = device_gen.peripheral_generators();

    Ok(quote! {
      $(device_gen.generate_imports()?)
      $(DocComment([format!("Device {}", self.device.id)]))$['\r']
      struct $(peripherals_type)<'a> {$['\r']
        __marker: std::marker::PhantomData<&'a ()>,
        $(for (id, peripheral_gen) in &peripheral_gens => 
           $(*id): $(peripheral_gen.generate_declare()?),$['\r']
        )
      }

      impl<'a> $(peripherals_type)<'a> {
        pub fn new() -> anyhow::Result<$(peripherals_type)<'a>> {
          let $(device_var) = $(device_gen.take_peripherals()?);
          Ok($(peripherals_type) {
            __marker: std::marker::PhantomData,
            $(for (id, peripheral_gen) in &peripheral_gens => 
              $(*id): $(peripheral_gen.generate_initialize(device_var)?)?,$['\r']
            )  
          })
        }
      }
    })
  }
}


pub trait IDeviceGenerator<'a> {
  fn generate_imports(&self) -> anyhow::Result<rust::Tokens>;
  fn take_peripherals(&self) -> anyhow::Result<rust::Tokens>;
  fn peripheral_generators(&self) -> Vec<(&'a str, &'a dyn IPeripheralGenerator)>;
}

pub trait IPeripheralGenerator {
  fn generate_declare(&self) -> anyhow::Result<rust::Tokens>;
  fn generate_initialize(&self, device_var: &str) -> anyhow::Result<rust::Tokens>;
}