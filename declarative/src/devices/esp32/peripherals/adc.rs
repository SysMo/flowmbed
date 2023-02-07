use serde::{Serialize, Deserialize};
use strum::Display;
use crate::dsl::device::{PeripheralConfig, Peripheral};
use crate::gen::device::{PeripheralConfigGenerator};
use crate::util::context::{PeripheralContext};
use crate::util::context::{GenerationContext};
use genco::prelude::{rust, quote};
use super::super::IMPORTS;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnalogReader {
  pub adc: ADC,
  pub resolution: Option<AdcResolution>,
  pub calibration: Option<bool>,
  pub pin: String,
  pub attenuation: Option<AdcAttenuation>

}


#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnalogReaderMultiChannel {
  pub adc: ADC,
  pub resolution: Option<AdcResolution>,
  pub calibration: Option<bool>,
  pub channels: Vec<AnalogChannel>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct AnalogChannel {
  pub pin: String,
  pub attenuation: Option<AdcAttenuation>
}

#[derive(Debug, Serialize, Deserialize, Display)]
#[allow(non_camel_case_types)]
pub enum ADC {
  adc1, 
  adc2
}

impl ADC {
  pub fn get_type(&self) -> rust::Tokens {
    let adc = &IMPORTS.adc;
    match &self {
      ADC::adc1 => quote!($(adc)::ADC1),
      ADC::adc2 => quote!($(adc)::ADC2),
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Display, Default, Clone)]
pub enum AdcResolution {
  #[default]
  #[serde(rename = "9bit")]
  Resolution9Bit,
  #[serde(rename = "10bit")]
  Resolution10Bit,
  #[serde(rename = "11bit")]
  Resolution11Bit,
  #[serde(rename = "12bit")]
  Resolution12Bit,
  #[serde(rename = "13bit")]
  Resolution13Bit,
}

#[derive(Debug, Serialize, Deserialize, Default, Display, Clone)]
pub enum AdcAttenuation {
  #[serde(rename = "0dB")]
  #[default]
  Atten0dB,
  #[serde(rename = "2.5dB")]
  Atten2p5dB,
  #[serde(rename = "6dB")]
  Atten6dB,
  #[serde(rename = "11dB")]
  Atten11dB,
}

#[typetag::serde]
impl PeripheralConfig for AnalogReader {}

impl PeripheralConfigGenerator for AnalogReader {
  fn gen_type(&self, _: &PeripheralContext) -> anyhow::Result<rust::Tokens> {
    let gpio = &IMPORTS.gpio;
    let adc = &rust::import("esp_idf_hal", "adc");
    let atten = &self.attenuation.clone().unwrap_or_default();
    let reader = &rust::import("flowmbed_esp32::hal", "ADCReader");
    let adc_type = &self.adc.get_type();
    let pin_type = quote!(
      $(gpio)::Gpio$(&self.pin)
    );
    Ok(quote!(
      $(reader)<'a, $(adc_type), $(pin_type), $(adc)::$(atten.to_string())<$(adc_type)>>
    ))

  }

  fn gen_initialize(&self, context: &PeripheralContext) -> anyhow::Result<rust::Tokens> {
    // let gpio = &IMPORTS.gpio;
    let adc = &rust::import("esp_idf_hal", "adc");
    let reader = &rust::import("flowmbed_esp32::hal", "ADCReader");
    let calibration = &self.calibration.unwrap_or(true);
    let resolution = &self.resolution.clone().unwrap_or_default();
    let device_context = context.find_device()?;

    Ok(quote!(
      $(reader) {
        driver: $(adc)::AdcDriver::new(
          $(device_context.var_internal_periph()).$(&self.adc.to_string()), 
          &adc::config::Config::new()
            .calibration($(calibration.to_string()))
            .resolution(adc::config::Resolution::$(resolution.to_string()))
        )?,
        channel: adc::AdcChannelDriver::new(
          $(device_context.var_internal_periph()).pins.gpio$(&self.pin)
        )?
      }
    ))
  }
}

#[typetag::serde]
impl PeripheralConfig for AnalogChannel {}

impl PeripheralConfigGenerator for AnalogChannel {
  fn gen_type(&self, context: &PeripheralContext) -> anyhow::Result<rust::Tokens> {
    // use std::any::Any;
    let gpio = &IMPORTS.gpio;
    let hal = &IMPORTS.esp32hal;
    let adc = &IMPORTS.adc;
    let pin_type = quote!(
      $(gpio)::Gpio$(&self.pin)
    );

    let adc_type = context.parent_peripheral_config_as::<AnalogReaderMultiChannel>()?
      .adc.get_type();

    let attenuation = &self.attenuation.clone().unwrap_or_default();
    // let device_context = context.find_device()?;

    Ok(quote!(
      $(hal)::AnalogChannel<'a, $(pin_type), $(adc)::$(attenuation.to_string())<$(adc_type)>>
    ))

  }

  fn gen_initialize(&self, context: &PeripheralContext) -> anyhow::Result<rust::Tokens> {
    let hal = &IMPORTS.esp32hal;
    let device_context = context.find_device()?;
    Ok(quote!(
      $(hal)::AnalogChannel::new($(device_context.var_internal_periph()).pins.gpio$(&self.pin))?
    ))
  }
}


#[typetag::serde]
impl PeripheralConfig for AnalogReaderMultiChannel {}

impl PeripheralConfigGenerator for AnalogReaderMultiChannel {
  fn gen_type(&self, _: &PeripheralContext) -> anyhow::Result<rust::Tokens> {
    let hal = &IMPORTS.esp32hal;
    // let adc = &IMPORTS.adc;
    let n_channels = self.channels.len();
    Ok(quote!(
      $(hal)::AnalogReaderMultiChannel<'a, $(self.adc.get_type()), $(n_channels.to_string())>
    ))
  }

  fn gen_initialize(&self, context: &PeripheralContext) -> anyhow::Result<rust::Tokens> {
    let hal = &IMPORTS.esp32hal;
    let adc = &IMPORTS.adc;
    let adc_channel_config = "adc_channel_config";
    let device_context = context.find_device()?;
    Ok(quote!(
      let $(adc_channel_config) = $(adc)::config::Config::new()
        .calibration(true);
  
      $(hal)::AnalogReaderMultiChannel {
        driver: $(adc)::AdcDriver::new(
          $(device_context.var_internal_periph()).$(&self.adc.to_string()),
          &$(adc_channel_config)
        )?,
        channels: [
          $(for child in self.child_peripherals(context) =>
            $(device_context.var_device_periph())
              .$(context.push_peripheral(&child).long_id()).mut_ref()?,$['\r']
          )
        ]
      }
    ))
  }

  fn child_peripherals<'a>(&self, _context: &'a PeripheralContext) -> Vec<Peripheral> {    
    self.channels.iter().enumerate().map(|(i, ch)| {
      Peripheral {
        id: format!("channel_{}", i).to_owned(),
        config: Box::new((*ch).clone())
      }
    }).collect()
  }
}

