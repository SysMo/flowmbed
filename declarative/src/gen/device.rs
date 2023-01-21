use crate::dsl::device::{
  Device, DeviceConfig, Peripheral, PeripheralConfig
};
use super::traits::CodeGenerator;
use genco::prelude::{rust, quote};
use super::comments::{Comment, DocComment};
use convert_case::{Case, Casing};

pub struct DeviceGenerator<'a> {
  pub device: &'a Device,
}

impl<'a> DeviceGenerator<'a> {
  pub fn new(device: &'a Device) -> Self {
    DeviceGenerator { 
      device
    }
  }
}

impl<'a> CodeGenerator for DeviceGenerator<'a> {
  fn generate(&self) -> anyhow::Result<rust::Tokens> {
    let device_name = &self.device.id;

    let device_struct = &format!("{}Device",
      device_name.from_case(Case::Snake).to_case(Case::UpperCamel));


    let peripherals_struct = &format!("{}Peripherals",
      device_name.from_case(Case::Snake).to_case(Case::UpperCamel));
      
    let conf_gen = self.device.gen();

    let peripherals = self.device.config.peripherals();

    Ok(quote!(
      $(conf_gen.gen_imports()?)

      struct $(peripherals_struct)<'a> {
        __marker: std::marker::PhantomData<&'a ()>,
        $(for peripheral in peripherals =>
          $(&peripheral.id): $(peripheral.gen().gen_type()?),$['\r']
        )  
      }

      impl<'a> $(peripherals_struct)<'a> {
        pub fn new() -> anyhow::Result<$(peripherals_struct)<'a>> {
          let peripherals = $(conf_gen.gen_take_peripherals()?);
          Ok($(peripherals_struct) {
            __marker: std::marker::PhantomData,
            $(for peripheral in peripherals =>
              $(&peripheral.id): $(peripheral.gen().gen_initialize()?),$['\r']
            )  
          })
        }
      }


    ))
  }
}

// let peripheral_gens = self.device.config
//   .peripherals();

// Ok(quote! {
//   $(device_gen.generate_imports()?)
//   $(DocComment([format!("Device {}", self.device.id)]))$['\r']
//   struct $(peripherals_type)<'a> {$['\r']
//     __marker: std::marker::PhantomData<&'a ()>,
//     $(for (id, peripheral_gen) in &peripheral_gens => 
//        $(*id): $(peripheral_gen.generate_declare()?),$['\r']
//     )
//   }

//   impl<'a> $(peripherals_type)<'a> {
//     pub fn new() -> anyhow::Result<$(peripherals_type)<'a>> {
//       let $(device_var) = $(device_gen.take_peripherals()?);
//       Ok($(peripherals_type) {
//         __marker: std::marker::PhantomData,
//         $(for (id, peripheral_gen) in &peripheral_gens => 
//           $(*id): $(peripheral_gen.generate_initialize(device_var)?)?,$['\r']
//         )  
//       })
//     }
//   }
// })

pub struct PeripheralGenerator<'a> {
  pub peripheral: &'a Peripheral,
}

impl<'a> PeripheralGenerator<'a> {
  pub fn new(peripheral: &'a Peripheral) -> Self {
    PeripheralGenerator { 
      peripheral
    }
  }

}


pub trait DeviceConfigGenerator {
  fn gen_imports(&self) -> anyhow::Result<rust::Tokens>;
  fn gen_take_peripherals(&self) -> anyhow::Result<rust::Tokens>;
}

pub trait PeripheralConfigGenerator {
  fn gen_type(&self) -> anyhow::Result<rust::Tokens>;
  fn gen_initialize(&self) -> anyhow::Result<rust::Tokens>;
}