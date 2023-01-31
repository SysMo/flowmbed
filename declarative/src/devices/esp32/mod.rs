pub mod peripherals;

use serde::{Serialize, Deserialize};
use genco::prelude::{rust, quote};
use crate::dsl::device::{DeviceConfig, Peripheral};
use crate::gen::device::DeviceConfigGenerator;
use crate::util::GenerationContext;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ESP32DeviceConfig {
  pub peripherals: Vec<Peripheral>
}

#[typetag::serde(name = "ESP32")]
impl DeviceConfig for ESP32DeviceConfig {
  fn peripherals<'a>(&'a self) -> &'a Vec<Peripheral> {
    &self.peripherals
  }

}

// =========== Generation ===========

use lazy_static::lazy_static;

#[allow(non_snake_case)]
#[allow(dead_code)]
struct Imports {
  pub gpio: rust::Import,
  pub adc: rust::Import,
  pub ledc: rust::Import,
  pub units: rust::Import,
  pub Peripherals: rust::Import,  
  pub esp32hal: rust::Import,
}

lazy_static! {
  static ref IMPORTS: Imports = Imports {
    gpio: rust::import("esp_idf_hal", "gpio"),
    adc: rust::import("esp_idf_hal", "adc"),
    ledc: rust::import("esp_idf_hal", "ledc"),
    units: rust::import("esp_idf_hal", "units"),
    Peripherals: rust::import("esp_idf_hal::peripherals", "Peripherals"),
    esp32hal: rust::import("flowmbed_esp32", "hal").with_alias("esp32hal")
  };  
}

impl DeviceConfigGenerator for ESP32DeviceConfig {
  fn gen_imports(&self, _: &GenerationContext) -> anyhow::Result<rust::Tokens> {
    Ok(quote!(
      // #[allow(unused_imports)]
      // use esp_idf_hal::prelude::*;
    ))
  }

  fn gen_take_peripherals(&self, _: &GenerationContext) -> anyhow::Result<rust::Tokens> {
    let peripherals = &IMPORTS.Peripherals;
    Ok(quote!(
      match $(peripherals)::take() {
        Some(x) => x,
        None => anyhow::bail!("Peripherals already taken!")
      }
    ))
  }
}