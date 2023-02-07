use serde::{Serialize, Deserialize};
use strum::Display;
use crate::dsl::device::{PeripheralConfig};
use crate::gen::device::PeripheralConfigGenerator;
use crate::util::context::{GenerationContext, PeripheralContext};
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
  fn gen_type(&self, _: &PeripheralContext) -> anyhow::Result<rust::Tokens> {
    let gpio = &IMPORTS.gpio;
    let esp32hal = &IMPORTS.esp32hal;
    let pin_type = quote!(
      $(gpio)::Gpio$(&self.pin)
    );
    Ok(quote!(
      $(esp32hal)::DigitalOutputPin<'a, $(pin_type)>
    ))
  }

  fn gen_initialize(&self, context: &PeripheralContext) -> anyhow::Result<rust::Tokens> {
    let gpio = &IMPORTS.gpio;
    let var_internal_periph = context.find_device()?.var_internal_periph();
    Ok(quote!(
      $(gpio)::PinDriver::output($(var_internal_periph).pins.gpio$(&self.pin))?.into()
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
  fn gen_type(&self, _context: &PeripheralContext) -> anyhow::Result<rust::Tokens> {
    let gpio = &IMPORTS.gpio;
    let esp32hal = &IMPORTS.esp32hal;
    let pin_type = quote!(
      $(gpio)::Gpio$(&self.pin)
    );
    Ok(quote!(
      $(esp32hal)::DigitalInputPin<'a, $(pin_type)>
    ))

  }

  fn gen_initialize(&self, context: &PeripheralContext) -> anyhow::Result<rust::Tokens> {
    let gpio = &IMPORTS.gpio;
    let var_internal_periph = context.find_device()?.var_internal_periph();
    Ok(quote!({
      let mut driver = 
        $(gpio)::PinDriver::input($(var_internal_periph).pins.gpio$(&self.pin))?;
      driver.set_pull($(gpio)::Pull::$(&self.pull.to_string()))?;
      driver.into()
      
    }))
  }

}