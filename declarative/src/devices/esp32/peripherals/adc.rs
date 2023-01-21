use serde::{Serialize, Deserialize};
use strum::Display;
use crate::dsl::device::{PeripheralConfig};
use crate::gen::device::PeripheralConfigGenerator;
use genco::prelude::{rust, quote};
use super::super::IMPORTS;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnalogInputPin {
  // pub driver: AdcDriver,
  // pub channel: AdcChannel
  pub adc: ADC,
  pub resolution: Option<AdcResolution>,
  pub calibration: Option<bool>,
  pub pin: String,
  pub attenuation: Option<AdcAttenuation>

}


// #[derive(Debug, Serialize, Deserialize)]
// pub struct AdcDriver {
//   pub adc: Option<ADC>,
//   pub resolution: Option<AdcResolution>,
//   pub calibration: Option<bool>
// }

#[derive(Debug, Serialize, Deserialize, Display)]
pub enum ADC {
  adc1, 
  adc2
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

// #[derive(Debug, Serialize, Deserialize)]
// pub struct AdcChannel {
//   pub pin: String,
//   pub attenuation: Option<AdcAttenuation>
// }

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
impl PeripheralConfig for AnalogInputPin {}

impl PeripheralConfigGenerator for AnalogInputPin {
  fn gen_type(&self) -> anyhow::Result<rust::Tokens> {
    let gpio = &IMPORTS.gpio;
    let adc = &rust::import("esp_idf_hal", "adc");
    let atten = &self.attenuation.clone().unwrap_or_default();
    let reader = &rust::import("flowmbed_esp32::hal", "ADCReader");
    let adc_type = match &self.adc {
      ADC::adc1 => "ADC1",
      ADC::adc2 => "ADC2",
    };
    let pin_type = quote!(
      $(gpio)::Gpio$(&self.pin)
    );
    Ok(quote!(
      $(reader)<'a, $(adc)::$(adc_type), $(pin_type), $(adc)::$(atten.to_string())<$(adc)::$(adc_type)>>
    ))

  }

  fn gen_initialize(&self) -> anyhow::Result<rust::Tokens> {
    let gpio = &IMPORTS.gpio;
    let adc = &rust::import("esp_idf_hal", "adc");
    let reader = &rust::import("flowmbed_esp32::hal", "ADCReader");
    let calibration = &self.calibration.unwrap_or(true);
    let resolution = &self.resolution.clone().unwrap_or_default();
    Ok(quote!(
      $(reader) {
        driver: $(adc)::AdcDriver::new(
          peripherals.$(&self.adc.to_string()), 
          &adc::config::Config::new()
            .calibration($(calibration.to_string()))
            .resolution(adc::config::Resolution::$(resolution.to_string()))
        )?,
        channel: adc::AdcChannelDriver::new(peripherals.pins.gpio$(&self.pin))?
      }
    ))
  }
}
