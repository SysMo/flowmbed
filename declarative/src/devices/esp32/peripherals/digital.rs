use serde::{Serialize, Deserialize};
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
    Ok(quote!($(gpio)::PinDriver::<'a, $(gpio)::Gpio$(&self.pin), $(gpio)::Output>))

  }

  fn gen_initialize(&self) -> anyhow::Result<rust::Tokens> {
    let gpio = &IMPORTS.gpio;
    Ok(quote!($(gpio)::PinDriver::output(peripherals.pins.gpio$(&self.pin))?))
  }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DigitalInputPin {
  pub pin: String
}

#[typetag::serde]
impl PeripheralConfig for DigitalInputPin {}

impl PeripheralConfigGenerator for DigitalInputPin {
  fn gen_type(&self) -> anyhow::Result<rust::Tokens> {
    let gpio = &IMPORTS.gpio;
    Ok(quote!($(gpio)::PinDriver::<'a, $(gpio)::Gpio$(&self.pin), $(gpio)::Input>))

  }

  fn gen_initialize(&self) -> anyhow::Result<rust::Tokens> {
    let gpio = &IMPORTS.gpio;
    Ok(quote!($(gpio)::PinDriver::input(peripherals.pins.gpio$(&self.pin))?))
  }
}