use serde::{Serialize, Deserialize};
use strum::Display;
use crate::dsl::device::{PeripheralConfig};
use crate::gen::device::{PeripheralConfigGenerator};
use crate::util::context::{PeripheralContext};
use crate::util::context::{GenerationContext};
use genco::prelude::{rust, quote};
use super::super::IMPORTS;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PwmMultiChannel {
  pub timer: PwmTimer,
  pub channels: Vec<PwmChannel>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PwmTimer {
  id: Esp32Timer,
  freq: f32
}

#[derive(Debug, Serialize, Deserialize, Display)]
#[serde(deny_unknown_fields)]
#[allow(non_camel_case_types)]
pub enum Esp32Timer {
  timer0,
  timer1,
  timer2,
  timer3
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PwmChannel {
  id: Esp32Channel,
  pin: String
}

#[derive(Debug, Serialize, Deserialize, Display)]
#[serde(deny_unknown_fields)]
#[allow(non_camel_case_types)]
pub enum Esp32Channel {
  channel0, channel1, channel2,
  channel3, channel4, channel5,
  channel6, channel7,
}

#[typetag::serde]
impl PeripheralConfig for PwmMultiChannel {}

impl PeripheralConfigGenerator for PwmMultiChannel {
  fn gen_type(&self, _: &PeripheralContext) -> anyhow::Result<rust::Tokens> {
    let esp32hal = &IMPORTS.esp32hal;
    let n_channels = self.channels.len();
    Ok(quote!(
      $(esp32hal)::PwmMultiChannel<'a, $(n_channels)>
    ))
  }

  fn gen_initialize(&self, context: &PeripheralContext) -> anyhow::Result<rust::Tokens> {
    let esp32hal = &IMPORTS.esp32hal;
    let units = &IMPORTS.units;
    let timer = self.timer.id.to_string();
    let freq = self.timer.freq.to_string();
    let var_internal_periph = context.find_device()?.var_internal_periph();

    Ok(quote!(
      $(esp32hal)::PwmMultiChannel::builder($(units)::Hertz($(freq)), $(var_internal_periph).ledc.$(timer))?
      $(for channel in &self.channels => 
        .add_channel(
          $(var_internal_periph).ledc.$(&channel.id.to_string()), 
          $(var_internal_periph).pins.gpio$(&channel.pin))?$['\r']
      )
      .build()?
    ))      
  }
}