use serde::{Serialize, Deserialize};
use strum::Display;
use crate::dsl::device::{PeripheralConfig};
use crate::gen::device::PeripheralConfigGenerator;
use genco::prelude::{rust, quote};
use super::super::IMPORTS;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DigitalOutputPin {
  pub pin: String
}

#[typetag::serde]
impl PeripheralConfig for DigitalOutputPin {}

impl PeripheralConfigGenerator for DigitalOutputPin {
  fn gen_type(&self) -> anyhow::Result<rust::Tokens> {
    let gpio = &IMPORTS.gpio;
    let esp32hal = &IMPORTS.esp32hal;
    let pin_type = quote!(
      $(gpio)::Gpio$(&self.pin)
    );
    Ok(quote!(
      $(esp32hal)::DigitalOutputPin<'a, $(pin_type)>
    ))
    // Ok(quote!($(gpio)::PinDriver::<'a, $(gpio)::Gpio$(&self.pin), $(gpio)::Output>))

  }

  fn gen_initialize(&self) -> anyhow::Result<rust::Tokens> {
    let gpio = &IMPORTS.gpio;
    Ok(quote!(
      $(gpio)::PinDriver::output(peripherals.pins.gpio$(&self.pin))?
    ))
  }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DigitalInputPin {
  pub pin: String,
  #[serde(default)]
  pub pull: PullConfig,
}

#[derive(Debug, Serialize, Deserialize, Default, Display)]
#[serde(rename_all = "snake_case")]
pub enum PullConfig {
  #[default]
  Floating,
  Up,
  Down,
  UpDown,
}

#[typetag::serde]
impl PeripheralConfig for DigitalInputPin {}

impl PeripheralConfigGenerator for DigitalInputPin {
  fn gen_type(&self) -> anyhow::Result<rust::Tokens> {
    let gpio = &IMPORTS.gpio;
    let esp32hal = &IMPORTS.esp32hal;
    let pin_type = quote!(
      $(gpio)::Gpio$(&self.pin)
    );
    Ok(quote!(
      $(esp32hal)::DigitalInputPin<'a, $(pin_type)>
    ))

  }

  fn gen_initialize(&self) -> anyhow::Result<rust::Tokens> {
    let gpio = &IMPORTS.gpio;
    Ok(quote!(
      $(gpio)::PinDriver::input(peripherals.pins.gpio$(&self.pin))?
    ))
  }

  fn gen_modifiers(&self, id: &str) -> anyhow::Result<rust::Tokens> {
    let gpio = &IMPORTS.gpio;
    Ok(quote!(
      $(id).set_pull($(gpio)::Pull::$(&self.pull.to_string()))?;
    ))
  }
}